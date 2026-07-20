// PURPOSE: SetupContainer — wiring for project-setup feature (root layer, wiring only)
use shared::project_setup::contract_setup_aggregate::SetupManagementAggregate;
use shared::project_setup::contract_setup_protocol::ISetupManagementProtocol;
use std::sync::Arc;

pub struct SetupContainer {
    aggregate: Arc<dyn SetupManagementAggregate>,
    protocol: Arc<dyn ISetupManagementProtocol>,
}

impl SetupContainer {
    pub fn new() -> Self {
        let installer =
            Arc::new(crate::capabilities_setup_installer_adapter::SetupInstallerAdapter::new());
        let protocol =
            Arc::new(crate::capabilities_setup_processor::SetupManagementProcessor::new(installer));
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
impl Default for SetupContainer {
    fn default() -> Self {
        Self::new()
    }
}
