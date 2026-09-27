// PURPOSE: MaintenanceRequest — request payload for the maintenance aggregate

use crate::common::taxonomy_action_vo::JobId;
use crate::common::taxonomy_path_vo::FilePath;

pub enum MaintenanceRequest {
    /// Collect file/line/dir counts for a project.
    Stats { project_path: FilePath },
    /// Remove build artifacts and caches.
    Clean,
    /// Update dependencies to their latest compatible versions.
    Update,
    /// Install the latest lint-arwaky release binary from GitHub.
    SelfUpdate {
        /// Check the latest release and report status without installing.
        check_only: bool,
    },
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
    /// Request a self-update check/install from GitHub releases.
    pub fn self_update(check_only: bool) -> Self {
        Self::SelfUpdate { check_only }
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
