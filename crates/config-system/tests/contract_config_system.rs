// Verify that all concrete types implement their declared contract traits.
use config_system_lint_arwaky::agent_config_orchestrator::ConfigOrchestrator;
use config_system_lint_arwaky::capabilities_parser_provider::ConfigParserProvider;
use config_system_lint_arwaky::capabilities_rules_validator::ConfigRulesValidator;
use config_system_lint_arwaky::capabilities_workspace_detector::WorkspaceDetector;
use config_system_lint_arwaky::capabilities_yaml_reader::ConfigYamlReader;
use shared::config_system::{
    IConfigOrchestratorAggregate, IConfigParseProtocol, IConfigReadProtocol,
    IConfigValidateProtocol, IWorkspaceDetectProtocol,
};

#[test]
fn config_orchestrator_implements_aggregate() {
    fn assert_trait<T: IConfigOrchestratorAggregate>() {}
    assert_trait::<ConfigOrchestrator>();
}

#[test]
fn config_yaml_reader_implements_read_protocol() {
    fn assert_trait<T: IConfigReadProtocol>() {}
    assert_trait::<ConfigYamlReader>();
}

#[test]
fn config_rules_validator_implements_validate_protocol() {
    fn assert_trait<T: IConfigValidateProtocol>() {}
    assert_trait::<ConfigRulesValidator>();
}

#[test]
fn workspace_detector_implements_detect_protocol() {
    fn assert_trait<T: IWorkspaceDetectProtocol>() {}
    assert_trait::<WorkspaceDetector>();
}

#[test]
fn config_parser_provider_implements_parse_protocol() {
    fn assert_trait<T: IConfigParseProtocol>() {}
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
