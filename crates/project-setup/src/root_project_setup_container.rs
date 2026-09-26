// PURPOSE: SetupContainer — wiring for project-setup feature (root layer, wiring only)

use shared::filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use shared::project_setup::{ISetupAggregate, ISetupManagementProtocol};

use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct SetupContainer {
    aggregate: Arc<dyn ISetupAggregate>,
    protocol: Arc<dyn ISetupManagementProtocol>,
}

// ─── Block 2: Container Construction ──────────────────────

impl SetupContainer {
    pub fn new(io: Arc<dyn IFileSystemIOProtocol>) -> Self {
        let installer =
            Arc::new(crate::capabilities_setup_installer_adapter::SetupInstallerAdapter::new());
        let protocol = Arc::new(
            crate::capabilities_setup_processor::SetupManagementProcessor::new(installer, io),
        );
        let aggregate = Arc::new(
            crate::agent_setup_orchestrator::SetupManagementOrchestrator::new(protocol.clone()),
        );
        Self {
            aggregate,
            protocol,
        }
    }

    pub fn aggregate(&self) -> Arc<dyn ISetupAggregate> {
        self.aggregate.clone()
    }

    pub fn protocol(&self) -> Arc<dyn ISetupManagementProtocol> {
        self.protocol.clone()
    }
}

// ─── Note: No Default impl ────────────────────────────────
// SetupContainer requires a filesystem instance — construction is only
// possible via SetupContainer::new(filesystem). Providing a Default impl
// would be a bypass violation (AES304) since there is no valid zero-arg
// construction path.
