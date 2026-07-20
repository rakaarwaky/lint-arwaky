// PURPOSE: Module declarations for config-system (orchestrator, validators, providers)
pub mod agent_config_loading_orchestrator;
pub use agent_config_loading_orchestrator::ConfigLoadingOrchestrator;
pub mod capabilities_rules_validator;
pub use capabilities_rules_validator::ConfigRulesValidator;
pub mod capabilities_workspace_detector_provider;
pub use capabilities_workspace_detector_provider::WorkspaceDetector;
pub mod capabilities_parser_provider;
pub use capabilities_parser_provider::ConfigParserProvider;
pub mod capabilities_yaml_reader;
pub use capabilities_yaml_reader::ConfigYamlReader;
pub mod agent_multi_project_orchestrator;
pub use agent_multi_project_orchestrator::MultiProjectOrchestrator;
pub mod root_config_system_container;
