// PURPOSE: Smoke test — verify the tui crate boots and responds within 5 seconds.
// Layer: Smoke (must complete < 5s).

use shared::tui::contract_tui_aggregate::ITuiAggregate;
use shared::tui::taxonomy_state_vo::AppState;
use shared::tui::taxonomy_tui_event::TuiEvent;
use std::sync::Arc;
use tui_lint_arwaky::agent_tui_orchestrator::TuiOrchestrator;
use tui_lint_arwaky::capabilities_action_handler::ActionHandler;
use tui_lint_arwaky::capabilities_lint_executor::LintExecutor;

#[test]
fn smoke_tui_crate_boots_and_responds() {
    // 1. All components instantiate without panic
    let executor = Arc::new(LintExecutor::new(
        code_analysis::root_code_analysis_container::CodeAnalysisContainer::default()
            .code_analysis_linter(),
    ));
    let handler = Arc::new(ActionHandler::new(executor));

    // 2. Orchestrator instantiates
    let orchestrator = TuiOrchestrator::new(handler);

    // 3. Methods respond without panic
    let mut state = AppState::new(".".to_string());
    let event = TuiEvent::Quit;

    orchestrator.handle_event(&mut state, event);
}
