// PURPOSE: RoleContainer — wiring for role-rules feature (root layer, wiring only)
use std::sync::Arc;
use crate::role_rules::contract_role_runner_aggregate::IRoleRunnerAggregate;

pub struct RoleContainer {
    aggregate: Box<dyn crate::role_rules::contract_role_aggregate::IRoleAggregate>,
}

impl RoleContainer {
    pub fn new() -> Self {
        Self {
            aggregate: Box::new(crate::role_rules::agent_role_container::RoleAggregateImpl::new()),
        }
    }

    pub fn orchestrator(&self) -> Arc<dyn IRoleRunnerAggregate> {
        Arc::new(crate::role_rules::agent_role_orchestrator::RoleOrchestrator::new(
            // Clone isn't possible on Box<dyn>, so we create a new one
            Box::new(crate::role_rules::agent_role_container::RoleAggregateImpl::new()),
        ))
    }
}
