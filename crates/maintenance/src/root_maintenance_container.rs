use crate::agent_maintenance_orchestrator::{MaintenanceCommandsOrchestrator, MaintenanceDeps};
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use shared::maintenance::contract_maintenance_aggregate::IMaintenanceAggregate;
use shared::maintenance::contract_maintenance_protocol::{
    IAdapterHealthProtocol, ICacheCleanupProtocol, IDependencyReportProtocol, IDoctorProtocol,
    IProjectStatsProtocol, ISecurityScanProtocol, ISelfUpdateProtocol, IToolUpdateProtocol,
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
        let doctor: Arc<dyn IDoctorProtocol> = Arc::new(crate::DoctorChecker::new(io.clone()));
        let stats: Arc<dyn IProjectStatsProtocol> =
            Arc::new(crate::ProjectStatsChecker::new(io.clone()));
        let clean: Arc<dyn ICacheCleanupProtocol> =
            Arc::new(crate::CacheCleanupChecker::new(io.clone()));
        let update: Arc<dyn IToolUpdateProtocol> =
            Arc::new(crate::ToolUpdateChecker::new(io.clone()));
        let security: Arc<dyn ISecurityScanProtocol> =
            Arc::new(crate::SecurityScanChecker::new(io.clone()));
        let deps_report: Arc<dyn IDependencyReportProtocol> =
            Arc::new(crate::DependencyReportChecker::new(io.clone()));
        let health: Arc<dyn IAdapterHealthProtocol> =
            Arc::new(crate::AdapterHealthChecker::new(io.clone()));
        let self_update: Arc<dyn ISelfUpdateProtocol> =
            Arc::new(crate::SelfUpdateChecker::new(io.clone()));
        let orchestrator: Arc<dyn IMaintenanceAggregate> =
            Arc::new(MaintenanceCommandsOrchestrator::new(MaintenanceDeps {
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
