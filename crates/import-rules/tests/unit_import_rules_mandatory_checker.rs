// PURPOSE: Unit tests for ArchImportMandatoryChecker (AES202)
// Uses temp files because the checker reads from disk internally.

use import_rules_lint_arwaky::capabilities_import_mandatory_checker::ArchImportMandatoryChecker;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::common::taxonomy_common_vo::{BooleanVO, Count, PatternList};
use shared::common::taxonomy_definition_vo::{LayerDefinition, LayerMapVO};
use shared::common::taxonomy_layer_vo::LayerNameVO;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::import_rules::contract_import_mandatory_protocol::IImportMandatoryProtocol;
use std::collections::HashMap;
use std::io::Write;

fn sut() -> ArchImportMandatoryChecker {
    ArchImportMandatoryChecker::new()
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

fn layer_map_with_mandatory_contract() -> LayerMapVO {
    let mut layers = HashMap::new();
    layers.insert(
        LayerNameVO::new("capabilities"),
        LayerDefinition {
            allowed: PatternList::new(vec![
                "taxonomy".to_string(),
                "contract".to_string(),
                "utility".to_string(),
            ]),
            forbidden: PatternList::new(Vec::<String>::new()),
            mandatory: PatternList::new(vec!["contract".to_string()]),
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

// ─── Happy Path: Mandatory Import Present ─────────────────

#[tokio::test]
async fn capabilities_with_contract_import_passes() {
    let dir = tempfile::tempdir().unwrap();
    let content = r#"
use shared::import_rules::contract_unused_import_protocol::IUnusedImportProtocol;

pub struct MyChecker;

impl IUnusedImportProtocol for MyChecker {
    fn find_unused_imports(&self, _p: &FilePath) -> Vec<LintMessage> { vec![] }
    fn check_unused_imports(&self, _f: &str, _c: &str, _v: &mut Vec<LintResult>) {}
}
"#;
    let file = write_temp_rs(dir.path(), "capabilities_my_checker.rs", content);

    let config = test_config();
    let layer_map = layer_map_with_mandatory_contract();
    let files = FilePathList::new(vec![file]);
    let root = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let mut results = LintResultList::new(Vec::new());

    sut()
        .run_mandatory_imports(&config, &layer_map, &files, &root, &mut results)
        .await;

    assert!(
        results.is_empty(),
        "Capabilities with contract import should pass"
    );
}

// ─── Missing Mandatory Import ─────────────────────────────

#[tokio::test]
async fn capabilities_without_contract_import_flagged() {
    let dir = tempfile::tempdir().unwrap();
    let content = r#"
use std::collections::HashMap;

pub struct MyChecker {
    data: HashMap<String, i32>,
}
"#;
    let file = write_temp_rs(dir.path(), "capabilities_my_checker.rs", content);

    let config = test_config();
    let layer_map = layer_map_with_mandatory_contract();
    let files = FilePathList::new(vec![file]);
    let root = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let mut results = LintResultList::new(Vec::new());

    sut()
        .run_mandatory_imports(&config, &layer_map, &files, &root, &mut results)
        .await;

    assert!(
        !results.is_empty(),
        "Capabilities without contract import should produce AES202"
    );
    assert!(results.values.iter().all(|v| v.code.value() == "AES202"));
}

// ─── Exception File ───────────────────────────────────────

#[tokio::test]
async fn excepted_file_skips_mandatory_check() {
    let dir = tempfile::tempdir().unwrap();
    let content = "pub struct Helper;\n";
    let file = write_temp_rs(dir.path(), "capabilities_helper.rs", content);

    let mut config = test_config();
    config.rules.push(
        shared::config_system::taxonomy_config_vo::ArchitectureRule {
            name: shared::common::taxonomy_suggestion_vo::DescriptionVO::new("AES202".to_string()),
            exceptions: PatternList::new(vec!["capabilities_helper.rs".to_string()]),
            ..Default::default()
        },
    );

    let layer_map = layer_map_with_mandatory_contract();
    let files = FilePathList::new(vec![file]);
    let root = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let mut results = LintResultList::new(Vec::new());

    sut()
        .run_mandatory_imports(&config, &layer_map, &files, &root, &mut results)
        .await;

    assert!(
        results.is_empty(),
        "Excepted file should skip mandatory check"
    );
}

// ─── Severity ─────────────────────────────────────────────

#[tokio::test]
async fn mandatory_violation_severity_is_high() {
    let dir = tempfile::tempdir().unwrap();
    let content = "pub struct NoContract;\n";
    let file = write_temp_rs(dir.path(), "capabilities_no_contract.rs", content);

    let config = test_config();
    let layer_map = layer_map_with_mandatory_contract();
    let files = FilePathList::new(vec![file]);
    let root = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let mut results = LintResultList::new(Vec::new());

    sut()
        .run_mandatory_imports(&config, &layer_map, &files, &root, &mut results)
        .await;

    for v in &results.values {
        assert_eq!(
            v.severity,
            shared::cli_commands::taxonomy_severity_vo::Severity::HIGH
        );
    }
}

// ─── Non-capabilities File Not Checked ────────────────────

#[tokio::test]
async fn taxonomy_file_not_checked_for_contract_mandatory() {
    let dir = tempfile::tempdir().unwrap();
    let content = "pub struct Foo;\n";
    let file = write_temp_rs(dir.path(), "taxonomy_foo_vo.rs", content);

    let config = test_config();
    let layer_map = layer_map_with_mandatory_contract();
    let files = FilePathList::new(vec![file]);
    let root = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let mut results = LintResultList::new(Vec::new());

    sut()
        .run_mandatory_imports(&config, &layer_map, &files, &root, &mut results)
        .await;

    assert!(
        results.is_empty(),
        "Taxonomy files don't require contract imports"
    );
}
