// Smoke tests — quick boot and basic audit within time budget.
use role_rules_lint_arwaky::root_role_rules_container::RoleContainer;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::filesystem::taxonomy_filesystem_vo::{FileEntry, Language};
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
    assert_eq!(orch.name(), "role-rules");
}

#[test]
fn smoke_orchestrator_creation() {
    use role_rules_lint_arwaky::agent_role_orchestrator::{RoleCheckerDeps, RoleOrchestrator};
    use std::sync::Arc;

    let config = ArchitectureConfig::default();
    let deps = RoleCheckerDeps {
        taxonomy: Arc::new(role_rules_lint_arwaky::TaxonomyRoleChecker::new()),
        contract: Arc::new(role_rules_lint_arwaky::ContractRoleChecker::new()),
        capabilities: Arc::new(role_rules_lint_arwaky::CapabilitiesRoleChecker::new()),
        surface: Arc::new(role_rules_lint_arwaky::SurfaceRoleChecker::new()),
        agent: Arc::new(role_rules_lint_arwaky::AgentRoleChecker::new()),
        utility: Arc::new(role_rules_lint_arwaky::UtilityRoleChecker::new()),
    };
    let orch = RoleOrchestrator::new(deps, &config);
    assert_eq!(orch.name(), "role-rules");
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
    let results = orch.run_audit_with_entries(&[file]);
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
    let results = orch.run_audit_with_entries(&[file]);
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
        make_file("src/capabilities_two.rs", Language::Rust, "pub struct B {}\n"),
        make_file("src/utility_three.rs", Language::Rust, "pub struct C {}\n"),
    ];
    let results = orch.run_audit_with_entries(&files);
    // At least some of these should produce violations
    assert!(
        !results.is_empty(),
        "multiple non-conforming files should produce violations"
    );
}
