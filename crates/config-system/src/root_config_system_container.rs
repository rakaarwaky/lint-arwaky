use crate::agent_config_orchestrator::{ConfigOrchestrator, ConfigOrchestratorDeps};
use crate::capabilities_parser_provider::ConfigParserProvider;
use crate::capabilities_rules_validator::ConfigRulesValidator;
use crate::capabilities_workspace_detector::WorkspaceDetector;
use crate::capabilities_yaml_reader::ConfigYamlReader;
// Utility modules wired into entry for orphan reachability (AES504)
use crate::utility_config_merger;
use crate::utility_config_parser;
use shared::config_system::{
    IConfigOrchestratorAggregate, IConfigParserProtocol, IConfigReaderProtocol,
    IConfigValidatorProtocol,
};
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;

use std::sync::Arc;

pub struct ConfigContainer {
    orchestrator: Arc<dyn IConfigOrchestratorAggregate>,
    reader: Arc<dyn IConfigReaderProtocol>,
    parser: Arc<dyn IConfigParserProtocol>,
    validator: Arc<dyn IConfigValidatorProtocol>,
}

impl ConfigContainer {
    pub fn new(filesystem: Arc<dyn IFilesystemAggregate>) -> Self {
        let workspace_detector = Arc::new(WorkspaceDetector::new(filesystem.clone()));
        let yaml_reader = Arc::new(ConfigYamlReader::new(filesystem.clone()));
        let validator = Arc::new(ConfigRulesValidator::new());
        let parser = Arc::new(ConfigParserProvider::new(filesystem.clone()));

        Self {
            orchestrator: Arc::new(ConfigOrchestrator::new(ConfigOrchestratorDeps {
                workspace_detector,
                config_reader: yaml_reader.clone(),
                parser: parser.clone(),
                validator: validator.clone(),
                filesystem,
            })),
            reader: yaml_reader,
            parser,
            validator,
        }
    }

    pub fn orchestrator(&self) -> Arc<dyn IConfigOrchestratorAggregate> {
        self.orchestrator.clone()
    }

    pub fn reader(&self) -> Arc<dyn IConfigReaderProtocol> {
        self.reader.clone()
    }

    pub fn parser(&self) -> Arc<dyn IConfigParserProtocol> {
        self.parser.clone()
    }

    pub fn validator(&self) -> Arc<dyn IConfigValidatorProtocol> {
        self.validator.clone()
    }

    /// Get default AES configuration (from shared parser).
    pub fn default_config(&self) -> shared::config_system::taxonomy_config_vo::ArchitectureConfig {
        shared::config_system::utility_config_parser::default_aes_config()
    }

    /// Parse score threshold from YAML (uses utility_config_parser).
    pub fn parse_score_threshold(&self, yaml_str: &str) -> Option<f64> {
        utility_config_parser::parse_score_threshold(yaml_str)
    }

    /// Merge configuration layers (uses utility_config_merger).
    pub fn merge_layers(
        &self,
        config: &shared::config_system::taxonomy_config_vo::ArchitectureConfig,
    ) {
        let _ = utility_config_merger::merge_config(config);
    }
}
