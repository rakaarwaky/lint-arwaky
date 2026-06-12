// PURPOSE: LifecycleContainer — wiring for lifecycle-state feature (root layer, wiring only)
use shared::lifecycle_state::contract_lifecycle_aggregate::AgentLifecycleAggregate;
use std::sync::Arc;

pub struct LifecycleContainer {
    aggregate: Arc<dyn AgentLifecycleAggregate>,
}

impl LifecycleContainer {
    pub fn new() -> Self {
        Self {
            aggregate: Arc::new(crate::agent_status_lifecycle::LifecycleStateManager::new()),
        }
    }

    pub fn aggregate(&self) -> Arc<dyn AgentLifecycleAggregate> {
        self.aggregate.clone()
    }
}
impl Default for LifecycleContainer {
    fn default() -> Self {
        Self::new()
    }
}
