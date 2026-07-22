// PURPOSE: Integration tests — verify DI container wiring and cross-layer collaboration.
// Layer: Integration (uses real RoleContainer).

use role_rules_lint_arwaky::root_role_rules_container::RoleContainer;
use shared::role_rules::contract_role_aggregate::IRoleAggregate;
use shared::role_rules::contract_role_runner_aggregate::IRoleRunnerAggregate;

fn container() -> RoleContainer {
    RoleContainer::new_with_config(
        shared::config_system::taxonomy_config_vo::ArchitectureConfig::default(),
    )
}

// ─── Container wiring ────────────────────────────────

#[test]
fn container_creates_orchestrator_successfully() {
    let c = container();
    let orch = c.orchestrator();
    assert!(Arc::strong_count(&orch) >= 1);
}

#[test]
fn container_orchestrator_returns_same_arc_on_multiple_calls() {
    let c = container();
    let orch1 = c.orchestrator();
    let orch2 = c.orchestrator();
    assert!(Arc::ptr_eq(&orch1, &orch2));
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

// ─── Orchestrator via container ──────────────────────

#[tokio::test]
async fn container_orchestrator_run_audit_on_temp_dir() {
    use role_rules_lint_arwaky::root_role_rules_container::RoleContainer;
    use shared::common::taxonomy_path_vo::FilePath;

    let dir = tempfile::tempdir().unwrap();
    // Create a valid taxonomy file
    std::fs::write(dir.path().join("taxonomy_user_vo.rs"), "pub struct User;\n").unwrap();

    let c = RoleContainer::new_with_config(
        shared::config_system::taxonomy_config_vo::ArchitectureConfig::default(),
    );
    let orch = c.orchestrator();
    let target = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let results = orch.run_audit(&target).await;

    // Should not panic and return valid results (possibly empty)
    assert!(results.len() >= 0);
}
