// PURPOSE: SetupContainer — wiring for project-setup feature (root layer, wiring only)
use std::sync::Arc;
use shared::project_setup::contract_setup_aggregate::SetupManagementAggregate;
use shared::project_setup::contract_setup_protocol::ISetupManagementProtocol;

pub struct SetupContainer {
    aggregate: Arc<dyn SetupManagementAggregate>,
    protocol: Arc<dyn ISetupManagementProtocol>,
}

impl SetupContainer {
    pub fn new() -> Self {
        Self {
            aggregate: Arc::new(
                crate::agent_setup_orchestrator::SetupManagementOrchestrator::new(),
            ),
            protocol: Arc::new(
                crate::capabilities_setup_processor::SetupManagementProcessor::new(),
            ),
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

