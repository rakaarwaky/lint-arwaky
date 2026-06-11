// PURPOSE: ConfigContainer — wiring for config-system feature (root layer, wiring only)
use std::sync::Arc;
use crate::config_system::contract_discovery_port::IConfigDiscoveryPort;
use crate::config_system::contract_orchestration_aggregate::IConfigOrchestrationAggregate;
use crate::config_system::contract_parser_port::IConfigParserPort;
use crate::config_system::contract_validator_protocol::IConfigValidatorProtocol;

pub struct ConfigContainer {
    discovery: Arc<dyn IConfigDiscoveryPort>,
    orchestrator: Arc<dyn IConfigOrchestrationAggregate>,
    parser: Arc<dyn IConfigParserPort>,
    validator: Arc<dyn IConfigValidatorProtocol>,
}

impl ConfigContainer {
    pub fn new(path_norm: Arc<dyn crate::source_parsing::contract_path_normalization_port::IPathNormalizationPort>) -> Self {
        Self {
            discovery: Arc::new(
                crate::config_system::infrastructure_discovery_provider::ConfigDiscoveryProvider::new(),
            ),
            orchestrator: Arc::new(
                crate::config_system::agent_config_loading_orchestrator::ConfigLoadingOrchestrator::new(
                    Arc::new(
                        crate::config_system::infrastructure_detector_provider::LanguageDetectorProvider::new(),
                    ),
                    Arc::new(
                        crate::config_system::infrastructure_yaml_reader::ConfigYamlReader::new(path_norm.clone()),
                    ),
                ),
            ),
            parser: Arc::new(
                crate::config_system::infrastructure_parser_provider::ConfigParserProvider::new(),
            ),
            validator: Arc::new(
                crate::config_system::capabilities_rules_validator::ConfigRulesValidator::new(
                    crate::config_system::taxonomy_setting_vo::ProjectConfig::defaults(),
                ),
            ),
        }
    }

    pub fn discovery(&self) -> Arc<dyn IConfigDiscoveryPort> {
        self.discovery.clone()
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
}
