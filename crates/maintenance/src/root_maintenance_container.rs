// PURPOSE: MaintenanceContainer — wiring for maintenance feature (root layer, wiring only)
use std::sync::Arc;

use crate::agent_maintenance_orchestrator::MaintenanceCommandsOrchestrator;
use crate::capabilities_maintenance_checker::MaintenanceChecker;
use crate::infrastructure_filesystem_maintenance_adapter::FileSystemMaintenanceAdapter;
use crate::infrastructure_tool_executor_adapter::ToolExecutorAdapter;
use shared::project_setup::contract_filesystem_maintenance_port::IFileSystemMaintenancePort;
use shared::project_setup::contract_maintenance_aggregate::MaintenanceCommandsAggregate;
use shared::project_setup::contract_maintenance_protocol::IMaintenanceCheckerProtocol;
use shared::project_setup::contract_tool_executor_port::IToolExecutorPort;

// Block 1: struct Definition
pub struct MaintenanceContainer {
    orchestrator: Arc<dyn MaintenanceCommandsAggregate>,
}

// Block 2: impl Trait for Struct (Public Contract)
impl MaintenanceContainer {
    pub fn orchestrator(&self) -> Arc<dyn MaintenanceCommandsAggregate> {
        self.orchestrator.clone()
    }
}

// Block 3: constructors
impl MaintenanceContainer {
    pub fn new() -> Self {
        let tool_executor: Arc<dyn IToolExecutorPort> = Arc::new(ToolExecutorAdapter::new());
        let fs: Arc<dyn IFileSystemMaintenancePort> = Arc::new(FileSystemMaintenanceAdapter::new());
        let checker: Arc<dyn IMaintenanceCheckerProtocol> =
            Arc::new(MaintenanceChecker::new(tool_executor.clone(), fs.clone()));
        let orchestrator: Arc<dyn MaintenanceCommandsAggregate> = Arc::new(
            MaintenanceCommandsOrchestrator::new(tool_executor, fs, checker),
        );
        Self { orchestrator }
    }
}

impl Default for MaintenanceContainer {
    fn default() -> Self {
        Self::new()
    }
}
