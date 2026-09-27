// PURPOSE: maintenance-domain capability contracts (AES102 `_protocol`).
//
// One file for the maintenance feature. Each trait below is one capability
// seam: a trait carries every method that capability implements, with one
// concrete return type each, so a capability implements its trait outright
// and never carries unimplemented stubs.

use crate::common::taxonomy_path_vo::FilePath;
use crate::maintenance::taxonomy_maintenance_vo::MaintenanceStatsVO;
pub use crate::maintenance::taxonomy_maintenance_vo::ToolOutput;
use crate::maintenance::taxonomy_maintenance_vo::{
    DependencyReport, DoctorResultVO, HealthCheckResult, SecurityScanReport, SelfUpdateResultVO,
    ToolchainDiagnostics,
};

pub trait IMaintenanceCheckerProtocol: Send + Sync {
    fn diagnose_toolchain(&self) -> ToolchainDiagnostics;
    fn health_check(&self) -> HealthCheckResult;
    fn run_security_scan(&self, project_path: &FilePath) -> SecurityScanReport;
    fn run_dependency_report(&self, project_path: &FilePath) -> Result<DependencyReport, String>;
    fn stats(&self, project_path: &FilePath) -> MaintenanceStatsVO;
    fn clean(&self);
    fn update(&self);
    fn doctor(&self) -> DoctorResultVO;
    /// Check the latest GitHub release and optionally install it.
    ///
    /// When `check_only` is `true`, only query the API and report the result
    /// without downloading or replacing any binary.
    /// When `check_only` is `false`, download the latest release binary and
    /// install it to `$CARGO_HOME/bin` or fall back to `$HOME/.cargo/bin`.
    fn self_update(&self, check_only: bool) -> SelfUpdateResultVO;
}

pub trait IToolExecutorProtocol: Send + Sync {
    fn run_tool(&self, name: &str, args: &[&str]) -> ToolOutput;
    fn run_tool_in_dir(&self, name: &str, args: &[&str], dir: &FilePath) -> ToolOutput;
    fn tool_exists(&self, name: &str) -> bool;
    fn get_binary_path(&self) -> FilePath;
}
