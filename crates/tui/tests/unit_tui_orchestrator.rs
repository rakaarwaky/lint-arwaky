// PURPOSE: Unit tests for TuiOrchestrator — agent layer orchestration.
// Layer: Agent (TuiOrchestrator)

use shared::tui::contract_tui_aggregate::ITuiAggregate;
use shared::tui::taxonomy_state_vo::AppState;
use shared::tui::taxonomy_tui_event::TuiEvent;
use std::sync::Arc;
use tui_lint_arwaky::agent_tui_orchestrator::TuiOrchestrator;
use tui_lint_arwaky::capabilities_action_handler::ActionHandler;
use tui_lint_arwaky::capabilities_lint_executor::LintExecutor;

fn build_orchestrator() -> TuiOrchestrator {
    let handler = Arc::new(ActionHandler::new(Arc::new(LintExecutor::new(Arc::new(
        shared::code_analysis::root_code_analysis_container::CodeAnalysisContainer::default(),
    )))));
    TuiOrchestrator::new(handler)
}

// ─── Verify orchestrator implements ITuiAggregate ──

#[test]
fn tui_orchestrator_implements_aggregate() {
    let orch = build_orchestrator();
    let _: &dyn ITuiAggregate = &orch;
}

// ─── handle_event: Event handling delegation ──

#[test]
fn orchestrator_handles_event() {
    let orch = build_orchestrator();
    let mut state = AppState::default();
    let event = TuiEvent::Key(shared::tui::taxonomy_tui_event::KeyEvent::normal(
        "q",
        shared::crossterm::terminal::ModifiersInformation::NONE,
    ));

    // Should not panic on valid event
    orch.handle_event(&mut state, event);
}

// ─── load_directory: Directory loading delegation ──

#[test]
fn orchestrator_loads_directory() {
    let orch = build_orchestrator();
    let mut state = AppState::default();

    // Should handle directory loading without panic
    orch.load_directory(&mut state, "/tmp");
}

// ─── load_preview: Preview loading delegation ──

#[test]
fn orchestrator_loads_preview() {
    let orch = build_orchestrator();
    let mut state = AppState::default();

    // Should handle preview loading without panic
    orch.load_preview(&mut state);
}

// ─── poll_watch: Watch polling delegation ──

#[test]
fn orchestrator_polls_watch() {
    let orch = build_orchestrator();
    let mut state = AppState::default();

    // Should handle watch polling without panic
    orch.poll_watch(&mut state);
}

// ─── Default trait ──

#[test]
fn tui_orchestrator_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<TuiOrchestrator>();
}
