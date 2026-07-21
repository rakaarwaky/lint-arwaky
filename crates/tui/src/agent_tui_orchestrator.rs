// PURPOSE: Orchestrator (Agent layer) that implements ITuiAggregate.
// Delegates all events/commands to the ActionHandler (Capabilities layer).
// This is the top-level aggregate that mediates between the TUI surface and business logic.

use shared::tui::contract_action_handler_protocol::IActionHandlerProtocol;
use shared::tui::contract_tui_aggregate::ITuiAggregate;
use shared::tui::taxonomy_scan_update_vo::ScanUpdate;
use shared::tui::taxonomy_state_vo::AppState;
use shared::tui::taxonomy_tui_event::TuiEvent;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────
/// TuiOrchestrator — the agent-level aggregate for the TUI.
/// Wraps an IActionHandlerProtocol and forwards all ITuiAggregate calls to it.
pub struct TuiOrchestrator {
    action_handler: Arc<dyn IActionHandlerProtocol>,
}

// ─── Block 2: Aggregate Trait Implementation ──────────────
impl ITuiAggregate for TuiOrchestrator {
    /// Handle a TuiEvent by delegating to the action handler's state machine.
    fn handle_event(&self, state: &mut AppState, event: TuiEvent) {
        self.action_handler.handle(state, event);
    }

    /// Load a directory listing by delegating to the action handler.
    fn load_directory(&self, state: &mut AppState, path: &str) {
        self.action_handler.load_directory(state, path);
    }

    /// Load file preview by delegating to the action handler.
    fn load_preview(&self, state: &mut AppState) {
        self.action_handler.load_preview(state);
    }

    fn poll_watch(&self, state: &mut AppState) {
        self.action_handler.poll_watch(state);
    }

    fn start_scan(&self, state: &mut AppState) -> Option<std::sync::mpsc::Receiver<ScanUpdate>> {
        self.action_handler.start_scan(state)
    }

    fn poll_scan(&self, state: &mut AppState, rx: &std::sync::mpsc::Receiver<ScanUpdate>) {
        self.action_handler.poll_scan(state, rx);
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────
impl TuiOrchestrator {
    pub fn new(action_handler: Arc<dyn IActionHandlerProtocol>) -> Self {
        Self { action_handler }
    }
}
