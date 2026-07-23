// PURPOSE: Module declarations for role-rules (role auditors, orchestrator, container)
pub use shared::role_rules::contract_agent_role_protocol::IAgentRoleChecker;
pub use shared::role_rules::contract_capabilities_role_protocol::ICapabilitiesRoleChecker;
pub use agent_role_orchestrator::RoleCheckerDeps;
pub use shared::role_rules::contract_role_contract_protocol::IContractRoleChecker;
pub use shared::role_rules::contract_role_runner_aggregate::IRoleRunnerAggregate;
pub use shared::role_rules::contract_surface_role_protocol::ISurfaceRoleChecker;
pub use shared::role_rules::contract_taxonomy_role_protocol::ITaxonomyRoleChecker;
pub use shared::role_rules::taxonomy_layer_names_vo::{
    layer_agent, layer_capabilities, layer_contract, layer_global, layer_root, layer_surfaces,
    layer_taxonomy, LayerNames,
};
pub mod agent_role_orchestrator;
pub use agent_role_orchestrator::RoleOrchestrator;
pub mod capabilities_agent_role_auditor;
pub use capabilities_agent_role_auditor::AgentRoleChecker;

pub mod capabilities_contract_role_auditor;
pub use capabilities_contract_role_auditor::ContractRoleChecker;
pub mod capabilities_capabilities_role_auditor;
pub use capabilities_capabilities_role_auditor::CapabilitiesRoleChecker;
pub mod capabilities_surface_role_auditor;
pub use capabilities_surface_role_auditor::SurfaceRoleChecker;
pub mod capabilities_utility_role_auditor;
pub use capabilities_utility_role_auditor::UtilityRoleChecker;
pub mod capabilities_taxonomy_role_auditor;
pub use capabilities_taxonomy_role_auditor::TaxonomyRoleChecker;
pub mod root_role_rules_container;
