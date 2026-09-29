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

#[derive(Debug, Clone)]
pub struct LintExecutionResult {
    pub output: String,
    pub violation_count: usize,
    pub success: bool,
}

impl LintExecutionResult {
    pub fn success(output: impl Into<String>, violations: usize) -> Self {
        Self {
            output: output.into(),
            violation_count: violations,
            success: true,
        }
    }

    pub fn failure(output: impl Into<String>) -> Self {
        Self {
            output: output.into(),
            violation_count: 0,
            success: false,
        }
    }
}

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

use serde::{Deserialize, Serialize};

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
pub struct AppState {
    pub project_root: String,
    pub current_dir: String,
    pub entries: Vec<FileEntry>,
    pub selected_index: usize,
    pub scroll_offset: usize,
    pub panel_focus: PanelFocus,
    pub preview_mode: PreviewMode,
    pub preview_text: String,
    pub status_message: String,
    pub action_flags: ActionFlags,
    pub search_query: String,
    pub search_mode: bool,
    pub show_help: bool,
    pub show_path_dialog: bool,
    pub path_input: String,
    pub should_quit: bool,
    pub violation_count: usize,
    pub tree_scroll: usize,
    pub preview_scroll: usize,
    pub terminal_height: u16,
    pub terminal_width: u16,
    /// Indices into `entries` matching the current search query (empty when not filtering).
    pub filtered_indices: Vec<usize>,
    /// Position within `filtered_indices` — which matching entry is selected.
    pub filter_pos: usize,
    /// Whether file watching is active (w key toggles this).
    pub watching: bool,
    /// Receiver for watch-mode lint updates from background thread.
    pub watch_receiver: Option<std::sync::mpsc::Receiver<WatchMessage>>,
    /// Latest watch output to display in preview panel.
    pub watch_results: String,
    /// Whether a background scan is currently running.
    pub scanning: bool,
    /// Whether a background global action (install/doctor/init/mcp-config) is running.
    pub action_pending: bool,
    /// Receiver for the background global-action thread's result (mirror of the scan receiver).
    pub action_result_rx: Option<std::sync::mpsc::Receiver<LintExecutionResult>>,
    /// Shared cancel flag read by the background scan thread; set when the user presses Esc mid-scan.
    pub scan_cancel: Option<std::sync::Arc<std::sync::atomic::AtomicBool>>,
    /// Destructive action awaiting user confirmation (I5 confirm gate).
    pub pending_confirm: Option<ConfirmState>,
    /// Preview mode that was active before opening the help overlay (restored on close).
    pub last_preview_mode: Option<PreviewMode>,
    /// Current phase description shown during scanning (e.g. "AES checks").
    pub scan_phase: String,
    /// Number of files processed so far.
    pub scan_files_done: usize,
    /// Total file count for the current scan phase.
    pub scan_files_total: usize,
    /// Violations found so far during the scan.
    pub scan_violations: usize,
}

impl AppState {
    pub fn new(project_root: String) -> Self {
        let current_dir = project_root.clone();
        Self {
            project_root,
            current_dir,
            entries: Vec::new(),
            selected_index: 0,
            scroll_offset: 0,
            panel_focus: PanelFocus::FileList,
            preview_mode: PreviewMode::ActionOutput,
            preview_text: String::new(),
            status_message: "Ready".to_string(),
            action_flags: ActionFlags::default(),
            search_query: String::new(),
            search_mode: false,
            show_help: false,
            show_path_dialog: true,
            path_input: String::new(),
            should_quit: false,
            violation_count: 0,
            tree_scroll: 0,
            preview_scroll: 0,
            terminal_height: 0,
            terminal_width: 0,
            filtered_indices: Vec::new(),
            filter_pos: 0,
            watching: false,
            watch_receiver: None,
            watch_results: String::new(),
            scanning: false,
            action_pending: false,
            action_result_rx: None,
            scan_cancel: None,
            pending_confirm: None,
            last_preview_mode: None,
            scan_phase: String::new(),
            scan_files_done: 0,
            scan_files_total: 0,
            scan_violations: 0,
        }
    }

    pub fn select_next(&mut self) {
        if self.search_mode && !self.search_query.is_empty() {
            if !self.filtered_indices.is_empty()
                && self.filter_pos < self.filtered_indices.len() - 1
            {
                self.filter_pos += 1;
                self.selected_index = self.filtered_indices[self.filter_pos];
                self.adjust_scroll(self.file_list_visible_height());
            }
        } else if !self.entries.is_empty() && self.selected_index < self.entries.len() - 1 {
            self.selected_index += 1;
            self.adjust_scroll(self.file_list_visible_height());
        }
    }

    pub fn select_prev(&mut self) {
        if self.search_mode && !self.search_query.is_empty() {
            if self.filter_pos > 0 {
                self.filter_pos -= 1;
                self.selected_index = self.filtered_indices[self.filter_pos];
                self.adjust_scroll(self.file_list_visible_height());
            }
        } else if self.selected_index > 0 {
            self.selected_index -= 1;
            self.adjust_scroll(self.file_list_visible_height());
        }
    }

