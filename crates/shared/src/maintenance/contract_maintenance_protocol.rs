// PURPOSE: IMaintenanceCheckerProtocol — protocol for maintenance checker capabilities
use crate::common::taxonomy_path_vo::FilePath;
use crate::maintenance::taxonomy_doctor_vo::{
    DependencyReport, DoctorResultVO, SecurityScanReport, ToolchainDiagnostics,
};
use crate::maintenance::taxonomy_stats_vo::MaintenanceStatsVO;
use async_trait::async_trait;

#[async_trait]
pub trait IMaintenanceCheckerProtocol: Send + Sync {
    async fn diagnose_toolchain(&self) -> ToolchainDiagnostics;
    async fn run_security_scan(&self, project_path: &FilePath) -> SecurityScanReport;
    async fn run_dependency_report(
        &self,
        project_path: &FilePath,
    ) -> Result<DependencyReport, String>;
    async fn stats(&self, project_path: &FilePath) -> MaintenanceStatsVO;
    async fn clean(&self);
    async fn update(&self);
    async fn doctor(&self) -> DoctorResultVO;
}
