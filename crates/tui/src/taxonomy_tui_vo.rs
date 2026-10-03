// PURPOSE: TUI value objects — action flags, adapter info, confirm state, file entries,
//          lint execution results, scan updates, app state, and watch messages.
use std::fmt;
use std::path::Path;

// ─── Action flags ──────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct ActionFlags {
    pub git_diff: bool,
    pub dry_run: bool,
    pub threshold: u32,
    pub global_config: bool,
    pub use_sudo: bool,
    pub mcp_client: String,
}

impl Default for ActionFlags {
    fn default() -> Self {
        Self {
            git_diff: false,
            dry_run: false,
            threshold: 80,
            global_config: false,
            use_sudo: false,
            mcp_client: "claude".to_string(),
        }
    }
}

impl ActionFlags {
    pub fn toggle_git_diff(&mut self) {
        self.git_diff = !self.git_diff;
    }

    pub fn toggle_dry_run(&mut self) {
        self.dry_run = !self.dry_run;
    }

    pub fn toggle_global(&mut self) {
        self.global_config = !self.global_config;
    }

    pub fn toggle_sudo(&mut self) {
        self.use_sudo = !self.use_sudo;
    }

    pub fn set_threshold(&mut self, value: u32) {
        self.threshold = value;
    }

    pub fn set_mcp_client(&mut self, client: impl Into<String>) {
        self.mcp_client = client.into();
    }
}

// ─── Adapter info ──────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdapterInfo {
    pub name: String,
    pub label: String,
    pub installed: bool,
}

impl fmt::Display for AdapterInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({})",
            self.name,
            if self.installed {
                "installed"
            } else {
                "missing"
            }
        )
    }
}

// ─── Confirm state ─────────────────────────────────────────────────────

/// I5: pending destructive action awaiting user confirmation.
///
/// `pending` holds the TuiEvent to execute once the user confirms (Enter/y).
/// Stored on `AppState.pending_confirm`; `None` means no confirmation is pending.
#[derive(Debug, Clone)]
pub struct ConfirmState {
    /// TuiEvent to execute on confirmation.
    pub pending: super::taxonomy_tui_event::TuiEvent,
    /// Human-readable action label shown in the confirm prompt.
    pub label: String,
}

// ─── File entry and layer badge ─────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AesLayer {
    Taxonomy,
    Contract,
    Utility,
    Capabilities,
    Agent,
    Surfaces,
    Root,
    None,
}

impl AesLayer {
    pub fn badge_label(&self) -> &str {
        match self {
            AesLayer::Taxonomy => "[tax]",
            AesLayer::Contract => "[con]",
            AesLayer::Utility => "[uti]",
            AesLayer::Capabilities => "[cap]",
            AesLayer::Agent => "[agt]",
            AesLayer::Surfaces => "[sur]",
            AesLayer::Root => "[root]",
            AesLayer::None => "[---]",
        }
    }

    pub fn color_index(&self) -> u8 {
        match self {
            AesLayer::Taxonomy => 14,
            AesLayer::Contract => 12,
            AesLayer::Utility => 11,
            AesLayer::Capabilities => 13,
            AesLayer::Agent => 10,
            AesLayer::Surfaces => 9,
            AesLayer::Root => 15,
            AesLayer::None => 8,
        }
    }

    pub fn from_filename(filename: &str) -> Self {
        let stem = Path::new(filename)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or_default();

        if stem.starts_with("taxonomy_") {
            AesLayer::Taxonomy
        } else if stem.starts_with("contract_") {
            AesLayer::Contract
        } else if stem.starts_with("utility_") {
            AesLayer::Utility
        } else if stem.starts_with("capabilities_") {
            AesLayer::Capabilities
        } else if stem.starts_with("agent_") {
            AesLayer::Agent
        } else if stem.starts_with("surface_") {
            AesLayer::Surfaces
        } else if stem.starts_with("root_") {
            AesLayer::Root
        } else {
            AesLayer::None
        }
    }
}

#[derive(Debug, Clone)]
pub struct FileEntry {
    pub name: String,
    pub full_path: String,
    pub is_dir: bool,
    pub layer: AesLayer,
    pub violation_count: usize,
    pub extension: String,
    pub size_bytes: u64,
}

impl FileEntry {
    pub fn from_path(path: &Path) -> Option<Self> {
        let name = path.file_name()?.to_str()?.to_string();
        let metadata = path.metadata().ok()?;
        let is_dir = metadata.is_dir();
        let layer = if is_dir {
            AesLayer::None
        } else {
            AesLayer::from_filename(&name)
        };
        let extension = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_string();

        Some(Self {
            name,
            full_path: path.to_string_lossy().to_string(),
            is_dir,
            layer,
            violation_count: 0,
            extension,
            size_bytes: metadata.len(),
        })
    }

    pub fn display_name(&self) -> String {
        if self.is_dir {
            format!("{}/", self.name)
        } else {
            self.name.clone()
        }
    }
}

// ─── Lint execution result ─────────────────────────────────────────────
// Re-exported from shared-tui (single source of truth).
pub use shared_tui::LintExecutionResult;

