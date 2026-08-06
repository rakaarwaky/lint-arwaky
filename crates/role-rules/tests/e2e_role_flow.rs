// E2E tests — full pipeline: create temp dir with files → construct FileEntry list → run audit → verify violations.
use role_rules_lint_arwaky::root_role_rules_container::RoleContainer;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::filesystem::taxonomy_filesystem_vo::{FileEntry, Language};
use std::fs;

/// Create a FileEntry from a file on disk (simulates filesystem → orchestrator pipeline).
fn file_entry_from_path(path: &std::path::Path, content: &str, lang: Language) -> FileEntry {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_string();
    FileEntry {
        path: path.to_path_buf(),
        extension: ext,
        language: lang,
        size: content.len() as u64,
        content: content.to_string(),
        parse_ok: true,
        parse_metadata: None,
    }
}

#[test]
fn e2e_taxonomy_violation_detected() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path();

    // Create a taxonomy constant file with a struct (AES401 violation)
    // Basename must end with "_constant.rs" to trigger check_constant
    let path = dir.join("taxonomy_app_constant.rs");
    let content = "pub struct AppConfig {\n    pub name: String,\n}\n";
    fs::write(&path, content).unwrap();

    let file = file_entry_from_path(&path, content, Language::Rust);

    let config = ArchitectureConfig::default();
    let container = RoleContainer::new_with_config(config);
    let orch = container.orchestrator();

    let results = orch.run_audit_with_entries(&[file]);
    let aes401: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES401")
        .collect();
    assert!(
        !aes401.is_empty(),
        "taxonomy constant with struct should produce AES401"
    );
}

#[test]
fn e2e_capability_violation_detected() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path();

    // Capabilities file with 4 structs (too many types)
    let path = dir.join("capabilities_feature.rs");
    let content = "pub struct A {}\npub struct B {}\npub struct C {}\npub struct D {}\n";
    fs::write(&path, content).unwrap();

    let file = file_entry_from_path(&path, content, Language::Rust);

    let config = ArchitectureConfig::default();
    let container = RoleContainer::new_with_config(config);
    let orch = container.orchestrator();

    let results = orch.run_audit_with_entries(&[file]);
    let aes403: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES403")
        .collect();
    assert!(
        !aes403.is_empty(),
        "capabilities with 4 types should produce AES403"
    );
}

#[test]
fn e2e_utility_violation_detected() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path();

    // Utility file with a struct (AES404 violation)
    let path = dir.join("utility_helpers.rs");
    let content = "pub struct HelperState {\n    count: i32,\n}\n";
    fs::write(&path, content).unwrap();

    let file = file_entry_from_path(&path, content, Language::Rust);

    let config = ArchitectureConfig::default();
    let container = RoleContainer::new_with_config(config);
    let orch = container.orchestrator();

    let results = orch.run_audit_with_entries(&[file]);
    let aes404: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES404")
        .collect();
    assert!(
        !aes404.is_empty(),
        "utility with struct should produce AES404"
    );
}

#[test]
fn e2e_agent_violation_detected() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path();

    // Agent file with no implementor (AES405 violation)
    let path = dir.join("agent_dispatcher.rs");
    let content = "pub struct Dispatcher {}\n";
    fs::write(&path, content).unwrap();

    let file = file_entry_from_path(&path, content, Language::Rust);

    let config = ArchitectureConfig::default();
    let container = RoleContainer::new_with_config(config);
    let orch = container.orchestrator();

    let results = orch.run_audit_with_entries(&[file]);
    let aes405: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES405")
        .collect();
    assert!(
        !aes405.is_empty(),
        "agent without implementor should produce AES405"
    );
}

#[test]
fn e2e_surface_violation_detected() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path();

    // Passive surface with excessive control flow (AES406 violation)
    let mut lines = Vec::new();
    for i in 0..51 {
        lines.push(format!("if condition_{} {{}}", i));
    }
    let content = lines.join("\n");
    let path = dir.join("surface_dashboard.rs");
    fs::write(&path, &content).unwrap();

    let file = file_entry_from_path(&path, &content, Language::Rust);

    let config = ArchitectureConfig::default();
    let container = RoleContainer::new_with_config(config);
    let orch = container.orchestrator();

    let results = orch.run_audit_with_entries(&[file]);
    let aes406: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES406")
        .collect();
    assert!(
        !aes406.is_empty(),
        "passive surface with excess control flow should produce AES406"
    );
}

#[test]
fn e2e_mixed_files_produce_correct_violations() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path();

    // Create multiple files with different violations
    // Basename must end with "_constant.rs" to trigger check_constant
    let taxonomy_path = dir.join("taxonomy_settings_constant.rs");
    let taxonomy_content = "pub struct Settings {}\n";
    fs::write(&taxonomy_path, taxonomy_content).unwrap();

    let utility_path = dir.join("utility_formatter.rs");
    let utility_content = "pub struct Formatter {}\n";
    fs::write(&utility_path, utility_content).unwrap();

    let agent_path = dir.join("agent_runner.rs");
    let agent_content = "pub struct Runner {}\n";
    fs::write(&agent_path, agent_content).unwrap();

    // A clean file should produce no violations
    let clean_path = dir.join("some_module.rs");
    let clean_content = "pub fn helper() -> i32 { 42 }\n";
    fs::write(&clean_path, clean_content).unwrap();

    let files = vec![
        file_entry_from_path(&taxonomy_path, taxonomy_content, Language::Rust),
        file_entry_from_path(&utility_path, utility_content, Language::Rust),
        file_entry_from_path(&agent_path, agent_content, Language::Rust),
        file_entry_from_path(&clean_path, clean_content, Language::Rust),
    ];

    let config = ArchitectureConfig::default();
    let container = RoleContainer::new_with_config(config);
    let orch = container.orchestrator();

    let results = orch.run_audit_with_entries(&files);

    let codes: Vec<&str> = results.iter().map(|r| r.code.code()).collect();
    assert!(
        codes.contains(&"AES401"),
        "should detect taxonomy violation"
    );
    assert!(codes.contains(&"AES404"), "should detect utility violation");
    assert!(codes.contains(&"AES405"), "should detect agent violation");
}
