// role-rules — taxonomy and contract types
pub mod contract_role_protocol;
pub mod contract_role_runner_aggregate;
pub mod taxonomy_layer_names_constant;
pub mod taxonomy_layer_names_vo;
pub mod taxonomy_role_request_vo;
pub mod taxonomy_violation_role_vo;

// ─── Re-exports ────────────────────────────────────────────
// Barrel re-export pattern: allows consumers to import directly

// ── Contract traits ──
pub use contract_role_protocol::IAgentRoleProtocol;
pub use contract_role_protocol::ICapabilitiesRoleProtocol;
pub use contract_role_protocol::IContractRoleProtocol;
pub use contract_role_protocol::ISurfaceRoleProtocol;
pub use contract_role_protocol::ITaxonomyRoleProtocol;
pub use contract_role_protocol::IUtilityRoleProtocol;
pub use contract_role_runner_aggregate::IRoleRunnerAggregate;

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
pub use taxonomy_role_request_vo::RoleRequest;
pub use taxonomy_role_request_vo::RoleResponse;
pub use taxonomy_violation_role_vo::AesRoleViolation;
