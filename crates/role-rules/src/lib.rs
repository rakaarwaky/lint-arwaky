// PURPOSE: Module declarations for role-rules (role auditors, orchestrator, container)
pub use agent_role_orchestrator::RoleCheckerDeps;
pub use shared::role_rules::IAgentRoleProtocol;
pub use shared::role_rules::ICapabilitiesRoleProtocol;
pub use shared::role_rules::IContractRoleProtocol;
pub use shared::role_rules::IRoleRunnerAggregate;
pub use shared::role_rules::ISurfaceRoleProtocol;
pub use shared::role_rules::ITaxonomyRoleProtocol;
pub use shared::role_rules::taxonomy_role_vo::{
    LayerNames, layer_agent, layer_capabilities, layer_contract, layer_global, layer_root,
    layer_surfaces, layer_taxonomy,
};
pub mod agent_role_orchestrator;
pub use agent_role_orchestrator::RoleOrchestrator;
pub mod capabilities_agent_python_role_auditor;
pub use capabilities_agent_python_role_auditor::AgentPythonRoleAuditor;
pub mod capabilities_agent_rust_role_auditor;
pub use capabilities_agent_rust_role_auditor::AgentRustRoleAuditor;
pub mod capabilities_agent_ts_role_auditor;
pub use capabilities_agent_ts_role_auditor::AgentTsRoleAuditor;
pub mod capabilities_capabilities_python_role_auditor;
pub use capabilities_capabilities_python_role_auditor::CapabilitiesPythonRoleAuditor;
pub mod capabilities_capabilities_rust_role_auditor;
pub use capabilities_capabilities_rust_role_auditor::CapabilitiesRustRoleAuditor;
pub mod capabilities_capabilities_ts_role_auditor;
pub use capabilities_capabilities_ts_role_auditor::CapabilitiesTypeScriptRoleAuditor;
pub mod capabilities_contract_python_role_auditor;
pub use capabilities_contract_python_role_auditor::ContractPythonRoleAuditor;
pub mod capabilities_contract_rust_role_auditor;
pub use capabilities_contract_rust_role_auditor::ContractRustRoleAuditor;
pub mod capabilities_contract_ts_role_auditor;
pub use capabilities_contract_ts_role_auditor::ContractTypeScriptRoleAuditor;
pub mod capabilities_surface_role_auditor;
pub use capabilities_surface_role_auditor::SurfaceRoleChecker;
pub mod capabilities_taxonomy_role_auditor;
pub use capabilities_taxonomy_role_auditor::TaxonomyRoleChecker;
pub mod capabilities_utility_python_role_auditor;
pub use capabilities_utility_python_role_auditor::UtilityPythonRoleAuditor;
pub mod capabilities_utility_rust_role_auditor;
pub use capabilities_utility_rust_role_auditor::UtilityRustRoleAuditor;
pub mod capabilities_utility_ts_role_auditor;
pub use capabilities_utility_ts_role_auditor::UtilityTypeScriptRoleAuditor;
pub mod root_role_rules_container;
pub mod utility_agent_role_checker;
pub mod utility_capabilities_role_checker;
pub mod utility_contract_role_checker;
pub mod utility_role_reference_scanner;
pub mod utility_utility_role_checker;
