// PURPOSE: MaintenanceCommandsSurface — maintenance business logic, no formatting.
// Delegates all operations through IMaintenanceAggregate.
// No direct std::process::Command or filesystem I/O — aggregate handles subprocess execution.
use shared_common::FilePath;
use shared_maintenance::MaintenanceRequest;
use shared_maintenance::{
    DependencyReport, HealthCheckResult, IMaintenanceAggregate, SecurityScanReport,
    SelfUpdateResultVO, ToolchainDiagnostics,
};
use std::sync::Arc;

pub fn collect_doctor(maintenance: Arc<dyn IMaintenanceAggregate>) -> ToolchainDiagnostics {
    maintenance
        .execute(MaintenanceRequest::diagnose_toolchain())
        .into_toolchain()
}

pub fn collect_security(
    maintenance: Arc<dyn IMaintenanceAggregate>,
    path: Option<FilePath>,
) -> Result<SecurityScanReport, String> {
    let target = match &path {
        Some(p) => p.value().to_string(),
        None => ".".to_string(),
    };
    let fp = FilePath::new(target).map_err(|_| "invalid path".to_string())?;
    Ok(maintenance
        .execute(MaintenanceRequest::security_scan(&fp))
        .into_security_report())
}

pub fn collect_dependencies(
    maintenance: Arc<dyn IMaintenanceAggregate>,
    path: Option<FilePath>,
) -> Result<DependencyReport, String> {
    let target = match &path {
        Some(p) => p.value().to_string(),
        None => ".".to_string(),
    };
    let fp = FilePath::new(target).map_err(|_| "invalid path".to_string())?;
    maintenance
        .execute(MaintenanceRequest::dependency_report(&fp))
        .into_dependency_report()
        .map_err(|e| format!("Error: {e}"))
}

pub fn collect_health_check(maintenance: Arc<dyn IMaintenanceAggregate>) -> HealthCheckResult {
    maintenance
        .execute(MaintenanceRequest::health_check())
        .into_health()
}

pub fn collect_self_update(
    maintenance: Arc<dyn IMaintenanceAggregate>,
    check_only: bool,
) -> SelfUpdateResultVO {
    maintenance
        .execute(MaintenanceRequest::self_update(check_only))
        .into_self_update()
}
