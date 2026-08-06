// Acceptance test AES403 — Capability implementation.
// Capabilities must have >= 1 implementor and max 3 types per file.
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

fn run_audit(files: Vec<FileEntry>) -> Vec<shared::common::LintResult> {
    let config = ArchitectureConfig::default();
    let container = RoleContainer::new_with_config(config);
    let orch = container.orchestrator();
    orch.run_audit_with_entries(&files)
}

// ── No implementor → CapabilityNoImplementor ──

#[test]
fn aes403_no_implementor_detected() {
    let file = make_file(
        "src/capabilities_user_service.rs",
        Language::Rust,
        "pub struct UserService {}\n",
    );
    let results = run_audit(vec![file]);
    let aes403: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES403")
        .collect();
    assert!(
        !aes403.is_empty(),
        "capability without implementor should trigger AES403"
    );
}

// ── Too many types → CapabilityTooManyTypes ──

#[test]
fn aes403_too_many_types_detected() {
    let file = make_file(
        "src/capabilities_feature.rs",
        Language::Rust,
        "pub struct A {}\npub struct B {}\npub struct C {}\npub struct D {}\n",
    );
    let results = run_audit(vec![file]);
    let aes403: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES403")
        .collect();
    assert!(
        !aes403.is_empty(),
        "capability with 4 types should trigger AES403"
    );
    assert_eq!(aes403[0].severity, shared::common::Severity::HIGH);
}

// ── Valid capability with implementor → no violation ──

#[test]
fn aes403_valid_capability_no_violation() {
    let file = make_file(
        "src/capabilities_user_service.rs",
        Language::Rust,
        "pub struct UserService {}\nimpl IUserServiceProtocol for UserService {}\n",
    );
    let results = run_audit(vec![file]);
    let aes403: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES403")
        .collect();
    assert!(
        aes403.is_empty(),
        "valid capability with implementor should not trigger AES403"
    );
}

// ── Python: no parent class → CapabilityNoImplementor ──

#[test]
fn aes403_python_no_parent_detected() {
    let file = make_file(
        "src/capabilities_user_service.py",
        Language::Python,
        "class UserService:\n    pass\n",
    );
    let results = run_audit(vec![file]);
    let aes403: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES403")
        .collect();
    assert!(
        !aes403.is_empty(),
        "python capability without parent should trigger AES403"
    );
}

// ── Python: with parent → no violation ──

#[test]
fn aes403_python_with_parent_no_violation() {
    let file = make_file(
        "src/capabilities_user_service.py",
        Language::Python,
        "class UserService(IUserServiceProtocol):\n    pass\n",
    );
    let results = run_audit(vec![file]);
    let aes403: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES403")
        .collect();
    assert!(
        aes403.is_empty(),
        "python capability with parent should not trigger AES403"
    );
}

// ── Non-capabilities file is not checked ──

#[test]
fn aes403_non_capability_file_ignored() {
    let file = make_file(
        "src/agent_orchestrator.rs",
        Language::Rust,
        "pub struct Orchestrator {}\n",
    );
    let results = run_audit(vec![file]);
    let aes403: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES403")
        .collect();
    assert!(
        aes403.is_empty(),
        "non-capability file should not trigger AES403"
    );
}
