// PURPOSE: SetupContainer — wiring for project-setup feature (root layer, wiring only)
use shared::project_setup::contract_setup_aggregate::SetupManagementAggregate;
use shared::project_setup::contract_setup_protocol::ISetupManagementProtocol;
use std::sync::Arc;

// Block 1: struct Definition
// ─── Block 1: Struct Definition ───────────────────────────
pub struct SetupContainer {
    aggregate: Arc<dyn SetupManagementAggregate>,
    protocol: Arc<dyn ISetupManagementProtocol>,
}

// ─── Block 2: Public Contract ─────────────────────────────
// (No trait impl — root container is wiring only)

// Block 3: constructors & public API
// ─── Block 3: Constructors & Helpers ──────────────────────
impl SetupContainer {
    pub fn new() -> Self {
        let installer =
            Arc::new(crate::infrastructure_setup_installer_adapter::SetupInstallerAdapter::new());
        let fs_port: Arc<dyn shared::project_setup::contract_filesystem_maintenance_port::IFileSystemMaintenancePort> =
            Arc::new(crate::infrastructure_filesystem_maintenance_adapter::FileSystemMaintenanceAdapter::new());
        let protocol = Arc::new(
            crate::capabilities_setup_processor::SetupManagementProcessor::new(installer, fs_port),
        );
        let aggregate = Arc::new(
            crate::agent_setup_orchestrator::SetupManagementOrchestrator::new(protocol.clone()),
        );
        Self {
            aggregate,
            protocol,
        }
    }

    pub fn aggregate(&self) -> Arc<dyn SetupManagementAggregate> {
        self.aggregate.clone()
    }

    pub fn protocol(&self) -> Arc<dyn ISetupManagementProtocol> {
        self.protocol.clone()
    }
}
// ─── Block 2: Public Contract ─────────────────────────────
impl Default for SetupContainer {
    fn default() -> Self {
        Self::new()
    }
}
