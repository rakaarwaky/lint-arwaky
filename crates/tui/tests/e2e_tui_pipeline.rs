// PURPOSE: E2E tests — verify full tui pipeline from orchestrator to all components.
// Layer: E2E (full integration of all layers).

use shared::tui::contract_tui_aggregate::ITuiAggregate;
use std::sync::Arc;
use tui_lint_arwaky::agent_tui_orchestrator::TuiOrchestrator;
use tui_lint_arwaky::capabilities_action_handler::ActionHandler;
use tui_lint_arwaky::capabilities_lint_executor::LintExecutor;

fn build_full_pipeline() -> TuiOrchestrator {
    let executor = LintExecutor::new(Arc::new(
        shared::code_analysis::root_code_analysis_container::CodeAnalysisContainer::default(),
    ));
    let handler = ActionHandler::new(Arc::new(executor));
    TuiOrchestrator::new(handler)
}

// ─── E2E: Full event handling pipeline ──

#[test]
fn e2e_event_handling_pipeline() {
    let orchestrator = build_full_pipeline();
    let mut state = shared::tui::taxonomy_state_vo::AppState::default();

    // Simulate a full event sequence
    let quit_event = shared::tui::taxonomy_tui_event::TuiEvent::Key(
        shared::tui::taxonomy_tui_event::KeyEvent::normal(
            "q",
            shared::crossterm::terminal::ModifiersInformation::NONE,
        ),
    );

    // Should not panic on any event
    orchestrator.handle_event(&mut state, quit_event);
}

// ─── E2E: Directory loading pipeline ──

#[test]
fn e2e_directory_loading_pipeline() {
    let orchestrator = build_full_pipeline();
    let mut state = shared::tui::taxonomy_state_vo::AppState::default();

    // Load directory and verify it doesn't panic
    orchestrator.load_directory(&mut state, "/tmp");
}

// ─── E2E: Preview loading pipeline ──

#[test]
fn e2e_preview_loading_pipeline() {
    let orchestrator = build_full_pipeline();
    let mut state = shared::tui::taxonomy_state_vo::AppState::default();

    // Load preview and verify it doesn't panic
    orchestrator.load_preview(&mut state);
}

// ─── E2E: Watch polling pipeline ──

#[test]
fn e2e_watch_polling_pipeline() {
    let orchestrator = build_full_pipeline();
    let mut state = shared::tui::taxonomy_state_vo::AppState::default();

    // Poll watch and verify it doesn't panic
    orchestrator.poll_watch(&mut state);
}
