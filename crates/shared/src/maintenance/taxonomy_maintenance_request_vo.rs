// PURPOSE: MaintenanceRequest/MaintenanceResponse — request/response VOs for the maintenance aggregate
use crate::common::taxonomy_action_vo::JobId;
use crate::common::taxonomy_path_vo::FilePath;
use crate::maintenance::taxonomy_doctor_vo::{
    DependencyReport, DoctorResultVO, HealthCheckResult, SecurityScanReport, ToolchainDiagnostics,
};
use crate::maintenance::taxonomy_stats_vo::MaintenanceStatsVO;

/// Consumer verb carried by the maintenance aggregate's single entry point.
pub enum MaintenanceRequest {
    /// Collect file/line/dir counts for a project.
    Stats { project_path: FilePath },
    /// Remove build artifacts and caches.
    Clean,
    /// Update dependencies to their latest compatible versions.
    Update,
    /// Run environment and dependency diagnostics.
    Doctor,
    /// Cancel a running background job.
    Cancel { job_id: JobId },
    /// Report which external tools are present and at what version.
    DiagnoseToolchain,
    /// Check that required tools are installed and configured.
    HealthCheck,
    /// Scan for hardcoded secrets and unsafe patterns.
    SecurityScan { project_path: FilePath },
    /// Resolve and summarize the dependency tree.
    DependencyReport { project_path: FilePath },
}

impl MaintenanceRequest {
    pub fn stats(project_path: &FilePath) -> Self {
        Self::Stats {
            project_path: project_path.clone(),
        }
    }
    pub fn clean() -> Self {
        Self::Clean
    }
    pub fn update() -> Self {
        Self::Update
    }
    pub fn doctor() -> Self {
        Self::Doctor
    }
    pub fn cancel(job_id: JobId) -> Self {
        Self::Cancel { job_id }
    }
    pub fn diagnose_toolchain() -> Self {
        Self::DiagnoseToolchain
    }
    pub fn health_check() -> Self {
        Self::HealthCheck
    }
    pub fn security_scan(project_path: &FilePath) -> Self {
        Self::SecurityScan {
            project_path: project_path.clone(),
        }
    }
    pub fn dependency_report(project_path: &FilePath) -> Self {
        Self::DependencyReport {
            project_path: project_path.clone(),
        }
    }
}

/// Result of a maintenance aggregate request.
pub enum MaintenanceResponse {
    Stats {
        stats: MaintenanceStatsVO,
    },
    Clean,
    Update,
    Doctor {
        result: DoctorResultVO,
    },
    Cancelled,
    Toolchain {
        diagnostics: ToolchainDiagnostics,
    },
    Health {
        health: HealthCheckResult,
    },
    Security {
        report: SecurityScanReport,
    },
    Dependencies {
        report: Result<DependencyReport, String>,
    },
}

impl MaintenanceResponse {
    pub fn into_stats(self) -> MaintenanceStatsVO {
        match self {
            Self::Stats { stats } => stats,
            _ => MaintenanceStatsVO::default(),
        }
    }

    /// Report whether a cleanup or update ran. Returns false for other verbs.
    pub fn into_ran(self) -> bool {
        matches!(self, Self::Clean | Self::Update)
    }

    pub fn into_doctor(self) -> DoctorResultVO {
        match self {
            Self::Doctor { result } => result,
            _ => DoctorResultVO::default(),
        }
    }

    /// Report whether a job was cancelled. Returns false for other verbs.
    pub fn into_cancelled(self) -> bool {
        matches!(self, Self::Cancelled)
    }

    pub fn into_toolchain(self) -> ToolchainDiagnostics {
        match self {
            Self::Toolchain { diagnostics } => diagnostics,
            _ => ToolchainDiagnostics::default(),
        }
    }

    pub fn into_health(self) -> HealthCheckResult {
        match self {
            Self::Health { health } => health,
            _ => HealthCheckResult::default(),
        }
    }

    pub fn into_security_report(self) -> SecurityScanReport {
        match self {
            Self::Security { report } => report,
            _ => SecurityScanReport::default(),
        }
    }

    pub fn into_dependency_report(self) -> Result<DependencyReport, String> {
        match self {
            Self::Dependencies { report } => report,
            _ => Err(String::from("no dependency report available")),
        }
    }
}
