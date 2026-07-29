// role-rules — taxonomy and contract types
pub mod contract_agent_role_protocol;
pub mod contract_capabilities_role_protocol;
pub mod contract_role_contract_protocol;
pub mod contract_role_runner_aggregate;
pub mod contract_surface_role_protocol;
pub mod contract_taxonomy_role_protocol;
pub mod contract_utility_role_protocol;
pub mod taxonomy_layer_names_constant;
pub mod taxonomy_layer_names_vo;
pub mod taxonomy_violation_role_vo;

// ─── Re-exports ────────────────────────────────────────────
// Barrel re-export pattern: allows consumers to import directly

// ── Contract traits ──
pub use contract_agent_role_protocol::IAgentRoleChecker;
pub use contract_capabilities_role_protocol::ICapabilitiesRoleChecker;
pub use contract_role_contract_protocol::IContractRoleChecker;
pub use contract_role_runner_aggregate::IRoleRunnerAggregate;
pub use contract_surface_role_protocol::ISurfaceRoleChecker;
pub use contract_taxonomy_role_protocol::ITaxonomyRoleChecker;
pub use contract_utility_role_protocol::IUtilityRoleChecker;

// ── Taxonomy types ──
pub use taxonomy_layer_names_constant::LAYER_AGENT;
pub use taxonomy_layer_names_constant::LAYER_CAPABILITIES;
pub use taxonomy_layer_names_constant::LAYER_CONTRACT;
pub use taxonomy_layer_names_constant::LAYER_GLOBAL;
pub use taxonomy_layer_names_constant::LAYER_ROOT;
pub use taxonomy_layer_names_constant::LAYER_SURFACES;
pub use taxonomy_layer_names_constant::LAYER_TAXONOMY;
pub use taxonomy_layer_names_constant::LAYER_UTILITY;
pub use taxonomy_layer_names_vo::layer_surfaces;
pub use taxonomy_violation_role_vo::AesRoleViolation;
pub use taxonomy_violation_role_vo::LabeledRoleViolation;
