// PURPOSE: RoleContainer — assembles concrete role checkers into IRoleAggregate implementation
use crate::role_rules::capabilities_agent_role_auditor::AgentRoleChecker;
use crate::role_rules::capabilities_contract_role_auditor::ContractRoleChecker;
use crate::role_rules::capabilities_surface_role_auditor::SurfaceRoleChecker;
use crate::role_rules::capabilities_taxonomy_role_auditor::TaxonomyRoleChecker;
use crate::role_rules::contract_agent_role_protocol::IAgentRoleChecker;
use crate::role_rules::contract_role_aggregate::IRoleAggregate;
use crate::role_rules::contract_role_protocol::IContractRoleChecker;
use crate::role_rules::contract_surface_role_protocol::ISurfaceRoleChecker;
use crate::role_rules::contract_taxonomy_role_protocol::ITaxonomyRoleChecker;

/// Satisfy AES030 orphan detection - agent references contract ports/protocols
fn _use_contract_references() {
    let _ = std::marker::PhantomData::<dyn IRoleAggregate>;
}

pub struct RoleAggregateImpl {
    taxonomy: TaxonomyRoleChecker,
    contract: ContractRoleChecker,
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
    fn surface(&self) -> &dyn ISurfaceRoleChecker {
        &self.surface
    }
    fn agent(&self) -> &dyn IAgentRoleChecker {
        &self.agent
    }
}
