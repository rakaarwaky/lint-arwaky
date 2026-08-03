// Contract tests — verify all capabilities implement their declared protocol traits.
// Each checker struct must be usable as a trait object for its corresponding protocol.

use role_rules_lint_arwaky::capabilities_agent_role_auditor::AgentRoleChecker;
use role_rules_lint_arwaky::capabilities_capabilities_role_auditor::CapabilitiesRoleChecker;
use role_rules_lint_arwaky::capabilities_contract_role_auditor::ContractRoleChecker;
use role_rules_lint_arwaky::capabilities_surface_role_auditor::SurfaceRoleChecker;
use role_rules_lint_arwaky::capabilities_taxonomy_role_auditor::TaxonomyRoleChecker;
use role_rules_lint_arwaky::capabilities_utility_role_auditor::UtilityRoleChecker;
use role_rules_lint_arwaky::agent_role_orchestrator::RoleCheckerDeps;
use role_rules_lint_arwaky::agent_role_orchestrator::RoleOrchestrator;
use shared::common::LintResult;
use shared::filesystem::taxonomy_filesystem_vo::FileEntry;
use shared::role_rules::{
    IAgentRoleChecker, ICapabilitiesRoleChecker, IContractRoleChecker, IRoleRunnerAggregate,
    ISurfaceRoleChecker, ITaxonomyRoleChecker, IUtilityRoleChecker,
};
use std::sync::Arc;

fn dummy_file() -> FileEntry {
    FileEntry {
        path: std::path::PathBuf::from("src/test.rs"),
        extension: "rs".to_string(),
        language: shared::filesystem::taxonomy_filesystem_vo::Language::Rust,
        size: 10,
        content: "fn foo() {}".to_string(),
        parse_ok: true,
        parse_metadata: None,
    }
}

// ── TaxonomyRoleChecker → ITaxonomyRoleChecker ─────────────

#[test]
fn taxonomy_role_checker_implements_protocol() {
    let checker: Arc<dyn ITaxonomyRoleChecker> = Arc::new(TaxonomyRoleChecker::new());
    let file = dummy_file();
    let mut v: Vec<LintResult> = Vec::new();
    checker.check_entity(&file, &mut v);
    checker.check_error(&file, &mut v);
    checker.check_event(&file, &mut v);
    checker.check_constant(&file, &mut v);
}

// ── ContractRoleChecker → IContractRoleChecker ─────────────

#[test]
fn contract_role_checker_implements_protocol() {
    let checker: Arc<dyn IContractRoleChecker> = Arc::new(ContractRoleChecker::new());
    let file = dummy_file();
    let _proto: Vec<LintResult> = checker.check_protocol(&file);
    let _agg: Vec<LintResult> = checker.check_aggregate(&file);
}

// ── CapabilitiesRoleChecker → ICapabilitiesRoleChecker ─────

#[test]
fn capabilities_role_checker_implements_protocol() {
    let checker: Arc<dyn ICapabilitiesRoleChecker> = Arc::new(CapabilitiesRoleChecker::new());
    let file = dummy_file();
    let mut v: Vec<LintResult> = Vec::new();
    checker.check_capability_routing(&file, "capabilities", &mut v);
}

// ── SurfaceRoleChecker → ISurfaceRoleChecker ───────────────

#[test]
fn surface_role_checker_implements_protocol() {
    let checker: Arc<dyn ISurfaceRoleChecker> = Arc::new(SurfaceRoleChecker::new());
    let file = dummy_file();
    let mut v: Vec<LintResult> = Vec::new();
    checker.check_smart_surface(&file, &mut v);
    checker.check_utility_surface(&file, &mut v);
    checker.check_passive_surface(&file, &mut v);
    checker.check_fn_count_limit(&file, &mut v);
}

// ── AgentRoleChecker → IAgentRoleChecker ───────────────────

#[test]
fn agent_role_checker_implements_protocol() {
    let checker: Arc<dyn IAgentRoleChecker> = Arc::new(AgentRoleChecker::new());
    let file = dummy_file();
    let mut v: Vec<LintResult> = Vec::new();
    checker.check_agent_routing(&file, "agent", &mut v);
}

// ── UtilityRoleChecker → IUtilityRoleChecker ───────────────

#[test]
fn utility_role_checker_implements_protocol() {
    let checker: Arc<dyn IUtilityRoleChecker> = Arc::new(UtilityRoleChecker::new());
    let file = dummy_file();
    let mut v: Vec<LintResult> = Vec::new();
    checker.check_utility_convention(&file, &mut v);
}

// ── RoleOrchestrator → IRoleRunnerAggregate ────────────────

#[test]
fn role_orchestrator_implements_aggregate() {
    let config = shared::config_system::taxonomy_config_vo::ArchitectureConfig::default();
    let deps = RoleCheckerDeps {
        taxonomy: Arc::new(TaxonomyRoleChecker::new()),
        contract: Arc::new(ContractRoleChecker::new()),
        capabilities: Arc::new(CapabilitiesRoleChecker::new()),
        surface: Arc::new(SurfaceRoleChecker::new()),
        agent: Arc::new(AgentRoleChecker::new()),
        utility: Arc::new(UtilityRoleChecker::new()),
    };
    let orchestrator: Arc<dyn IRoleRunnerAggregate> =
        Arc::new(RoleOrchestrator::new(deps, &config));
    let results = orchestrator.run_audit_with_entries(&[]);
    assert!(results.is_empty());
    assert_eq!(orchestrator.name(), "role-rules");
}
