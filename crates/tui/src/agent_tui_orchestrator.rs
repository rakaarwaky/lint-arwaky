use crate::capabilities_action_handler::ActionHandler;
use crate::contract_file_system_port::IFileSystemPort;
use crate::contract_lint_executor_port::ILintExecutorPort;
use crate::contract_tui_aggregate::ITuiAggregate;
use crate::taxonomy_state_vo::AppState;
use crate::taxonomy_tui_event::TuiEvent;
use std::sync::Arc;

pub struct TuiOrchestrator {
    action_handler: ActionHandler,
}

impl TuiOrchestrator {
    pub fn new(fs_port: Arc<dyn IFileSystemPort>, lint_port: Arc<dyn ILintExecutorPort>) -> Self {
        Self {
            action_handler: ActionHandler::new(fs_port, lint_port),
        }
    }
}

impl ITuiAggregate for TuiOrchestrator {
    fn handle_event(&self, state: &mut AppState, event: TuiEvent) {
        self.action_handler.handle(state, event);
    }

    fn load_directory(&self, state: &mut AppState, path: &str) {
        self.action_handler.load_directory(state, path);
    }

    fn load_preview(&self, state: &mut AppState) {
        self.action_handler.load_preview(state);
    }
}
