// PURPOSE: MaintenanceCommandsSurface — maintenance business logic, no formatting.
// Delegates all operations through MaintenanceCommandsAggregate.
// No direct std::process::Command or filesystem I/O — aggregate handles subprocess execution.
use shared::common::FilePath;
use shared::maintenance::{
    DependencyReport, HealthCheckResult, MaintenanceCommandsAggregate, SecurityScanReport,
    ToolchainDiagnostics,
};
use std::sync::Arc;

pub fn collect_doctor(maintenance: Arc<dyn MaintenanceCommandsAggregate>) -> ToolchainDiagnostics {
    maintenance.diagnose_toolchain()
}

pub fn collect_security(
    maintenance: Arc<dyn MaintenanceCommandsAggregate>,
    path: Option<FilePath>,
) -> Result<SecurityScanReport, String> {
    let target = match &path {
        Some(p) => p.value().to_string(),
        None => ".".to_string(),
    };
    let fp = FilePath::new(target).map_err(|_| "invalid path".to_string())?;
    Ok(maintenance.run_security_scan(&fp))
}

pub fn collect_dependencies(
    maintenance: Arc<dyn MaintenanceCommandsAggregate>,
    path: Option<FilePath>,
) -> Result<DependencyReport, String> {
    let target = match &path {
        Some(p) => p.value().to_string(),
        None => ".".to_string(),
    };
    let fp = FilePath::new(target).map_err(|_| "invalid path".to_string())?;
    maintenance
        .run_dependency_report(&fp)
        .map_err(|e| format!("Error: {e}"))
}
