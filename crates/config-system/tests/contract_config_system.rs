// Verify that all concrete types implement their declared contract traits.
use config_system_lint_arwaky::agent_config_orchestrator::ConfigOrchestrator;
use config_system_lint_arwaky::capabilities_parser_provider::ConfigParserProvider;
use config_system_lint_arwaky::capabilities_rules_validator::ConfigRulesValidator;
use config_system_lint_arwaky::capabilities_workspace_detector::WorkspaceDetector;
use config_system_lint_arwaky::capabilities_yaml_reader::ConfigYamlReader;
use shared::config_system::contract_config_orchestrator_aggregate::IConfigOrchestratorAggregate;
use shared::config_system::contract_parser_protocol::IConfigParserProtocol;
use shared::config_system::contract_reader_protocol::IConfigReaderProtocol;
use shared::config_system::contract_validator_protocol::IConfigValidatorProtocol;
use shared::config_system::contract_workspace_detector_protocol::IWorkspaceDetectorProtocol;

#[test]
fn config_orchestrator_implements_aggregate() {
    fn assert_trait<T: IConfigOrchestratorAggregate>() {}
    assert_trait::<ConfigOrchestrator>();
}

#[test]
fn config_yaml_reader_implements_reader_protocol() {
    fn assert_trait<T: IConfigReaderProtocol>() {}
    assert_trait::<ConfigYamlReader>();
}

#[test]
fn config_rules_validator_implements_validator_protocol() {
    fn assert_trait<T: IConfigValidatorProtocol>() {}
    assert_trait::<ConfigRulesValidator>();
}

#[test]
fn workspace_detector_implements_detector_protocol() {
    fn assert_trait<T: IWorkspaceDetectorProtocol>() {}
    assert_trait::<WorkspaceDetector>();
}

#[test]
fn config_parser_provider_implements_parser_protocol() {
    fn assert_trait<T: IConfigParserProtocol>() {}
    assert_trait::<ConfigParserProvider>();
}

#[test]
fn all_contracts_are_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<ConfigOrchestrator>();
    assert_send_sync::<ConfigYamlReader>();
    assert_send_sync::<ConfigRulesValidator>();
    assert_send_sync::<WorkspaceDetector>();
    assert_send_sync::<ConfigParserProvider>();
}
