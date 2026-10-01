// Verify that all concrete types implement their declared contract traits.
use maintenance_lint_arwaky::agent_maintenance_orchestrator::MaintenanceCommandsOrchestrator;
use maintenance_lint_arwaky::{
    AdapterHealthChecker, CacheCleanupChecker, DependencyReportChecker, DoctorChecker,
    ProjectStatsChecker, SecurityScanChecker, SelfUpdateChecker, ToolUpdateChecker,
};
use shared_maintenance::{
    IAdapterHealthProtocol, ICacheCleanupProtocol, IDependencyReportProtocol, IDoctorProtocol,
    IMaintenanceAggregate, IProjectStatsProtocol, ISecurityScanProtocol, ISelfUpdateProtocol,
    IToolUpdateProtocol,
};

#[test]
fn orchestrator_implements_commands_aggregate() {
    fn assert_trait<T: IMaintenanceAggregate>() {}
    assert_trait::<MaintenanceCommandsOrchestrator>();
}

#[test]
fn doctor_checker_serves_doctor_and_toolchain() {
    fn assert_trait<T: IDoctorProtocol>() {}
    assert_trait::<DoctorChecker>();
}

#[test]
fn project_stats_checker_implements_project_stats_protocol() {
    fn assert_trait<T: IProjectStatsProtocol>() {}
    assert_trait::<ProjectStatsChecker>();
}

#[test]
fn cache_cleanup_checker_implements_cache_cleanup_protocol() {
    fn assert_trait<T: ICacheCleanupProtocol>() {}
    assert_trait::<CacheCleanupChecker>();
}

#[test]
fn tool_update_checker_implements_tool_update_protocol() {
    fn assert_trait<T: IToolUpdateProtocol>() {}
    assert_trait::<ToolUpdateChecker>();
}

#[test]
fn security_scan_checker_implements_security_scan_protocol() {
    fn assert_trait<T: ISecurityScanProtocol>() {}
    assert_trait::<SecurityScanChecker>();
}

#[test]
fn dependency_report_checker_implements_dependency_report_protocol() {
    fn assert_trait<T: IDependencyReportProtocol>() {}
    assert_trait::<DependencyReportChecker>();
}

#[test]
fn adapter_health_checker_implements_adapter_health_protocol() {
    fn assert_trait<T: IAdapterHealthProtocol>() {}
    assert_trait::<AdapterHealthChecker>();
}

#[test]
fn self_update_checker_implements_self_update_protocol() {
    fn assert_trait<T: ISelfUpdateProtocol>() {}
    assert_trait::<SelfUpdateChecker>();
}

#[test]
fn all_contracts_are_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<MaintenanceCommandsOrchestrator>();
    assert_send_sync::<DoctorChecker>();
    assert_send_sync::<ProjectStatsChecker>();
    assert_send_sync::<CacheCleanupChecker>();
    assert_send_sync::<ToolUpdateChecker>();
    assert_send_sync::<SecurityScanChecker>();
    assert_send_sync::<DependencyReportChecker>();
    assert_send_sync::<AdapterHealthChecker>();
    assert_send_sync::<SelfUpdateChecker>();
}

#[test]
fn every_maintenance_protocol_is_arc_trait_object() {
    fn assert_arc<T: ?Sized>() {}
    assert_arc::<dyn IDoctorProtocol>();
    assert_arc::<dyn IProjectStatsProtocol>();
    assert_arc::<dyn ICacheCleanupProtocol>();
    assert_arc::<dyn IToolUpdateProtocol>();
    assert_arc::<dyn ISecurityScanProtocol>();
    assert_arc::<dyn IDependencyReportProtocol>();
    assert_arc::<dyn IAdapterHealthProtocol>();
    assert_arc::<dyn ISelfUpdateProtocol>();
}
