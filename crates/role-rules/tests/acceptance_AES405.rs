// Acceptance test AES405 — Agent composition.
// Agent files must have >= 1 aggregate implementor and max 3 types.
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

// ── No implementor → AgentNoImplementor ──

#[test]
fn aes405_no_implementor_detected() {
    let file = make_file(
        "src/agent_dispatcher.rs",
        Language::Rust,
        "pub struct Dispatcher {}\n",
    );
    let results = run_audit(vec![file]);
    let aes405: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES405")
        .collect();
    assert!(
        !aes405.is_empty(),
        "agent without implementor should trigger AES405"
    );
}

// ── Too many types → AgentTooManyTypes ──

#[test]
fn aes405_too_many_types_detected() {
    let file = make_file(
        "src/agent_orchestrator.rs",
        Language::Rust,
        "pub struct A {}\npub struct B {}\npub struct C {}\npub struct D {}\n",
    );
    let results = run_audit(vec![file]);
    let aes405: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES405")
        .collect();
    assert!(
        !aes405.is_empty(),
        "agent with 4 types should trigger AES405"
    );
    assert_eq!(aes405[0].severity, shared::common::Severity::HIGH);
}

// ── Valid agent with implementor → no violation ──

#[test]
fn aes405_valid_agent_no_violation() {
    let file = make_file(
        "src/agent_dispatcher.rs",
        Language::Rust,
        "pub struct Dispatcher {}\nimpl IDispatcher for Dispatcher {}\n",
    );
    let results = run_audit(vec![file]);
    let aes405: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES405")
        .collect();
    assert!(
        aes405.is_empty(),
        "valid agent with implementor should not trigger AES405"
    );
}

// ── Python: no parent class → AgentNoImplementor ──

#[test]
fn aes405_python_no_parent_detected() {
    let file = make_file(
        "src/agent_dispatcher.py",
        Language::Python,
        "class Dispatcher:\n    pass\n",
    );
    let results = run_audit(vec![file]);
    let aes405: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES405")
        .collect();
    assert!(
        !aes405.is_empty(),
        "python agent without parent should trigger AES405"
    );
}

// ── Python: with parent → no violation ──

#[test]
fn aes405_python_with_parent_no_violation() {
    let file = make_file(
        "src/agent_dispatcher.py",
        Language::Python,
        "class Dispatcher(IDispatcher):\n    pass\n",
    );
    let results = run_audit(vec![file]);
    let aes405: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES405")
        .collect();
    assert!(
        aes405.is_empty(),
        "python agent with parent should not trigger AES405"
    );
}

// ── Non-agent file is not checked ──

#[test]
fn aes405_non_agent_file_ignored() {
    let file = make_file(
        "src/capabilities_feature.rs",
        Language::Rust,
        "pub struct Feature {}\n",
    );
    let results = run_audit(vec![file]);
    let aes405: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES405")
        .collect();
    assert!(
        aes405.is_empty(),
        "non-agent file should not trigger AES405"
    );
}
