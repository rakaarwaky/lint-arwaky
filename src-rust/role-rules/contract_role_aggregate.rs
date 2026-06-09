use crate::role_rules::contract_agentrole_protocol::IAgentRoleChecker;
use crate::role_rules::contract_contractrole_protocol::IContractRoleChecker;
use crate::role_rules::contract_surfacerole_protocol::ISurfaceRoleChecker;
use crate::role_rules::contract_taxonomyrole_protocol::ITaxonomyRoleChecker;

pub trait IRoleAggregate: Send + Sync {
    fn taxonomy(&self) -> &dyn ITaxonomyRoleChecker;
    fn contract(&self) -> &dyn IContractRoleChecker;
    fn surface(&self) -> &dyn ISurfaceRoleChecker;
    fn agent(&self) -> &dyn IAgentRoleChecker;
}
