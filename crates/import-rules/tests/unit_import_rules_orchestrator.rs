// PURPOSE: Unit tests for ImportOrchestrator (agent layer)
// Tests orchestration flow: file collection, ignore logic, enabled gate.

use import_rules_lint_arwaky::root_import_rules_container::ImportContainer;
use shared::common::taxonomy_common_vo::{BooleanVO, Count};
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use std::collections::HashMap;
use std::io::Write;

fn test_config(enabled: bool) -> ArchitectureConfig {
    ArchitectureConfig {
        enabled: BooleanVO::new(enabled),
        layers: HashMap::new(),
        rules: Vec::new(),
        naming: shared::common::taxonomy_definition_vo::NamingConfig::new(Count::new(2)),
        ignored_paths: FilePathList { values: vec![] },
        mandatory_class_definition: BooleanVO::new(false),
    }
}

fn write_temp_rs(dir: &std::path::Path, name: &str, content: &str) -> FilePath {
    let path = dir.join(name);
    let mut file = std::fs::File::create(&path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
    FilePath::new(path.to_string_lossy().to_string()).unwrap()
}

// ─── Enabled Gate ─────────────────────────────────────────

#[tokio::test]
async fn disabled_config_returns_empty_results() {
    let dir = tempfile::tempdir().unwrap();
    let _file = write_temp_rs(dir.path(), "taxonomy_x_vo.rs", "pub struct X;\n");

    let config = test_config(false);
    let container = ImportContainer::new_with_config(config);
    let orch = container.orchestrator();

    let target = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let result = orch.run_audit(&target).await.unwrap();

    assert!(
        result.is_empty(),
        "Disabled config should short-circuit to empty"
    );
}

// ─── Nonexistent Path ─────────────────────────────────────

#[tokio::test]
async fn nonexistent_target_returns_error() {
    let config = test_config(true);
    let container = ImportContainer::new_with_config(config);
    let orch = container.orchestrator();

    let target = FilePath::new("/nonexistent/path/that/does/not/exist").unwrap();
    let result = orch.run_audit(&target).await;

    assert!(result.is_err(), "Nonexistent path should return ScanError");
}

// ─── Single File Audit ────────────────────────────────────

#[tokio::test]
async fn single_clean_file_passes() {
    let dir = tempfile::tempdir().unwrap();
    let file = write_temp_rs(dir.path(), "taxonomy_clean_vo.rs", "pub struct Clean;\n");

    let config = test_config(true);
    let container = ImportContainer::new_with_config(config);
    let orch = container.orchestrator();

    let result = orch.run_audit(&file).await.unwrap();
    assert!(
        result.is_empty(),
        "Clean taxonomy file should produce no violations"
    );
}

// ─── Directory Audit ──────────────────────────────────────

#[tokio::test]
async fn directory_with_multiple_files_audited() {
    let dir = tempfile::tempdir().unwrap();
    write_temp_rs(dir.path(), "taxonomy_a_vo.rs", "pub struct A;\n");
    write_temp_rs(dir.path(), "taxonomy_b_vo.rs", "pub struct B;\n");

    let config = test_config(true);
    let container = ImportContainer::new_with_config(config);
    let orch = container.orchestrator();

    let target = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let result = orch.run_audit(&target).await.unwrap();

    // Clean files should produce no violations
    assert!(result.is_empty());
}

// ─── Ignored Directories ──────────────────────────────────

#[tokio::test]
async fn target_dir_is_skipped() {
    let dir = tempfile::tempdir().unwrap();
    let target_subdir = dir.path().join("target");
    std::fs::create_dir_all(&target_subdir).unwrap();
    write_temp_rs(
        &target_subdir,
        "taxonomy_ignored_vo.rs",
        "use crate::capabilities_x::X;\npub struct Y;\n",
    );
    write_temp_rs(dir.path(), "taxonomy_real_vo.rs", "pub struct Real;\n");

    let config = test_config(true);
    let container = ImportContainer::new_with_config(config);
    let orch = container.orchestrator();

    let target = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let result = orch.run_audit(&target).await.unwrap();

    // Files in target/ should be skipped
    assert!(
        !result.iter().any(|v| v.file.value().contains("target")),
        "Files in target/ directory should be ignored"
    );
}

// ─── Orchestrator Name ────────────────────────────────────

#[tokio::test]
async fn orchestrator_reports_correct_name() {
    let config = test_config(true);
    let container = ImportContainer::new_with_config(config);
    let orch = container.orchestrator();
    assert_eq!(orch.name(), "import-rules");
}
