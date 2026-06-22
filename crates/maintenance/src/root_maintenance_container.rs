// PURPOSE: MaintenanceContainer — wiring for maintenance feature (root layer, wiring only)
use std::sync::Arc;

use crate::agent_maintenance_orchestrator::MaintenanceCommandsOrchestrator;
use crate::capabilities_maintenance_checker::MaintenanceChecker;
use shared::project_setup::contract_maintenance_aggregate::MaintenanceCommandsAggregate;

pub struct MaintenanceContainer {
    orchestrator: Arc<dyn MaintenanceCommandsAggregate>,
}

impl MaintenanceContainer {
    pub fn new() -> Self {
        let _checker = MaintenanceChecker::new();
        let orchestrator: Arc<dyn MaintenanceCommandsAggregate> =
            Arc::new(MaintenanceCommandsOrchestrator::new());
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
