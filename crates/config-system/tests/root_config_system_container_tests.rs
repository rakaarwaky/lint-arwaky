use config_system_lint_arwaky::root_config_system_container::ConfigContainer;

#[test]
fn container_can_be_constructed() {
    let container = ConfigContainer::new();
    let _orchestrator = container.orchestrator();
    let _parser = container.parser();
    let _validator = container.validator();
    let _multi_project = container.multi_project_orchestrator();
}

#[test]
fn container_default_is_same_as_new() {
    let c1 = ConfigContainer::new();
    let c2 = ConfigContainer::default();
    let r1 = c1.validator().validate_thresholds();
    let r2 = c2.validator().validate_thresholds();
    assert_eq!(r1.is_valid, r2.is_valid);
}

#[test]
fn container_orchestrator_is_accessible() {
    use shared::config_system::contract_orchestration_aggregate::IConfigOrchestrationAggregate;
    let container = ConfigContainer::new();
    let orch = container.orchestrator();
    let _ws = orch.workspace_detector();
    let _reader = orch.config_reader();
}

#[test]
fn container_parser_is_accessible() {
    use shared::config_system::contract_parser_port::IConfigParserPort;
    use shared::common::taxonomy_path_vo::FilePath;
    let container = ConfigContainer::new();
    let parser = container.parser();
    // Parser should handle nonexistent paths gracefully
    let fp = FilePath::new("/nonexistent/config.yaml".to_string()).unwrap();
    let result = parser.parse_yaml_config(&fp);
    assert!(result.is_err());
}

#[test]
fn container_validator_uses_default_config() {
    use shared::config_system::contract_validator_protocol::IConfigValidatorProtocol;
    let container = ConfigContainer::new();
    let validator = container.validator();
    // Default thresholds should be valid
    assert!(validator.validate_thresholds().is_valid);
    // Unlisted adapters default to enabled
    assert!(validator.is_adapter_enabled(
        &shared::common::taxonomy_adapter_name_vo::AdapterName::raw("any_tool")
    ));
}

#[test]
fn container_multi_project_orchestrator_is_accessible() {
    use shared::config_system::contract_multi_project_orchestrator_aggregate::MultiProjectOrchestratorAggregate;
    use shared::common::taxonomy_path_vo::FilePath;
    let container = ConfigContainer::new();
    let mp = container.multi_project_orchestrator();
    // Should handle nonexistent paths without panic
    let fp = FilePath::new("/nonexistent/path".to_string()).unwrap();
    let rt = tokio::runtime::Runtime::new().unwrap();
    let workspaces = rt.block_on(mp.discover_workspaces(&fp));
    assert!(workspaces.is_empty());
}
