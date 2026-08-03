// Integration tests — verify RoleContainer DI wiring and orchestrator dispatch.
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
fn container_creates_with_default_config() {
    let config = ArchitectureConfig::default();
    let container = RoleContainer::new_with_config(config);
    let orch = container.orchestrator();
    assert_eq!(orch.name(), "role-rules");
}

#[test]
fn orchestrator_returns_empty_for_no_files() {
    let config = ArchitectureConfig::default();
    let container = RoleContainer::new_with_config(config);
    let orch = container.orchestrator();
    let results = orch.run_audit_with_entries(&[]);
    assert!(results.is_empty());
}

#[test]
fn orchestrator_dispatches_taxonomy_files() {
    let config = ArchitectureConfig::default();
    let container = RoleContainer::new_with_config(config);
    let orch = container.orchestrator();

    // taxonomy_constant.rs with a struct should trigger AES401 ConstantPurity
    let file = make_file(
        "src/taxonomy_my_constant.rs",
        Language::Rust,
        "pub struct BadConstant {\n    val: i32,\n}\n",
    );
    let results = orch.run_audit_with_entries(&[file]);
    let aes401: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES401")
        .collect();
    assert!(
        !aes401.is_empty(),
        "taxonomy constant file with struct should produce AES401 violations"
    );
}

#[test]
fn orchestrator_dispatches_capability_files() {
    let config = ArchitectureConfig::default();
    let container = RoleContainer::new_with_config(config);
    let orch = container.orchestrator();

    // capabilities file without impl → AES403
    let file = make_file(
        "src/capabilities_my_feature.rs",
        Language::Rust,
        "pub struct Foo {}\npub struct Bar {}\n",
    );
    let results = orch.run_audit_with_entries(&[file]);
    let aes403: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES403")
        .collect();
    assert!(
        !aes403.is_empty(),
        "capability file without implementor should produce AES403"
    );
}

#[test]
fn orchestrator_dispatches_agent_files() {
    let config = ArchitectureConfig::default();
    let container = RoleContainer::new_with_config(config);
    let orch = container.orchestrator();

    // agent file without impl → AES405
    let file = make_file(
        "src/agent_my_orchestrator.rs",
        Language::Rust,
        "pub struct MyOrchestrator {}\n",
    );
    let results = orch.run_audit_with_entries(&[file]);
    let aes405: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES405")
        .collect();
    assert!(
        !aes405.is_empty(),
        "agent file without implementor should produce AES405"
    );
}

#[test]
fn orchestrator_dispatches_utility_files() {
    let config = ArchitectureConfig::default();
    let container = RoleContainer::new_with_config(config);
    let orch = container.orchestrator();

    // utility file with struct → AES404
    let file = make_file(
        "src/utility_my_helper.rs",
        Language::Rust,
        "pub struct BadHelper {}\n",
    );
    let results = orch.run_audit_with_entries(&[file]);
    let aes404: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES404")
        .collect();
    assert!(
        !aes404.is_empty(),
        "utility file with struct should produce AES404"
    );
}

#[test]
fn orchestrator_skips_barrel_files() {
    let config = ArchitectureConfig::default();
    let container = RoleContainer::new_with_config(config);
    let orch = container.orchestrator();

    let files = vec![
        make_file("src/taxonomy/mod.rs", Language::Rust, "pub struct Bad {}"),
        make_file("src/taxonomy/lib.rs", Language::Rust, "pub struct Bad {}"),
    ];
    let results = orch.run_audit_with_entries(&files);
    assert!(
        results.is_empty(),
        "barrel files (mod.rs, lib.rs) should be skipped"
    );
}

#[test]
fn orchestrator_skips_unparseable_files() {
    let config = ArchitectureConfig::default();
    let container = RoleContainer::new_with_config(config);
    let orch = container.orchestrator();

    let mut file = make_file("src/taxonomy_bad.rs", Language::Rust, "pub struct Bad {}");
    file.parse_ok = false;
    let results = orch.run_audit_with_entries(&[file]);
    assert!(results.is_empty(), "unparseable files should be skipped");
}

#[test]
fn orchestrator_skips_empty_content() {
    let config = ArchitectureConfig::default();
    let container = RoleContainer::new_with_config(config);
    let orch = container.orchestrator();

    let file = make_file("src/taxonomy_empty.rs", Language::Rust, "");
    let results = orch.run_audit_with_entries(&[file]);
    assert!(results.is_empty(), "empty content files should be skipped");
}
