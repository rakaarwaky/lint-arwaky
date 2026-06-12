// PURPOSE: MultiProjectContainer — wiring for multi-project feature (root layer, wiring only)
use shared::multi_project::contract_orchestrator_aggregate::MultiProjectOrchestratorAggregate;
use std::sync::Arc;

pub struct MultiProjectContainer {
    aggregate: Arc<dyn MultiProjectOrchestratorAggregate>,
}

impl MultiProjectContainer {
    pub fn new() -> Self {
        Self {
            aggregate: Arc::new(crate::agent_project_orchestrator::MultiProjectOrchestrator::new()),
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
