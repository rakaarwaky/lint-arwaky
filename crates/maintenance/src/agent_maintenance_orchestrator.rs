use shared::common::taxonomy_action_vo::JobId;
use shared::common::taxonomy_path_vo::FilePath;

use shared::maintenance::contract_maintenance_aggregate::IMaintenanceAggregate;
use shared::maintenance::contract_maintenance_protocol::{
    IAdapterHealthProtocol, ICacheCleanupProtocol, IDependencyReportProtocol, IDoctorProtocol,
    IProjectStatsProtocol, ISecurityScanProtocol, ISelfUpdateProtocol, IToolUpdateProtocol,
    IToolchainDiagnosticProtocol,
};
use shared::maintenance::taxonomy_maintenance_request::MaintenanceRequest;
use shared::maintenance::taxonomy_maintenance_response::MaintenanceResponse;

use shared::maintenance::taxonomy_maintenance_vo::MaintenanceStatsVO;
use shared::maintenance::taxonomy_maintenance_vo::{
    DependencyReport, DoctorResultVO, HealthCheckResult, SecurityScanReport, SelfUpdateResultVO,
    ToolchainDiagnostics,
};
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

/// One injected seam per maintenance FR; each of the nine capability checkers
/// supplies its own protocol implementation.
pub struct MaintenanceDeps {
    pub toolchain: Arc<dyn IToolchainDiagnosticProtocol>,
    pub doctor: Arc<dyn IDoctorProtocol>,
    pub stats: Arc<dyn IProjectStatsProtocol>,
    pub clean: Arc<dyn ICacheCleanupProtocol>,
    pub update: Arc<dyn IToolUpdateProtocol>,
    pub self_update: Arc<dyn ISelfUpdateProtocol>,
    pub health: Arc<dyn IAdapterHealthProtocol>,
    pub security: Arc<dyn ISecurityScanProtocol>,
    pub dependency_report: Arc<dyn IDependencyReportProtocol>,
}

pub struct MaintenanceCommandsOrchestrator {
    toolchain: Arc<dyn IToolchainDiagnosticProtocol>,
    doctor: Arc<dyn IDoctorProtocol>,
    stats: Arc<dyn IProjectStatsProtocol>,
    clean: Arc<dyn ICacheCleanupProtocol>,
    update: Arc<dyn IToolUpdateProtocol>,
    self_update: Arc<dyn ISelfUpdateProtocol>,
    health: Arc<dyn IAdapterHealthProtocol>,
    security: Arc<dyn ISecurityScanProtocol>,
    dependency_report: Arc<dyn IDependencyReportProtocol>,
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
        Self {
            toolchain: deps.toolchain,
            doctor: deps.doctor,
            stats: deps.stats,
            clean: deps.clean,
            update: deps.update,
            self_update: deps.self_update,
            health: deps.health,
            security: deps.security,
            dependency_report: deps.dependency_report,
        }
    }

    pub fn stats(&self, project_path: &FilePath) -> MaintenanceStatsVO {
        self.stats.stats(project_path)
    }

    pub fn clean(&self) {
        self.clean.clean()
    }

    pub fn update(&self) {
        self.update.update()
    }

    pub fn self_update(&self, check_only: bool) -> SelfUpdateResultVO {
        self.self_update.self_update(check_only)
    }

    pub fn doctor(&self) -> DoctorResultVO {
        self.doctor.doctor()
    }

    pub fn cancel(&self, _job_id: JobId) {}

    pub fn diagnose_toolchain(&self) -> ToolchainDiagnostics {
        self.toolchain.diagnose_toolchain()
    }

    pub fn health_check(&self) -> HealthCheckResult {
        self.health.health_check()
    }

    pub fn run_security_scan(&self, project_path: &FilePath) -> SecurityScanReport {
        self.security.run_security_scan(project_path)
    }

    pub fn run_dependency_report(
        &self,
        project_path: &FilePath,
    ) -> Result<DependencyReport, String> {
        self.dependency_report.run_dependency_report(project_path)
    }
}
