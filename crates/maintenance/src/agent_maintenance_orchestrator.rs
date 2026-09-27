use shared::common::taxonomy_action_vo::JobId;
use shared::common::taxonomy_path_vo::FilePath;

use shared::maintenance::contract_maintenance_aggregate::IMaintenanceAggregate;
use shared::maintenance::contract_maintenance_protocol::IMaintenanceCheckerProtocol;
use shared::maintenance::taxonomy_maintenance_request::MaintenanceRequest;
use shared::maintenance::taxonomy_maintenance_response::MaintenanceResponse;

use shared::maintenance::taxonomy_maintenance_vo::MaintenanceStatsVO;
use shared::maintenance::taxonomy_maintenance_vo::{
    DependencyReport, DoctorResultVO, HealthCheckResult, SecurityScanReport, SelfUpdateResultVO,
    ToolchainDiagnostics,
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
impl IMaintenanceAggregate for MaintenanceCommandsOrchestrator {
    fn execute(&self, request: MaintenanceRequest) -> MaintenanceResponse {
        match request {
            MaintenanceRequest::Stats { project_path } => MaintenanceResponse::Stats {
                stats: self.stats(&project_path),
            },
            MaintenanceRequest::Clean => {
                self.clean();
                MaintenanceResponse::Clean
            }
            MaintenanceRequest::Update => {
                self.update();
                MaintenanceResponse::Update
            }
            MaintenanceRequest::SelfUpdate { check_only } => MaintenanceResponse::SelfUpdate {
                result: self.self_update(check_only),
            },
            MaintenanceRequest::Doctor => MaintenanceResponse::Doctor {
                result: self.doctor(),
            },
            MaintenanceRequest::Cancel { job_id } => {
                self.cancel(job_id);
                MaintenanceResponse::Cancelled
            }
            MaintenanceRequest::DiagnoseToolchain => MaintenanceResponse::Toolchain {
                diagnostics: self.diagnose_toolchain(),
            },
            MaintenanceRequest::HealthCheck => MaintenanceResponse::Health {
                health: self.health_check(),
            },
            MaintenanceRequest::SecurityScan { project_path } => MaintenanceResponse::Security {
                report: self.run_security_scan(&project_path),
            },
            MaintenanceRequest::DependencyReport { project_path } => {
                MaintenanceResponse::Dependencies {
                    report: self.run_dependency_report(&project_path),
                }
            }
        }
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl MaintenanceCommandsOrchestrator {
    pub fn new(deps: MaintenanceDeps) -> Self {
        Self { deps }
    }

    pub fn stats(&self, project_path: &FilePath) -> MaintenanceStatsVO {
        self.deps.checker.stats(project_path)
    }

    pub fn clean(&self) {
        self.deps.checker.clean()
    }

    pub fn update(&self) {
        self.deps.checker.update()
    }

    pub fn self_update(&self, check_only: bool) -> SelfUpdateResultVO {
        self.deps.checker.self_update(check_only)
    }

    pub fn doctor(&self) -> DoctorResultVO {
        self.deps.checker.doctor()
    }

    pub fn cancel(&self, _job_id: JobId) {}

    pub fn diagnose_toolchain(&self) -> ToolchainDiagnostics {
        self.deps.checker.diagnose_toolchain()
    }

    pub fn health_check(&self) -> HealthCheckResult {
        self.deps.checker.health_check()
    }

    pub fn run_security_scan(&self, project_path: &FilePath) -> SecurityScanReport {
        self.deps.checker.run_security_scan(project_path)
    }

    pub fn run_dependency_report(
        &self,
        project_path: &FilePath,
    ) -> Result<DependencyReport, String> {
        self.deps.checker.run_dependency_report(project_path)
    }
}
