// PURPOSE: Integration tests — verify DI container wiring and cross-layer collaboration.
// Layer: Integration (uses real RoleContainer).

use role_rules_lint_arwaky::root_role_rules_container::RoleContainer;
use shared::role_rules::contract_role_aggregate::IRoleAggregate;
use std::sync::Arc;

fn container() -> RoleContainer {
    RoleContainer::new_with_config(
        shared::config_system::taxonomy_config_vo::ArchitectureConfig::default(),
    )
}

// ─── Container wiring tests ──

#[test]
fn container_creates_orchestrator_successfully() {
    let c = container();
    let _orch = c.orchestrator();
}

#[test]
fn container_aggregate_returns_trait_object() {
    let c = container();
    let agg: Arc<dyn IRoleAggregate> = c.aggregate();
    // Verify all 6 checkers are accessible
    let _taxonomy = agg.taxonomy();
    let _contract = agg.contract();
    let _capabilities = agg.capabilities();
    let _surface = agg.surface();
    let _agent = agg.agent();
    let _utility = agg.utility();
}

#[test]
fn container_orchestrator_is_send_sync() {
    let c = container();
    let orch = c.orchestrator();
    // Verify orchestrator is Send + Sync
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<
        std::sync::Arc<
            dyn shared::role_rules::contract_role_runner_aggregate::IRoleRunnerAggregate,
        >,
    >();
    let _ = orch;
}
