// PURPOSE: Smoke test — verify the tui crate boots and responds within 5 seconds.
// Layer: Smoke (must complete < 5s).

use std::sync::Arc;
use tui_lint_arwaky::agent_tui_orchestrator::TuiOrchestrator;
use tui_lint_arwaky::capabilities_action_handler::ActionHandler;
use tui_lint_arwaky::capabilities_lint_executor::LintExecutor;
use shared::tui::contract_tui_aggregate::ITuiAggregate;
use shared::tui::taxonomy_state_vo::AppState;
use shared::tui::taxonomy_tui_event::TuiEvent;

#[test]
fn smoke_tui_crate_boots_and_responds() {
    // 1. All components instantiate without panic
    let executor = LintExecutor::new(Arc::new(
        code_analysis::root_code_analysis_container::CodeAnalysisContainer::default(),
    ));
    let handler = Arc::new(ActionHandler::new(Arc::new(executor)));

    // 2. Orchestrator instantiates
    let orchestrator = TuiOrchestrator::new(handler);

    // 3. Methods respond without panic
    let mut state = AppState::new(".".to_string());
    let event = TuiEvent::Key(crossterm::event::KeyEvent::from(
        crossterm::event::KeyCode::Char('q'),
    ));

    orchestrator.handle_event(&mut state, event);
}
