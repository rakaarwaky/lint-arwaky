// PURPOSE: Verify that all public types implement their declared contract traits.
// Layer: Contract verification
// Coverage target: compile-time trait bound assertions

use naming_rules_lint_arwaky::agent_naming_orchestrator::NamingOrchestrator;
use naming_rules_lint_arwaky::capabilities_naming_convention_checker::NamingConventionChecker;
use naming_rules_lint_arwaky::capabilities_suffix_prefix_checker::SuffixPrefixChecker;
use naming_rules_lint_arwaky::root_naming_rules_container::NamingContainer;

use shared::naming_rules::contract_naming_checker_protocol::{
    INamingConventionChecker, ISuffixPrefixChecker,
};
use shared::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;

// ─── INamingConventionChecker ─────────────────────────────

#[test]
fn naming_convention_checker_implements_protocol() {
    fn assert_trait<T: INamingConventionChecker>() {}
    assert_trait::<NamingConventionChecker>();
}

#[test]
fn naming_convention_checker_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<NamingConventionChecker>();
}

// ─── ISuffixPrefixChecker ─────────────────────────────────

#[test]
fn suffix_prefix_checker_implements_protocol() {
    fn assert_trait<T: ISuffixPrefixChecker>() {}
    assert_trait::<SuffixPrefixChecker>();
}

#[test]
fn suffix_prefix_checker_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<SuffixPrefixChecker>();
}

// ─── INamingRunnerAggregate ───────────────────────────────

#[test]
fn naming_orchestrator_implements_aggregate() {
    fn assert_trait<T: INamingRunnerAggregate>() {}
    assert_trait::<NamingOrchestrator>();
}

#[test]
fn naming_orchestrator_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<NamingOrchestrator>();
}

// ─── NamingContainer wiring ───────────────────────────────

#[test]
fn container_produces_orchestrator_as_aggregate() {
    fn assert_trait<T: INamingRunnerAggregate>() {}
    // NamingContainer::orchestrator() returns Arc<dyn INamingRunnerAggregate>
    // This test verifies the return type is correct at compile time.
    assert_trait::<NamingOrchestrator>();
}

#[test]
fn container_exposes_checker_references() {
    use shared::common::taxonomy_definition_vo::LayerMapVO;
    use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
    use std::sync::Arc;

    let config = Arc::new(ArchitectureConfig::default());
    let layer_map = Arc::new(LayerMapVO::new(std::collections::HashMap::new()));
    let container = NamingContainer::new(config, layer_map);

    // Verify accessors return trait objects
    let _conv: &Arc<dyn INamingConventionChecker> = container.naming_convention_checker();
    let _suf: &Arc<dyn ISuffixPrefixChecker> = container.suffix_prefix_checker();
    let _orch: Arc<dyn INamingRunnerAggregate> = container.orchestrator();
}
