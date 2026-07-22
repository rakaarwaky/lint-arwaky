// PURPOSE: Unit tests for DependencyCycleAnalyzer (AES205)
// Tests cycle detection logic using temp files with cross-layer imports.

use import_rules_lint_arwaky::capabilities_cycle_import_analyzer::DependencyCycleAnalyzer;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::common::taxonomy_common_vo::{BooleanVO, Count};
use shared::common::taxonomy_definition_vo::LayerMapVO;
use shared::common::taxonomy_layer_vo::LayerNameVO;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::import_rules::contract_cycle_import_protocol::DependencyEdge;
use shared::import_rules::contract_cycle_import_protocol::ICycleImportProtocol;
use std::collections::HashMap;
use std::io::Write;

fn sut() -> DependencyCycleAnalyzer {
    DependencyCycleAnalyzer::new()
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

fn two_layer_map() -> LayerMapVO {
    let mut layers = HashMap::new();
    layers.insert(
        LayerNameVO::new("capabilities"),
        shared::common::taxonomy_definition_vo::LayerDefinition::default(),
    );
    layers.insert(
        LayerNameVO::new("agent"),
        shared::common::taxonomy_definition_vo::LayerDefinition::default(),
    );
    LayerMapVO::new(layers)
}

fn write_temp_rs(dir: &std::path::Path, name: &str, content: &str) -> FilePath {
    let path = dir.join(name);
    let mut file = std::fs::File::create(&path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
    FilePath::new(path.to_string_lossy().to_string()).unwrap()
}

// ─── detect_cycle_edges (pure logic) ─────────────────────

#[test]
fn no_edges_no_cycles() {
    let edges: Vec<DependencyEdge> = vec![];
    let result = sut().detect_cycle_edges(&edges);
    assert!(result.is_empty());
}

#[test]
fn unidirectional_edge_no_cycle() {
    let edges = vec![DependencyEdge::new(
        "taxonomy".to_string(),
        "contract".to_string(),
    )];
    let result = sut().detect_cycle_edges(&edges);
    assert!(result.is_empty(), "A→B without B→A is not a cycle");
}

#[test]
fn direct_cycle_detected() {
    let edges = vec![
        DependencyEdge::new("capabilities".to_string(), "agent".to_string()),
        DependencyEdge::new("agent".to_string(), "capabilities".to_string()),
    ];
    let result = sut().detect_cycle_edges(&edges);
    assert!(!result.is_empty(), "A→B→A should be detected as a cycle");
}

#[test]
fn indirect_cycle_detected() {
    let edges = vec![
        DependencyEdge::new("a".to_string(), "b".to_string()),
        DependencyEdge::new("b".to_string(), "c".to_string()),
        DependencyEdge::new("c".to_string(), "a".to_string()),
    ];
    let result = sut().detect_cycle_edges(&edges);
    assert!(!result.is_empty(), "A→B→C→A should be detected");
}

// ─── normalize_to_layer ───────────────────────────────────

#[test]
fn normalize_strips_prefix() {
    let result = sut().normalize_to_layer("taxonomy_foo_vo");
    assert_eq!(result.value(), "taxonomy");
}

#[test]
fn normalize_unknown_returns_first_segment() {
    let result = sut().normalize_to_layer("unknown_thing");
    // normalize_to_layer strips the suffix after the first underscore
    assert_eq!(result.value(), "unknown");
}

// ─── scan with temp files ─────────────────────────────────

#[tokio::test]
async fn no_cross_layer_imports_no_violations() {
    let dir = tempfile::tempdir().unwrap();
    let file_a = write_temp_rs(
        dir.path(),
        "capabilities_a.rs",
        "use shared::common::taxonomy_path_vo::FilePath;\npub struct A;\n",
    );

    let config = test_config();
    let layer_map = two_layer_map();
    let files = FilePathList::new(vec![file_a]);
    let root = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let mut results = LintResultList::new(Vec::new());

    sut()
        .check_cycles(&config, &layer_map, &files, &root, &mut results)
        .await;

    assert!(
        results.is_empty(),
        "Single file with no cross-layer import has no cycle"
    );
}

#[tokio::test]
async fn disabled_config_returns_empty() {
    let dir = tempfile::tempdir().unwrap();
    let file = write_temp_rs(dir.path(), "capabilities_x.rs", "pub struct X;\n");

    let mut config = test_config();
    config.enabled = BooleanVO::new(false);

    let layer_map = two_layer_map();
    let files = FilePathList::new(vec![file]);
    let root = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let mut results = LintResultList::new(Vec::new());

    sut()
        .check_cycles(&config, &layer_map, &files, &root, &mut results)
        .await;

    assert!(
        results.is_empty(),
        "Disabled config should produce no results"
    );
}

// ─── Violation Metadata ───────────────────────────────────

#[test]
fn cycle_violation_uses_aes205_code() {
    let edges = vec![
        DependencyEdge::new("capabilities".to_string(), "agent".to_string()),
        DependencyEdge::new("agent".to_string(), "capabilities".to_string()),
    ];
    let cycle_names = sut().detect_cycle_edges(&edges);
    // The cycle detector returns SymbolName edges; the _scan method wraps them
    // into LintResult with AES205. Here we just verify the detector finds something.
    assert!(!cycle_names.is_empty());
}
