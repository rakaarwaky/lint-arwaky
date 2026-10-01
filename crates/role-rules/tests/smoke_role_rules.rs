// Smoke tests — quick boot and basic audit within time budget.
use role_rules_lint_arwaky::root_role_rules_container::RoleContainer;
use shared_config_system::taxonomy_config_system_vo::ArchitectureConfig;
use shared_filesystem::taxonomy_filesystem_vo::{FileEntry, Language};
use shared_role_rules::IRoleRunnerAggregate;
use shared_role_rules::taxonomy_role_rules_request::RoleRequest;
use std::path::PathBuf;

fn make_file(path: &str, lang: Language, content: &str) -> FileEntry {
    FileEntry {
        path: PathBuf::from(path),
        extension: match lang {
            Language::Rust => "rs",
            Language::Python => "py",
            Language::TypeScript | Language::JavaScript => "ts",
            _ => "txt",
        }
        .to_string(),
        language: lang,
        size: content.len() as u64,
        content: content.to_string(),
        parse_ok: true,
        parse_metadata: None,
    }
}

#[test]
fn smoke_container_creation() {
    let config = ArchitectureConfig::default();
    let container = RoleContainer::new_with_config(config);
    let orch = container.orchestrator();
    assert_eq!(orch.execute(RoleRequest::Name).into_name(), "role-rules");
}

#[test]
fn smoke_orchestrator_creation() {
    use role_rules_lint_arwaky::agent_role_orchestrator::{RoleCheckerDeps, RoleOrchestrator};
    use std::sync::Arc;

    let config = ArchitectureConfig::default();
    let rust_auditor = Arc::new(role_rules_lint_arwaky::CapabilitiesRustRoleAuditor::new());
    let deps = RoleCheckerDeps {
        classifier: Arc::new(role_rules_lint_arwaky::RoleClassifier::new()),
        taxonomy: Arc::new(role_rules_lint_arwaky::TaxonomyRoleChecker::new()),
        contract_rust: Arc::new(role_rules_lint_arwaky::ContractRustRoleAuditor::new()),
        contract_python: Arc::new(role_rules_lint_arwaky::ContractPythonRoleAuditor::new()),
        contract_typescript: Arc::new(role_rules_lint_arwaky::ContractTypeScriptRoleAuditor::new()),
        capabilities_rust: rust_auditor.clone(),
        capabilities_python: Arc::new(role_rules_lint_arwaky::CapabilitiesPythonRoleAuditor::new()),
        capabilities_typescript: Arc::new(
            role_rules_lint_arwaky::CapabilitiesTypeScriptRoleAuditor::new(),
        ),
        capabilities: rust_auditor,
        surface: Arc::new(role_rules_lint_arwaky::SurfaceRoleChecker::new()),
        agent_rust: Arc::new(role_rules_lint_arwaky::AgentRustRoleAuditor::new()),
        agent_python: Arc::new(role_rules_lint_arwaky::AgentPythonRoleAuditor::new()),
        agent_ts: Arc::new(role_rules_lint_arwaky::AgentTsRoleAuditor::new()),
        utility_rust: Arc::new(role_rules_lint_arwaky::UtilityRustRoleAuditor::new()),
        utility_python: Arc::new(role_rules_lint_arwaky::UtilityPythonRoleAuditor::new()),
        utility_typescript: Arc::new(role_rules_lint_arwaky::UtilityTypeScriptRoleAuditor::new()),
    };
    let orch = RoleOrchestrator::new(deps, &config);
    assert_eq!(orch.execute(RoleRequest::Name).into_name(), "role-rules");
}

#[test]
fn smoke_basic_audit_clean_file() {
    let config = ArchitectureConfig::default();
    let container = RoleContainer::new_with_config(config);
    let orch = container.orchestrator();

    let file = make_file(
        "src/some_module.rs",
        Language::Rust,
        "pub fn hello() -> String {\n    \"hi\".to_string()\n}\n",
    );
    let results = orch.execute(RoleRequest::audit(&[file])).into_violations();
    // A generic module file (no role prefix) should produce no violations
    assert!(results.is_empty());
}

#[test]
fn smoke_basic_audit_detects_violation() {
    let config = ArchitectureConfig::default();
    let container = RoleContainer::new_with_config(config);
    let orch = container.orchestrator();

    // Agent file with no implementor → AES405
    let file = make_file(
        "src/agent_bare.rs",
        Language::Rust,
        "pub struct BareAgent {}\n",
    );
    let results = orch.execute(RoleRequest::audit(&[file])).into_violations();
    assert!(
        !results.is_empty(),
        "bare agent file should produce at least one violation"
    );
    assert_eq!(results[0].code.code(), "AES405");
}

#[test]
fn smoke_multiple_files() {
    let config = ArchitectureConfig::default();
    let container = RoleContainer::new_with_config(config);
    let orch = container.orchestrator();

    let files = vec![
        make_file("src/agent_one.rs", Language::Rust, "pub struct A {}\n"),
        make_file(
            "src/capabilities_two.rs",
            Language::Rust,
            "pub struct B {}\n",
        ),
        make_file("src/utility_three.rs", Language::Rust, "pub struct C {}\n"),
    ];
    let results = orch.execute(RoleRequest::audit(&files)).into_violations();
    // At least some of these should produce violations
    assert!(
        !results.is_empty(),
        "multiple non-conforming files should produce violations"
    );
}
