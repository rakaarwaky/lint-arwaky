use shared::common::taxonomy_line_count_vo::LineCount;
use shared::common::taxonomy_path_vo::FilePath;
use shared::tui::contract_action_handler_protocol::IActionHandlerProtocol;
use shared::tui::contract_file_system_port::IFileSystemPort;
use shared::tui::contract_lint_executor_protocol::ILintExecutorProtocol;
use shared::tui::taxonomy_lint_result_vo::LintExecutionResult;
use shared::tui::taxonomy_state_vo::{AppState, PreviewMode};
use shared::tui::taxonomy_tui_event::TuiEvent;
use std::sync::Arc;

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

    pub fn handle(&self, state: &mut AppState, event: TuiEvent) {
        match event {
            TuiEvent::MoveDown => state.select_next(),
            TuiEvent::MoveUp => state.select_prev(),
            TuiEvent::MoveTop => state.select_first(),
            TuiEvent::MoveBottom => state.select_last(),
            TuiEvent::FocusNext => state.cycle_focus_forward(),
            TuiEvent::FocusPrev => state.cycle_focus_backward(),
            TuiEvent::NavigateBack => self.navigate_back(state),
            TuiEvent::NavigateForward => self.navigate_forward(state),
            TuiEvent::ToggleHelp => {
                state.show_help = !state.show_help;
                if state.show_help {
                    state.preview_mode = PreviewMode::HelpOverlay;
                } else {
                    state.preview_mode = PreviewMode::FileContent;
                }
            }
            TuiEvent::ToggleSearch => {
                state.search_mode = !state.search_mode;
                if !state.search_mode {
                    state.search_query.clear();
                }
            }
            TuiEvent::SearchInput(ch) => {
                if state.search_mode {
                    state.search_query.push(ch);
                }
            }
            TuiEvent::SearchBackspace => {
                state.search_query.pop();
            }
            TuiEvent::SearchConfirm | TuiEvent::SearchCancel => {
                state.search_mode = false;
                state.search_query.clear();
            }
            TuiEvent::ActionCheck => self.run_action(state, |lp, p, f| lp.check(p, f)),
            TuiEvent::ActionScan => self.run_action(state, |lp, p, _f| lp.scan(p)),
            TuiEvent::ActionFix => self.run_action(state, |lp, p, f| lp.fix(p, f)),
            TuiEvent::ActionCi => self.run_action(state, |lp, p, f| lp.ci(p, f)),
            TuiEvent::ActionOrphan => self.run_action(state, |lp, p, _f| lp.orphan(p)),
            TuiEvent::ActionSecurity => self.run_action(state, |lp, p, _f| lp.security(p)),
            TuiEvent::ActionDuplicates => self.run_action(state, |lp, p, _f| lp.duplicates(p)),
            TuiEvent::ActionDependencies => self.run_action(state, |lp, p, _f| lp.dependencies(p)),
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
            TuiEvent::PathInput(ch) => state.path_input.push(ch),
            TuiEvent::PathBackspace => {
                state.path_input.pop();
            }
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
            TuiEvent::PathUseCurrent => {
                let cwd = std::env::current_dir()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|_| ".".to_string());
                state.project_root = cwd.clone();
                state.current_dir = cwd.clone();
                state.show_path_dialog = false;
                self.load_directory(state, &state.current_dir.clone());
            }
            TuiEvent::Quit => state.should_quit = true,
            TuiEvent::MouseScrollUp => {
                if state.scroll_offset > 0 {
                    state.scroll_offset -= 1;
                }
            }
            TuiEvent::MouseScrollDown => {
                state.scroll_offset += 1;
            }
            _ => {}
        }
    }

    fn navigate_back(&self, state: &mut AppState) {
        let current = FilePath::new(state.current_dir.clone()).unwrap_or_default();
        if let Some(parent) = self.fs_port.parent_directory(&current) {
            if parent.len() >= state.project_root.len() {
                state.current_dir = parent.value.clone();
                self.load_directory(state, &state.current_dir.clone());
            }
        }
    }

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

    pub fn load_directory(&self, state: &mut AppState, path: &str) {
        let fp = FilePath::new(path).unwrap_or_default();
        state.entries = self.fs_port.list_directory(&fp);
        state.entries.sort_by(|a, b| match (a.is_dir, b.is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        });
        state.selected_index = 0;
        state.scroll_offset = 0;
        state.preview_mode = PreviewMode::FileContent;
        state.set_status(format!("Dir: {}", path));
    }

    fn load_file_preview(&self, state: &mut AppState, path: &str) {
        let fp = FilePath::new(path).unwrap_or_default();
        let max_lines = LineCount::new(100);
        state.preview_text = self.fs_port.read_file_preview(&fp, &max_lines).to_string();
        state.preview_mode = PreviewMode::FileContent;
    }

    pub fn load_preview(&self, state: &mut AppState) {
        let path = state.selected_path();
        let is_dir = state.selected_entry().map(|e| e.is_dir).unwrap_or(true);
        if !is_dir {
            self.load_file_preview(state, &path);
        }
    }

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
        state.preview_mode = PreviewMode::LintResults;
        let status = if result.success { "Done" } else { "Error" };
        state.set_status(format!(
            "{}: {} | {} violations",
            status, path, result.violation_count
        ));
    }

    fn run_action_no_path<F>(&self, state: &mut AppState, action: F)
    where
        F: FnOnce(&dyn ILintExecutorProtocol) -> LintExecutionResult,
    {
        let result = action(self.lint_port.as_ref());
        state.preview_text = result.output;
        state.violation_count = result.violation_count;
        state.preview_mode = PreviewMode::ActionOutput;
        let status = if result.success { "Done" } else { "Error" };
        state.set_status(status);
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::common::taxonomy_byte_count_vo::ByteCount;
    use shared::common::taxonomy_display_content_vo::DisplayContent;
    use shared::common::taxonomy_path_vo::FilePath;
    use shared::tui::taxonomy_action_flags_vo::ActionFlags;
    use shared::tui::taxonomy_file_entry_vo::{AesLayer, FileEntry};
    use shared::tui::taxonomy_lint_result_vo::LintExecutionResult;
    use shared::tui::taxonomy_state_vo::PanelFocus;
    use std::sync::Arc;

    struct MockFs;

    impl IFileSystemPort for MockFs {
        fn list_directory(&self, _path: &FilePath) -> Vec<FileEntry> {
            vec![
                FileEntry {
                    name: "dir_a".to_string(),
                    full_path: "/root/dir_a".to_string(),
                    is_dir: true,
                    layer: AesLayer::None,
                    violation_count: 0,
                    extension: String::new(),
                    size_bytes: 0,
                },
                FileEntry {
                    name: "file_b.rs".to_string(),
                    full_path: "/root/file_b.rs".to_string(),
                    is_dir: false,
                    layer: AesLayer::None,
                    violation_count: 0,
                    extension: "rs".to_string(),
                    size_bytes: 100,
                },
            ]
        }
        fn read_file_preview(&self, _path: &FilePath, _max: &LineCount) -> DisplayContent {
            DisplayContent::new("preview content".to_string())
        }
        fn is_valid_directory(&self, path: &FilePath) -> bool {
            path.value == "/valid" || path.value == "/root"
        }
        fn parent_directory(&self, path: &FilePath) -> Option<FilePath> {
            if path.value == "/root" {
                Some(FilePath::new("/".to_string()).unwrap_or_default())
            } else if path.value == "/root/dir_a" {
                Some(FilePath::new("/root".to_string()).unwrap_or_default())
            } else {
                None
            }
        }
        fn file_size_human(&self, _bytes: &ByteCount) -> DisplayContent {
            DisplayContent::new("100B".to_string())
        }
        fn path_components(&self, path: &FilePath) -> Vec<FilePath> {
            vec![path.clone()]
        }
    }

    struct MockLint;

    impl ILintExecutorProtocol for MockLint {
        fn check(&self, _path: &str, _f: &ActionFlags) -> LintExecutionResult {
            LintExecutionResult::success("checked", 3)
        }
        fn scan(&self, _path: &str) -> LintExecutionResult {
            LintExecutionResult::success("scanned", 5)
        }
        fn fix(&self, _path: &str, _f: &ActionFlags) -> LintExecutionResult {
            LintExecutionResult::success("fixed", 0)
        }
        fn ci(&self, _path: &str, _f: &ActionFlags) -> LintExecutionResult {
            LintExecutionResult::success("ci pass", 0)
        }
        fn orphan(&self, _path: &str) -> LintExecutionResult {
            LintExecutionResult::success("orphan ok", 0)
        }
        fn security(&self, _path: &str) -> LintExecutionResult {
            LintExecutionResult::success("secure", 0)
        }
        fn duplicates(&self, _path: &str) -> LintExecutionResult {
            LintExecutionResult::success("no dupes", 0)
        }
        fn dependencies(&self, _path: &str) -> LintExecutionResult {
            LintExecutionResult::success("deps ok", 0)
        }
        fn doctor(&self) -> LintExecutionResult {
            LintExecutionResult::success("healthy", 0)
        }
        fn init(&self, _f: &ActionFlags) -> LintExecutionResult {
            LintExecutionResult::success("init done", 0)
        }
        fn install(&self, _f: &ActionFlags) -> LintExecutionResult {
            LintExecutionResult::success("installed", 0)
        }
        fn mcp_config(&self, _f: &ActionFlags) -> LintExecutionResult {
            LintExecutionResult::success("mcp config", 0)
        }
        fn config_show(&self) -> LintExecutionResult {
            LintExecutionResult::success("config shown", 0)
        }
        fn install_hook(&self) -> LintExecutionResult {
            LintExecutionResult::success("hook installed", 0)
        }
        fn uninstall_hook(&self) -> LintExecutionResult {
            LintExecutionResult::success("hook removed", 0)
        }
        fn adapters(&self) -> LintExecutionResult {
            LintExecutionResult::success("adapters listed", 0)
        }
        fn version(&self) -> LintExecutionResult {
            LintExecutionResult::success("v1.0", 0)
        }
    }

    fn make_handler() -> ActionHandler {
        ActionHandler::new(Arc::new(MockFs), Arc::new(MockLint))
    }

    fn make_entries() -> Vec<FileEntry> {
        vec![
            FileEntry {
                name: "file_a.rs".to_string(),
                full_path: "/root/file_a.rs".to_string(),
                is_dir: false,
                layer: AesLayer::None,
                violation_count: 0,
                extension: "rs".to_string(),
                size_bytes: 50,
            },
            FileEntry {
                name: "file_b.rs".to_string(),
                full_path: "/root/file_b.rs".to_string(),
                is_dir: false,
                layer: AesLayer::None,
                violation_count: 0,
                extension: "rs".to_string(),
                size_bytes: 100,
            },
        ]
    }

    #[test]
    fn test_move_down() {
        let h = make_handler();
        let mut state = AppState::new("/root".to_string());
        state.entries = make_entries();
        h.handle(&mut state, TuiEvent::MoveDown);
        assert_eq!(state.selected_index, 1);
    }

    #[test]
    fn test_move_down_stops_at_end() {
        let h = make_handler();
        let mut state = AppState::new("/root".to_string());
        state.entries = make_entries();
        state.selected_index = 1;
        h.handle(&mut state, TuiEvent::MoveDown);
        assert_eq!(state.selected_index, 1);
    }

    #[test]
    fn test_move_up() {
        let h = make_handler();
        let mut state = AppState::new("/root".to_string());
        state.entries = make_entries();
        state.selected_index = 1;
        h.handle(&mut state, TuiEvent::MoveUp);
        assert_eq!(state.selected_index, 0);
    }

    #[test]
    fn test_move_up_stops_at_zero() {
        let h = make_handler();
        let mut state = AppState::new("/root".to_string());
        state.entries = make_entries();
        h.handle(&mut state, TuiEvent::MoveUp);
        assert_eq!(state.selected_index, 0);
    }

    #[test]
    fn test_move_top_and_bottom() {
        let h = make_handler();
        let mut state = AppState::new("/root".to_string());
        state.entries = make_entries();
        h.handle(&mut state, TuiEvent::MoveBottom);
        assert_eq!(state.selected_index, 1);
        h.handle(&mut state, TuiEvent::MoveTop);
        assert_eq!(state.selected_index, 0);
        assert_eq!(state.scroll_offset, 0);
    }

    #[test]
    fn test_focus_cycle() {
        let h = make_handler();
        let mut state = AppState::new("/root".to_string());
        assert_eq!(state.panel_focus, PanelFocus::FileList);
        h.handle(&mut state, TuiEvent::FocusNext);
        assert_eq!(state.panel_focus, PanelFocus::Preview);
        h.handle(&mut state, TuiEvent::FocusNext);
        assert_eq!(state.panel_focus, PanelFocus::Tree);
        h.handle(&mut state, TuiEvent::FocusPrev);
        assert_eq!(state.panel_focus, PanelFocus::Preview);
    }

    #[test]
    fn test_toggle_help() {
        let h = make_handler();
        let mut state = AppState::new("/root".to_string());
        assert!(!state.show_help);
        h.handle(&mut state, TuiEvent::ToggleHelp);
        assert!(state.show_help);
        assert_eq!(state.preview_mode, PreviewMode::HelpOverlay);
        h.handle(&mut state, TuiEvent::ToggleHelp);
        assert!(!state.show_help);
        assert_eq!(state.preview_mode, PreviewMode::FileContent);
    }

    #[test]
    fn test_search_mode() {
        let h = make_handler();
        let mut state = AppState::new("/root".to_string());
        h.handle(&mut state, TuiEvent::ToggleSearch);
        assert!(state.search_mode);
        h.handle(&mut state, TuiEvent::SearchInput('a'));
        h.handle(&mut state, TuiEvent::SearchInput('b'));
        assert_eq!(state.search_query, "ab");
        h.handle(&mut state, TuiEvent::SearchBackspace);
        assert_eq!(state.search_query, "a");
        h.handle(&mut state, TuiEvent::SearchConfirm);
        assert!(!state.search_mode);
        assert!(state.search_query.is_empty());
    }

    #[test]
    fn test_search_cancel() {
        let h = make_handler();
        let mut state = AppState::new("/root".to_string());
        state.search_mode = true;
        state.search_query = "test".to_string();
        h.handle(&mut state, TuiEvent::SearchCancel);
        assert!(!state.search_mode);
        assert!(state.search_query.is_empty());
    }

    #[test]
    fn test_search_input_ignored_when_not_search_mode() {
        let h = make_handler();
        let mut state = AppState::new("/root".to_string());
        assert!(!state.search_mode);
        h.handle(&mut state, TuiEvent::SearchInput('x'));
        assert!(state.search_query.is_empty());
    }

    #[test]
    fn test_quit() {
        let h = make_handler();
        let mut state = AppState::new("/root".to_string());
        assert!(!state.should_quit);
        h.handle(&mut state, TuiEvent::Quit);
        assert!(state.should_quit);
    }

    #[test]
    fn test_mouse_scroll() {
        let h = make_handler();
        let mut state = AppState::new("/root".to_string());
        assert_eq!(state.scroll_offset, 0);
        h.handle(&mut state, TuiEvent::MouseScrollDown);
        assert_eq!(state.scroll_offset, 1);
        h.handle(&mut state, TuiEvent::MouseScrollUp);
        assert_eq!(state.scroll_offset, 0);
        h.handle(&mut state, TuiEvent::MouseScrollUp);
        assert_eq!(state.scroll_offset, 0);
    }

    #[test]
    fn test_load_directory_sorts_dirs_first() {
        let h = make_handler();
        let mut state = AppState::new("/root".to_string());
        h.load_directory(&mut state, "/root");
        assert!(!state.entries.is_empty());
        assert!(state.entries[0].is_dir);
        assert_eq!(state.selected_index, 0);
        assert!(state.status_message.contains("/root"));
    }

    #[test]
    fn test_action_check_updates_state() {
        let h = make_handler();
        let mut state = AppState::new("/root".to_string());
        h.handle(&mut state, TuiEvent::ActionCheck);
        assert_eq!(state.preview_mode, PreviewMode::LintResults);
        assert!(state.preview_text.contains("checked"));
        assert_eq!(state.violation_count, 3);
    }

    #[test]
    fn test_action_doctor() {
        let h = make_handler();
        let mut state = AppState::new("/root".to_string());
        h.handle(&mut state, TuiEvent::ActionDoctor);
        assert_eq!(state.preview_mode, PreviewMode::ActionOutput);
        assert!(state.preview_text.contains("healthy"));
    }

    #[test]
    fn test_path_input_and_backspace() {
        let h = make_handler();
        let mut state = AppState::new("/root".to_string());
        h.handle(&mut state, TuiEvent::PathInput('/'));
        h.handle(&mut state, TuiEvent::PathInput('v'));
        h.handle(&mut state, TuiEvent::PathInput('a'));
        h.handle(&mut state, TuiEvent::PathInput('l'));
        h.handle(&mut state, TuiEvent::PathInput('i'));
        h.handle(&mut state, TuiEvent::PathInput('d'));
        assert_eq!(state.path_input, "/valid");
        h.handle(&mut state, TuiEvent::PathBackspace);
        assert_eq!(state.path_input, "/vali");
    }

    #[test]
    fn test_path_confirm_invalid() {
        let h = make_handler();
        let mut state = AppState::new("/root".to_string());
        state.path_input = "/nonexistent".to_string();
        h.handle(&mut state, TuiEvent::PathConfirm);
        assert!(state.show_path_dialog);
        assert!(state.status_message.contains("Invalid"));
    }

    #[test]
    fn test_path_confirm_valid() {
        let h = make_handler();
        let mut state = AppState::new("/root".to_string());
        state.path_input = "/root".to_string();
        h.handle(&mut state, TuiEvent::PathConfirm);
        assert!(!state.show_path_dialog);
        assert_eq!(state.project_root, "/root");
    }

    #[test]
    fn test_empty_entries() {
        let h = make_handler();
        let mut state = AppState::new("/root".to_string());
        assert_eq!(state.selected_index, 0);
        h.handle(&mut state, TuiEvent::MoveDown);
        assert_eq!(state.selected_index, 0);
        h.handle(&mut state, TuiEvent::MoveBottom);
        assert_eq!(state.selected_index, 0);
    }
}
