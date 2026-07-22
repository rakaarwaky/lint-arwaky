// PURPOSE: Unit tests for ActionHandler — central event dispatch logic.
// Layer: Capabilities (ActionHandler)

use shared::tui::contract_action_handler_protocol::IActionHandlerProtocol;
use shared::tui::taxonomy_state_vo::AppState;
use shared::tui::taxonomy_tui_event::TuiEvent;
use std::sync::Arc;
use tui_lint_arwaky::capabilities_action_handler::ActionHandler;
use tui_lint_arwaky::capabilities_lint_executor::LintExecutor;

fn build_handler() -> ActionHandler {
    let lint_executor = Arc::new(LintExecutor::new(
        code_analysis::root_code_analysis_container::CodeAnalysisContainer::default()
            .code_analysis_linter(),
    ));
    ActionHandler::new(lint_executor)
}

// ─── handle: Event dispatch verification ──

#[test]
fn action_handler_handles_key_press() {
    let handler = build_handler();
    let mut state = AppState::new(".".to_string());
    let event = TuiEvent::Quit;

    // Should not panic on valid event
    handler.handle(&mut state, event);
}

// ─── handle: Navigation events ──

#[test]
fn action_handler_handles_navigation() {
    let handler = build_handler();
    let mut state = AppState::new(".".to_string());

    // Test up/down navigation
    let event_up = TuiEvent::MoveUp;
    handler.handle(&mut state, event_up);

    let event_down = TuiEvent::MoveDown;
    handler.handle(&mut state, event_down);
}

// ─── load_directory: Path validation ──

#[test]
fn action_handler_loads_valid_directory() {
    let handler = build_handler();
    let mut state = AppState::new(".".to_string());

    // Should handle directory loading without panic
    handler.load_directory(&mut state, "/tmp");
}

// ─── load_preview: Preview mode toggle ──

#[test]
fn action_handler_loads_preview() {
    let handler = build_handler();
    let mut state = AppState::new(".".to_string());

    // Should handle preview loading without panic
    handler.load_preview(&mut state);
}

// ─── Default constructor ──

#[test]
fn action_handler_default_creates_valid_instance() {
    let lint_executor = Arc::new(LintExecutor::new(
        code_analysis::root_code_analysis_container::CodeAnalysisContainer::default()
            .code_analysis_linter(),
    ));
    let _ = ActionHandler::new(lint_executor);
}
