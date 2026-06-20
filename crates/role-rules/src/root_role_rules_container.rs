// PURPOSE: RoleContainer — wiring for role-rules feature (root layer, wiring only)
use crate::agent_role_orchestrator::RoleOrchestrator;
use shared::role_rules::contract_role_aggregate::IRoleAggregate;
use shared::role_rules::contract_role_runner_aggregate::IRoleRunnerAggregate;
use std::sync::Arc;

use crate::capabilities_agent_role_auditor::AgentRoleChecker;
use crate::capabilities_capabilities_role_auditor::CapabilitiesRoleChecker;
use crate::capabilities_contract_role_auditor::ContractRoleChecker;
use crate::capabilities_infrastructure_role_auditor::InfrastructureRoleChecker;
use crate::capabilities_surface_role_auditor::SurfaceRoleChecker;
use crate::capabilities_taxonomy_role_auditor::TaxonomyRoleChecker;

use crate::agent_role_orchestrator::RoleAggregateImpl;

pub struct RoleContainer {
    aggregate: Arc<dyn IRoleAggregate>,
    config: shared::config_system::taxonomy_config_vo::ArchitectureConfig,
}

impl RoleContainer {
    pub fn new() -> Self {
        Self::new_with_config(shared::config_system::taxonomy_config_vo::default_aes_config())
    }

    pub fn new_with_config(
        config: shared::config_system::taxonomy_config_vo::ArchitectureConfig,
    ) -> Self {
        let aggregate: Arc<dyn IRoleAggregate> = Arc::new(RoleAggregateImpl::new(
            Arc::new(TaxonomyRoleChecker::new()),
            Arc::new(ContractRoleChecker::new()),
            Arc::new(InfrastructureRoleChecker::new()),
            Arc::new(CapabilitiesRoleChecker::new()),
            Arc::new(SurfaceRoleChecker::new()),
            Arc::new(AgentRoleChecker::new()),
        ));
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
