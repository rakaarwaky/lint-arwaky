// Acceptance test AES404 — Utility purity.
// Utility files must not define structs, enums, or classes.
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

// ── Rust utility with struct → AES404 ──

#[test]
fn aes404_rust_utility_with_struct_detected() {
    let file = make_file(
        "src/utility_helpers.rs",
        Language::Rust,
        "pub struct HelperState {\n    count: i32,\n}\n",
    );
    let results = run_audit(vec![file]);
    let aes404: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES404")
        .collect();
    assert!(
        !aes404.is_empty(),
        "utility with struct should trigger AES404"
    );
}

// ── Rust utility with enum → AES404 ──

#[test]
fn aes404_rust_utility_with_enum_detected() {
    let file = make_file(
        "src/utility_formatters.rs",
        Language::Rust,
        "pub enum Format {\n    Json,\n    Csv,\n}\n",
    );
    let results = run_audit(vec![file]);
    let aes404: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES404")
        .collect();
    assert!(
        !aes404.is_empty(),
        "utility with enum should trigger AES404"
    );
}

// ── Clean Rust utility (pure functions) → no violation ──

#[test]
fn aes404_clean_utility_no_violation() {
    let file = make_file(
        "src/utility_helpers.rs",
        Language::Rust,
        "pub fn add(a: i32, b: i32) -> i32 { a + b }\npub fn mul(a: i32, b: i32) -> i32 { a * b }\n",
    );
    let results = run_audit(vec![file]);
    let aes404: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES404")
        .collect();
    assert!(
        aes404.is_empty(),
        "clean utility with pure functions should not trigger AES404"
    );
}

// ── Python utility with class → AES404 ──

#[test]
fn aes404_python_utility_with_class_detected() {
    let file = make_file(
        "src/utility_converters.py",
        Language::Python,
        "class Converter:\n    pass\n",
    );
    let results = run_audit(vec![file]);
    let aes404: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES404")
        .collect();
    assert!(
        !aes404.is_empty(),
        "python utility with class should trigger AES404"
    );
}

// ── TypeScript utility with class → AES404 ──

#[test]
fn aes404_typescript_utility_with_class_detected() {
    let file = make_file(
        "src/utility_formatters.ts",
        Language::TypeScript,
        "export class Formatter {\n    format(s: string): string { return s; }\n}\n",
    );
    let results = run_audit(vec![file]);
    let aes404: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES404")
        .collect();
    assert!(
        !aes404.is_empty(),
        "typescript utility with class should trigger AES404"
    );
}

// ── Non-utility file is not checked ──

#[test]
fn aes404_non_utility_file_ignored() {
    let file = make_file(
        "src/capabilities_feature.rs",
        Language::Rust,
        "pub struct Feature {}\nimpl IFeature for Feature {}\n",
    );
    let results = run_audit(vec![file]);
    let aes404: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES404")
        .collect();
    assert!(
        aes404.is_empty(),
        "non-utility file should not trigger AES404"
    );
}
