// PURPOSE: RoleContainer — wiring for role-rules feature (root layer, wiring only)
use crate::role_rules::contract_role_aggregate::IRoleAggregate;
use crate::role_rules::contract_role_runner_aggregate::IRoleRunnerAggregate;
use std::sync::Arc;

pub struct RoleContainer {
    aggregate: Arc<dyn IRoleAggregate>,
}

impl RoleContainer {
    pub fn new() -> Self {
        let aggregate: Arc<dyn IRoleAggregate> = Arc::new(crate::role_rules::agent_role_container::RoleAggregateImpl::new());
        Self { aggregate }
    }

    pub fn aggregate(&self) -> Arc<dyn IRoleAggregate> {
        self.aggregate.clone()
    }

    pub fn orchestrator(&self) -> Arc<dyn IRoleRunnerAggregate> {
        Arc::new(
            crate::role_rules::agent_role_orchestrator::RoleOrchestrator::new(
                self.aggregate.clone(),
            ),
        )
    }
}

impl Default for RoleContainer {
    fn default() -> Self {
        Self::new()
    }
}