// PURPOSE: Unit tests for RoleOrchestrator — dispatch logic and enabled gate.
// Layer: Agent (RoleOrchestrator)

use role_rules_lint_arwaky::agent_role_orchestrator::{RoleCheckerDeps, RoleOrchestrator};
use role_rules_lint_arwaky::capabilities_agent_role_auditor::AgentRoleChecker;
use role_rules_lint_arwaky::capabilities_capabilities_role_auditor::CapabilitiesRoleChecker;
use role_rules_lint_arwaky::capabilities_contract_role_auditor::ContractRoleChecker;
use role_rules_lint_arwaky::capabilities_surface_role_auditor::SurfaceRoleChecker;
use role_rules_lint_arwaky::capabilities_taxonomy_role_auditor::TaxonomyRoleChecker;
use role_rules_lint_arwaky::capabilities_utility_role_auditor::UtilityRoleChecker;
use std::sync::Arc;

// ─── Helper: build a minimal orchestrator with real checkers ─

fn build_orchestrator() -> RoleOrchestrator {
    let config = shared::config_system::taxonomy_config_vo::ArchitectureConfig::default();
    let deps = RoleCheckerDeps {
        taxonomy: Arc::new(TaxonomyRoleChecker::new()),
        contract: Arc::new(ContractRoleChecker::new()),
        capabilities: Arc::new(CapabilitiesRoleChecker::new()),
        surface: Arc::new(SurfaceRoleChecker::new()),
        agent: Arc::new(AgentRoleChecker::new()),
        utility: Arc::new(UtilityRoleChecker::new()),
    };
    RoleOrchestrator::new(deps, &config)
}

// ─── name — accessed via trait object ────────────────

#[test]
fn orchestrator_name_via_trait() {
    use shared::role_rules::contract_role_runner_aggregate::IRoleRunnerAggregate;
    let orch = build_orchestrator();
    let name: &dyn IRoleRunnerAggregate = &orch;
    assert_eq!(name.name(), "role-rules");
}

// ─── Enabled gate: disabled config skips checks ──────

#[test]
fn disabled_config_skips_all_checks() {
    let config = shared::config_system::taxonomy_config_vo::ArchitectureConfig {
        enabled: shared::common::taxonomy_common_vo::BooleanVO::new(false),
        ..Default::default()
    };
    let deps = RoleCheckerDeps {
        taxonomy: Arc::new(TaxonomyRoleChecker::new()),
        contract: Arc::new(ContractRoleChecker::new()),
        capabilities: Arc::new(CapabilitiesRoleChecker::new()),
        surface: Arc::new(SurfaceRoleChecker::new()),
        agent: Arc::new(AgentRoleChecker::new()),
        utility: Arc::new(UtilityRoleChecker::new()),
    };
    let orch = RoleOrchestrator::new(deps, &config);

    let files: Vec<String> = vec![
        "taxonomy_foo_vo.rs".to_string(),
        "contract_bar_protocol.rs".to_string(),
        "capabilities_baz_checker.rs".to_string(),
    ];
    let mut violations = Vec::new();
    orch.run_all_role_checks(&files, &mut violations);
    assert!(violations.is_empty());
}

// ─── Send + Sync ──────────────────────────────────

#[test]
fn orchestrator_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<RoleOrchestrator>();
}
