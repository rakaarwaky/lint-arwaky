// PURPOSE: Acceptance tests — verify FRD requirements for tui.
// Layer: Acceptance (FRD requirement validation).

use shared::tui::contract_tui_aggregate::ITuiAggregate;
use shared::tui::taxonomy_state_vo::AppState;
use shared::tui::taxonomy_tui_event::TuiEvent;
use std::sync::Arc;
use tui_lint_arwaky::agent_tui_orchestrator::TuiOrchestrator;
use tui_lint_arwaky::capabilities_action_handler::ActionHandler;
use tui_lint_arwaky::capabilities_lint_executor::LintExecutor;

fn build_orchestrator() -> TuiOrchestrator {
    let executor = Arc::new(LintExecutor::new(
        code_analysis::root_code_analysis_container::CodeAnalysisContainer::default()
            .code_analysis_linter(),
    ));
    let handler = Arc::new(ActionHandler::new(executor));
    TuiOrchestrator::new(handler)
}

// ─── Acceptance: TUI handles keyboard events ──

#[test]
fn acceptance_tui_handles_quit_key() {
    // FRD requirement: TUI must handle quit key (q/Q)
    let orchestrator = build_orchestrator();
    let mut state = AppState::new(".".to_string());

    let quit_event = TuiEvent::Quit;

    // Should not panic
    orchestrator.handle_event(&mut state, quit_event);
}

// ─── Acceptance: TUI handles navigation keys ──

#[test]
fn acceptance_tui_handles_navigation_keys() {
    // FRD requirement: TUI must handle navigation (j/k/arrow keys)
    let orchestrator = build_orchestrator();
    let mut state = AppState::new(".".to_string());

    let up_event = TuiEvent::MoveUp;
    orchestrator.handle_event(&mut state, up_event);

    let down_event = TuiEvent::MoveDown;
    orchestrator.handle_event(&mut state, down_event);
}

// ─── Acceptance: TUI handles mouse events ──

#[test]
fn acceptance_tui_handles_mouse_events() {
    // FRD requirement: TUI must handle mouse clicks and dragging
    let orchestrator = build_orchestrator();
    let mut state = AppState::new(".".to_string());

    // Mouse click event
    let click_event = TuiEvent::MouseClick(10, 10);

    // Should not panic on mouse event
    orchestrator.handle_event(&mut state, click_event);
}

// ─── Acceptance: TUI handles directory loading ──

#[test]
fn acceptance_tui_loads_directory() {
    // FRD requirement: TUI must load and display directory contents
    let orchestrator = build_orchestrator();
    let mut state = AppState::new(".".to_string());

    // Should handle directory loading without panic
    orchestrator.load_directory(&mut state, "/tmp");
}

// ─── Acceptance: TUI handles preview mode ──

#[test]
fn acceptance_tui_loads_preview() {
    // FRD requirement: TUI must load and display file preview
    let orchestrator = build_orchestrator();
    let mut state = AppState::new(".".to_string());

    // Should handle preview loading without panic
    orchestrator.load_preview(&mut state);
}

// ─── Acceptance: TUI handles watch polling ──

#[test]
fn acceptance_tui_polls_watch() {
    // FRD requirement: TUI must poll file watch for changes
    let orchestrator = build_orchestrator();
    let mut state = AppState::new(".".to_string());

    // Should handle watch polling without panic
    orchestrator.poll_watch(&mut state);
}
