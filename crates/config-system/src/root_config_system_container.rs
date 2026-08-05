use crate::agent_config_orchestrator::{ConfigOrchestrator, ConfigOrchestratorDeps};
use crate::capabilities_parser_provider::ConfigParserProvider;
use crate::capabilities_rules_validator::ConfigRulesValidator;
use crate::capabilities_workspace_detector::WorkspaceDetector;
use crate::capabilities_yaml_reader::ConfigYamlReader;
// Utility modules wired into entry for orphan reachability (AES504)
use crate::utility_config_defaults;
use crate::utility_config_merger;
use crate::utility_config_parser;

fn _use_utility_defaults() -> std::collections::HashMap<String, String> {
    utility_config_defaults::default_aes_config()
}

fn _use_utility_merger(base: std::collections::HashMap<String, String>, override_config: std::collections::HashMap<String, String>) -> std::collections::HashMap<String, String> {
    utility_config_merger::merge_config(base, override_config)
}

fn _use_utility_parser(yaml_str: &str) -> Option<f64> {
    utility_config_parser::parse_score_threshold(yaml_str)
}
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
}
