// Contract tests — verify all capabilities implement their declared protocol traits.
use naming_rules_lint_arwaky::agent_naming_orchestrator::{
    NamingOrchestrator, NamingOrchestratorDeps,
};
use naming_rules_lint_arwaky::capabilities_naming_convention_checker::NamingConventionChecker;
use naming_rules_lint_arwaky::capabilities_suffix_policy_checker::SuffixPolicyChecker;
use shared::common::taxonomy_definition_vo::LayerMapVO;
use shared::config_system::taxonomy_config_system_vo::ArchitectureConfig;
use shared::naming_rules::INamingConventionProtocol;
use shared::naming_rules::INamingRunnerAggregate;
use shared::naming_rules::ISuffixPolicyProtocol;
use shared::naming_rules::taxonomy_naming_rules_request::NamingRequest;
use shared::naming_rules::taxonomy_naming_rules_response::NamingResponse;
use std::sync::Arc;

/// Compile-time trait bound assertion.
fn assert_naming_convention_trait<T: INamingConventionProtocol>() {}
fn assert_suffix_policy_trait<T: ISuffixPolicyProtocol>() {}
fn assert_naming_runner_aggregate_trait<T: INamingRunnerAggregate>() {}

#[test]
fn naming_checker_implements_protocol() {
    assert_naming_convention_trait::<NamingConventionChecker>();
    assert_suffix_policy_trait::<SuffixPolicyChecker>();
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
        naming_convention: Arc::new(NamingConventionChecker::new()),
        suffix_policy: Arc::new(SuffixPolicyChecker::new()),
        config,
        layer_map,
    };
    let orch = NamingOrchestrator::new(deps);
    match orch.execute(NamingRequest::Name) {
        NamingResponse::Name { name } => assert_eq!(name, "naming-rules"),
        NamingResponse::Audit { .. } => panic!("expected a name response"),
    }
}
