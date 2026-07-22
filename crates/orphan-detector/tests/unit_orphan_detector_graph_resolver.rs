// PURPOSE: Unit tests for OrphanGraphResolver — graph building and entry point identification.
// Layer: Capabilities (OrphanGraphResolver)
// Speed: ms

use orphan_detector_lint_arwaky::capabilities_orphan_graph_resolver::OrphanGraphResolver;
use shared::orphan_detector::contract_orphan_graph_resolver_protocol::IOrphanGraphResolverProtocol;
use shared::orphan_detector::taxonomy_orphan_contract_vo::{
    OrphanEntryPatternListVO, OrphanFileListVO,
};

fn resolver() -> OrphanGraphResolver {
    OrphanGraphResolver::new()
}

// ─── build_graph_context ──────────────────────────────────

#[test]
fn build_graph_context_empty_files_returns_empty_graph() {
    let r = resolver();
    let files = vec![OrphanFileListVO::new(vec![])];
    let ctx = r.build_graph_context(&files, "/tmp/project");
    assert!(ctx.import_graph.mapping.is_empty());
    assert!(ctx.inbound_links.mapping.is_empty());
}

#[test]
fn build_graph_context_single_file_no_imports() {
    let r = resolver();
    let files = vec![OrphanFileListVO::new(vec![
        "/tmp/project/src/lib.rs".to_string()
    ])];
    let ctx = r.build_graph_context(&files, "/tmp/project");
    // lib.rs should appear in the graph even with no imports
    assert!(ctx
        .import_graph
        .mapping
        .contains_key("/tmp/project/src/lib.rs"));
}

// ─── identify_entry_points ────────────────────────────────

#[test]
fn identify_entry_points_no_configured_patterns_uses_defaults() {
    let r = resolver();
    let files = vec![OrphanFileListVO::new(vec![
        "/tmp/project/src/main.rs".to_string(),
        "/tmp/project/src/lib.rs".to_string(),
        "/tmp/project/src/root_app_container.rs".to_string(),
        "/tmp/project/src/capabilities_foo_analyzer.rs".to_string(),
    ])];
    let result = r.identify_entry_points(&files, &[]);
    // main.rs, lib.rs, and *_container.rs should be entry points
    assert!(result
        .values
        .contains(&"/tmp/project/src/main.rs".to_string()));
    assert!(result
        .values
        .contains(&"/tmp/project/src/lib.rs".to_string()));
    assert!(result
        .values
        .contains(&"/tmp/project/src/root_app_container.rs".to_string()));
    // capabilities file should NOT be an entry point
    assert!(!result
        .values
        .contains(&"/tmp/project/src/capabilities_foo_analyzer.rs".to_string()));
}

#[test]
fn identify_entry_points_with_configured_patterns() {
    let r = resolver();
    let files = vec![OrphanFileListVO::new(vec![
        "/tmp/project/src/main.rs".to_string(),
        "/tmp/project/src/custom_entry.rs".to_string(),
        "/tmp/project/src/other.rs".to_string(),
    ])];
    let configured = vec![OrphanEntryPatternListVO::new(vec![
        "custom_entry.rs".to_string()
    ])];
    let result = r.identify_entry_points(&files, &configured);
    assert!(result
        .values
        .contains(&"/tmp/project/src/custom_entry.rs".to_string()));
    assert!(!result
        .values
        .contains(&"/tmp/project/src/other.rs".to_string()));
}

#[test]
fn identify_entry_points_empty_files_returns_empty() {
    let r = resolver();
    let files = vec![OrphanFileListVO::new(vec![])];
    let result = r.identify_entry_points(&files, &[]);
    assert!(result.values.is_empty());
}

#[test]
fn identify_entry_points_root_prefix_matched() {
    let r = resolver();
    let files = vec![OrphanFileListVO::new(vec![
        "/tmp/project/src/root_orphan_detector_container.rs".to_string(),
    ])];
    let result = r.identify_entry_points(&files, &[]);
    assert!(result
        .values
        .contains(&"/tmp/project/src/root_orphan_detector_container.rs".to_string()));
}

#[test]
fn identify_entry_points_index_files_matched() {
    let r = resolver();
    let files = vec![OrphanFileListVO::new(vec![
        "/tmp/project/src/index.ts".to_string(),
        "/tmp/project/src/index.js".to_string(),
    ])];
    let result = r.identify_entry_points(&files, &[]);
    assert!(result
        .values
        .contains(&"/tmp/project/src/index.ts".to_string()));
    assert!(result
        .values
        .contains(&"/tmp/project/src/index.js".to_string()));
}

// ─── Default trait ────────────────────────────────────────

#[test]
fn default_creates_valid_instance() {
    let r = OrphanGraphResolver::default();
    let files = vec![OrphanFileListVO::new(vec![])];
    let ctx = r.build_graph_context(&files, "/tmp");
    assert!(ctx.import_graph.mapping.is_empty());
}