// ─── Scan update ───────────────────────────────────────────────────────

/// Messages sent from a background scan thread to the TUI event loop.
#[derive(Debug, Clone)]
pub enum ScanUpdate {
    /// Periodic progress report during the scan.
    Progress {
        phase: String,
        done: usize,
        total: usize,
    },
    /// Scan completed — carry the final result.
    Complete {
        output: String,
        violation_count: usize,
        success: bool,
    },
    /// Scan aborted before completion (user pressed Esc while scanning).
    Cancelled,
}

// ─── App state ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PanelFocus {
    Tree,
    FileList,
    Preview,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreviewMode {
    FileContent,
    LintResults,
    HelpOverlay,
    ActionOutput,
}

#[derive(Debug)]
pub struct NavigationState {
    pub project_root: String,
    pub current_dir: String,
    pub entries: Vec<FileEntry>,
    pub selected_index: usize,
    pub scroll_offset: usize,
    pub panel_focus: PanelFocus,
    pub tree_scroll: usize,
}

#[derive(Debug, Default)]
pub struct SearchState {
    pub query: String,
    pub mode: bool,
    pub filtered_indices: Vec<usize>,
    pub filter_pos: usize,
}

#[derive(Debug)]
pub struct PathDialogState {
    pub visible: bool,
    pub input: String,
}

#[derive(Debug)]
pub struct PreviewState {
    pub mode: PreviewMode,
    pub text: String,
    pub scroll: usize,
    pub last_mode: Option<PreviewMode>,
}

#[derive(Debug)]
pub struct ScanProgressState {
    pub running: bool,
    pub cancel: Option<std::sync::Arc<std::sync::atomic::AtomicBool>>,
    pub phase: String,
    pub files_done: usize,
    pub files_total: usize,
    pub violations: usize,
}

#[derive(Debug)]
pub struct ActionState {
    pub flags: ActionFlags,
    pub pending: bool,
    pub result_rx: Option<std::sync::mpsc::Receiver<LintExecutionResult>>,
    pub pending_confirm: Option<ConfirmState>,
    /// The view to activate when a background action completes.
    pub result_mode: PreviewMode,
    /// Status-only actions (for example clipboard copy) must not replace preview text.
    pub update_preview: bool,
}

/// State is grouped by feature boundary so views and handlers no longer depend on
/// a flat, thirty-field global store. The groups are intentionally public because
/// the shared value object is consumed by the surface crate, but each group keeps
/// related mutations together and makes ownership explicit.
#[derive(Debug)]
pub struct AppState {
    pub navigation: NavigationState,
    pub search: SearchState,
    pub path_dialog: PathDialogState,
    pub preview: PreviewState,
    pub scan: ScanProgressState,
    pub actions: ActionState,
    pub status_message: String,
    pub show_help: bool,
    pub should_quit: bool,
    pub violation_count: usize,
    pub terminal_height: u16,
    pub terminal_width: u16,
}

impl AppState {
    pub fn new(project_root: String) -> Self {
        let current_dir = project_root.clone();
        Self {
            navigation: NavigationState {
                project_root,
                current_dir,
                entries: Vec::new(),
                selected_index: 0,
                scroll_offset: 0,
                panel_focus: PanelFocus::FileList,
                tree_scroll: 0,
            },
            search: SearchState::default(),
            path_dialog: PathDialogState {
                visible: true,
                input: String::new(),
            },
            preview: PreviewState {
                mode: PreviewMode::ActionOutput,
                text: String::new(),
                scroll: 0,
                last_mode: None,
            },
            scan: ScanProgressState {
                running: false,
                cancel: None,
                phase: String::new(),
                files_done: 0,
                files_total: 0,
                violations: 0,
            },
            actions: ActionState {
                flags: ActionFlags::default(),
                pending: false,
                result_rx: None,
                pending_confirm: None,
                result_mode: PreviewMode::ActionOutput,
                update_preview: true,
            },
            status_message: "Ready".to_string(),
            show_help: false,
            should_quit: false,
            violation_count: 0,
            terminal_height: 0,
            terminal_width: 0,
        }
    }

    pub fn select_next(&mut self) {
        if self.search.mode && !self.search.query.is_empty() {
            if !self.search.filtered_indices.is_empty()
                && self.search.filter_pos < self.search.filtered_indices.len() - 1
            {
                self.search.filter_pos += 1;
                self.navigation.selected_index =
                    self.search.filtered_indices[self.search.filter_pos];
                self.adjust_scroll(self.file_list_visible_height());
            }
        } else if !self.navigation.entries.is_empty()
            && self.navigation.selected_index < self.navigation.entries.len() - 1
        {
            self.navigation.selected_index += 1;
            self.adjust_scroll(self.file_list_visible_height());
        }
    }

    pub fn select_prev(&mut self) {
        if self.search.mode && !self.search.query.is_empty() {
            if self.search.filter_pos > 0 {
                self.search.filter_pos -= 1;
                self.navigation.selected_index =
                    self.search.filtered_indices[self.search.filter_pos];
                self.adjust_scroll(self.file_list_visible_height());
            }
        } else if self.navigation.selected_index > 0 {
            self.navigation.selected_index -= 1;
            self.adjust_scroll(self.file_list_visible_height());
        }
    }

