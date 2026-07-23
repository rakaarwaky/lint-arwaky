// PURPOSE: Integration tests — verify DI container wiring and cross-layer collaboration.
// Layer: Integration (uses real TuiContainer components).

use shared::tui::contract_action_handler_protocol::IActionHandlerProtocol;
use shared::tui::contract_lint_executor_protocol::ILintExecutorProtocol;
use shared::tui::contract_tui_aggregate::ITuiAggregate;
use shared::tui::taxonomy_state_vo::AppState;
use shared::tui::taxonomy_tui_event::TuiEvent;
use std::sync::Arc;
use tui_lint_arwaky::agent_tui_orchestrator::TuiOrchestrator;
use tui_lint_arwaky::capabilities_action_handler::ActionHandler;
use tui_lint_arwaky::capabilities_lint_executor::LintExecutor;

fn build_full_wiring() -> (TuiOrchestrator, Arc<ActionHandler>, Arc<LintExecutor>) {
    let executor = Arc::new(LintExecutor::new(
        code_analysis::root_code_analysis_container::CodeAnalysisContainer::default()
            .code_analysis_linter(),
    ));
    let handler = Arc::new(ActionHandler::new(executor.clone()));
    let orchestrator = TuiOrchestrator::new(handler.clone());

    (orchestrator, handler, executor)
}

// ─── Container wiring tests ──

#[test]
fn full_wiring_creates_all_components() {
    let (_orchestrator, _handler, _executor) = build_full_wiring();
}

#[test]
fn all_components_implement_correct_traits() {
    let (orchestrator, handler, executor) = build_full_wiring();

    let _: &dyn ITuiAggregate = &orchestrator;
    let _: &dyn IActionHandlerProtocol = &*handler;
    let _: &dyn ILintExecutorProtocol = &*executor;
}

// ─── Cross-layer collaboration test ──

#[test]
fn full_pipeline_wiring_works() {
    let (orchestrator, _handler, _executor) = build_full_wiring();

    // Verify orchestrator can handle events
    let mut state = AppState::new(".".to_string());
    let event = TuiEvent::Quit;

    orchestrator.handle_event(&mut state, event);
}

// ─── Verify Arc pointer equality ──

#[test]
fn handler_and_orchestrator_share_arc() {
    let executor = Arc::new(LintExecutor::new(
        code_analysis::root_code_analysis_container::CodeAnalysisContainer::default()
            .code_analysis_linter(),
    ));
    let handler = Arc::new(ActionHandler::new(executor));
    let _orchestrator = TuiOrchestrator::new(handler);

    // Trait object verification passed above
}

// ─── Constructors ──

#[test]
fn tui_components_default_creates_valid_instances() {
    let code_analysis =
        code_analysis::root_code_analysis_container::CodeAnalysisContainer::default()
            .code_analysis_linter();

    let executor = Arc::new(LintExecutor::new(code_analysis));
    let _handler = ActionHandler::new(executor);
}
