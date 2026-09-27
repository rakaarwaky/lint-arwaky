// config-system — taxonomy and contract types
pub mod contract_config_orchestrator_aggregate;
pub mod contract_config_protocol;
pub mod taxonomy_config_error;
pub mod taxonomy_config_request;
pub use crate::common::taxonomy_config_language_vo;
pub mod taxonomy_config_response;
pub mod taxonomy_config_vo;
pub mod utility_config_parser;

// ─── Re-exports ────────────────────────────────────────────
// Barrel re-export pattern: allows consumers to import directly

// ── Contract traits ──
pub use contract_config_orchestrator_aggregate::IConfigOrchestratorAggregate;
pub use contract_config_protocol::IConfigParserProtocol;
pub use contract_config_protocol::IConfigReaderProtocol;
pub use contract_config_protocol::IConfigValidatorProtocol;
pub use contract_config_protocol::IWorkspaceDetectorProtocol;
pub use taxonomy_config_vo::WorkspaceType;

// ── Taxonomy types ──
pub use crate::common::taxonomy_definition_vo::OrphanRuleVO;
pub use taxonomy_config_error::ConfigError;
pub use taxonomy_config_language_vo::ConfigLanguage;
pub use taxonomy_config_request::ConfigRequest;
pub use taxonomy_config_response::ConfigResponse;
pub use taxonomy_config_vo::AdapterEntry;
pub use taxonomy_config_vo::AdapterStatus;
pub use taxonomy_config_vo::ArchitectureConfig;
pub use taxonomy_config_vo::ArchitectureRule;
pub use taxonomy_config_vo::ConfigKey;
pub use taxonomy_config_vo::ConfigResult;
pub use taxonomy_config_vo::ConfigSource;
pub use taxonomy_config_vo::NamingRuleVO;
pub use taxonomy_config_vo::ProjectConfig;
pub use taxonomy_config_vo::RoleRuleVO;
pub use taxonomy_config_vo::Thresholds;
pub use taxonomy_config_vo::ValidationResult;
pub use taxonomy_config_vo::WorkspaceInfo;