    pub fn select_first(&mut self) {
        if self.search.mode && !self.search.query.is_empty() {
            if !self.search.filtered_indices.is_empty() {
                self.search.filter_pos = 0;
                self.navigation.selected_index = self.search.filtered_indices[0];
            }
            self.navigation.scroll_offset = 0;
        } else {
            self.navigation.selected_index = 0;
            self.navigation.scroll_offset = 0;
        }
    }

    pub fn select_last(&mut self) {
        if self.search.mode && !self.search.query.is_empty() {
            if !self.search.filtered_indices.is_empty() {
                self.search.filter_pos = self.search.filtered_indices.len() - 1;
                self.navigation.selected_index =
                    self.search.filtered_indices[self.search.filter_pos];
                self.adjust_scroll(self.file_list_visible_height());
            }
        } else if !self.navigation.entries.is_empty() {
            self.navigation.selected_index = self.navigation.entries.len() - 1;
            self.adjust_scroll(self.file_list_visible_height());
        }
    }

    pub fn selected_entry(&self) -> Option<&FileEntry> {
        self.navigation.entries.get(self.navigation.selected_index)
    }

    pub fn selected_path(&self) -> String {
        match self.selected_entry() {
            Some(entry) => entry.full_path.clone(),
            None => self.navigation.current_dir.clone(),
        }
    }

    pub fn set_status(&mut self, msg: impl Into<String>) {
        self.status_message = msg.into();
    }

    pub fn adjust_scroll(&mut self, visible_height: usize) {
        if visible_height == 0 {
            return;
        }
        if self.navigation.selected_index < self.navigation.scroll_offset {
            self.navigation.scroll_offset = self.navigation.selected_index;
        }
        if self.navigation.selected_index >= self.navigation.scroll_offset + visible_height {
            self.navigation.scroll_offset = self.navigation.selected_index - visible_height + 1;
        }
    }

    /// Recompute `filtered_indices` from the current search query.
    /// Call after ToggleSearch, SearchInput, SearchBackspace, SearchConfirm, SearchCancel,
    /// and after loading a new directory while search mode is active.
    pub fn compute_filtered_indices(&mut self) {
        if self.search.mode && !self.search.query.is_empty() {
            let query = self.search.query.to_lowercase();
            self.search.filtered_indices = self
                .navigation
                .entries
                .iter()
                .enumerate()
                .filter(|(_, entry)| entry.name.to_lowercase().contains(&query))
                .map(|(i, _)| i)
                .collect();
            // Clamp filter_pos to valid range
            if self.search.filter_pos >= self.search.filtered_indices.len() {
                self.search.filter_pos = self.search.filtered_indices.len().saturating_sub(1);
            }
            // Sync selected_index from the current filter position
            if !self.search.filtered_indices.is_empty() {
                self.navigation.selected_index =
                    self.search.filtered_indices[self.search.filter_pos];
            }
        } else {
            self.search.filtered_indices.clear();
            self.search.filter_pos = 0;
        }
    }

    /// Compute the visible height of the file list panel from terminal_height.
    /// Layout: 1 header row + 3 shortcut rows + 1 status row = 5 rows overhead.
    fn file_list_visible_height(&self) -> usize {
        (self.terminal_height as usize).saturating_sub(5)
    }

    /// Mark the scan as started with the given phase and total file count.
    pub fn set_scanning(&mut self, scanning: bool, phase: String, total: usize) {
        self.scan.running = scanning;
        self.scan.phase = if scanning { phase } else { String::new() };
        self.scan.files_done = 0;
        self.scan.files_total = total;
        self.scan.violations = 0;
    }

    /// Update in-progress scan metrics. The dispatcher owns the file count;
    /// violation totals are updated only when the scan completes.
    pub fn update_scan_progress(&mut self, phase: String, done: usize, total: usize) {
        self.scan.phase = phase;
        self.scan.files_done = done;
        self.scan.files_total = total;
    }

    /// Mark the scan as finished and record the final violation count.
    pub fn finish_scan(&mut self, total_violations: usize) {
        self.scan.running = false;
        self.scan.phase.clear();
        self.scan.files_done = 0;
        self.scan.files_total = 0;
        self.scan.violations = total_violations;
    }

    /// Cycle panel focus forward; the Tree panel is display-only and never a target.
    pub fn cycle_focus_forward(&mut self) {
        self.navigation.panel_focus = match self.navigation.panel_focus {
            PanelFocus::Tree => PanelFocus::FileList,
            PanelFocus::FileList => PanelFocus::Preview,
            PanelFocus::Preview => PanelFocus::FileList,
        };
    }

    /// Cycle panel focus backward; the Tree panel is display-only and never a target.
    pub fn cycle_focus_backward(&mut self) {
        self.navigation.panel_focus = match self.navigation.panel_focus {
            PanelFocus::Tree => PanelFocus::Preview,
            PanelFocus::Preview => PanelFocus::FileList,
            PanelFocus::FileList => PanelFocus::Preview,
        };
    }
}
