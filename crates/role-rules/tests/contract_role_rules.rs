// Contract tests — verify all capabilities implement their declared protocol traits.
// Each checker struct must be usable as a trait object for its corresponding protocol.

use role_rules_lint_arwaky::agent_role_orchestrator::RoleCheckerDeps;
use role_rules_lint_arwaky::agent_role_orchestrator::RoleOrchestrator;
use role_rules_lint_arwaky::capabilities_agent_role_auditor::AgentRoleChecker;
use role_rules_lint_arwaky::capabilities_capabilities_role_auditor::CapabilitiesRoleChecker;
use role_rules_lint_arwaky::capabilities_contract_role_auditor::ContractRoleChecker;
use role_rules_lint_arwaky::capabilities_surface_role_auditor::SurfaceRoleChecker;
use role_rules_lint_arwaky::capabilities_taxonomy_role_auditor::TaxonomyRoleChecker;
use role_rules_lint_arwaky::capabilities_utility_role_auditor::UtilityRoleChecker;
use shared::common::LintResult;
use shared::filesystem::taxonomy_filesystem_vo::FileEntry;
use shared::role_rules::{
    IAgentRoleProtocol, ICapabilitiesRoleProtocol, IContractRoleProtocol, IRoleRunnerAggregate,
    ISurfaceRoleProtocol, ITaxonomyRoleProtocol, IUtilityRoleProtocol,
};
use std::sync::Arc;
use shared::role_rules::taxonomy_role_request_vo::{RoleRequest, RoleResponse};

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

// ── TaxonomyRoleChecker → ITaxonomyRoleProtocol ─────────────

#[test]
fn taxonomy_role_checker_implements_protocol() {
    let checker: Arc<dyn ITaxonomyRoleProtocol> = Arc::new(TaxonomyRoleChecker::new());
    let file = dummy_file();
    let mut v: Vec<LintResult> = Vec::new();
    checker.check_entity(&file, &mut v);
    checker.check_error(&file, &mut v);
    checker.check_event(&file, &mut v);
    checker.check_constant(&file, &mut v);
}

// ── ContractRoleChecker → IContractRoleProtocol ─────────────

#[test]
fn contract_role_checker_implements_protocol() {
    let checker: Arc<dyn IContractRoleProtocol> = Arc::new(ContractRoleChecker::new());
    let file = dummy_file();
    let _proto: Vec<LintResult> = checker.check_protocol(&file);
    let _agg: Vec<LintResult> = checker.check_aggregate(&file);
}

// ── CapabilitiesRoleChecker → ICapabilitiesRoleProtocol ─────

#[test]
fn capabilities_role_checker_implements_protocol() {
    let checker: Arc<dyn ICapabilitiesRoleProtocol> = Arc::new(CapabilitiesRoleChecker::new());
    let file = dummy_file();
    let mut v: Vec<LintResult> = Vec::new();
    checker.check_capability_routing(&file, "capabilities", &mut v);
}

// ── SurfaceRoleChecker → ISurfaceRoleProtocol ───────────────

#[test]
fn surface_role_checker_implements_protocol() {
    let checker: Arc<dyn ISurfaceRoleProtocol> = Arc::new(SurfaceRoleChecker::new());
    let file = dummy_file();
    let mut v: Vec<LintResult> = Vec::new();
    checker.check_smart_surface(&file, &mut v);
    checker.check_utility_surface(&file, &mut v);
    checker.check_passive_surface(&file, &mut v);
    checker.check_fn_count_limit(&file, &mut v);
}

// ── AgentRoleChecker → IAgentRoleProtocol ───────────────────

#[test]
fn agent_role_checker_implements_protocol() {
    let checker: Arc<dyn IAgentRoleProtocol> = Arc::new(AgentRoleChecker::new());
    let file = dummy_file();
    let mut v: Vec<LintResult> = Vec::new();
    checker.check_agent_routing(&file, "agent", &mut v);
}

// ── UtilityRoleChecker → IUtilityRoleProtocol ───────────────

#[test]
fn utility_role_checker_implements_protocol() {
    let checker: Arc<dyn IUtilityRoleProtocol> = Arc::new(UtilityRoleChecker::new());
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
    let results = orchestrator.execute(RoleRequest::audit(&[])).into_violations();
    assert!(results.is_empty());
    assert_eq!(orchestrator.execute(RoleRequest::Name).into_name(), "role-rules");
}
