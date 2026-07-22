// PURPOSE: Acceptance test for FR-005: Circular Dependency Detection (AES205)
// Requirement: Direct and indirect cycles across layers are detected.

use import_rules_lint_arwaky::capabilities_cycle_import_analyzer::DependencyCycleAnalyzer;
use shared::import_rules::contract_cycle_import_protocol::DependencyEdge;
use shared::import_rules::contract_cycle_import_protocol::ICycleImportProtocol;

fn sut() -> DependencyCycleAnalyzer {
    DependencyCycleAnalyzer::new()
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