    pub fn select_first(&mut self) {
        if self.search_mode && !self.search_query.is_empty() {
            if !self.filtered_indices.is_empty() {
                self.filter_pos = 0;
                self.selected_index = self.filtered_indices[0];
            }
            self.scroll_offset = 0;
        } else {
            self.selected_index = 0;
            self.scroll_offset = 0;
        }
    }

    pub fn select_last(&mut self) {
        if self.search_mode && !self.search_query.is_empty() {
            if !self.filtered_indices.is_empty() {
                self.filter_pos = self.filtered_indices.len() - 1;
                self.selected_index = self.filtered_indices[self.filter_pos];
                self.adjust_scroll(self.file_list_visible_height());
            }
        } else if !self.entries.is_empty() {
            self.selected_index = self.entries.len() - 1;
            self.adjust_scroll(self.file_list_visible_height());
        }
    }

    pub fn selected_entry(&self) -> Option<&FileEntry> {
        self.entries.get(self.selected_index)
    }

    pub fn selected_path(&self) -> String {
        match self.selected_entry() {
            Some(entry) => entry.full_path.clone(),
            None => self.current_dir.clone(),
        }
    }

    pub fn set_status(&mut self, msg: impl Into<String>) {
        self.status_message = msg.into();
    }

    pub fn adjust_scroll(&mut self, visible_height: usize) {
        if visible_height == 0 {
            return;
        }
        if self.selected_index < self.scroll_offset {
            self.scroll_offset = self.selected_index;
        }
        if self.selected_index >= self.scroll_offset + visible_height {
            self.scroll_offset = self.selected_index - visible_height + 1;
        }
    }

    /// Recompute `filtered_indices` from the current search query.
    /// Call after ToggleSearch, SearchInput, SearchBackspace, SearchConfirm, SearchCancel,
    /// and after loading a new directory while search mode is active.
    pub fn compute_filtered_indices(&mut self) {
        if self.search_mode && !self.search_query.is_empty() {
            let query = self.search_query.to_lowercase();
            self.filtered_indices = self
                .entries
                .iter()
                .enumerate()
                .filter(|(_, entry)| entry.name.to_lowercase().contains(&query))
                .map(|(i, _)| i)
                .collect();
            // Clamp filter_pos to valid range
            if self.filter_pos >= self.filtered_indices.len() {
                self.filter_pos = self.filtered_indices.len().saturating_sub(1);
            }
            // Sync selected_index from the current filter position
            if !self.filtered_indices.is_empty() {
                self.selected_index = self.filtered_indices[self.filter_pos];
            }
        } else {
            self.filtered_indices.clear();
            self.filter_pos = 0;
        }
    }

    /// Compute the visible height of the file list panel from terminal_height.
    /// Layout: 1 header row + 3 shortcut rows + 1 status row = 5 rows overhead.
    fn file_list_visible_height(&self) -> usize {
        (self.terminal_height as usize).saturating_sub(5)
    }

    /// Mark the scan as started with the given phase and total file count.
    pub fn set_scanning(&mut self, scanning: bool, phase: String, total: usize) {
        self.scanning = scanning;
        self.scan_phase = if scanning { phase } else { String::new() };
        self.scan_files_done = 0;
        self.scan_files_total = total;
        self.scan_violations = 0;
    }

    /// Update in-progress scan metrics.
    pub fn update_scan_progress(&mut self, phase: String, done: usize, violations: usize) {
        self.scan_phase = phase;
        self.scan_files_done = done;
        self.scan_violations = violations;
    }

    /// Mark the scan as finished and record the final violation count.
    pub fn finish_scan(&mut self, total_violations: usize) {
        self.scanning = false;
        self.scan_phase.clear();
        self.scan_files_done = 0;
        self.scan_files_total = 0;
        self.scan_violations = total_violations;
    }

    /// Cycle panel focus forward; the Tree panel is display-only and never a target.
    pub fn cycle_focus_forward(&mut self) {
        self.panel_focus = match self.panel_focus {
            PanelFocus::Tree => PanelFocus::FileList,
            PanelFocus::FileList => PanelFocus::Preview,
            PanelFocus::Preview => PanelFocus::FileList,
        };
    }

    /// Cycle panel focus backward; the Tree panel is display-only and never a target.
    pub fn cycle_focus_backward(&mut self) {
        self.panel_focus = match self.panel_focus {
            PanelFocus::Tree => PanelFocus::Preview,
            PanelFocus::Preview => PanelFocus::FileList,
            PanelFocus::FileList => PanelFocus::Preview,
        };
    }
}

// ─── Watch message ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct WatchMessage {
    pub value: String,
}

impl WatchMessage {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}
