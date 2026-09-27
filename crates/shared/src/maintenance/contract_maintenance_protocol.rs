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

/// FR-Maintenance-001: Environment Health Check (doctor).
pub trait IDoctorProtocol: Send + Sync {
    fn doctor(&self) -> DoctorResultVO;
}

/// FR-Maintenance-002: Project Statistics (stats).
pub trait IProjectStatsProtocol: Send + Sync {
    fn stats(&self, project_path: &FilePath) -> MaintenanceStatsVO;
}

/// FR-Maintenance-003: Cache Cleanup (clean).
pub trait ICacheCleanupProtocol: Send + Sync {
    fn clean(&self);
}

/// FR-Maintenance-004: Tool Update (update).
pub trait IToolUpdateProtocol: Send + Sync {
    fn update(&self);
}

/// FR-Maintenance-005: Diagnose Toolchain.
pub trait IToolchainDiagnosticProtocol: Send + Sync {
    fn diagnose_toolchain(&self) -> ToolchainDiagnostics;
}

/// FR-Maintenance-006: Security Scan.
pub trait ISecurityScanProtocol: Send + Sync {
    fn run_security_scan(&self, project_path: &FilePath) -> SecurityScanReport;
}

/// FR-Maintenance-007: Dependency Report.
pub trait IDependencyReportProtocol: Send + Sync {
    fn run_dependency_report(&self, project_path: &FilePath) -> Result<DependencyReport, String>;
}

/// FR-Maintenance-008: Adapter Health Check.
pub trait IAdapterHealthProtocol: Send + Sync {
    fn health_check(&self) -> HealthCheckResult;
}

/// FR-Maintenance-009: Self-Update (binary).
pub trait ISelfUpdateProtocol: Send + Sync {
    fn self_update(&self, check_only: bool) -> SelfUpdateResultVO;
}

/// IToolExecutorProtocol — internal tool-execution infra seam (no FR).
pub trait IToolExecutorProtocol: Send + Sync {
    fn run_tool(&self, name: &str, args: &[&str]) -> ToolOutput;
    fn run_tool_in_dir(&self, name: &str, args: &[&str], dir: &FilePath) -> ToolOutput;
    fn tool_exists(&self, name: &str) -> bool;
    fn get_binary_path(&self) -> FilePath;
}
