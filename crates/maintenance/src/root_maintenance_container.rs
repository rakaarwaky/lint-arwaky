use crate::agent_maintenance_orchestrator::{MaintenanceCommandsOrchestrator, MaintenanceDeps};
use crate::capabilities_maintenance_checker::MaintenanceChecker;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::maintenance::{IMaintenanceCheckerProtocol, MaintenanceCommandsAggregate};
use std::sync::Arc;

pub struct MaintenanceContainer {
    orchestrator: Arc<dyn MaintenanceCommandsAggregate>,
}

impl MaintenanceContainer {
    pub fn new(filesystem: Arc<dyn IFilesystemAggregate>) -> Self {
        let checker: Arc<dyn IMaintenanceCheckerProtocol> =
            Arc::new(MaintenanceChecker::new(filesystem));
        let orchestrator: Arc<dyn MaintenanceCommandsAggregate> =
            Arc::new(MaintenanceCommandsOrchestrator::new(MaintenanceDeps {
                checker,
            }));
        Self { orchestrator }
    }

    pub fn orchestrator(&self) -> Arc<dyn MaintenanceCommandsAggregate> {
        self.orchestrator.clone()
    }
}
