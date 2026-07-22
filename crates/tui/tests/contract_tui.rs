// PURPOSE: Contract tests — verify all trait implementations for tui types.
// Layer: Contract (trait verification).

use shared::tui::contract_action_handler_protocol::IActionHandlerProtocol;
use shared::tui::contract_lint_executor_protocol::ILintExecutorProtocol;
use shared::tui::contract_tui_aggregate::ITuiAggregate;
use std::sync::Arc;
use tui_lint_arwaky::agent_tui_orchestrator::TuiOrchestrator;
use tui_lint_arwaky::capabilities_action_handler::ActionHandler;
use tui_lint_arwaky::capabilities_lint_executor::LintExecutor;

// ─── Verify ActionHandler implements IActionHandlerProtocol ──

#[test]
fn action_handler_implements_protocol() {
    let lint_executor = Arc::new(LintExecutor::new(Arc::new(
        shared::code_analysis::root_code_analysis_container::CodeAnalysisContainer::default(),
    )));
    let handler = ActionHandler::new(lint_executor);
    let _: &dyn IActionHandlerProtocol = &handler;
}

// ─── Verify LintExecutor implements ILintExecutorProtocol ──

#[test]
fn lint_executor_implements_protocol() {
    let executor = LintExecutor::new(Arc::new(
        shared::code_analysis::root_code_analysis_container::CodeAnalysisContainer::default(),
    ));
    let _: &dyn ILintExecutorProtocol = &executor;
}

// ─── Verify TuiOrchestrator implements ITuiAggregate ──

#[test]
fn tui_orchestrator_implements_aggregate() {
    let handler = Arc::new(ActionHandler::new(Arc::new(LintExecutor::new(Arc::new(
        shared::code_analysis::root_code_analysis_container::CodeAnalysisContainer::default(),
    )))));
    let orchestrator = TuiOrchestrator::new(handler);
    let _: &dyn ITuiAggregate = &orchestrator;
}

// ─── Verify all public methods are accessible via traits ──

#[test]
fn action_handler_all_methods_accessible() {
    let lint_executor = Arc::new(LintExecutor::new(Arc::new(
        shared::code_analysis::root_code_analysis_container::CodeAnalysisContainer::default(),
    )));
    let handler = ActionHandler::new(lint_executor);

    // Verify all trait methods are accessible
    let _: &dyn IActionHandlerProtocol = &handler;
}

#[test]
fn lint_executor_all_methods_accessible() {
    let executor = LintExecutor::new(Arc::new(
        shared::code_analysis::root_code_analysis_container::CodeAnalysisContainer::default(),
    ));

    // Verify all trait methods are accessible
    let _: &dyn ILintExecutorProtocol = &executor;
}

#[test]
fn tui_orchestrator_all_methods_accessible() {
    let handler = Arc::new(ActionHandler::new(Arc::new(LintExecutor::new(Arc::new(
        shared::code_analysis::root_code_analysis_container::CodeAnalysisContainer::default(),
    )))));
    let orchestrator = TuiOrchestrator::new(handler);

    // Verify all trait methods are accessible
    let _: &dyn ITuiAggregate = &orchestrator;
}
