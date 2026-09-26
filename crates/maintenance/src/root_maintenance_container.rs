use crate::agent_maintenance_orchestrator::{MaintenanceCommandsOrchestrator, MaintenanceDeps};
use crate::capabilities_maintenance_checker::MaintenanceChecker;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use shared::maintenance::{IMaintenanceAggregate, IMaintenanceCheckerProtocol};
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
        let checker: Arc<dyn IMaintenanceCheckerProtocol> = Arc::new(MaintenanceChecker::new(io));
        let orchestrator: Arc<dyn IMaintenanceAggregate> =
            Arc::new(MaintenanceCommandsOrchestrator::new(MaintenanceDeps {
                checker,
            }));
        Self { orchestrator }
    }

    pub fn orchestrator(&self) -> Arc<dyn IMaintenanceAggregate> {
        self.orchestrator.clone()
    }
}
