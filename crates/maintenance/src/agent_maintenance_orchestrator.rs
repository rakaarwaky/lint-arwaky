// PURPOSE: MaintenanceCommandsOrchestrator — implements MaintenanceCommandsAggregate for env diagnostics, stats, cleanup
//
// The maintenance crate provides background health operations:
//   - doctor: check environment for required tools (ruff, mypy, bandit, git, etc.)
//   - stats: count and ratio of Python files vs test files in a project
//   - clean: remove cache directories (.pytest_cache, __pycache__, etc.)
//   - update: upgrade external linter tools via pip
//   - security_scan: run dependency vulnerability scans
//   - dependency_report: analyze project dependencies
//   - diagnose_toolchain: check Rust/Python/Node.js toolchain versions
//
// This is the least "lint-like" crate — it handles ops, not code quality.
// All I/O is delegated to IMaintenanceCheckerProtocol (capabilities layer).
use shared::common::taxonomy_action_vo::JobId;
use shared::common::taxonomy_path_vo::FilePath;
use shared::project_setup::contract_maintenance_aggregate::MaintenanceCommandsAggregate;
use shared::project_setup::contract_maintenance_protocol::IMaintenanceCheckerProtocol;
use shared::project_setup::taxonomy_doctor_vo::{
    DependencyReport, DoctorResultVO, SecurityScanReport, ToolchainDiagnostics,
};
use shared::project_setup::taxonomy_stats_vo::MaintenanceStatsVO;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct MaintenanceDeps {
    pub checker: Arc<dyn IMaintenanceCheckerProtocol>,
}

pub struct MaintenanceCommandsOrchestrator {
    deps: MaintenanceDeps,
}

use async_trait::async_trait;

// ─── Block 2: Aggregate Trait Implementation ──────────────
#[async_trait]
impl MaintenanceCommandsAggregate for MaintenanceCommandsOrchestrator {
    async fn stats(&self, project_path: &FilePath) -> MaintenanceStatsVO {
        self.deps.checker.stats(project_path).await
    }

    async fn clean(&self) {
        self.deps.checker.clean().await
    }

    async fn update(&self) {
        self.deps.checker.update().await
    }

    async fn doctor(&self) -> DoctorResultVO {
        self.deps.checker.doctor().await
    }

    async fn cancel(&self, _job_id: JobId) {}

    async fn diagnose_toolchain(&self) -> ToolchainDiagnostics {
        self.deps.checker.diagnose_toolchain().await
    }

    async fn run_security_scan(&self, project_path: &FilePath) -> SecurityScanReport {
        self.deps.checker.run_security_scan(project_path).await
    }

    async fn run_dependency_report(
        &self,
        project_path: &FilePath,
    ) -> Result<DependencyReport, String> {
        self.deps.checker.run_dependency_report(project_path).await
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl MaintenanceCommandsOrchestrator {
    pub fn new(deps: MaintenanceDeps) -> Self {
        Self { deps }
    }
}
