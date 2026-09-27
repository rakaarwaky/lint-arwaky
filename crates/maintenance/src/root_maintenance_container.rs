use crate::agent_maintenance_orchestrator::{MaintenanceCommandsOrchestrator, MaintenanceDeps};
use crate::capabilities_maintenance_checker::MaintenanceChecker;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use shared::maintenance::contract_maintenance_aggregate::IMaintenanceAggregate;
use shared::maintenance::contract_maintenance_protocol::{
    IAdapterHealthProtocol, ICacheCleanupProtocol, IDependencyReportProtocol, IDoctorProtocol,
    IProjectStatsProtocol, ISecurityScanProtocol, ISelfUpdateProtocol, IToolUpdateProtocol,
    IToolchainDiagnosticProtocol,
};
use std::sync::Arc;

pub struct MaintenanceContainer {
    orchestrator: Arc<dyn IMaintenanceAggregate>,
}

impl MaintenanceContainer {
    pub fn new(
        filesystem: Arc<dyn IFilesystemAggregate>,
        io: Arc<dyn IFileSystemIOProtocol>,
    ) -> Self {
        let _ = filesystem;
        let checker = Arc::new(MaintenanceChecker::new(io));
        let doctor: Arc<dyn IDoctorProtocol> = checker.clone();
        let stats: Arc<dyn IProjectStatsProtocol> = checker.clone();
        let clean: Arc<dyn ICacheCleanupProtocol> = checker.clone();
        let update: Arc<dyn IToolUpdateProtocol> = checker.clone();
        let toolchain: Arc<dyn IToolchainDiagnosticProtocol> = checker.clone();
        let security: Arc<dyn ISecurityScanProtocol> = checker.clone();
        let deps_report: Arc<dyn IDependencyReportProtocol> = checker.clone();
        let health: Arc<dyn IAdapterHealthProtocol> = checker.clone();
        let self_update: Arc<dyn ISelfUpdateProtocol> = checker.clone();
        let orchestrator: Arc<dyn IMaintenanceAggregate> =
            Arc::new(MaintenanceCommandsOrchestrator::new(MaintenanceDeps {
                toolchain,
                doctor,
                stats,
                clean,
                update,
                self_update,
                health,
                security,
                dependency_report: deps_report,
            }));
        Self { orchestrator }
    }

    pub fn orchestrator(&self) -> Arc<dyn IMaintenanceAggregate> {
        self.orchestrator.clone()
    }
}
