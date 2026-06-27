use crate::tui::taxonomy_action_flags_vo::ActionFlags;
use crate::tui::taxonomy_file_entry_vo::FileEntry;

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

#[derive(Debug, Clone)]
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
    pub terminal_height: u16,
    /// Indices into `entries` matching the current search query (empty when not filtering).
    pub filtered_indices: Vec<usize>,
    /// Position within `filtered_indices` — which matching entry is selected.
    pub filter_pos: usize,
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
            preview_mode: PreviewMode::FileContent,
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
            terminal_height: 0,
            filtered_indices: Vec::new(),
            filter_pos: 0,
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

    pub fn cycle_focus_forward(&mut self) {
        self.panel_focus = match self.panel_focus {
            PanelFocus::Tree => PanelFocus::FileList,
            PanelFocus::FileList => PanelFocus::Preview,
            PanelFocus::Preview => PanelFocus::Tree,
        };
    }

    pub fn cycle_focus_backward(&mut self) {
        self.panel_focus = match self.panel_focus {
            PanelFocus::Tree => PanelFocus::Preview,
            PanelFocus::FileList => PanelFocus::Tree,
            PanelFocus::Preview => PanelFocus::FileList,
        };
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
}
