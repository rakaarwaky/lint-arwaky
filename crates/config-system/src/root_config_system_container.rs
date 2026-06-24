use shared::config_system::contract_multi_project_orchestrator_aggregate::MultiProjectOrchestratorAggregate;
use shared::config_system::contract_orchestration_aggregate::IConfigOrchestrationAggregate;
use shared::config_system::contract_parser_port::IConfigParserPort;
use shared::config_system::contract_validator_protocol::IConfigValidatorProtocol;
use std::sync::Arc;

pub struct ConfigContainer {
    orchestrator: Arc<dyn IConfigOrchestrationAggregate>,
    parser: Arc<dyn IConfigParserPort>,
    validator: Arc<dyn IConfigValidatorProtocol>,
    multi_project_orchestrator: Arc<dyn MultiProjectOrchestratorAggregate>,
}

impl Default for ConfigContainer {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigContainer {
    pub fn new() -> Self {
        let workspace_detector =
            Arc::new(crate::infrastructure_workspace_detector_provider::WorkspaceDetector::new());
        let yaml_reader = Arc::new(crate::infrastructure_yaml_reader::ConfigYamlReader::new());

        Self {
            orchestrator: Arc::new(
                crate::agent_config_loading_orchestrator::ConfigLoadingOrchestrator::new(
                    workspace_detector.clone(),
                    yaml_reader.clone(),
                ),
            ),
            parser: Arc::new(crate::infrastructure_parser_provider::ConfigParserProvider::new()),
            validator: Arc::new(
                crate::capabilities_rules_validator::ConfigRulesValidator::new(
                    shared::config_system::taxonomy_setting_vo::ProjectConfig::defaults(),
                ),
            ),
            multi_project_orchestrator: Arc::new(
                crate::agent_multi_project_orchestrator::MultiProjectOrchestrator::new(
                    workspace_detector,
                    yaml_reader,
                ),
            ),
        }
    }

    pub fn orchestrator(&self) -> Arc<dyn IConfigOrchestrationAggregate> {
        self.orchestrator.clone()
    }

    pub fn parser(&self) -> Arc<dyn IConfigParserPort> {
        self.parser.clone()
    }

    pub fn validator(&self) -> Arc<dyn IConfigValidatorProtocol> {
        self.validator.clone()
    }

    pub fn multi_project_orchestrator(&self) -> Arc<dyn MultiProjectOrchestratorAggregate> {
        self.multi_project_orchestrator.clone()
    }
}
