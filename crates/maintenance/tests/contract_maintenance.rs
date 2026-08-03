// Verify that all concrete types implement their declared contract traits.
use maintenance_lint_arwaky::agent_maintenance_orchestrator::MaintenanceCommandsOrchestrator;
use maintenance_lint_arwaky::capabilities_maintenance_checker::MaintenanceChecker;
use maintenance_lint_arwaky::capabilities_tool_executor_adapter::ToolExecutorAdapter;
use shared::maintenance::{
    IMaintenanceCheckerProtocol, IToolExecutorProtocol, MaintenanceCommandsAggregate,
};

#[test]
fn orchestrator_implements_commands_aggregate() {
    fn assert_trait<T: MaintenanceCommandsAggregate>() {}
    assert_trait::<MaintenanceCommandsOrchestrator>();
}

#[test]
fn maintenance_checker_implements_checker_protocol() {
    fn assert_trait<T: IMaintenanceCheckerProtocol>() {}
    assert_trait::<MaintenanceChecker>();
}

#[test]
fn tool_executor_adapter_implements_executor_protocol() {
    fn assert_trait<T: IToolExecutorProtocol>() {}
    assert_trait::<ToolExecutorAdapter>();
}

#[test]
fn all_contracts_are_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<MaintenanceCommandsOrchestrator>();
    assert_send_sync::<MaintenanceChecker>();
    assert_send_sync::<ToolExecutorAdapter>();
}
