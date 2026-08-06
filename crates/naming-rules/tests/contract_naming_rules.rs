// Contract tests — verify all capabilities implement their declared protocol traits.
use naming_rules_lint_arwaky::agent_naming_orchestrator::{NamingOrchestrator, NamingOrchestratorDeps};
use naming_rules_lint_arwaky::capabilities_naming_convention_checker::NamingConventionChecker;
use naming_rules_lint_arwaky::capabilities_suffix_prefix_checker::SuffixPrefixChecker;
use shared::common::taxonomy_definition_vo::LayerMapVO;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::naming_rules::INamingConventionChecker;
use shared::naming_rules::INamingRunnerAggregate;
use shared::naming_rules::ISuffixPrefixChecker;
use std::sync::Arc;

/// Compile-time trait bound assertion.
fn assert_naming_convention_checker_trait<T: INamingConventionChecker>() {}
fn assert_suffix_prefix_checker_trait<T: ISuffixPrefixChecker>() {}
fn assert_naming_runner_aggregate_trait<T: INamingRunnerAggregate>() {}

#[test]
fn naming_convention_checker_implements_protocol() {
    assert_naming_convention_checker_trait::<NamingConventionChecker>();
}

#[test]
fn suffix_prefix_checker_implements_protocol() {
    assert_suffix_prefix_checker_trait::<SuffixPrefixChecker>();
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
        naming_convention_checker: Arc::new(NamingConventionChecker::new()),
        suffix_prefix_checker: Arc::new(SuffixPrefixChecker::new()),
        config,
        layer_map,
    };
    let orch = NamingOrchestrator::new(deps);
    assert_eq!(orch.name(), "naming-rules");
}
