// config-system — taxonomy and contract types
pub mod contract_config_orchestrator_aggregate;
pub mod contract_parser_protocol;
pub mod contract_reader_protocol;
pub mod contract_validator_protocol;
pub mod contract_workspace_detector_protocol;
pub mod taxonomy_config_error;
pub mod taxonomy_config_language_vo;
pub mod taxonomy_config_parser;
pub mod taxonomy_config_vo;
pub mod taxonomy_identifier_vo;
pub mod taxonomy_multi_project_workspace_info_vo;
pub mod taxonomy_setting_vo;
pub mod taxonomy_source_vo;
pub mod taxonomy_validation_vo;

// ─── Re-exports ────────────────────────────────────────────
// Barrel re-export pattern: allows consumers to import directly

// ── Contract traits ──
pub use contract_config_orchestrator_aggregate::IConfigOrchestratorAggregate;
pub use contract_parser_protocol::IConfigParserProtocol;
pub use contract_reader_protocol::IConfigReaderProtocol;
pub use contract_validator_protocol::IConfigValidatorProtocol;
pub use contract_workspace_detector_protocol::IWorkspaceDetectorProtocol;
pub use contract_workspace_detector_protocol::WorkspaceType;

// ── Taxonomy types ──
pub use crate::common::taxonomy_definition_vo::OrphanRuleVO;
pub use taxonomy_config_error::ConfigError;
pub use taxonomy_config_language_vo::ConfigLanguage;
pub use taxonomy_config_vo::ArchitectureConfig;
pub use taxonomy_config_vo::ArchitectureRule;
pub use taxonomy_config_vo::NamingRuleVO;
pub use taxonomy_config_vo::RoleRuleVO;
pub use taxonomy_identifier_vo::ConfigKey;
pub use taxonomy_multi_project_workspace_info_vo::WorkspaceInfo;
pub use taxonomy_setting_vo::AdapterEntry;
pub use taxonomy_setting_vo::AdapterStatus;
pub use taxonomy_setting_vo::ProjectConfig;
pub use taxonomy_setting_vo::Thresholds;
pub use taxonomy_source_vo::ConfigResult;
pub use taxonomy_source_vo::ConfigSource;
pub use taxonomy_validation_vo::ValidationResult;
