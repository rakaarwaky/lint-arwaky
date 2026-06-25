use crate::contract_action_handler_protocol::IActionHandlerProtocol;
use crate::contract_tui_aggregate::ITuiAggregate;
use crate::taxonomy_state_vo::AppState;
use crate::taxonomy_tui_event::TuiEvent;
use std::sync::Arc;

pub struct TuiOrchestrator {
    action_handler: Arc<dyn IActionHandlerProtocol>,
}

impl TuiOrchestrator {
    pub fn new(action_handler: Arc<dyn IActionHandlerProtocol>) -> Self {
        Self { action_handler }
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
