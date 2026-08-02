// PURPOSE: SetupContainer — wiring for project-setup feature (root layer, wiring only)

use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::project_setup::{ISetupManagementProtocol, SetupManagementAggregate};

use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct SetupContainer {
    aggregate: Arc<dyn SetupManagementAggregate>,
    protocol: Arc<dyn ISetupManagementProtocol>,
}

// ─── Block 2: Container Construction ──────────────────────

impl SetupContainer {
    pub fn new(filesystem: Arc<dyn IFilesystemAggregate>) -> Self {
        let installer =
            Arc::new(crate::capabilities_setup_installer_adapter::SetupInstallerAdapter::new());
        let protocol = Arc::new(
            crate::capabilities_setup_processor::SetupManagementProcessor::new(
                installer, filesystem,
            ),
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

// ─── Block 3: Default Trait Implementation ────────────────

impl Default for SetupContainer {
    fn default() -> Self {
        panic!(
            "SetupContainer::default() requires a filesystem instance — use SetupContainer::new(filesystem) instead"
        )
    }
}
