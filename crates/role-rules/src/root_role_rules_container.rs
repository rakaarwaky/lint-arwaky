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

use shared::role_rules::contract_agent_role_protocol::IAgentRoleChecker;
use shared::role_rules::contract_capabilities_role_protocol::ICapabilitiesRoleChecker;
use shared::role_rules::contract_infrastructure_role_protocol::IInfrastructureRoleChecker;
use shared::role_rules::contract_role_protocol::IContractRoleChecker;
use shared::role_rules::contract_surface_role_protocol::ISurfaceRoleChecker;
use shared::role_rules::contract_taxonomy_role_protocol::ITaxonomyRoleChecker;

pub struct RoleAggregateImpl {
    taxonomy: TaxonomyRoleChecker,
    contract: ContractRoleChecker,
    infrastructure: InfrastructureRoleChecker,
    capabilities: CapabilitiesRoleChecker,
    surface: SurfaceRoleChecker,
    agent: AgentRoleChecker,
}

impl Default for RoleAggregateImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl RoleAggregateImpl {
    pub fn new() -> Self {
        Self {
            taxonomy: TaxonomyRoleChecker::new(),
            contract: ContractRoleChecker::new(),
            infrastructure: InfrastructureRoleChecker::new(),
            capabilities: CapabilitiesRoleChecker::new(),
            surface: SurfaceRoleChecker::new(),
            agent: AgentRoleChecker::new(),
        }
    }
}

impl IRoleAggregate for RoleAggregateImpl {
    fn taxonomy(&self) -> &dyn ITaxonomyRoleChecker {
        &self.taxonomy
    }
    fn contract(&self) -> &dyn IContractRoleChecker {
        &self.contract
    }
    fn infrastructure(&self) -> &dyn IInfrastructureRoleChecker {
        &self.infrastructure
    }
    fn capabilities(&self) -> &dyn ICapabilitiesRoleChecker {
        &self.capabilities
    }
    fn surface(&self) -> &dyn ISurfaceRoleChecker {
        &self.surface
    }
    fn agent(&self) -> &dyn IAgentRoleChecker {
        &self.agent
    }
}

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
