// PURPOSE: IMaintenanceCheckerProtocol — protocol for maintenance checker capabilities
use crate::common::taxonomy_path_vo::FilePath;
use crate::maintenance::taxonomy_doctor_vo::{
    DependencyReport, DoctorResultVO, HealthCheckResult, SecurityScanReport, ToolchainDiagnostics,
};
use crate::maintenance::taxonomy_stats_vo::MaintenanceStatsVO;

pub trait IMaintenanceCheckerProtocol: Send + Sync {
    fn diagnose_toolchain(&self) -> ToolchainDiagnostics;
    fn health_check(&self) -> HealthCheckResult;
    fn run_security_scan(&self, project_path: &FilePath) -> SecurityScanReport;
    fn run_dependency_report(&self, project_path: &FilePath) -> Result<DependencyReport, String>;
    fn stats(&self, project_path: &FilePath) -> MaintenanceStatsVO;
    fn clean(&self);
    fn update(&self);
    fn doctor(&self) -> DoctorResultVO;
}
