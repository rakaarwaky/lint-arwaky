// PURPOSE: Benchmark — measure tui component throughput.
// Layer: Benchmark (performance validation).

use shared::tui::contract_tui_aggregate::ITuiAggregate;
use std::time::Instant;
use tui_lint_arwaky::agent_tui_orchestrator::TuiOrchestrator;
use tui_lint_arwaky::capabilities_action_handler::ActionHandler;
use tui_lint_arwaky::capabilities_lint_executor::LintExecutor;

fn build_orchestrator() -> TuiOrchestrator {
    let executor = LintExecutor::new(Arc::new(
        shared::code_analysis::root_code_analysis_container::CodeAnalysisContainer::default(),
    ));
    let handler = ActionHandler::new(Arc::new(executor));
    TuiOrchestrator::new(handler)
}

// ─── Benchmark: Component instantiation throughput ──

#[test]
fn bench_component_instantiation() {
    let start = Instant::now();
    for _ in 0..1000 {
        let _executor = LintExecutor::new(Arc::new(
            shared::code_analysis::root_code_analysis_container::CodeAnalysisContainer::default(),
        ));
        let _handler = ActionHandler::new(Arc::new(_executor));
        let _orchestrator = TuiOrchestrator::new(_handler);
    }
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_millis() < 1000,
        "1000 component instantiations took {}ms",
        elapsed.as_millis()
    );
}

// ─── Benchmark: Event handling throughput ──

#[test]
fn bench_event_handling() {
    let orchestrator = build_orchestrator();
    let mut state = shared::tui::taxonomy_state_vo::AppState::default();

    let event = shared::tui::taxonomy_tui_event::TuiEvent::Key(
        shared::tui::taxonomy_tui_event::KeyEvent::normal(
            "q",
            shared::crossterm::terminal::ModifiersInformation::NONE,
        ),
    );

    let start = Instant::now();
    for _ in 0..1000 {
        orchestrator.handle_event(&mut state, event.clone());
    }
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_millis() < 5000,
        "1000 event handles took {}ms",
        elapsed.as_millis()
    );
}

// ─── Benchmark: Directory loading throughput ──

#[test]
fn bench_directory_loading() {
    let orchestrator = build_orchestrator();
    let mut state = shared::tui::taxonomy_state_vo::AppState::default();

    let start = Instant::now();
    for _ in 0..100 {
        orchestrator.load_directory(&mut state, "/tmp");
    }
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_millis() < 5000,
        "Directory loading took {}ms",
        elapsed.as_millis()
    );
}

// ─── Benchmark: Preview loading throughput ──

#[test]
fn bench_preview_loading() {
    let orchestrator = build_orchestrator();
    let mut state = shared::tui::taxonomy_state_vo::AppState::default();

    let start = Instant::now();
    for _ in 0..100 {
        orchestrator.load_preview(&mut state);
    }
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_millis() < 5000,
        "Preview loading took {}ms",
        elapsed.as_millis()
    );
}

// ─── Benchmark: Full pipeline throughput ──

#[test]
fn bench_full_pipeline() {
    let orchestrator = build_orchestrator();
    let mut state = shared::tui::taxonomy_state_vo::AppState::default();

    let start = Instant::now();
    for _ in 0..50 {
        orchestrator.handle_event(
            &mut state,
            shared::tui::taxonomy_tui_event::TuiEvent::Key(
                shared::tui::taxonomy_tui_event::KeyEvent::normal(
                    "j",
                    shared::crossterm::terminal::ModifiersInformation::NONE,
                ),
            ),
        );
        orchestrator.load_directory(&mut state, "/tmp");
        orchestrator.load_preview(&mut state);
    }
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_millis() < 10000,
        "Full pipeline took {}ms",
        elapsed.as_millis()
    );
}
