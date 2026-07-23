// PURPOSE: Integration tests — verify DI container wiring and cross-layer collaboration.
// Layer: Integration (uses real RoleContainer).

use role_rules_lint_arwaky::root_role_rules_container::RoleContainer;

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
fn container_orchestrator_is_usable() {
    let c = container();
    let orch = c.orchestrator();
    // Verify orchestrator is usable — name() returns the feature identifier
    assert_eq!(orch.name(), "role-rules");
}

#[test]
fn container_orchestrator_is_send_sync() {
    let c = container();
    let orch = c.orchestrator();
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<
        std::sync::Arc<
            dyn shared::role_rules::contract_role_runner_aggregate::IRoleRunnerAggregate,
        >,
    >();
    let _ = orch;
}
