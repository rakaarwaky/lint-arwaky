// PURPOSE: Unit tests for ActionHandler — central event dispatch logic.
// Layer: Capabilities (ActionHandler)

use shared::tui::{AppState, TuiEvent};

use std::sync::Arc;
use tui_lint_arwaky::capabilities_action_handler::ActionHandler;
use tui_lint_arwaky::capabilities_lint_executor::LintExecutor;

fn build_handler() -> ActionHandler {
    let lint_executor = Arc::new(LintExecutor::new(
        code_analysis::root_code_analysis_container::CodeAnalysisContainer::default()
            .code_analysis_linter(),
        None,
        Arc::new(filesystem::FilesystemOrchestrator::new()),
    ));
    ActionHandler::new(
        lint_executor,
        Arc::new(filesystem::FilesystemOrchestrator::new()),
    )
}

// ─── handle: Event dispatch verification ──

#[test]
fn action_handler_handles_key_press() {
    let handler = build_handler();
    let mut state = AppState::new(".".to_string());
    let event = TuiEvent::Quit;

    handler.handle(&mut state, event);
}

#[test]
fn action_handler_handles_char_event() {
    let handler = build_handler();
    let mut state = AppState::new(".".to_string());
    let event = TuiEvent::SearchInput('q');

    handler.handle(&mut state, event);
}

// ─── load_directory: Directory loading test ──

#[test]
fn action_handler_loads_directory() {
    let handler = build_handler();
    let mut state = AppState::new(".".to_string());

    handler.load_directory(&mut state, "src");

    // Directory entries should be populated
    assert!(!state.entries.is_empty());
}

// ─── Default constructor ──

#[test]
fn action_handler_default_creates_valid_instance() {
    let lint_executor = Arc::new(LintExecutor::new(
        code_analysis::root_code_analysis_container::CodeAnalysisContainer::default()
            .code_analysis_linter(),
        None,
        Arc::new(filesystem::FilesystemOrchestrator::new()),
    ));
    let _ = ActionHandler::new(
        lint_executor,
        Arc::new(filesystem::FilesystemOrchestrator::new()),
    );
}
