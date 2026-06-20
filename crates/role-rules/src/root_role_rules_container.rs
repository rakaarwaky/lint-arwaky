// PURPOSE: RoleContainer — wiring for role-rules feature (root layer, wiring only)
use crate::agent_role_orchestrator::{RoleAggregateImpl, RoleOrchestrator};
use shared::role_rules::contract_role_aggregate::IRoleAggregate;
use shared::role_rules::contract_role_runner_aggregate::IRoleRunnerAggregate;
use std::sync::Arc;

pub struct RoleContainer {
    aggregate: Arc<dyn IRoleAggregate>,
    config: shared::config_system::taxonomy_config_vo::ArchitectureConfig,
}

impl RoleContainer {
    pub fn new() -> Self {
        Self::new_with_config(shared::config_system::taxonomy_config_vo::default_aes_config())
    }

    pub fn new_with_config(config: shared::config_system::taxonomy_config_vo::ArchitectureConfig) -> Self {
        let aggregate: Arc<dyn IRoleAggregate> = Arc::new(RoleAggregateImpl::new());
        Self { aggregate, config }
    }

    pub fn aggregate(&self) -> Arc<dyn IRoleAggregate> {
        self.aggregate.clone()
    }

    pub fn orchestrator(&self) -> Arc<dyn IRoleRunnerAggregate> {
        Arc::new(RoleOrchestrator::new(self.aggregate.clone(), &self.config))
    }
}

impl Default for RoleContainer {
    fn default() -> Self {
        Self::new()
    }
}
