// Acceptance test AES406 — Surface passive role.
// Passive surfaces must not contain domain logic or orchestration.
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

// ── Passive surface with excessive control flow → AES406 ──

#[test]
fn aes406_passive_surface_excess_control_flow_detected() {
    let mut lines = Vec::new();
    for i in 0..6 {
        lines.push(format!("if condition_{} {{}}", i));
    }
    let file = make_file(
        "src/surface_dashboard.rs",
        Language::Rust,
        &lines.join("\n"),
    );
    let results = run_audit(vec![file]);
    let aes406: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES406")
        .collect();
    assert!(
        !aes406.is_empty(),
        "passive surface with excess control flow should trigger AES406"
    );
}

// ── Smart surface exempt from passive checks ──

#[test]
fn aes406_smart_surface_exempt() {
    let mut lines = Vec::new();
    for i in 0..6 {
        lines.push(format!("if condition_{} {{}}", i));
    }
    let file = make_file(
        "src/surface_my_command.rs",
        Language::Rust,
        &lines.join("\n"),
    );
    let results = run_audit(vec![file]);
    let aes406: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES406")
        .collect();
    assert!(
        aes406.is_empty(),
        "smart surface (suffix _command) should be exempt from passive checks"
    );
}

// ── Too many functions → SurfaceRoleViolation ──

#[test]
fn aes406_too_many_functions_detected() {
    let content: String = (0..20)
        .map(|i| format!("fn func_{}() {{}}", i))
        .collect::<Vec<_>>()
        .join("\n");
    let file = make_file("src/surface_helpers.rs", Language::Rust, &content);
    let results = run_audit(vec![file]);
    let aes406: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES406")
        .collect();
    assert!(
        !aes406.is_empty(),
        "surface with >15 functions should trigger AES406"
    );
}

// ── Clean passive surface → no violation ──

#[test]
fn aes406_clean_surface_no_violation() {
    let file = make_file(
        "src/surface_display.rs",
        Language::Rust,
        "pub fn render() -> String {\n    \"hello\".to_string()\n}\n",
    );
    let results = run_audit(vec![file]);
    let aes406: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES406")
        .collect();
    assert!(
        aes406.is_empty(),
        "clean passive surface should not trigger AES406"
    );
}

// ── Non-surface file is not checked ──

#[test]
fn aes406_non_surface_file_ignored() {
    let mut lines = Vec::new();
    for i in 0..6 {
        lines.push(format!("if condition_{} {{}}", i));
    }
    let file = make_file(
        "src/capabilities_feature.rs",
        Language::Rust,
        &lines.join("\n"),
    );
    let results = run_audit(vec![file]);
    let aes406: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES406")
        .collect();
    assert!(
        aes406.is_empty(),
        "non-surface file should not trigger AES406"
    );
}
