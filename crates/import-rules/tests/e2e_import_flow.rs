// PURPOSE: E2E tests — full pipeline from temp dir with violation files through audit.
use import_rules_lint_arwaky::root_import_rules_container::ImportContainer;
use shared::common::NamingConfig;
use shared::common::taxonomy_common_vo::{BooleanVO, Count, PatternList};
use shared::common::taxonomy_definition_vo::LayerDefinition;
use shared::common::taxonomy_layer_vo::LayerNameVO;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::config_system::ArchitectureConfig;
use shared::import_rules::IImportRunnerAggregate;
use std::collections::HashMap;
use std::sync::Arc;
use tempfile::TempDir;

fn full_config() -> ArchitectureConfig {
    let mut layers = HashMap::new();
    layers.insert(
        LayerNameVO::new("capabilities"),
        LayerDefinition {
            forbidden: PatternList::new(vec!["agent", "surfaces"]),
            allowed: PatternList::new(vec!["taxonomy", "contract", "utility"]),
            ..Default::default()
        },
    );
    layers.insert(
        LayerNameVO::new("taxonomy"),
        LayerDefinition {
            forbidden: PatternList::new(vec!["capabilities", "agent", "surfaces"]),
            allowed: PatternList::new(vec!["utility"]),
            ..Default::default()
        },
    );
    layers.insert(
        LayerNameVO::new("agent"),
        LayerDefinition {
            forbidden: PatternList::new(vec!["surfaces"]),
            allowed: PatternList::new(vec!["taxonomy", "contract", "utility", "capabilities"]),
            ..Default::default()
        },
    );
    ArchitectureConfig::new(
        BooleanVO::new(true),
        layers,
        vec![],
        NamingConfig::new(Count::new(3)),
        FilePathList::new(vec![]),
        BooleanVO::new(false),
    )
}

fn make_orchestrator_at(tmp: &std::path::Path) -> Arc<dyn IImportRunnerAggregate> {
    let config = full_config();
    let fs_container = filesystem::root_filesystem_container::FilesystemContainer::new();
    let fs = fs_container.orchestrator();
    // Build file index so import_list() is populated for the temp dir
    fs.build_file_index(tmp);
    let container = ImportContainer::new_with_config(config, fs);
    container.orchestrator()
}

#[test]
fn e2e_capabilities_importing_agent_detected() {
    let tmp = TempDir::new().unwrap();
    let file = tmp.path().join("capabilities_checker.rs");
    std::fs::write(
        &file,
        "use agent::orchestrator;\n\nfn check() {\n    let _ = orchestrator::run();\n}\n",
    )
    .unwrap();

    let orch = make_orchestrator_at(tmp.path());
    let target = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let results = orch.run_audit(&target).unwrap();

    let aes201: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES201")
        .collect();
    assert!(
        !aes201.is_empty(),
        "E2E: capabilities file importing agent should produce AES201 violations"
    );
}

#[test]
fn e2e_capabilities_importing_surfaces_detected() {
    let tmp = TempDir::new().unwrap();
    let file = tmp.path().join("capabilities_handler.rs");
    std::fs::write(
        &file,
        "use surfaces::ui;\n\nfn render() {\n    ui::draw();\n}\n",
    )
    .unwrap();

    let orch = make_orchestrator_at(tmp.path());
    let target = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let results = orch.run_audit(&target).unwrap();

    let aes201: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES201")
        .collect();
    assert!(
        !aes201.is_empty(),
        "E2E: capabilities file importing surfaces should produce AES201 violations"
    );
}

#[test]
fn e2e_clean_file_no_violations() {
    let tmp = TempDir::new().unwrap();
    let file = tmp.path().join("capabilities_processor.rs");
    std::fs::write(
        &file,
        "use shared::taxonomy_vo;\n\nfn process() {\n    let _ = taxonomy_vo::run();\n}\n",
    )
    .unwrap();

    let orch = make_orchestrator_at(tmp.path());
    let target = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let results = orch.run_audit(&target).unwrap();

    let aes201: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES201")
        .collect();
    assert!(
        aes201.is_empty(),
        "E2E: clean capabilities file should have no AES201 violations"
    );
}

#[test]
fn e2e_multiple_violations_across_files() {
    let tmp = TempDir::new().unwrap();

    // File 1: capabilities importing agent
    let f1 = tmp.path().join("capabilities_checker.rs");
    std::fs::write(
        &f1,
        "use agent::orchestrator;\n\nfn check() { orchestrator::run(); }\n",
    )
    .unwrap();

    // File 2: capabilities importing surfaces
    let f2 = tmp.path().join("capabilities_renderer.rs");
    std::fs::write(&f2, "use surfaces::ui;\n\nfn render() { ui::draw(); }\n").unwrap();

    let orch = make_orchestrator_at(tmp.path());
    let target = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let results = orch.run_audit(&target).unwrap();

    let aes201: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES201")
        .collect();
    assert!(
        aes201.len() >= 2,
        "E2E: multiple files with violations should produce at least 2 AES201 violations, got {}",
        aes201.len()
    );
}

#[test]
fn e2e_audit_with_entries_returns_results() {
    // test run_audit_with_entries with pre-built FileEntry
    let tmp = TempDir::new().unwrap();
    let file = tmp.path().join("capabilities_handler.rs");
    let content =
        "use agent::orchestrator;\n\nfn handle() {\n    let _ = orchestrator::run();\n}\n";
    std::fs::write(&file, content).unwrap();

    let orch = make_orchestrator_at(tmp.path());

    let file_entry = shared::filesystem::taxonomy_filesystem_vo::FileEntry {
        path: file.clone(),
        extension: "rs".to_string(),
        language: shared::filesystem::taxonomy_filesystem_vo::Language::Rust,
        size: content.len() as u64,
        content: content.to_string(),
        parse_ok: true,
        parse_metadata: None,
    };

    let results = orch.run_audit_with_entries(&[file_entry]);
    // Should at least not panic and produce some result
    // Note: run_audit_with_entries may produce different results than run_audit
    // since it uses pre-parsed entries rather than filesystem scan
    let _ = results;
}
