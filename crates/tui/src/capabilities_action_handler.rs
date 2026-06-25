use shared::common::taxonomy_line_count_vo::LineCount;
use shared::source_parsing::taxonomy_path_vo::FilePath;
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
