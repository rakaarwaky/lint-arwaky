// PURPOSE: MaintenanceContainer — wiring for maintenance feature (root layer, wiring only)
use std::sync::Arc;

use crate::agent_maintenance_orchestrator::{MaintenanceCommandsOrchestrator, MaintenanceDeps};
use crate::capabilities_maintenance_checker::MaintenanceChecker;
use shared::maintenance::contract_maintenance_aggregate::MaintenanceCommandsAggregate;
use shared::maintenance::contract_maintenance_protocol::IMaintenanceCheckerProtocol;

pub struct MaintenanceContainer {
    orchestrator: Arc<dyn MaintenanceCommandsAggregate>,
}

impl MaintenanceContainer {
    pub fn new() -> Self {
        let checker: Arc<dyn IMaintenanceCheckerProtocol> = Arc::new(MaintenanceChecker::new());
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

impl Default for MaintenanceContainer {
    fn default() -> Self {
        Self::new()
    }
}
