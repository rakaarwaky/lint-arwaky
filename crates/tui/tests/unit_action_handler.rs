// PURPOSE: Unit tests for ActionHandler — central event dispatch logic.
// Layer: Capabilities (ActionHandler)

use shared::tui::contract_action_handler_protocol::IActionHandlerProtocol;
use std::sync::Arc;
use tui_lint_arwaky::capabilities_action_handler::ActionHandler;
use tui_lint_arwaky::capabilities_lint_executor::LintExecutor;

fn build_handler() -> ActionHandler {
    let lint_executor = Arc::new(LintExecutor::new(Arc::new(
        shared::code_analysis::root_code_analysis_container::CodeAnalysisContainer::default(),
    )));
    ActionHandler::new(lint_executor)
}

// ─── handle: Event dispatch verification ──

#[test]
fn action_handler_handles_key_press() {
    let handler = build_handler();
    let mut state = shared::tui::taxonomy_state_vo::AppState::default();
    let event = shared::tui::taxonomy_tui_event::TuiEvent::Key(
        shared::tui::taxonomy_tui_event::KeyEvent::normal(
            "q",
            shared::crossterm::terminal::ModifiersInformation::NONE,
        ),
    );

    // Should not panic on valid event
    handler.handle(&mut state, event);
}

// ─── handle: Navigation events ──

#[test]
fn action_handler_handles_navigation() {
    let handler = build_handler();
    let mut state = shared::tui::taxonomy_state_vo::AppState::default();

    // Test up/down navigation
    let event_up = shared::tui::taxonomy_tui_event::TuiEvent::Key(
        shared::tui::taxonomy_tui_event::KeyEvent::normal(
            "k",
            shared::crossterm::terminal::ModifiersInformation::NONE,
        ),
    );
    handler.handle(&mut state, event_up);

    let event_down = shared::tui::taxonomy_tui_event::TuiEvent::Key(
        shared::tui::taxonomy_tui_event::KeyEvent::normal(
            "j",
            shared::crossterm::terminal::ModifiersInformation::NONE,
        ),
    );
    handler.handle(&mut state, event_down);
}

// ─── load_directory: Path validation ──

#[test]
fn action_handler_loads_valid_directory() {
    let handler = build_handler();
    let mut state = shared::tui::taxonomy_state_vo::AppState::default();

    // Should handle directory loading without panic
    handler.load_directory(&mut state, "/tmp");
}

// ─── load_preview: Preview mode toggle ──

#[test]
fn action_handler_loads_preview() {
    let handler = build_handler();
    let mut state = shared::tui::taxonomy_state_vo::AppState::default();

    // Should handle preview loading without panic
    handler.load_preview(&mut state);
}

// ─── copy_to_clipboard: Clipboard operation ──

#[test]
fn action_handler_copies_to_clipboard() {
    let handler = build_handler();
    let mut state = shared::tui::taxonomy_state_vo::AppState::default();

    // Should handle clipboard copy without panic
    handler.copy_to_clipboard(&mut state);
}

// ─── Default trait ──

#[test]
fn action_handler_default_creates_valid_instance() {
    let lint_executor = Arc::new(LintExecutor::new(Arc::new(
        shared::code_analysis::root_code_analysis_container::CodeAnalysisContainer::default(),
    )));
    let _ = ActionHandler::default(lint_executor);
}
