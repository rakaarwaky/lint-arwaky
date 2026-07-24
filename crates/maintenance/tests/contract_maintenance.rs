// PURPOSE: Verify that all public structs implement their required contract traits.
// Layer: Contract verification — runs in ms, every PR.

use maintenance_lint_arwaky::agent_maintenance_orchestrator::MaintenanceCommandsOrchestrator;
use maintenance_lint_arwaky::capabilities_maintenance_checker::MaintenanceChecker;
use maintenance_lint_arwaky::capabilities_tool_executor_adapter::ToolExecutorAdapter;
use maintenance_lint_arwaky::root_maintenance_container::MaintenanceContainer;

use shared::maintenance::contract_maintenance_aggregate::MaintenanceCommandsAggregate;
use shared::maintenance::contract_maintenance_protocol::IMaintenanceCheckerProtocol;
use shared::project_setup::contract_tool_executor_protocol::IToolExecutorProtocol;

// ─── MaintenanceChecker implements IMaintenanceCheckerProtocol ───

#[test]
fn maintenance_checker_implements_i_maintenance_checker_protocol() {
    fn assert_trait<T: IMaintenanceCheckerProtocol>() {}
    assert_trait::<MaintenanceChecker>();
}

#[test]
fn maintenance_checker_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<MaintenanceChecker>();
}

// ─── ToolExecutorAdapter implements IToolExecutorProtocol ───

#[test]
fn tool_executor_adapter_implements_i_tool_executor_protocol() {
    fn assert_trait<T: IToolExecutorProtocol>() {}
    assert_trait::<ToolExecutorAdapter>();
}

#[test]
fn tool_executor_adapter_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<ToolExecutorAdapter>();
}

// ─── MaintenanceCommandsOrchestrator implements MaintenanceCommandsAggregate ───

#[test]
fn orchestrator_implements_maintenance_commands_aggregate() {
    fn assert_trait<T: MaintenanceCommandsAggregate>() {}
    assert_trait::<MaintenanceCommandsOrchestrator>();
}

#[test]
fn orchestrator_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<MaintenanceCommandsOrchestrator>();
}

// ─── MaintenanceContainer wiring ───

#[test]
fn container_exposes_aggregate_as_trait_object() {
    let container = MaintenanceContainer::new();
    let orchestrator = container.orchestrator();
    // Verify the Arc<dyn MaintenanceCommandsAggregate> is usable
    let _ref: &dyn MaintenanceCommandsAggregate = orchestrator.as_ref();
}

// ─── Default trait implementations ───

#[test]
fn maintenance_checker_implements_default() {
    let checker = MaintenanceChecker;
    let _ = checker;
}

#[test]
fn tool_executor_adapter_implements_default() {
    let adapter = ToolExecutorAdapter;
    let _ = adapter;
}

#[test]
fn container_implements_default() {
    let container = MaintenanceContainer::default();
    let _ = container;
}
