// PURPOSE: IMaintenanceCheckerProtocol — protocol for maintenance checker capabilities
use crate::common::taxonomy_path_vo::FilePath;
use crate::project_setup::taxonomy_doctor_vo::{
    DependencyReport, SecurityScanReport, ToolchainDiagnostics,
};
use async_trait::async_trait;

#[async_trait]
pub trait IMaintenanceCheckerProtocol: Send + Sync {
    async fn diagnose_toolchain(&self) -> ToolchainDiagnostics;
    async fn run_security_scan(&self, project_path: &FilePath) -> SecurityScanReport;
    async fn run_dependency_report(
        &self,
        project_path: &FilePath,
    ) -> Result<DependencyReport, String>;
}
