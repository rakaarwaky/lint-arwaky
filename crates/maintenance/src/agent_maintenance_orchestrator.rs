use shared::common::{FilePath, JobId};

use shared::maintenance::{IMaintenanceCheckerProtocol, MaintenanceCommandsAggregate};

use shared::maintenance::MaintenanceStatsVO;
use shared::maintenance::{
    DependencyReport, DoctorResultVO, SecurityScanReport, ToolchainDiagnostics,
};
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct MaintenanceDeps {
    pub checker: Arc<dyn IMaintenanceCheckerProtocol>,
}

pub struct MaintenanceCommandsOrchestrator {
    deps: MaintenanceDeps,
}

// ─── Block 2: Aggregate Trait Implementation ──────────────
impl MaintenanceCommandsAggregate for MaintenanceCommandsOrchestrator {
    fn stats(&self, project_path: &FilePath) -> MaintenanceStatsVO {
        self.deps.checker.stats(project_path)
    }

    fn clean(&self) {
        self.deps.checker.clean()
    }

    fn update(&self) {
        self.deps.checker.update()
    }

    fn doctor(&self) -> DoctorResultVO {
        self.deps.checker.doctor()
    }

    fn cancel(&self, _job_id: JobId) {}

    fn diagnose_toolchain(&self) -> ToolchainDiagnostics {
        self.deps.checker.diagnose_toolchain()
    }

    fn run_security_scan(&self, project_path: &FilePath) -> SecurityScanReport {
        self.deps.checker.run_security_scan(project_path)
    }

    fn run_dependency_report(&self, project_path: &FilePath) -> Result<DependencyReport, String> {
        self.deps.checker.run_dependency_report(project_path)
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl MaintenanceCommandsOrchestrator {
    pub fn new(deps: MaintenanceDeps) -> Self {
        Self { deps }
    }
}
