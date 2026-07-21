use shared::config_system::contract_config_orchestrator_aggregate::IConfigOrchestratorAggregate;
use shared::config_system::contract_parser_protocol::IConfigParserProtocol;
use shared::config_system::contract_reader_protocol::IConfigReaderProtocol;
use shared::config_system::contract_validator_protocol::IConfigValidatorProtocol;
use std::sync::Arc;

pub struct ConfigContainer {
    orchestrator: Arc<dyn IConfigOrchestratorAggregate>,
    reader: Arc<dyn IConfigReaderProtocol>,
    parser: Arc<dyn IConfigParserProtocol>,
    validator: Arc<dyn IConfigValidatorProtocol>,
}

impl Default for ConfigContainer {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigContainer {
    pub fn new() -> Self {
        let workspace_detector =
            Arc::new(crate::capabilities_workspace_detector_provider::WorkspaceDetector::new());
        let yaml_reader = Arc::new(crate::capabilities_yaml_reader::ConfigYamlReader::new());

        Self {
            orchestrator: Arc::new(crate::agent_config_orchestrator::ConfigOrchestrator::new(
                workspace_detector,
                yaml_reader.clone(),
            )),
            reader: yaml_reader,
            parser: Arc::new(crate::capabilities_parser_provider::ConfigParserProvider::new()),
            validator: Arc::new(crate::capabilities_rules_validator::ConfigRulesValidator::new()),
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
