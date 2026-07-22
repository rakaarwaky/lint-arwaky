// PURPOSE: Unit tests for ArchImportForbiddenChecker (AES201)
// Uses temp files because the checker reads from disk internally.

use import_rules_lint_arwaky::capabilities_import_forbidden_checker::ArchImportForbiddenChecker;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::common::taxonomy_common_vo::{BooleanVO, Count, PatternList};
use shared::common::taxonomy_definition_vo::{LayerDefinition, LayerMapVO};
use shared::common::taxonomy_layer_vo::LayerNameVO;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::import_rules::contract_import_forbidden_protocol::IImportForbiddenProtocol;
use std::collections::HashMap;
use std::io::Write;

fn sut() -> ArchImportForbiddenChecker {
    ArchImportForbiddenChecker::new()
}

fn test_config() -> ArchitectureConfig {
    ArchitectureConfig {
        enabled: BooleanVO::new(true),
        layers: HashMap::new(),
        rules: Vec::new(),
        naming: shared::common::taxonomy_definition_vo::NamingConfig::new(Count::new(2)),
        ignored_paths: FilePathList { values: vec![] },
        mandatory_class_definition: BooleanVO::new(false),
    }
}

fn layer_map_with_taxonomy_forbidden() -> LayerMapVO {
    let mut layers = HashMap::new();
    layers.insert(
        LayerNameVO::new("taxonomy"),
        LayerDefinition {
            allowed: PatternList::new(Vec::<String>::new()),
            forbidden: PatternList::new(vec![
                "contract".to_string(),
                "utility".to_string(),
                "capabilities".to_string(),
                "agent".to_string(),
                "surfaces".to_string(),
                "root".to_string(),
            ]),
            mandatory: PatternList::new(Vec::<String>::new()),
            word_count: Count::new(2),
            exceptions: PatternList::new(Vec::<String>::new()),
            recursive: BooleanVO::new(false),
            ..Default::default()
        },
    );
    LayerMapVO::new(layers)
}

fn write_temp_rs(dir: &std::path::Path, name: &str, content: &str) -> FilePath {
    let path = dir.join(name);
    let mut file = std::fs::File::create(&path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
    FilePath::new(path.to_string_lossy().to_string()).unwrap()
}

// ─── Happy Path: Valid Import ─────────────────────────────

#[tokio::test]
async fn taxonomy_importing_nothing_passes() {
    let dir = tempfile::tempdir().unwrap();
    let file = write_temp_rs(dir.path(), "taxonomy_foo_vo.rs", "pub struct Foo;\n");

    let config = test_config();
    let layer_map = layer_map_with_taxonomy_forbidden();
    let files = FilePathList::new(vec![file]);
    let root = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let mut results = LintResultList::new(Vec::new());

    sut()
        .check_forbidden_imports(&config, &layer_map, &files, &root, &mut results)
        .await;

    assert!(results.is_empty(), "Taxonomy with no imports should pass");
}

// ─── Forbidden Import Detected ────────────────────────────

#[tokio::test]
async fn taxonomy_importing_capabilities_flagged() {
    let dir = tempfile::tempdir().unwrap();
    let content = r#"
use crate::capabilities_some_checker::SomeChecker;

pub struct Foo;
"#;
    let file = write_temp_rs(dir.path(), "taxonomy_foo_vo.rs", content);

    let config = test_config();
    let layer_map = layer_map_with_taxonomy_forbidden();
    let files = FilePathList::new(vec![file]);
    let root = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let mut results = LintResultList::new(Vec::new());

    sut()
        .check_forbidden_imports(&config, &layer_map, &files, &root, &mut results)
        .await;

    assert!(
        !results.is_empty(),
        "Taxonomy importing capabilities should produce AES201 violation"
    );
    assert!(results.values.iter().all(|v| v.code.code() == "AES201"));
}

#[tokio::test]
async fn taxonomy_importing_agent_flagged() {
    let dir = tempfile::tempdir().unwrap();
    let content = r#"
use crate::agent_import_orchestrator::ImportOrchestrator;

pub struct Bar;
"#;
    let file = write_temp_rs(dir.path(), "taxonomy_bar_vo.rs", content);

    let config = test_config();
    let layer_map = layer_map_with_taxonomy_forbidden();
    let files = FilePathList::new(vec![file]);
    let root = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let mut results = LintResultList::new(Vec::new());

    sut()
        .check_forbidden_imports(&config, &layer_map, &files, &root, &mut results)
        .await;

    assert!(
        !results.is_empty(),
        "Taxonomy importing agent should be flagged"
    );
}

// ─── Exception Handling ───────────────────────────────────

#[tokio::test]
async fn excepted_file_skips_check() {
    let dir = tempfile::tempdir().unwrap();
    let content = r#"
use crate::capabilities_some_checker::SomeChecker;
pub struct Foo;
"#;
    let file = write_temp_rs(dir.path(), "taxonomy_special_vo.rs", content);

    let mut config = test_config();
    config.rules.push(
        shared::config_system::taxonomy_config_vo::ArchitectureRule {
            name: shared::common::taxonomy_suggestion_vo::DescriptionVO::new("AES201".to_string()),
            exceptions: PatternList::new(vec!["taxonomy_special_vo.rs".to_string()]),
            ..Default::default()
        },
    );

    let layer_map = layer_map_with_taxonomy_forbidden();
    let files = FilePathList::new(vec![file]);
    let root = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let mut results = LintResultList::new(Vec::new());

    sut()
        .check_forbidden_imports(&config, &layer_map, &files, &root, &mut results)
        .await;

    assert!(
        results.is_empty(),
        "Excepted file should not produce violations"
    );
}

// ─── Severity ─────────────────────────────────────────────

#[tokio::test]
async fn forbidden_violation_severity_is_critical() {
    let dir = tempfile::tempdir().unwrap();
    let content = "use crate::capabilities_x::X;\npub struct Z;\n";
    let file = write_temp_rs(dir.path(), "taxonomy_z_vo.rs", content);

    let config = test_config();
    let layer_map = layer_map_with_taxonomy_forbidden();
    let files = FilePathList::new(vec![file]);
    let root = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let mut results = LintResultList::new(Vec::new());

    sut()
        .check_forbidden_imports(&config, &layer_map, &files, &root, &mut results)
        .await;

    for v in &results.values {
        assert_eq!(
            v.severity,
            shared::cli_commands::taxonomy_severity_vo::Severity::CRITICAL
        );
    }
}
