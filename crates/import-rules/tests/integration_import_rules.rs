// PURPOSE: Integration tests — ImportContainer wiring with real filesystem aggregate.
use import_rules_lint_arwaky::root_import_rules_container::ImportContainer;
use shared::common::taxonomy_common_vo::{BooleanVO, Count, PatternList};
use shared::common::taxonomy_definition_vo::{LayerDefinition, LayerMapVO};
use shared::common::taxonomy_layer_vo::LayerNameVO;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::config_system::ArchitectureConfig;
use shared::config_system::taxonomy_config_vo::NamingConfig;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use std::collections::HashMap;
use std::sync::Arc;
use tempfile::TempDir;

fn test_config() -> ArchitectureConfig {
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
    ArchitectureConfig::new(
        BooleanVO::new(true),
        layers,
        vec![],
        NamingConfig::new(Count::new(3)),
        FilePathList::new(vec![]),
        BooleanVO::new(false),
    )
}

fn make_filesystem() -> Arc<dyn IFilesystemAggregate> {
    let container = filesystem_lint_arwaky::root_filesystem_container::FilesystemContainer::new();
    container.orchestrator()
}

#[test]
fn container_creates_orchestrator() {
    let config = test_config();
    let fs = make_filesystem();
    let container = ImportContainer::new_with_config(config, fs);
    let orchestrator = container.orchestrator();
    assert_eq!(orchestrator.name(), "import-rules");
}

#[test]
fn orchestrator_returns_empty_for_disabled_config() {
    let mut layers = HashMap::new();
    layers.insert(
        LayerNameVO::new("capabilities"),
        LayerDefinition::default(),
    );
    let config = ArchitectureConfig::new(
        BooleanVO::new(false), // disabled
        layers,
        vec![],
        NamingConfig::new(Count::new(3)),
        FilePathList::new(vec![]),
        BooleanVO::new(false),
    );
    let fs = make_filesystem();
    let container = ImportContainer::new_with_config(config, fs);
    let orchestrator = container.orchestrator();

    let target = FilePath::new("/tmp/nonexistent_path".to_string()).unwrap();
    let result = orchestrator.run_audit(&target);
    // disabled config returns Ok(empty) regardless of path
    assert!(result.is_ok());
    assert!(result.unwrap().is_empty());
}

#[test]
fn orchestrator_errors_on_nonexistent_target() {
    let config = test_config();
    let fs = make_filesystem();
    let container = ImportContainer::new_with_config(config, fs);
    let orchestrator = container.orchestrator();

    let target = FilePath::new("/tmp/definitely_does_not_exist_12345".to_string()).unwrap();
    let result = orchestrator.run_audit(&target);
    assert!(result.is_err());
}

#[test]
fn orchestrator_scans_empty_temp_dir_without_errors() {
    let tmp = TempDir::new().unwrap();
    let config = test_config();
    let fs = make_filesystem();
    let container = ImportContainer::new_with_config(config, fs);
    let orchestrator = container.orchestrator();

    let target = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let result = orchestrator.run_audit(&target);
    assert!(result.is_ok());
    assert!(
        result.unwrap().is_empty(),
        "Empty dir should produce no violations"
    );
}

#[test]
fn orchestrator_scans_temp_dir_with_clean_rust_files() {
    let tmp = TempDir::new().unwrap();
    // Create a clean Rust file in capabilities layer — no forbidden imports
    let file_path = tmp.path().join("capabilities_handler.rs");
    std::fs::write(
        &file_path,
        "use shared::taxonomy_vo;\n\nfn process() {\n    let _ = taxonomy_vo::do_stuff();\n}\n",
    )
    .unwrap();

    let config = test_config();
    let fs = make_filesystem();
    let container = ImportContainer::new_with_config(config, fs);
    let orchestrator = container.orchestrator();

    let target = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let result = orchestrator.run_audit(&target);
    assert!(result.is_ok());
}

#[test]
fn orchestrator_detects_forbidden_import_in_temp_dir() {
    let tmp = TempDir::new().unwrap();
    // capabilities file importing from agent — forbidden
    let file_path = tmp.path().join("capabilities_checker.rs");
    std::fs::write(
        &file_path,
        "use agent::orchestrator;\n\nfn check() {\n    let _ = orchestrator::run();\n}\n",
    )
    .unwrap();

    let config = test_config();
    let fs = make_filesystem();
    let container = ImportContainer::new_with_config(config, fs);
    let orchestrator = container.orchestrator();

    let target = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let result = orchestrator.run_audit(&target).unwrap();
    assert!(
        result.iter().any(|r| r.code.code() == "AES201"),
        "Should detect AES201 forbidden import violation"
    );
}
