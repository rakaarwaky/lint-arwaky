use crate::role_rules::contract_agent_role_protocol::IAgentRoleChecker;
use crate::role_rules::contract_role_protocol::IContractRoleChecker;
use crate::role_rules::contract_surface_role_protocol::ISurfaceRoleChecker;
use crate::role_rules::contract_taxonomy_role_protocol::ITaxonomyRoleChecker;

pub trait IRoleAggregate: Send + Sync {
    fn taxonomy(&self) -> &dyn ITaxonomyRoleChecker;
    fn contract(&self) -> &dyn IContractRoleChecker;
    fn surface(&self) -> &dyn ISurfaceRoleChecker;
    fn agent(&self) -> &dyn IAgentRoleChecker;
}
