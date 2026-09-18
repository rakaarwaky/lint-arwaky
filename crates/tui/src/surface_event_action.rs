use crate::surface_lint_action::SurfaceLintExecutor;
use shared::common::FilePath;
use shared::tui::{ConfirmState, LintExecutionResult, ScanUpdate};

use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::tui::TuiEvent;
use shared::tui::{AppState, PanelFocus, PreviewMode};
use std::sync::Arc;

// PURPOSE: Surface-layer action handler — the central state machine for TUI events.
// Translates every TuiEvent into a state mutation or I/O operation (filesystem/lint).
// Calls the surface lint executor and filesystem aggregate directly (no abstraction).
// This is the largest single file in the TUI crate; it owns all event→action mappings.

use crate::utility_file_system;

// ─── Block 1: Struct Definition ───────────────────────────

/// SurfaceActionHandler — pure state machine for TUI interaction.
/// Owns the surface lint executor, bridging UI events to backend operations.
/// Filesystem operations use direct utility calls instead of protocol ports.
pub struct SurfaceActionHandler {
    lint_port: Arc<SurfaceLintExecutor>,
    filesystem: Arc<dyn IFilesystemAggregate>,
}

// ─── Block 2: Background Task Methods ─────────────────────

impl SurfaceActionHandler {
    pub fn start_scan(
        &self,
        state: &mut AppState,
    ) -> Option<std::sync::mpsc::Receiver<ScanUpdate>> {
        // Guard: don't start a second scan while one is running.
        if state.scanning {
            return None;
        }
        let path = state.selected_path();
        state.set_scanning(true, "Starting scan...".to_string(), 0);
        state.set_status(format!("Scanning {}...", path));
        // Reset preview to show scan output when complete.
        state.preview_text.clear();
        state.preview_scroll = 0;

        let lint_port = self.lint_port.clone();
        let cancel = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        state.scan_cancel = Some(cancel.clone());
        let (tx, rx) = std::sync::mpsc::sync_channel(16);
        std::thread::spawn(move || {
            let _ = tx.send(ScanUpdate::Progress {
                phase: "Running scan...".to_string(),
                done: 0,
                total: 0,
            });
            // User requested cancellation (Esc) while the scan was in flight.
            if cancel.load(std::sync::atomic::Ordering::Relaxed) {
                let _ = tx.send(ScanUpdate::Cancelled);
                return;
            }
            let result = lint_port.scan(&path);
            let _ = tx.send(ScanUpdate::Complete {
                output: result.output,
                violation_count: result.violation_count,
                success: result.success,
            });
        });

        Some(rx)
    }

    pub fn poll_scan(&self, state: &mut AppState, rx: &std::sync::mpsc::Receiver<ScanUpdate>) {
        while let Ok(update) = rx.try_recv() {
            match update {
                ScanUpdate::Progress { phase, done, total } => {
                    state.update_scan_progress(phase, done, total);
                }
                ScanUpdate::Complete {
                    output,
                    violation_count,
                    success,
                } => {
                    state.finish_scan(violation_count);
                    state.preview_text = output;
                    state.violation_count = violation_count;
                    state.preview_scroll = 0;
                    state.preview_mode = PreviewMode::LintResults;
                    let status = if success { "Done" } else { "Error" };
                    state.set_status(format!(
                        "{}: {} | {} violations",
                        status,
                        state.selected_path(),
                        violation_count
                    ));
                }
                ScanUpdate::Cancelled => {
                    state.finish_scan(0);
                    state.scanning = false;
                    state.scan_cancel = None;
                    state.preview_mode = PreviewMode::ActionOutput;
                    state.preview_scroll = 0;
                    state.set_status("Scan cancelled");
                }
            }
        }
    }

    /// Start a long-running global action (install/doctor/init/mcp-config) on a
    /// background thread so the event loop keeps pumping events (50 ms poll cycle).
    /// The result receiver is stored on `state.action_result_rx`; returns `false`
    /// when an action is already in flight.
    pub fn start_background_action(
        &self,
        state: &mut AppState,
        label: &str,
        action: Box<dyn FnOnce(&SurfaceLintExecutor) -> LintExecutionResult + Send>,
    ) -> bool {
        if state.action_pending {
            return false;
        }
        state.action_pending = true;
        state.set_status(format!("Running {label}..."));
        let lint_port = self.lint_port.clone();
        let (tx, rx) = std::sync::mpsc::sync_channel(4);
        std::thread::spawn(move || {
            let result = action(lint_port.as_ref());
            let _ = tx.send(result);
        });
        state.action_result_rx = Some(rx);
        true
    }

