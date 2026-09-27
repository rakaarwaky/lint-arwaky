// PURPOSE: Integration tests — ImportContainer wiring with real filesystem aggregate.
use import_rules_lint_arwaky::root_import_rules_container::ImportContainer;
use shared::common::NamingConfig;
use shared::common::taxonomy_common_vo::{BooleanVO, Count, PatternList};
use shared::common::taxonomy_definition_vo::LayerDefinition;
use shared::common::taxonomy_layer_vo::LayerNameVO;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::config_system::ArchitectureConfig;
use shared::filesystem::FilesystemRequest;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use shared::filesystem::contract_filesystem_protocol::IParserProtocol;
use shared::filesystem::contract_filesystem_protocol::IWorkspaceProtocol;
use shared::import_rules::taxonomy_import_request::ImportRequest;
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

fn make_filesystem() -> (
    Arc<dyn IFilesystemAggregate>,
    Arc<dyn IFileSystemIOProtocol>,
    Arc<dyn IWorkspaceProtocol>,
    Arc<dyn IParserProtocol>,
) {
    let container = filesystem::root_filesystem_container::FilesystemContainer::new();
    (
        container.orchestrator(),
        container.io(),
        container.workspace(),
        container.parser(),
    )
}

#[test]
fn container_creates_orchestrator() {
    let config = test_config();
    let (fs, fs_io, ws, parser) = make_filesystem();
    let container = ImportContainer::new_with_config(config, fs, fs_io, ws, parser);
    let orchestrator = container.orchestrator();
    assert_eq!(
        orchestrator.execute(ImportRequest::Name).into_name(),
        "import-rules"
    );
}

#[test]
fn orchestrator_returns_empty_for_disabled_config() {
    let mut layers = HashMap::new();
    layers.insert(LayerNameVO::new("capabilities"), LayerDefinition::default());
    let config = ArchitectureConfig::new(
        BooleanVO::new(false), // disabled
        layers,
        vec![],
        NamingConfig::new(Count::new(3)),
        FilePathList::new(vec![]),
        BooleanVO::new(false),
    );
    let (fs, fs_io, ws, parser) = make_filesystem();
    let container = ImportContainer::new_with_config(config, fs, fs_io, ws, parser);
    let orchestrator = container.orchestrator();

    let target = FilePath::new("/tmp/nonexistent_path".to_string()).unwrap();
    let result = orchestrator
        .execute(ImportRequest::audit(&target))
        .into_result();
    // disabled config returns Ok(empty) regardless of path
    assert!(result.is_ok());
    assert!(result.unwrap().is_empty());
}

#[test]
fn orchestrator_errors_on_nonexistent_target() {
    let config = test_config();
    let (fs, fs_io, ws, parser) = make_filesystem();
    let container = ImportContainer::new_with_config(config, fs, fs_io, ws, parser);
    let orchestrator = container.orchestrator();

    let target = FilePath::new("/tmp/definitely_does_not_exist_12345".to_string()).unwrap();
    let result = orchestrator
        .execute(ImportRequest::audit(&target))
        .into_result();
    assert!(result.is_err());
}

#[test]
fn orchestrator_scans_empty_temp_dir_without_errors() {
    let tmp = TempDir::new().unwrap();
    let config = test_config();
    let (fs, fs_io, ws, parser) = make_filesystem();
    let container = ImportContainer::new_with_config(config, fs, fs_io, ws, parser);
    let orchestrator = container.orchestrator();

    let target = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let result = orchestrator
        .execute(ImportRequest::audit(&target))
        .into_result();
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
    let (fs, fs_io, ws, parser) = make_filesystem();
    let container = ImportContainer::new_with_config(config, fs, fs_io, ws, parser);
    let orchestrator = container.orchestrator();

    let target = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let result = orchestrator
        .execute(ImportRequest::audit(&target))
        .into_result();
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
    let (fs, fs_io, ws, parser) = make_filesystem();
    // Build file index so import_list() is populated
    fs.execute(FilesystemRequest::build_file_index(tmp.path()));
    let container = ImportContainer::new_with_config(config, fs, fs_io, ws, parser);
    let orchestrator = container.orchestrator();

    let target = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let result = orchestrator
        .execute(ImportRequest::audit(&target))
        .into_result()
        .unwrap();
    assert!(
        result.iter().any(|r| r.code.code() == "AES201"),
        "Should detect AES201 forbidden import violation"
    );
}
