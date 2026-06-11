// PURPOSE: MultiProjectContainer — wiring for multi-project feature (root layer, wiring only)
use std::sync::Arc;
use crate::multi_project::contract_orchestrator_aggregate::MultiProjectOrchestratorAggregate;

pub struct MultiProjectContainer {
    aggregate: Arc<dyn MultiProjectOrchestratorAggregate>,
}

impl MultiProjectContainer {
    pub fn new() -> Self {
        Self {
            aggregate: Arc::new(
                crate::multi_project::agent_project_orchestrator::MultiProjectOrchestrator::new(),
            ),
        }
    }

    pub fn aggregate(&self) -> Arc<dyn MultiProjectOrchestratorAggregate> {
        self.aggregate.clone()
    }
}
impl Default for MultiProjectContainer {
    fn default() -> Self {
        Self::new()
    }
}

