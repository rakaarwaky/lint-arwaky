use crate::agent_config_orchestrator::{ConfigOrchestrator, ConfigOrchestratorDeps};
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
        let workspace_detector =
            Arc::new(crate::capabilities_workspace_detector::WorkspaceDetector::new(filesystem.clone()));
        let yaml_reader = Arc::new(crate::capabilities_yaml_reader::ConfigYamlReader::new(
            filesystem.clone(),
        ));
        let validator = Arc::new(crate::capabilities_rules_validator::ConfigRulesValidator::new());
        let parser = Arc::new(
            crate::capabilities_parser_provider::ConfigParserProvider::new(filesystem.clone()),
        );

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
