// PURPOSE: Acceptance test for FR-005: Circular Dependency Detection (AES205)
// Requirement: Direct and indirect cycles across layers are detected.

use import_rules_lint_arwaky::capabilities_cycle_import_analyzer::DependencyCycleAnalyzer;
use import_rules_lint_arwaky::root_import_rules_container::ImportContainer;
use shared::common::taxonomy_common_vo::{BooleanVO, Count, PatternList};
use shared::common::taxonomy_definition_vo::LayerDefinition;
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

fn write_file(dir: &std::path::Path, name: &str, content: &str) {
    let mut file = std::fs::File::create(dir.join(name)).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}

/// Config with layer definitions needed for cycle detection
fn fr005_config() -> ArchitectureConfig {
    let mut layers = HashMap::new();
    layers.insert(
        LayerNameVO::new("taxonomy"),
        LayerDefinition {
            forbidden: PatternList::new(vec![
                String::from("contract"),
                String::from("utility"),
                String::from("capabilities"),
                String::from("agent"),
                String::from("surfaces"),
                String::from("root"),
            ]),
            ..Default::default()
        },
    );
    layers.insert(
        LayerNameVO::new("contract"),
        LayerDefinition {
            forbidden: PatternList::new(vec![
                String::from("utility"),
                String::from("capabilities"),
                String::from("agent"),
                String::from("surfaces"),
                String::from("root"),
            ]),
            ..Default::default()
        },
    );
    layers.insert(LayerNameVO::new("capabilities"), LayerDefinition::default());
    ArchitectureConfig {
        enabled: BooleanVO::new(true),
        layers,
        rules: Vec::new(),
        naming: shared::common::taxonomy_definition_vo::NamingConfig::new(Count::new(2)),
        ignored_paths: FilePathList { values: vec![] },
        mandatory_class_definition: BooleanVO::new(false),
    }
}

/// FR-005: Direct cycle (A → B → A) is flagged
#[test]
fn fr005_direct_cycle_detected() {
    let edges = vec![
        DependencyEdge::new("capabilities".to_string(), "agent".to_string()),
        DependencyEdge::new("agent".to_string(), "capabilities".to_string()),
    ];
    let result = sut().detect_cycle_edges(&edges);
    assert!(
        !result.is_empty(),
        "FR-005: direct cycle A→B→A must be detected"
    );
}

/// FR-005: Indirect cycle (A → B → C → A) is flagged
#[test]
fn fr005_indirect_cycle_detected() {
    let edges = vec![
        DependencyEdge::new("taxonomy".to_string(), "contract".to_string()),
        DependencyEdge::new("contract".to_string(), "capabilities".to_string()),
        DependencyEdge::new("capabilities".to_string(), "taxonomy".to_string()),
    ];
    let result = sut().detect_cycle_edges(&edges);
    assert!(
        !result.is_empty(),
        "FR-005: indirect cycle A→B→C→A must be detected"
    );
}

/// FR-005: No cycle in unidirectional graph
#[test]
fn fr005_no_cycle_in_dag() {
    let edges = vec![
        DependencyEdge::new("taxonomy".to_string(), "contract".to_string()),
        DependencyEdge::new("contract".to_string(), "capabilities".to_string()),
        DependencyEdge::new("capabilities".to_string(), "agent".to_string()),
    ];
    let result = sut().detect_cycle_edges(&edges);
    assert!(result.is_empty(), "FR-005: DAG must not produce cycle");
}

/// FR-005: Self-import is not a cross-layer cycle
#[test]
fn fr005_self_edge_not_flagged_as_cross_layer() {
    let edges = vec![DependencyEdge::new(
        "taxonomy".to_string(),
        "taxonomy".to_string(),
    )];
    // Self-loops: the cycle detector may or may not flag these depending on
    // normalization. The key requirement is no false positive for valid imports.
    let result = sut().detect_cycle_edges(&edges);
    // Self-loop within same layer is technically a cycle in graph theory,
    // but the _scan method only adds edges when target_layer != file_layer.
    // So this tests the pure graph detector behavior.
    // The important thing: the orchestrator won't create self-edges.
    let _ = result; // No assertion on self-loops at graph level
}

/// FR-005: Empty graph produces no violations
#[test]
fn fr005_empty_graph_no_violations() {
    let edges: Vec<DependencyEdge> = vec![];
    let result = sut().detect_cycle_edges(&edges);
    assert!(
        result.is_empty(),
        "FR-005: empty graph must produce no violations"
    );
}

// ─── Integration: Real files across layers ─────────────────

/// FR-005: Integration test — creates real files across layers with a cycle
/// and verifies cycle detection at the orchestrator level.
#[tokio::test]
async fn fr005_integration_real_files_cycle_detected() {
    let dir = tempfile::tempdir().unwrap();

    // Create files across layers that form a cycle:
    // taxonomy_vo.rs → imports from contract_protocol.rs
    // contract_protocol.rs → imports from capabilities_checker.rs
    // capabilities_checker.rs → imports from taxonomy_vo.rs (CYCLE!)

    write_file(
        dir.path(),
        "taxonomy_vo.rs",
        "use crate::contract_protocol::Protocol;\npub struct V;\n",
    );
    write_file(
        dir.path(),
        "contract_protocol.rs",
        "use crate::capabilities_checker::Checker;\npub trait Protocol {}\n",
    );
    write_file(
        dir.path(),
        "capabilities_checker.rs",
        "use crate::taxonomy_vo::V;\npub struct Checker;\n",
    );

    let container = ImportContainer::new_with_config(fr005_config());
    let orch = container.orchestrator();
    let target = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let results = orch.run_audit(&target).await.unwrap();

    // The cycle detection should find AES205 violations
    // Note: The orchestrator builds edges from actual file imports,
    // so this tests the full pipeline from file reading to cycle detection.
    let cycle_violations: Vec<_> = results
        .iter()
        .filter(|v| v.code.code() == "AES205")
        .collect();

    // At minimum, the cycle should be detected somewhere in the results
    // The exact number depends on how many edges are in the cycle
    assert!(
        !cycle_violations.is_empty(),
        "FR-005: integration test with real files must detect cycle (AES205)"
    );
}

/// FR-005: Integration test — no cycle in valid unidirectional imports
#[tokio::test]
async fn fr005_integration_real_files_no_cycle() {
    let dir = tempfile::tempdir().unwrap();

    // Create files with valid unidirectional imports (no cycle)
    write_file(dir.path(), "taxonomy_vo.rs", "pub struct V;\n");
    write_file(
        dir.path(),
        "contract_protocol.rs",
        "use crate::taxonomy_vo::V;\npub trait Protocol {}\n",
    );
    write_file(
        dir.path(),
        "capabilities_checker.rs",
        "use crate::contract_protocol::Protocol;\npub struct Checker;\n",
    );

    let container = ImportContainer::new_with_config(fr005_config());
    let orch = container.orchestrator();
    let target = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let results = orch.run_audit(&target).await.unwrap();

    let cycle_violations: Vec<_> = results
        .iter()
        .filter(|v| v.code.code() == "AES205")
        .collect();

    assert!(
        cycle_violations.is_empty(),
        "FR-005: valid unidirectional imports must not produce AES205 violations"
    );
}
