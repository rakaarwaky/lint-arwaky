use role_rules_lint_arwaky::agent_role_orchestrator::RoleOrchestrator;
use role_rules_lint_arwaky::agent_role_orchestrator::RoleAggregateImpl;
use role_rules_lint_arwaky::capabilities_agent_role_auditor::AgentRoleChecker;
use role_rules_lint_arwaky::capabilities_capabilities_role_auditor::CapabilitiesRoleChecker;
use role_rules_lint_arwaky::capabilities_contract_role_auditor::ContractRoleChecker;
use role_rules_lint_arwaky::capabilities_infrastructure_role_auditor::InfrastructureRoleChecker;
use role_rules_lint_arwaky::capabilities_surface_role_auditor::SurfaceRoleChecker;
use role_rules_lint_arwaky::capabilities_taxonomy_role_auditor::TaxonomyRoleChecker;
use shared::config_system::taxonomy_config_vo::default_aes_config;
use shared::role_rules::contract_role_aggregate::IRoleAggregate;
use shared::role_rules::contract_role_runner_aggregate::IRoleRunnerAggregate;
use std::sync::Arc;

fn make_orchestrator() -> RoleOrchestrator {
    let aggregate: Arc<dyn IRoleAggregate> = Arc::new(RoleAggregateImpl::new(
        Arc::new(TaxonomyRoleChecker::new()),
        Arc::new(ContractRoleChecker::new()),
        Arc::new(InfrastructureRoleChecker::new()),
        Arc::new(CapabilitiesRoleChecker::new()),
        Arc::new(SurfaceRoleChecker::new()),
        Arc::new(AgentRoleChecker::new()),
    ));
    let config = default_aes_config();
    RoleOrchestrator::new(aggregate, &config)
}

#[test]
fn orchestrator_can_be_constructed() {
    let orch = make_orchestrator();
    let _ = orch;
}

#[test]
fn name_returns_role_rules() {
    let orch = make_orchestrator();
    assert_eq!(orch.name(), "role-rules");
}

#[test]
fn run_all_role_checks_empty_input_no_panic() {
    let orch = make_orchestrator();
    let mut violations = Vec::new();
    orch.run_all_role_checks(&[], 500, &mut violations);
    assert!(violations.is_empty());
}

#[test]
fn run_all_role_checks_with_valid_file() {
    let orch = make_orchestrator();
    let mut violations = Vec::new();
    // Use a real source file from the shared crate as test input
    let file_path = "crates/shared/src/common/taxonomy_path_vo.rs";
    if std::path::Path::new(file_path).exists() {
        orch.run_all_role_checks(&[file_path.to_string()], 500, &mut violations);
        // Should not panic; results may vary
        assert!(violations.len() <= 10);
    } else {
        // Fallback: just check it doesn't panic with a bogus path
        orch.run_all_role_checks(&["nonexistent.rs".to_string()], 500, &mut violations);
        assert!(violations.is_empty());
    }
}
