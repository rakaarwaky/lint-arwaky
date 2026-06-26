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
    pub terminal_height: usize,
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
        }
    }

    pub fn select_next(&mut self) {
        if !self.entries.is_empty() && self.selected_index < self.entries.len() - 1 {
            self.selected_index += 1;
        }
    }

    pub fn select_prev(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    pub fn select_first(&mut self) {
        self.selected_index = 0;
        self.scroll_offset = 0;
    }

    pub fn select_last(&mut self) {
        if !self.entries.is_empty() {
            self.selected_index = self.entries.len() - 1;
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
}
