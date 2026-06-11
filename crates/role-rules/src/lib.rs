// PURPOSE: Module declarations and re-exports for role-rules (role auditors, protocols, orchestrator)
pub mod taxonomy_layer_names_vo;
pub use taxonomy_layer_names_vo::{
    layer_agent, layer_capabilities, layer_contract, layer_global, layer_infrastructure,
    layer_root, layer_surfaces, layer_taxonomy, LayerNames,
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
pub mod capabilities_taxonomy_role_auditor;
pub use capabilities_taxonomy_role_auditor::TaxonomyRoleChecker;

pub mod contract_agent_role_protocol;
pub use contract_agent_role_protocol::IAgentRoleChecker;
pub mod contract_role_protocol;
pub use contract_role_protocol::IContractRoleChecker;
pub mod contract_taxonomy_role_protocol;
pub use contract_taxonomy_role_protocol::ITaxonomyRoleChecker;
pub mod contract_role_aggregate;
pub use contract_role_aggregate::IRoleAggregate;
pub mod contract_role_runner_aggregate;
pub use contract_role_runner_aggregate::IRoleRunnerAggregate;
pub mod contract_surface_role_protocol;
pub use contract_surface_role_protocol::ISurfaceRoleChecker;
pub mod contract_capabilities_role_protocol;
pub use contract_capabilities_role_protocol::ICapabilitiesRoleChecker;
pub mod agent_role_container;
pub use agent_role_container::RoleAggregateImpl;
pub mod role_container;
