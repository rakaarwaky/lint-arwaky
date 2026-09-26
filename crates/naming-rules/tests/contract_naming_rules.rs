// Contract tests — verify all capabilities implement their declared protocol traits.
use naming_rules_lint_arwaky::agent_naming_orchestrator::{
    NamingOrchestrator, NamingOrchestratorDeps,
};
use naming_rules_lint_arwaky::capabilities_naming_checker::NamingChecker;
use shared::common::taxonomy_definition_vo::LayerMapVO;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::naming_rules::INamingCheckerProtocol;
use shared::naming_rules::INamingRunnerAggregate;
use std::sync::Arc;

/// Compile-time trait bound assertion.
fn assert_naming_checker_trait<T: INamingCheckerProtocol>() {}
fn assert_naming_runner_aggregate_trait<T: INamingRunnerAggregate>() {}

#[test]
fn naming_checker_implements_protocol() {
    assert_naming_checker_trait::<NamingChecker>();
}

#[test]
fn naming_orchestrator_implements_aggregate_trait() {
    assert_naming_runner_aggregate_trait::<NamingOrchestrator>();
}

#[test]
fn naming_orchestrator_name_returns_expected() {
    let config = Arc::new(ArchitectureConfig::default());
    let layer_map = Arc::new(LayerMapVO::new(std::collections::HashMap::new()));
    let deps = NamingOrchestratorDeps {
        naming_checker: Arc::new(NamingChecker::new()),
        config,
        layer_map,
    };
    let orch = NamingOrchestrator::new(deps);
    assert_eq!(orch.name(), "naming-rules");
}
