// Contract tests — verify all capabilities implement their declared protocol traits.
// Each checker struct must be usable as a trait object for its corresponding protocol.

use role_rules_lint_arwaky::RoleClassifier;
use role_rules_lint_arwaky::agent_role_orchestrator::RoleCheckerDeps;
use role_rules_lint_arwaky::agent_role_orchestrator::RoleOrchestrator;
use role_rules_lint_arwaky::capabilities_agent_python_role_auditor::AgentPythonRoleAuditor;
use role_rules_lint_arwaky::capabilities_agent_rust_role_auditor::AgentRustRoleAuditor;
use role_rules_lint_arwaky::capabilities_agent_ts_role_auditor::AgentTsRoleAuditor;
use role_rules_lint_arwaky::capabilities_capabilities_python_role_auditor::CapabilitiesPythonRoleAuditor;
use role_rules_lint_arwaky::capabilities_capabilities_rust_role_auditor::CapabilitiesRustRoleAuditor;
use role_rules_lint_arwaky::capabilities_capabilities_ts_role_auditor::CapabilitiesTypeScriptRoleAuditor;
use role_rules_lint_arwaky::capabilities_contract_python_role_auditor::ContractPythonRoleAuditor;
use role_rules_lint_arwaky::capabilities_contract_rust_role_auditor::ContractRustRoleAuditor;
use role_rules_lint_arwaky::capabilities_contract_ts_role_auditor::ContractTypeScriptRoleAuditor;
use role_rules_lint_arwaky::capabilities_surface_role_auditor::SurfaceRoleChecker;
use role_rules_lint_arwaky::capabilities_taxonomy_role_auditor::TaxonomyRoleChecker;
use role_rules_lint_arwaky::capabilities_utility_python_role_auditor::UtilityPythonRoleAuditor;
use role_rules_lint_arwaky::capabilities_utility_rust_role_auditor::UtilityRustRoleAuditor;
use role_rules_lint_arwaky::capabilities_utility_ts_role_auditor::UtilityTypeScriptRoleAuditor;
use shared_common::LintResult;
use shared_filesystem::taxonomy_filesystem_vo::FileEntry;
use shared_role_rules::taxonomy_role_rules_request::RoleRequest;
use shared_role_rules::{
    IAgentRoleProtocol, ICapabilitiesRoleProtocol, IClassificationProtocol, IContractRoleProtocol,
    IRoleRunnerAggregate, ISurfaceRoleProtocol, ITaxonomyRoleProtocol, IUtilityRoleProtocol,
};
use std::sync::Arc;

fn dummy_file() -> FileEntry {
    FileEntry {
        path: std::path::PathBuf::from("src/test.rs"),
        extension: "rs".to_string(),
        language: shared_filesystem::taxonomy_filesystem_vo::Language::Rust,
        size: 10,
        content: "fn foo() {}".to_string(),
        parse_ok: true,
        parse_metadata: None,
    }
}

