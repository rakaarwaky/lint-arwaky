// PURPOSE: LifecycleContainer — wiring for lifecycle-state feature (root layer, wiring only)
use std::sync::Arc;
use crate::lifecycle_state::contract_lifecycle_aggregate::AgentLifecycleAggregate;

pub struct LifecycleContainer {
    aggregate: Arc<dyn AgentLifecycleAggregate>,
}

impl LifecycleContainer {
    pub fn new() -> Self {
        Self {
            aggregate: Arc::new(
                crate::lifecycle_state::agent_status_lifecycle::LifecycleStateManager::new(),
            ),
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

