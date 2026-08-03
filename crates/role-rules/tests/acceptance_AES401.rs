// Acceptance test AES401 — Taxonomy purity.
// Taxonomy files must not contain business logic or non-constant declarations.
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

// ── Constant file with struct → ConstantPurity violation ──

#[test]
fn aes401_constant_file_with_struct_detected() {
    // Basename must end with "_constant.rs" to trigger check_constant
    let file = make_file(
        "src/taxonomy_app_constant.rs",
        Language::Rust,
        "pub struct AppConfig {\n    pub name: String,\n}\n",
    );
    let results = run_audit(vec![file]);
    let aes401: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES401")
        .collect();
    assert!(
        !aes401.is_empty(),
        "taxonomy constant file with struct should trigger AES401"
    );
}

#[test]
fn aes401_clean_constant_file_no_violation() {
    // Basename must end with "_constant.rs" to trigger check_constant
    let file = make_file(
        "src/taxonomy_app_constant.rs",
        Language::Rust,
        "pub const MAX_RETRIES: u32 = 3;\npub const DEFAULT_TIMEOUT_MS: u64 = 5000;\n",
    );
    let results = run_audit(vec![file]);
    let aes401: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES401")
        .collect();
    assert!(
        aes401.is_empty(),
        "clean constant file should not trigger AES401"
    );
}

// ── Entity file with primitive types → PrimitiveUsage violation ──

#[test]
fn aes401_entity_with_primitive_detected() {
    // taxonomy_foo_entity.rs with field typed as String → primitive usage
    let file = make_file(
        "src/taxonomy_user_entity.rs",
        Language::Rust,
        "pub struct User {\n    name: String,\n    age: i32,\n}\n",
    );
    let results = run_audit(vec![file]);
    let aes401: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES401")
        .collect();
    assert!(
        !aes401.is_empty(),
        "entity file with primitive types should trigger AES401"
    );
}

// ── Error file with primitive types → PrimitiveUsage violation ──

#[test]
fn aes401_error_with_primitive_detected() {
    let file = make_file(
        "src/taxonomy_app_error.rs",
        Language::Rust,
        "pub struct AppError {\n    message: String,\n}\n",
    );
    let results = run_audit(vec![file]);
    let aes401: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES401")
        .collect();
    assert!(
        !aes401.is_empty(),
        "error file with primitive types should trigger AES401"
    );
}

// ── Event file with primitive types → PrimitiveUsage violation ──

#[test]
fn aes401_event_with_primitive_detected() {
    let file = make_file(
        "src/taxonomy_user_event.rs",
        Language::Rust,
        "pub struct UserEvent {\n    payload: String,\n}\n",
    );
    let results = run_audit(vec![file]);
    let aes401: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES401")
        .collect();
    assert!(
        !aes401.is_empty(),
        "event file with primitive types should trigger AES401"
    );
}

// ── Non-taxonomy file is not checked by taxonomy rules ──

#[test]
fn aes401_non_taxonomy_file_ignored() {
    let file = make_file(
        "src/capabilities_feature.rs",
        Language::Rust,
        "pub struct Foo {\n    val: String,\n}\n",
    );
    let results = run_audit(vec![file]);
    let aes401: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES401")
        .collect();
    assert!(
        aes401.is_empty(),
        "non-taxonomy file should not trigger AES401"
    );
}