    /// Poll the background global-action receiver (non-blocking) and apply its result.
    pub fn poll_background_action(
        &self,
        state: &mut AppState,
        rx: &std::sync::mpsc::Receiver<LintExecutionResult>,
    ) {
        if let Ok(result) = rx.try_recv() {
            state.action_pending = false;
            state.preview_text = result.output;
            state.violation_count = result.violation_count;
            state.preview_scroll = 0;
            state.preview_mode = PreviewMode::ActionOutput;
            let status = if result.success { "Done" } else { "Error" };
            state.set_status(status);
        }
    }

    /// Poll the stored `action_result_rx` (if any) while an action is pending.
    /// Clears `action_pending` when no receiver is left (already reaped).
    pub fn poll_pending_background_action(&self, state: &mut AppState) {
        let Some(rx) = state.action_result_rx.take() else {
            state.action_pending = false;
            return;
        };
        self.poll_background_action(state, &rx);
        state.action_result_rx = Some(rx);
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl SurfaceActionHandler {
    pub fn new(
        lint_port: Arc<SurfaceLintExecutor>,
        filesystem: Arc<dyn IFilesystemAggregate>,
    ) -> Self {
        Self {
            lint_port,
            filesystem,
        }
    }

    /// Main event dispatch — maps every TuiEvent variant to a concrete action.
    /// Categories: navigation, focus cycling, search, path dialog, lint actions, mouse.
    pub fn handle(&self, state: &mut AppState, event: TuiEvent) {
        match event {
            // ---- Navigation: list selection or preview scrolling based on focus ----
            TuiEvent::MoveDown => {
                if state.panel_focus == PanelFocus::Preview {
                    state.preview_scroll = state.preview_scroll.saturating_add(3);
                    state.preview_scroll = state.preview_scroll.min(self.max_preview_scroll(state));
                } else {
                    state.select_next();
                    self.load_preview(state);
                }
            }
            TuiEvent::MoveUp => {
                if state.panel_focus == PanelFocus::Preview {
                    state.preview_scroll = state.preview_scroll.saturating_sub(3);
                } else {
                    state.select_prev();
                    self.load_preview(state);
                }
            }
            TuiEvent::MoveTop => {
                if state.panel_focus == PanelFocus::Preview {
                    state.preview_scroll = 0;
                } else {
                    state.select_first();
                    self.load_preview(state);
                }
            }
            TuiEvent::MoveBottom => {
                if state.panel_focus == PanelFocus::Preview {
                    state.preview_scroll = self.max_preview_scroll(state);
                } else {
                    state.select_last();
                    self.load_preview(state);
                }
            }
            // ---- Preview panel scrolling ----
            TuiEvent::PreviewScrollUp => {
                state.preview_scroll = state.preview_scroll.saturating_sub(10);
            }
            TuiEvent::PreviewScrollDown => {
                state.preview_scroll = state.preview_scroll.saturating_add(10);
                state.preview_scroll = state.preview_scroll.min(self.max_preview_scroll(state));
            }
            // ---- Focus cycling between panels (Tree is display-only, never focused) ----
            TuiEvent::FocusNext => state.cycle_focus_forward(),
            TuiEvent::FocusPrev => state.cycle_focus_backward(),
            // ---- Directory navigation ----
            TuiEvent::NavigateBack => self.navigate_back(state),
            TuiEvent::NavigateForward => self.navigate_forward(state),
            // ---- Help overlay toggle ----
            TuiEvent::ToggleHelp => {
                state.show_help = !state.show_help;
                if state.show_help {
                    state.preview_mode = PreviewMode::HelpOverlay;
                } else {
                    // No file content preview — return to action output mode
                    state.preview_mode = PreviewMode::ActionOutput;
                }
            }
            // ---- Search mode: incremental file filtering ----
            TuiEvent::ToggleSearch => {
                state.search_mode = !state.search_mode;
                if !state.search_mode {
                    state.search_query.clear();
                }
                state.compute_filtered_indices();
            }
            TuiEvent::SearchInput(ch) => {
                if state.search_mode {
                    state.search_query.push(ch);
                    state.compute_filtered_indices();
                }
            }
            TuiEvent::SearchBackspace => {
                if state.search_mode {
                    state.search_query.pop();
                    state.compute_filtered_indices();
                }
            }
            // FR-007: Enter confirms search and exits search mode (keeps filter).
            // Esc cancels search and clears filter.
            TuiEvent::SearchConfirm => {
                state.search_mode = false;
                // Keep search_query and filtered_indices — filter persists
            }
            TuiEvent::SearchCancel => {
                state.search_mode = false;
                state.search_query.clear();
                state.compute_filtered_indices();
            }
            // ---- Lint actions that operate on the selected file/directory ----
            TuiEvent::ActionCheck => self.run_action(state, |lp, p, f| lp.check(p, f)),
            TuiEvent::ActionScan => self.run_action(state, |lp, p, _f| lp.scan(p)),
            // Plain `f` is always a dry-run fix; `F` (ActionFixLive) applies fixes.
            TuiEvent::ActionFix => {
                state.action_flags.dry_run = true;
                self.run_action(state, |lp, p, f| lp.fix(p, f))
            }
            TuiEvent::ActionFixLive => {
                state.action_flags.dry_run = false;
                self.run_action(state, |lp, p, f| lp.fix(p, f))
            }
            TuiEvent::ActionCi => self.run_action(state, |lp, p, f| lp.ci(p, f)),
            TuiEvent::ActionOrphan => self.run_action(state, |lp, p, _f| lp.orphan(p)),
            TuiEvent::ActionSecurity => self.run_action(state, |lp, p, _f| lp.security(p)),
            TuiEvent::ActionDependencies => self.run_action(state, |lp, p, _f| lp.dependencies(p)),
            // ---- Cancel an in-flight background scan (Esc while scanning) ----
            TuiEvent::CancelScan => {
                if let Some(cancel) = state.scan_cancel.as_ref() {
                    cancel.store(true, std::sync::atomic::Ordering::Relaxed);
                    state.set_status("Cancelling scan...");
                }
            }
            // ---- I5 confirm gate: destructive actions require explicit confirmation ----
            TuiEvent::ActionInstall => {
                state.pending_confirm = Some(ConfirmState {
                    pending: TuiEvent::ActionInstall,
                    label: "Install lint-arwaky binaries into PATH".to_string(),
                });
                state.set_status("Confirm: install?");
            }
            TuiEvent::ActionInit => {
                state.pending_confirm = Some(ConfirmState {
                    pending: TuiEvent::ActionInit,
                    label: "Initialize project setup".to_string(),
                });
                state.set_status("Confirm: init?");
            }
            TuiEvent::ActionUninstallHook => {
                state.pending_confirm = Some(ConfirmState {
                    pending: TuiEvent::ActionUninstallHook,
                    label: "Uninstall pre-commit hook".to_string(),
                });
                state.set_status("Confirm: uninstall hook?");
            }
            TuiEvent::ConfirmAction => {
                let Some(confirm) = state.pending_confirm.take() else {
                    return;
                };
                state.preview_mode = PreviewMode::ActionOutput;
                self.handle(state, confirm.pending);
            }
            TuiEvent::CancelConfirm => {
                let Some(confirm) = state.pending_confirm.take() else {
                    return;
                };
                state.preview_mode = PreviewMode::ActionOutput;
                state.set_status(format!("Cancelled: {}", confirm.label));
            }
            // ---- I4: re-open the project root dialog with current root pre-filled ----
            TuiEvent::ChangeProjectRoot => {
                state.path_input = state.project_root.clone();
                state.show_path_dialog = true;
            }
            // ---- Background global actions: run on a worker thread, poll in event loop ----
            TuiEvent::ActionDoctor => {
                self.start_background_action(
                    state,
                    "doctor",
                    Box::new(|lp: &SurfaceLintExecutor| lp.doctor()),
                );
            }
            TuiEvent::ActionMcpConfig => {
                let flags = state.action_flags.clone();
                self.start_background_action(
                    state,
                    "mcp-config",
                    Box::new(move |lp: &SurfaceLintExecutor| lp.mcp_config(&flags)),
                );
            }
            TuiEvent::ActionConfigShow => self.run_action_no_path(state, |lp| lp.config_show()),
            TuiEvent::ActionInstallHook => self.run_action_no_path(state, |lp| lp.install_hook()),
            TuiEvent::ActionAdapters => self.run_action_no_path(state, |lp| lp.adapters()),
            TuiEvent::ActionVersion => self.run_action_no_path(state, |lp| lp.version()),
            // ---- Watch: FR-006 says watch is NOT supported in TUI ----
            TuiEvent::ActionWatch => {
                state.preview_text = "Watch mode is not supported in TUI.\nUse `lint-arwaky-cli watch` in a terminal.".to_string();
                state.preview_mode = PreviewMode::ActionOutput;
                state.set_status("Watch not supported in TUI");
            }
            // ---- Path input dialog: character-by-character editing ----
            TuiEvent::PathInput(ch) => state.path_input.push(ch),
            TuiEvent::PathBackspace => {
                state.path_input.pop();
            }
            // ---- Path dialog: confirm typed path (empty input falls back to CWD) ----
            TuiEvent::PathConfirm => {
                let raw = if state.path_input.trim().is_empty() {
                    std::env::current_dir()
                        .map(|p| p.to_string_lossy().to_string())
                        .unwrap_or_else(|_| ".".to_string())
                } else {
                    state.path_input.trim().to_string()
                };
                let path = FilePath::new(raw.clone()).unwrap_or_default();
                if utility_file_system::is_valid_directory(&path) {
                    state.project_root = raw.clone();
                    state.current_dir = raw.clone();
                    state.show_path_dialog = false;
                    self.load_directory(state, &state.current_dir.clone());
                } else {
                    state.set_status(
                        "Invalid path — type a directory, or press Tab for current dir",
                    );
                }
            }
            // ---- Path dialog: use CWD as project root ----
            TuiEvent::PathUseCurrent => {
                let cwd = std::env::current_dir()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|_| ".".to_string());
                state.project_root = cwd.clone();
                state.current_dir = cwd.clone();
                state.show_path_dialog = false;
                self.load_directory(state, &state.current_dir.clone());
            }
            // ---- Resize: track terminal height for mouse click mapping ----
            TuiEvent::Resize(w, h) => {
                state.terminal_height = h;
                state.terminal_width = w;
            }
            // ---- Quit: Esc first closes the help overlay, then actually quits ----
            TuiEvent::Quit => {
                if state.show_help {
                    state.show_help = false;
                    state.preview_mode = PreviewMode::ActionOutput;
                } else {
                    state.should_quit = true;
                }
            }
            TuiEvent::MouseClick(col, row) => self.handle_mouse_click(state, col, row),
            TuiEvent::MouseDrag(col, row) => self.handle_mouse_drag(state, col, row),
            TuiEvent::MouseScrollUp(_col, _row) => {
                if state.panel_focus == PanelFocus::Preview {
                    state.preview_scroll = state.preview_scroll.saturating_sub(3);
                } else if state.scroll_offset > 0 {
                    state.scroll_offset -= 1;
                    if state.selected_index > 0 {
                        state.selected_index -= 1;
                    }
                }
            }
            TuiEvent::MouseScrollDown(_col, _row) => {
                if state.panel_focus == PanelFocus::Preview {
                    state.preview_scroll = state.preview_scroll.saturating_add(3);
                    state.preview_scroll = state.preview_scroll.min(self.max_preview_scroll(state));
                } else {
                    let max_scroll = state.entries.len().saturating_sub(1);
                    if state.scroll_offset < max_scroll {
                        state.scroll_offset += 1;
                        let max_idx = state.entries.len().saturating_sub(1);
                        if state.selected_index < max_idx {
                            state.selected_index += 1;
                        }
                    }
                }
            }
            TuiEvent::CopyToClipboard => self.copy_to_clipboard(state),
            TuiEvent::CopyToFile => self.copy_to_file(state),
            _ => {}
        }
    }

    /// Navigate to the parent directory, clamped to the project root boundary.
    fn navigate_back(&self, state: &mut AppState) {
        let current = FilePath::new(state.current_dir.clone()).unwrap_or_default();
        if let Some(parent) = utility_file_system::parent_directory(&current)
            && parent.value().starts_with(&state.project_root)
        {
            state.current_dir = parent.value().to_string();
            self.load_directory(state, &state.current_dir.clone());
        }
    }

    /// Navigate into a directory or select a file and load preview.
    fn navigate_forward(&self, state: &mut AppState) {
        let path = state.selected_path();
        let is_dir = state.selected_entry().map(|e| e.is_dir).unwrap_or(false);

        if is_dir {
            state.current_dir = path;
            self.load_directory(state, &state.current_dir.clone());
        } else {
            // FR-003: Entry is a file → preview loaded in Preview panel.
            self.load_file_preview(state, &path);
            state.set_status(format!("Selected: {}", path));
        }
    }

    /// Load and sort a directory listing: directories first, then alphabetically.
    /// Resets selection and scroll position after loading.
    pub fn load_directory(&self, state: &mut AppState, path: &str) {
        let fp = FilePath::new(path).unwrap_or_default();
        let dir_path = std::path::Path::new(fp.value());
        let paths = self
            .filesystem
            .read_dir_entries_as_pathbuf(dir_path)
            .unwrap_or_default();
        state.entries = paths
            .into_iter()
            .filter_map(|entry_path| {
                let name = entry_path.file_name()?.to_str()?;
                if name.starts_with('.') {
                    return None;
                }
                shared::tui::FileEntry::from_path(&entry_path)
            })
            .collect();
        if state.entries.is_empty() {
            state.set_status(format!("Empty or inaccessible: {}", path));
        }
        state.entries.sort_by(|a, b| match (a.is_dir, b.is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        });
        state.selected_index = 0;
        state.scroll_offset = 0;
        // No file content preview — Preview panel stays empty until action is run
        state.preview_mode = PreviewMode::ActionOutput;
        state.set_status(format!("Dir: {}", path));
        state.compute_filtered_indices();
    }

    /// Read up to 100 lines of a file for inline preview.
    pub fn load_file_preview(&self, state: &mut AppState, path: &str) {
        let fp = FilePath::new(path.to_string()).unwrap_or_default();
        let file_path = std::path::Path::new(fp.value());
        let max_lines = 100;
        let display = match self.filesystem.read_to_string(file_path) {
            Ok(content) => {
                let lines: Vec<&str> = content.value.lines().take(max_lines).collect();
                let mut output = String::new();
                for (i, line) in lines.iter().enumerate() {
                    output.push_str(&format!("{:>4} │ {}\n", i + 1, line));
                }
                let total = content.value.lines().count();
                if total > max_lines {
                    output.push_str(&format!("\n... ({} more lines)", total - max_lines));
                }
                shared::common::DisplayContent::new(output)
            }
            Err(e) => shared::common::DisplayContent::new(format!("Cannot read file: {e}")),
        };
        state.preview_text = display.to_string();
        state.preview_scroll = 0;
        state.preview_mode = PreviewMode::FileContent;
    }

    /// Load preview for the currently selected entry if it's a file.
    pub fn load_preview(&self, state: &mut AppState) {
        if let Some(entry) = state.selected_entry()
            && !entry.is_dir
        {
            let path = entry.full_path.clone();
            self.load_file_preview(state, &path);
        }
    }

    /// Run a lint action that requires a selected path.
    /// Dispatches to the lint hook and stores result output + violation count in state.
    fn run_action<F>(&self, state: &mut AppState, action: F)
    where
        F: FnOnce(
            &SurfaceLintExecutor,
            &str,
            &shared::tui::taxonomy_action_flags_vo::ActionFlags,
        ) -> LintExecutionResult,
    {
        let path = state.selected_path();
        state.set_status(format!("Running action on {}...", path));
        let result = action(self.lint_port.as_ref(), &path, &state.action_flags);
        state.preview_text = result.output;
        state.violation_count = result.violation_count;
        state.preview_scroll = 0;
        state.preview_mode = PreviewMode::LintResults;
        let status = if result.success { "Done" } else { "Error" };
        state.set_status(format!(
            "{}: {} | {} violations",
            status, path, result.violation_count
        ));
    }

    /// Run a global lint action that has no path parameter (e.g. doctor, version).
    fn run_action_no_path<F>(&self, state: &mut AppState, action: F)
    where
        F: FnOnce(&SurfaceLintExecutor) -> LintExecutionResult,
    {
        let result = action(self.lint_port.as_ref());
        state.preview_text = result.output;
        state.violation_count = result.violation_count;
        state.preview_scroll = 0;
        state.preview_mode = PreviewMode::ActionOutput;
        let status = if result.success { "Done" } else { "Error" };
        state.set_status(status);
    }

    /// Copy the current preview content to the system clipboard.
    /// Delegates I/O to utility_file_system::copy_text_to_clipboard().
    fn copy_to_clipboard(&self, state: &mut AppState) {
        let text = state.preview_text.clone();
        if text.is_empty() {
            state.set_status("Nothing to copy");
            return;
        }

        let success = utility_file_system::copy_text_to_clipboard(&text);

        if success {
            state.set_status("Copied to clipboard!");
        } else {
            state.set_status("Clipboard unavailable — install xclip or wl-copy");
        }
    }

    /// Copy the current preview content to a file `lint-results.txt` in the current directory.
    /// Delegates I/O to self.filesystem.write_text_to_file().
    fn copy_to_file(&self, state: &mut AppState) {
        let text = &state.preview_text;
        if text.is_empty() {
            state.set_status("Nothing to copy");
            return;
        }

        let path = std::path::Path::new("lint-results.txt");
        match self.filesystem.write_string(path, text) {
            Ok(()) => state.set_status("Saved to lint-results.txt"),
            Err(e) => state.set_status(format!("Save failed: {e}")),
        }
    }

    /// Handle mouse clicks on the file list, shortcut, preview, and scrollbar areas.
    fn handle_mouse_click(&self, state: &mut AppState, col: u16, row: u16) {
        let h = state.terminal_height;
        let w = state.terminal_width;
        if h < 5 || w < 10 {
            return;
        }
        let shortcuts_start = h - 4;
        let file_list_start: u16 = 1;
        let file_list_end = shortcuts_start - 1;
        let preview_start = file_list_end;
        let preview_end = shortcuts_start;
        let scrollbar_col = w.saturating_sub(3);

        if row >= shortcuts_start && row < h {
            return;
        }

        // Click on scrollbar thumb area → jump to proportional position
        if col >= scrollbar_col && row >= preview_start && row < preview_end {
            self.jump_to_scroll_position(state, row - preview_start, preview_end - preview_start);
            state.panel_focus = PanelFocus::Preview;
            return;
        }

        if row >= file_list_start && row < file_list_end {
            let panel_row = row - file_list_start;
            let new_index = state.scroll_offset + panel_row as usize;
            if new_index < state.entries.len() {
                state.selected_index = new_index;
                state.panel_focus = PanelFocus::FileList;
            }
        } else if row >= preview_start && row < preview_end {
            self.jump_to_scroll_position(state, row - preview_start, preview_end - preview_start);
            state.panel_focus = PanelFocus::Preview;
        }
    }

    /// Handle mouse drag on the scrollbar thumb area.
    fn handle_mouse_drag(&self, state: &mut AppState, col: u16, row: u16) {
        let h = state.terminal_height;
        let w = state.terminal_width;
        if h < 5 || w < 10 {
            return;
        }
        let shortcuts_start = h - 4;
        let file_list_end = shortcuts_start - 1;
        let preview_start = file_list_end;
        let preview_end = shortcuts_start;
        let scrollbar_col = w.saturating_sub(3);

        if col >= scrollbar_col && row >= preview_start && row < preview_end {
            self.jump_to_scroll_position(state, row - preview_start, preview_end - preview_start);
            state.panel_focus = PanelFocus::Preview;
        }
    }

    /// Map a click/drag row within the preview panel to a scroll position.
    fn jump_to_scroll_position(&self, state: &mut AppState, relative_row: u16, panel_height: u16) {
        if panel_height == 0 {
            return;
        }
        let content_length = state.preview_text.lines().count().max(1);
        let max_scroll = content_length.saturating_sub(panel_height as usize);
        let fraction = relative_row as f64 / panel_height as f64;
        let new_scroll = (fraction * max_scroll as f64).round() as usize;
        state.preview_scroll = new_scroll.min(max_scroll);
    }

    /// Compute the maximum valid preview scroll offset.
    fn max_preview_scroll(&self, state: &AppState) -> usize {
        let h = state.terminal_height;
        if h < 5 {
            return 0;
        }
        let preview_height = (h - 4) as usize;
        let content_length = state.preview_text.lines().count().max(1);
        content_length.saturating_sub(preview_height)
    }
}
