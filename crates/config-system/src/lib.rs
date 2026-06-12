// PURPOSE: Module declarations for config-system (orchestrator, validators, providers)
pub use shared::config_system::taxonomy_adapter_vo::{AdapterClassMap, AdapterMetadataList, AdapterNameList};
pub use shared::config_system::taxonomy_validation_vo::ValidationResult;
pub use shared::config_system::contract_validator_protocol::IConfigValidatorProtocol;
pub use shared::config_system::contract_detector_port::ILanguageDetectorPort;
pub use shared::config_system::contract_discovery_port::IConfigDiscoveryPort;
pub use shared::config_system::contract_orchestration_aggregate::IConfigOrchestrationAggregate;
pub use shared::config_system::contract_parser_port::IConfigParserPort;
pub use shared::config_system::contract_reader_port::IConfigReaderPort;
pub use shared::config_system::taxonomy_app_vo::AppConfig;
pub use shared::config_system::taxonomy_config_vo::{default_aes_config, default_config_for_language, ArchitectureConfig};
pub use shared::config_system::taxonomy_identifier_vo::ConfigKey;
pub use shared::config_system::taxonomy_config_error::ConfigError;
pub use shared::config_system::taxonomy_setting_vo::{
    ActualValue, AdapterEntry, AdapterStatus, ExpectedValue, ProjectConfig, Thresholds,
};
pub use shared::config_system::taxonomy_source_vo::{ConfigResult, ConfigSource};
pub mod agent_config_loading_orchestrator;
pub use agent_config_loading_orchestrator::ConfigLoadingOrchestrator;
pub mod capabilities_rules_validator;
pub use capabilities_rules_validator::ConfigRulesValidator;
pub mod infrastructure_detector_provider;
pub use infrastructure_detector_provider::LanguageDetectorProvider;
pub mod infrastructure_discovery_provider;
pub use infrastructure_discovery_provider::ConfigDiscoveryProvider;
pub mod infrastructure_parser_provider;
pub use infrastructure_parser_provider::ConfigParserProvider;
pub mod infrastructure_yaml_reader;
pub use infrastructure_yaml_reader::ConfigYamlReader;
pub mod root_config_container;
