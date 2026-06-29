// PURPOSE: Capabilities-layer action handler — the central state machine for TUI events.
// Translates every TuiEvent into a state mutation or I/O operation (filesystem/lint).
// This is the largest single file in the TUI crate; it owns all event→action mappings.

use shared::common::taxonomy_line_count_vo::LineCount;
use shared::common::taxonomy_path_vo::FilePath;
use shared::tui::contract_action_handler_protocol::IActionHandlerProtocol;
use shared::tui::contract_file_system_port::IFileSystemPort;
use shared::tui::contract_lint_executor_protocol::ILintExecutorProtocol;
use shared::tui::taxonomy_lint_result_vo::LintExecutionResult;
use shared::tui::taxonomy_scan_update_vo::ScanUpdate;
use shared::tui::taxonomy_state_vo::{AppState, PanelFocus, PreviewMode};
use shared::tui::taxonomy_tui_event::TuiEvent;
use std::sync::Arc;

/// ActionHandler — pure state machine for TUI interaction.
/// Owns the filesystem adapter and lint executor, bridging UI events to backend operations.
pub struct ActionHandler {
    fs_port: Arc<dyn IFileSystemPort>,
    lint_port: Arc<dyn ILintExecutorProtocol>,
}

impl ActionHandler {
    pub fn new(
        fs_port: Arc<dyn IFileSystemPort>,
        lint_port: Arc<dyn ILintExecutorProtocol>,
    ) -> Self {
        Self { fs_port, lint_port }
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
                }
            }
            TuiEvent::MoveBottom => {
                if state.panel_focus == PanelFocus::Preview {
                    state.preview_scroll = self.max_preview_scroll(state);
                } else {
                    state.select_last();
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
            // ---- Focus cycling between panels (FileList / Preview / Tree) ----
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
                    state.preview_mode = PreviewMode::FileContent;
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
            TuiEvent::SearchConfirm | TuiEvent::SearchCancel => {
                state.search_mode = false;
                state.search_query.clear();
                state.compute_filtered_indices();
            }
            // ---- Lint actions that operate on the selected file/directory ----
            TuiEvent::ActionCheck => self.run_action(state, |lp, p, f| lp.check(p, f)),
            TuiEvent::ActionScan => self.run_action(state, |lp, p, _f| lp.scan(p)),
            TuiEvent::ActionFix => self.run_action(state, |lp, p, f| lp.fix(p, f)),
            TuiEvent::ActionCi => self.run_action(state, |lp, p, f| lp.ci(p, f)),
            TuiEvent::ActionOrphan => self.run_action(state, |lp, p, _f| lp.orphan(p)),
            TuiEvent::ActionSecurity => self.run_action(state, |lp, p, _f| lp.security(p)),
            TuiEvent::ActionDuplicates => self.run_action(state, |lp, p, _f| lp.duplicates(p)),
            TuiEvent::ActionDependencies => self.run_action(state, |lp, p, _f| lp.dependencies(p)),
            // ---- Setup/global actions that don't need a selected path ----
            TuiEvent::ActionDoctor => self.run_action_no_path(state, |lp| lp.doctor()),
            TuiEvent::ActionInit => {
                let flags = state.action_flags.clone();
                self.run_action_no_path(state, |lp| lp.init(&flags))
            }
            TuiEvent::ActionInstall => {
                let flags = state.action_flags.clone();
                self.run_action_no_path(state, |lp| lp.install(&flags))
            }
            TuiEvent::ActionMcpConfig => {
                let flags = state.action_flags.clone();
                self.run_action_no_path(state, |lp| lp.mcp_config(&flags))
            }
            TuiEvent::ActionConfigShow => self.run_action_no_path(state, |lp| lp.config_show()),
            TuiEvent::ActionInstallHook => self.run_action_no_path(state, |lp| lp.install_hook()),
            TuiEvent::ActionUninstallHook => {
                self.run_action_no_path(state, |lp| lp.uninstall_hook())
            }
            TuiEvent::ActionAdapters => self.run_action_no_path(state, |lp| lp.adapters()),
            TuiEvent::ActionVersion => self.run_action_no_path(state, |lp| lp.version()),
            // ---- Watch: not yet implemented in TUI — redirect to CLI ----
            TuiEvent::ActionWatch => {
                state.preview_text = "File watch is not available in the TUI yet.\n\nUse the CLI command:\n  lint-arwaky-cli watch <path>\n\nThis will start a file watcher that re-runs\nthe linter on every file change.".to_string();
                state.preview_mode = PreviewMode::ActionOutput;
                state.set_status("File watch: use CLI `lint-arwaky-cli watch`");
            }
            // ---- Path input dialog: character-by-character editing ----
            TuiEvent::PathInput(ch) => state.path_input.push(ch),
            TuiEvent::PathBackspace => {
                state.path_input.pop();
            }
            // ---- Path dialog: confirm typed path ----
            TuiEvent::PathConfirm => {
                let path = FilePath::new(state.path_input.clone()).unwrap_or_default();
                if self.fs_port.is_valid_directory(&path) {
                    state.project_root = state.path_input.clone();
                    state.current_dir = state.path_input.clone();
                    state.show_path_dialog = false;
                    self.load_directory(state, &state.current_dir.clone());
                } else {
                    state.set_status("Invalid path");
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
            // ---- Quit and mouse scroll ----
            TuiEvent::Quit => state.should_quit = true,
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
        if let Some(parent) = self.fs_port.parent_directory(&current) {
            if parent.value.starts_with(&state.project_root) {
                state.current_dir = parent.value.clone();
                self.load_directory(state, &state.current_dir.clone());
            }
        }
    }

    /// Navigate into a directory or load a file preview.
    fn navigate_forward(&self, state: &mut AppState) {
        let path = state.selected_path();
        let is_dir = state.selected_entry().map(|e| e.is_dir).unwrap_or(false);

        if is_dir {
            state.current_dir = path;
            self.load_directory(state, &state.current_dir.clone());
        } else {
            self.load_file_preview(state, &path);
        }
    }

    /// Load and sort a directory listing: directories first, then alphabetically.
    /// Resets selection and scroll position after loading.
    pub fn load_directory(&self, state: &mut AppState, path: &str) {
        let fp = FilePath::new(path).unwrap_or_default();
        state.entries = self.fs_port.list_directory(&fp);
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
        state.preview_mode = PreviewMode::FileContent;
        state.set_status(format!("Dir: {}", path));
        state.compute_filtered_indices();
    }

    /// Read up to 100 lines of a file for inline preview.
    fn load_file_preview(&self, state: &mut AppState, path: &str) {
        let fp = FilePath::new(path).unwrap_or_default();
        let max_lines = LineCount::new(100);
        state.preview_text = self.fs_port.read_file_preview(&fp, &max_lines).to_string();
        state.preview_mode = PreviewMode::FileContent;
    }

    /// Load preview for the currently selected entry if it's a file.
    pub fn load_preview(&self, state: &mut AppState) {
        let path = state.selected_path();
        let is_dir = state.selected_entry().map(|e| e.is_dir).unwrap_or(true);
        if !is_dir {
            self.load_file_preview(state, &path);
        }
    }

    /// Run a lint action that requires a selected path.
    /// Dispatches to the lint port and stores result output + violation count in state.
    fn run_action<F>(&self, state: &mut AppState, action: F)
    where
        F: FnOnce(
            &dyn ILintExecutorProtocol,
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
        F: FnOnce(&dyn ILintExecutorProtocol) -> LintExecutionResult,
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
    /// Uses arboard if available, falls back to xclip/wl-copy shell commands.
    fn copy_to_clipboard(&self, state: &mut AppState) {
        let text = state.preview_text.clone();
        if text.is_empty() {
            state.set_status("Nothing to copy");
            return;
        }

        let mut success = false;

        // Try arboard first
        #[cfg(not(test))]
        {
            if let Ok(mut clipboard) = arboard::Clipboard::new() {
                success = clipboard.set_text(&text).is_ok();
            }
        }

        // Fallback to shell commands: xclip → wl-copy
        if !success {
            use std::io::Write;
            success = std::process::Command::new("sh")
                .arg("-c")
                .arg("xclip -selection clipboard 2>/dev/null || wl-copy 2>/dev/null")
                .stdin(std::process::Stdio::piped())
                .spawn()
                .and_then(|mut child| {
                    if let Some(ref mut stdin) = child.stdin {
                        let _ = stdin.write_all(text.as_bytes());
                    }
                    child.wait()
                })
                .map(|status| status.success())
                .unwrap_or(false);
        }

        if success {
            state.set_status("Copied to clipboard!");
        } else {
            state.set_status("Clipboard unavailable — install xclip or wl-copy");
        }
    }

    /// Copy the current preview content to a file `lint-results.txt` in the current directory.
    fn copy_to_file(&self, state: &mut AppState) {
        let text = &state.preview_text;
        if text.is_empty() {
            state.set_status("Nothing to copy");
            return;
        }

        let path = std::path::Path::new("lint-results.txt");
        match std::fs::write(path, text) {
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

impl IActionHandlerProtocol for ActionHandler {
    fn handle(&self, state: &mut AppState, event: TuiEvent) {
        ActionHandler::handle(self, state, event);
    }

    fn load_directory(&self, state: &mut AppState, path: &str) {
        ActionHandler::load_directory(self, state, path);
    }

    fn load_preview(&self, state: &mut AppState) {
        ActionHandler::load_preview(self, state);
    }

    fn poll_watch(&self, _state: &mut AppState) {}

    fn start_scan(&self, state: &mut AppState) -> Option<std::sync::mpsc::Receiver<ScanUpdate>> {
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
        let (tx, rx) = std::sync::mpsc::sync_channel(16);
        std::thread::spawn(move || {
            let _ = tx.send(ScanUpdate::Progress {
                phase: "Running scan...".to_string(),
                done: 0,
                total: 0,
            });
            let result = lint_port.scan(&path);
            let _ = tx.send(ScanUpdate::Complete {
                output: result.output,
                violation_count: result.violation_count,
                success: result.success,
            });
        });

        Some(rx)
    }

    fn poll_scan(&self, state: &mut AppState, rx: &std::sync::mpsc::Receiver<ScanUpdate>) {
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
            }
        }
    }
}
