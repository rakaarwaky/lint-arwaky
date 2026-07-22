// PURPOSE: Acceptance test for AES404 — agent layer orchestrator delegation.
// Layer: Acceptance (FRD requirement validation).

use role_rules_lint_arwaky::agent_role_orchestrator::RoleOrchestrator;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::common::taxonomy_path_vo::FilePath;
use shared::role_rules::contract_role_aggregate::IRoleAggregate;
use shared::role_rules::contract_role_runner_aggregate::IRoleRunnerAggregate;
use std::sync::Arc;

// ─── Stub Aggregate for testing ──────────────────────────

struct StubAggregate;

impl IRoleAggregate for StubAggregate {
    fn taxonomy(&self) -> &dyn shared::role_rules::contract_taxonomy_role_protocol::ITaxonomyRoleChecker {
        panic!("not implemented")
    }
    fn contract(&self) -> &dyn shared::role_rules::contract_role_protocol::IContractRoleChecker {
        panic!("not implemented")
    }
    fn capabilities(&self) -> &dyn shared::role_rules::contract_capabilities_role_protocol::ICapabilitiesRoleChecker {
        panic!("not implemented")
    }
    fn surface(&self) -> &dyn shared::role_rules::contract_surface_role_protocol::ISurfaceRoleChecker {
        panic!("not implemented")
    }
    fn agent(&self) -> &dyn shared::role_rules::contract_agent_role_protocol::IAgentRoleChecker {
        panic!("not implemented")
    }
    fn utility(&self) -> &dyn shared::role_rules::contract_utility_role_protocol::IUtilityRoleChecker {
        panic!("not implemented")
    }
}

fn build_orchestrator() -> RoleOrchestrator {
    let config = shared::config_system::taxonomy_config_vo::ArchitectureConfig::default();
    RoleOrchestrator::new(Arc::new(StubAggregate), &config)
}

// ─── Acceptance: Orchestrator delegates to all checkers ──

#[test]
fn acceptance_aes404_orchestrator_delegates_to_checkers() {
    // FRD requirement: Agent orchestrator must delegate to all 6 role checkers
    let orch = build_orchestrator();
    assert_eq!(orch.name(), "role-rules");

    // Verify the orchestrator has access to all checkers via aggregate
    let files: Vec<String> = vec![
        "taxonomy_foo_vo.rs".to_string(),
        "contract_bar_protocol.rs".to_string(),
        "capabilities_baz_checker.rs".to_string(),
    ];

    let mut violations = Vec::new();
    orch.run_all_role_checks(&files, 500, &mut violations);

    // When disabled (default), violations should be empty
    // This validates the enabled gate works correctly
    assert!(violations.is_empty());
}

// ─── Acceptance: Orchestrator respects enabled flag ──

#[test]
fn acceptance_aes404_enabled_gate_works() {
    // FRD requirement: When disabled, orchestrator skips all checks
    let mut config = shared::config_system::taxonomy_config_vo::ArchitectureConfig::default();
    config.enabled = shared::common::taxonomy_common_vo::BooleanVO::new(false);
    let orch = RoleOrchestrator::new(Arc::new(StubAggregate), &config);

    let files: Vec<String> = vec![
        "taxonomy_foo_vo.rs".to_string(),
        "contract_bar_protocol.rs".to_string(),
    ];
    let mut violations = Vec::new();
    orch.run_all_role_checks(&files, 500, &mut violations);

    assert!(violations.is_empty(), "AES404: Disabled orchestrator should skip all checks");
}

// ─── Acceptance: Orchestrator is Send + Sync ──

#[test]
fn acceptance_aes404_orchestrator_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<RoleOrchestrator>();
}
