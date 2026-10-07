// config-system — taxonomy and contract types
pub mod contract_config_orchestrator_aggregate;
pub mod contract_config_protocol;
pub mod taxonomy_config_system_error;
pub mod taxonomy_config_system_request;
pub use shared_common::taxonomy_language_vo;
pub mod taxonomy_config_system_response;
pub mod taxonomy_config_system_vo;
pub mod utility_architecture_constants;
pub mod utility_config_merger;
pub mod utility_config_parser;

// ─── Re-exports ────────────────────────────────────────────
// Barrel re-export pattern: allows consumers to import directly

// ── Contract traits ──
pub use contract_config_orchestrator_aggregate::IConfigOrchestratorAggregate;
pub use contract_config_protocol::IConfigMergeProtocol;
pub use contract_config_protocol::IConfigReadProtocol;
pub use contract_config_protocol::IWorkspaceMembersProtocol;
pub use taxonomy_config_system_vo::WorkspaceType;

// ── Taxonomy types ──
pub use shared_common::taxonomy_language_vo::ConfigLanguage;
pub use shared_common::taxonomy_layer_vo::OrphanRuleVO;
pub use taxonomy_config_system_error::ConfigError;
pub use taxonomy_config_system_request::ConfigRequest;
pub use taxonomy_config_system_response::ConfigResponse;
pub use taxonomy_config_system_vo::AdapterEntry;
pub use taxonomy_config_system_vo::AdapterStatus;
pub use taxonomy_config_system_vo::ArchitectureConfig;
pub use taxonomy_config_system_vo::ArchitectureRule;
pub use taxonomy_config_system_vo::ConfigKey;
pub use taxonomy_config_system_vo::ConfigResult;
pub use taxonomy_config_system_vo::ConfigSource;
pub use taxonomy_config_system_vo::NamingRuleVO;
pub use taxonomy_config_system_vo::ProjectConfig;
pub use taxonomy_config_system_vo::RoleRuleVO;
pub use taxonomy_config_system_vo::Thresholds;
pub use taxonomy_config_system_vo::ValidationResult;
pub use taxonomy_config_system_vo::WorkspaceInfo;