fn dummy_file_named(name: &str) -> FileEntry {
    FileEntry {
        path: std::path::PathBuf::from(format!("src/{name}")),
        ..dummy_file()
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

// ── Contract auditors → IContractRoleProtocol ──────────────

#[test]
fn contract_rust_role_auditor_implements_protocol() {
    let checker: Arc<dyn IContractRoleProtocol> = Arc::new(ContractRustRoleAuditor::new());
    let file = dummy_file();
    let _proto: Vec<LintResult> = checker.check_protocol(&file);
    let _agg: Vec<LintResult> = checker.check_aggregate(&file);
    let mut v: Vec<LintResult> = Vec::new();
    checker.check_contract_routing(&file, "contract", &mut v);
}

#[test]
fn contract_python_role_auditor_implements_protocol() {
    let checker: Arc<dyn IContractRoleProtocol> = Arc::new(ContractPythonRoleAuditor::new());
    let file = dummy_file();
    let _proto: Vec<LintResult> = checker.check_protocol(&file);
    let _agg: Vec<LintResult> = checker.check_aggregate(&file);
    let mut v: Vec<LintResult> = Vec::new();
    checker.check_contract_routing(&file, "contract", &mut v);
}

#[test]
fn contract_ts_role_auditor_implements_protocol() {
    let checker: Arc<dyn IContractRoleProtocol> = Arc::new(ContractTypeScriptRoleAuditor::new());
    let file = dummy_file();
    let _proto: Vec<LintResult> = checker.check_protocol(&file);
    let _agg: Vec<LintResult> = checker.check_aggregate(&file);
    let mut v: Vec<LintResult> = Vec::new();
    checker.check_contract_routing(&file, "contract", &mut v);
}

// ── Capabilities auditors → ICapabilitiesRoleProtocol ──────

#[test]
fn capabilities_rust_role_auditor_implements_protocol() {
    let checker: Arc<dyn ICapabilitiesRoleProtocol> = Arc::new(CapabilitiesRustRoleAuditor::new());
    let file = dummy_file();
    let mut v: Vec<LintResult> = Vec::new();
    checker.check_capability_routing(&file, "capabilities", &mut v);
}

#[test]
fn capabilities_python_role_auditor_implements_protocol() {
    let checker: Arc<dyn ICapabilitiesRoleProtocol> =
        Arc::new(CapabilitiesPythonRoleAuditor::new());
    let file = dummy_file();
    let mut v: Vec<LintResult> = Vec::new();
    checker.check_capability_routing(&file, "capabilities", &mut v);
}

#[test]
fn capabilities_typescript_role_auditor_implements_protocol() {
    let checker: Arc<dyn ICapabilitiesRoleProtocol> =
        Arc::new(CapabilitiesTypeScriptRoleAuditor::new());
    let file = dummy_file();
    let mut v: Vec<LintResult> = Vec::new();
    checker.check_capability_routing(&file, "capabilities", &mut v);
}

// ── Surface auditors → ISurfaceRoleProtocol ───────────────
//
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

// ── Agent auditors → IAgentRoleProtocol ─────────────────────

#[test]
fn agent_rust_auditor_implements_protocol() {
    let checker: Arc<dyn IAgentRoleProtocol> = Arc::new(AgentRustRoleAuditor::new());
    let file = dummy_file();
    let mut v: Vec<LintResult> = Vec::new();
    checker.check_agent_routing(&file, "agent", &mut v);
}

#[test]
fn agent_python_auditor_implements_protocol() {
    let checker: Arc<dyn IAgentRoleProtocol> = Arc::new(AgentPythonRoleAuditor::new());
    let file = dummy_file();
    let mut v: Vec<LintResult> = Vec::new();
    checker.check_agent_routing(&file, "agent", &mut v);
}

#[test]
fn agent_ts_auditor_implements_protocol() {
    let checker: Arc<dyn IAgentRoleProtocol> = Arc::new(AgentTsRoleAuditor::new());
    let file = dummy_file();
    let mut v: Vec<LintResult> = Vec::new();
    checker.check_agent_routing(&file, "agent", &mut v);
}

// ── Utility role auditors → IUtilityRoleProtocol ──────────

#[test]
fn utility_rust_role_auditor_implements_protocol() {
    let checker: Arc<dyn IUtilityRoleProtocol> = Arc::new(UtilityRustRoleAuditor::new());
    let file = dummy_file();
    let mut v: Vec<LintResult> = Vec::new();
    checker.check_utility_convention(&file, &mut v);
}

#[test]
fn utility_python_role_auditor_implements_protocol() {
    let checker: Arc<dyn IUtilityRoleProtocol> = Arc::new(UtilityPythonRoleAuditor::new());
    let file = dummy_file();
    let mut v: Vec<LintResult> = Vec::new();
    checker.check_utility_convention(&file, &mut v);
}

#[test]
fn utility_typescript_role_auditor_implements_protocol() {
    let checker: Arc<dyn IUtilityRoleProtocol> = Arc::new(UtilityTypeScriptRoleAuditor::new());
    let file = dummy_file();
    let mut v: Vec<LintResult> = Vec::new();
    checker.check_utility_convention(&file, &mut v);
}

// ── RoleOrchestrator → IRoleRunnerAggregate ────────────────

#[test]
fn role_classifier_capability_classifies_file_prefix() {
    // AES405: the orchestrator must not implement IClassificationProtocol itself.
    // The trait is fulfilled by the RoleClassifier capability, which the
    // orchestrator holds as a dependency and delegates to.
    let classifier = RoleClassifier::new();
    let file = dummy_file();
    assert!(classifier.classify_layer(&file).is_none());

    let rust_file = dummy_file_named("capabilities_rust_role_auditor.rs");
    assert_eq!(
        classifier
            .classify_layer(&rust_file)
            .map(|l| l.value.to_string()),
        Some("capabilities".to_string())
    );
}

#[test]
fn role_classifier_maps_every_layer_prefix() {
    let c = RoleClassifier::new();
    let layer = |name: &str| {
        c.classify_layer(&dummy_file_named(name))
            .map(|l| l.value.to_string())
    };
    assert_eq!(layer("taxonomy_x_vo.rs").as_deref(), Some("taxonomy"));
    assert_eq!(layer("contract_x_protocol.rs").as_deref(), Some("contract"));
    assert_eq!(layer("capabilities_x.rs").as_deref(), Some("capabilities"));
    assert_eq!(layer("capability_x.rs").as_deref(), Some("capabilities"));
    assert_eq!(layer("utility_x.rs").as_deref(), Some("utility"));
    assert_eq!(layer("agent_x.rs").as_deref(), Some("agent"));
    assert_eq!(layer("surface_x.rs").as_deref(), Some("surfaces"));
    assert_eq!(layer("surfaces_x.rs").as_deref(), Some("surfaces"));
}

#[test]
fn role_classifier_skips_root_and_unknown_prefixes() {
    let c = RoleClassifier::new();
    let layer = |name: &str| {
        c.classify_layer(&dummy_file_named(name))
            .map(|l| l.value.to_string())
    };
    // `root` is pure DI wiring; unrecognised prefixes are skipped too.
    assert_eq!(layer("root_x_container.rs"), None);
    assert_eq!(layer("shared_x_thing.rs"), None);
}

#[test]
fn role_classifier_skips_bare_stem_without_underscore() {
    // FR-RoleRules-001: the prefix is the first `_`-separated segment, so a
    // filename with no underscore has no prefix match. `split('_').next()`
    // alone would return the whole stem and classify `agent.rs` as an agent.
    let c = RoleClassifier::new();
    for name in ["agent.rs", "taxonomy.py", "utility.rs", "contract.ts"] {
        assert_eq!(
            c.classify_layer(&dummy_file_named(name))
                .map(|l| l.value.to_string()),
            None,
            "`{name}` has no `_`-separated prefix and must be skipped"
        );
    }
}

#[test]
fn role_orchestrator_implements_aggregate() {
    let config = shared_config_system::taxonomy_config_system_vo::ArchitectureConfig::default();
    let rust_auditor = Arc::new(CapabilitiesRustRoleAuditor::new());
    let deps = RoleCheckerDeps {
        classifier: Arc::new(RoleClassifier::new()),
        taxonomy: Arc::new(TaxonomyRoleChecker::new()),
        contract_rust: Arc::new(ContractRustRoleAuditor::new()),
        contract_python: Arc::new(ContractPythonRoleAuditor::new()),
        contract_typescript: Arc::new(ContractTypeScriptRoleAuditor::new()),
        capabilities_rust: rust_auditor.clone(),
        capabilities_python: Arc::new(CapabilitiesPythonRoleAuditor::new()),
        capabilities_typescript: Arc::new(CapabilitiesTypeScriptRoleAuditor::new()),
        capabilities: rust_auditor,
        surface: Arc::new(SurfaceRoleChecker::new()),
        agent_rust: Arc::new(AgentRustRoleAuditor::new()),
        agent_python: Arc::new(AgentPythonRoleAuditor::new()),
        agent_ts: Arc::new(AgentTsRoleAuditor::new()),
        utility_rust: Arc::new(UtilityRustRoleAuditor::new()),
        utility_python: Arc::new(UtilityPythonRoleAuditor::new()),
        utility_typescript: Arc::new(UtilityTypeScriptRoleAuditor::new()),
    };
    let orchestrator: Arc<dyn IRoleRunnerAggregate> =
        Arc::new(RoleOrchestrator::new(deps, &config));
    let results = orchestrator
        .execute(RoleRequest::audit(&[]))
        .into_violations();
    assert!(results.is_empty());
    assert_eq!(
        orchestrator.execute(RoleRequest::Name).into_name(),
        "role-rules"
    );
}
