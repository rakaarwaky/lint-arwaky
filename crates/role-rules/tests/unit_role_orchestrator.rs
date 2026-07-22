// PURPOSE: Unit tests for RoleOrchestrator — dispatch logic and enabled gate.
// Layer: Agent (RoleOrchestrator)

use role_rules_lint_arwaky::agent_role_orchestrator::RoleOrchestrator;
use shared::common::taxonomy_path_vo::FilePath;
use shared::role_rules::contract_role_aggregate::IRoleAggregate;
use std::sync::Arc;

// ─── Mock Aggregate ──────────────────────────────────

struct StubAggregate;

impl IRoleAggregate for StubAggregate {
    fn taxonomy(
        &self,
    ) -> &dyn shared::role_rules::contract_taxonomy_role_protocol::ITaxonomyRoleChecker {
        panic!("not implemented")
    }
    fn contract(&self) -> &dyn shared::role_rules::contract_role_protocol::IContractRoleChecker {
        panic!("not implemented")
    }
    fn capabilities(
        &self,
    ) -> &dyn shared::role_rules::contract_capabilities_role_protocol::ICapabilitiesRoleChecker
    {
        panic!("not implemented")
    }
    fn surface(
        &self,
    ) -> &dyn shared::role_rules::contract_surface_role_protocol::ISurfaceRoleChecker {
        panic!("not implemented")
    }
    fn agent(&self) -> &dyn shared::role_rules::contract_agent_role_protocol::IAgentRoleChecker {
        panic!("not implemented")
    }
    fn utility(
        &self,
    ) -> &dyn shared::role_rules::contract_utility_role_protocol::IUtilityRoleChecker {
        panic!("not implemented")
    }
}

fn build_orchestrator() -> RoleOrchestrator {
    let config = shared::config_system::taxonomy_config_vo::ArchitectureConfig::default();
    RoleOrchestrator::new(Arc::new(StubAggregate), &config)
}

// ─── name — accessed via trait object ────────────────

#[test]
fn orchestrator_name_via_trait() {
    use shared::role_rules::contract_role_runner_aggregate::IRoleRunnerAggregate;
    let orch = build_orchestrator();
    let name: &dyn IRoleRunnerAggregate = &orch;
    // name() is accessible via the trait
    assert_eq!(name.name(), "role-rules");
}

// ─── Enabled gate: disabled config skips checks ──────

#[test]
fn disabled_config_skips_all_checks() {
    let mut config = shared::config_system::taxonomy_config_vo::ArchitectureConfig::default();
    config.enabled = shared::common::taxonomy_common_vo::BooleanVO::new(false);
    let orch = RoleOrchestrator::new(Arc::new(StubAggregate), &config);

    let files: Vec<String> = vec![
        "taxonomy_foo_vo.rs".to_string(),
        "contract_bar_protocol.rs".to_string(),
        "capabilities_baz_checker.rs".to_string(),
    ];
    let mut violations = Vec::new();
    orch.run_all_role_checks(&files, 500, &mut violations);
    assert!(violations.is_empty());
}

// ─── Default trait ──────────────────────────────────

#[test]
fn orchestrator_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<RoleOrchestrator>();
}
