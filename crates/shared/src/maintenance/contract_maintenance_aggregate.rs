// PURPOSE: Aggregate: MaintenanceCommandsAggregate trait — contract for maintenance operations (stats, doctor, clean, update, cancel)
use crate::common::taxonomy_action_vo::JobId;
use crate::common::taxonomy_path_vo::FilePath;
use crate::maintenance::taxonomy_doctor_vo::{
    DependencyReport, DoctorResultVO, HealthCheckResult, SecurityScanReport, ToolchainDiagnostics,
};
use crate::maintenance::taxonomy_stats_vo::MaintenanceStatsVO;

pub trait MaintenanceCommandsAggregate: Send + Sync {
    fn stats(&self, project_path: &FilePath) -> MaintenanceStatsVO;
    fn clean(&self);
    fn update(&self);
    fn doctor(&self) -> DoctorResultVO;
    fn cancel(&self, job_id: JobId);
    fn diagnose_toolchain(&self) -> ToolchainDiagnostics;
    fn health_check(&self) -> HealthCheckResult;
    fn run_security_scan(&self, project_path: &FilePath) -> SecurityScanReport;
    fn run_dependency_report(&self, project_path: &FilePath) -> Result<DependencyReport, String>;
}
