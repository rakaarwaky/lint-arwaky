use crate::role_rules::contract_agent_role_protocol::IAgentRoleChecker;
use crate::role_rules::contract_capabilities_role_protocol::ICapabilitiesRoleChecker;
use crate::role_rules::contract_role_protocol::IContractRoleChecker;
use crate::role_rules::contract_surface_role_protocol::ISurfaceRoleChecker;
use crate::role_rules::contract_taxonomy_role_protocol::ITaxonomyRoleChecker;
use crate::role_rules::contract_utility_role_protocol::IUtilityRoleChecker;

pub trait IRoleAggregate: Send + Sync {
    fn taxonomy(&self) -> &dyn ITaxonomyRoleChecker;
    fn contract(&self) -> &dyn IContractRoleChecker;
    fn capabilities(&self) -> &dyn ICapabilitiesRoleChecker;
    fn surface(&self) -> &dyn ISurfaceRoleChecker;
    fn agent(&self) -> &dyn IAgentRoleChecker;
    fn utility(&self) -> &dyn IUtilityRoleChecker;
}
