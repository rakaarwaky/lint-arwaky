// Verify that all concrete types implement their declared contract traits.
use maintenance_lint_arwaky::agent_maintenance_orchestrator::MaintenanceCommandsOrchestrator;
use maintenance_lint_arwaky::capabilities_maintenance_checker::MaintenanceChecker;
use maintenance_lint_arwaky::capabilities_tool_executor_adapter::ToolExecutorAdapter;
use shared::maintenance::{
    IAdapterHealthProtocol, ICacheCleanupProtocol, IDependencyReportProtocol, IDoctorProtocol,
    IMaintenanceAggregate, IProjectStatsProtocol, ISecurityScanProtocol, ISelfUpdateProtocol,
    IToolExecutorProtocol, IToolUpdateProtocol, IToolchainDiagnosticProtocol,
};

#[test]
fn orchestrator_implements_commands_aggregate() {
    fn assert_trait<T: IMaintenanceAggregate>() {}
    assert_trait::<MaintenanceCommandsOrchestrator>();
}

#[test]
fn maintenance_checker_implements_toolchain_diagnostic_protocol() {
    fn assert_trait<T: IToolchainDiagnosticProtocol>() {}
    assert_trait::<MaintenanceChecker>();
}

#[test]
fn maintenance_checker_implements_doctor_protocol() {
    fn assert_trait<T: IDoctorProtocol>() {}
    assert_trait::<MaintenanceChecker>();
}

#[test]
fn maintenance_checker_implements_project_stats_protocol() {
    fn assert_trait<T: IProjectStatsProtocol>() {}
    assert_trait::<MaintenanceChecker>();
}

#[test]
fn maintenance_checker_implements_cache_cleanup_protocol() {
    fn assert_trait<T: ICacheCleanupProtocol>() {}
    assert_trait::<MaintenanceChecker>();
}

#[test]
fn maintenance_checker_implements_tool_update_protocol() {
    fn assert_trait<T: IToolUpdateProtocol>() {}
    assert_trait::<MaintenanceChecker>();
}

#[test]
fn maintenance_checker_implements_security_scan_protocol() {
    fn assert_trait<T: ISecurityScanProtocol>() {}
    assert_trait::<MaintenanceChecker>();
}

#[test]
fn maintenance_checker_implements_dependency_report_protocol() {
    fn assert_trait<T: IDependencyReportProtocol>() {}
    assert_trait::<MaintenanceChecker>();
}

#[test]
fn maintenance_checker_implements_adapter_health_protocol() {
    fn assert_trait<T: IAdapterHealthProtocol>() {}
    assert_trait::<MaintenanceChecker>();
}

#[test]
fn maintenance_checker_implements_self_update_protocol() {
    fn assert_trait<T: ISelfUpdateProtocol>() {}
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

#[test]
fn every_maintenance_protocol_is_arc_trait_object() {
    fn assert_arc<T: ?Sized>() {}
    assert_arc::<dyn IToolchainDiagnosticProtocol>();
    assert_arc::<dyn IDoctorProtocol>();
    assert_arc::<dyn IProjectStatsProtocol>();
    assert_arc::<dyn ICacheCleanupProtocol>();
    assert_arc::<dyn IToolUpdateProtocol>();
    assert_arc::<dyn ISecurityScanProtocol>();
    assert_arc::<dyn IDependencyReportProtocol>();
    assert_arc::<dyn IAdapterHealthProtocol>();
    assert_arc::<dyn ISelfUpdateProtocol>();
}
