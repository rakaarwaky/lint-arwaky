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

// ─── Workspace root detection (subfolder scanning bug fix) ─

/// Regression test: When scanning a subfolder (e.g. crates/shared),
/// the graph resolver must detect the workspace root and expand files
/// to include all workspace source files. Without this, cross-crate
/// imports are invisible and false orphan violations are reported.
///
/// Bug: root_dir was set to the subfolder path, so the resolver looked
/// for crates/shared/crates/ instead of crates/. This caused the resolver
/// to miss all cross-crate imports, inflating violation counts.
#[test]
fn build_graph_context_subfolder_expands_to_workspace_files() {
    let r = resolver();
    // Simulate scanning crates/shared (a subfolder of the workspace)
    let subfolder = std::env::current_dir().unwrap().join("crates/shared/src");
    if !subfolder.exists() {
        // Skip if not running in the lint-arwaky workspace
        return;
    }
    let subfolder_str = subfolder.to_string_lossy().to_string();

    // Get a few files from the subfolder
    let files: Vec<String> =
        shared::orphan_detector::utility_orphan_io::scan_directory_recursive(&subfolder)
            .into_iter()
            .filter(|f| f.ends_with(".rs"))
            .take(5)
            .collect();

    if files.is_empty() {
        return;
    }

    let file_vo = OrphanFileListVO::new(files.clone());
    let ctx = r.build_graph_context(&[file_vo], &subfolder_str);

    // The graph should contain files from OTHER crates (cross-crate imports)
    // If workspace root detection fails, only files in the subfolder are in the graph
    let has_cross_crate_files = ctx
        .import_graph
        .mapping
        .keys()
        .any(|k| !k.contains("/shared/"));

    // When scanning a subfolder, the graph must expand to include workspace files
    // This ensures cross-crate imports are visible
    assert!(
        has_cross_crate_files || ctx.import_graph.mapping.len() > files.len(),
        "Graph resolver must expand subfolder scan to include workspace files. \
         Found {} files in graph, expected more than {} input files.",
        ctx.import_graph.mapping.len(),
        files.len()
    );
}

/// Regression test: When scanning a subfolder, the inbound_links map
/// must include links from files in OTHER crates to files in the subfolder.
/// Without this, taxonomy/contract/utility files appear as orphans even
/// though they are imported by files in other crates.
#[test]
fn build_graph_context_subfolder_captures_cross_crate_inbound_links() {
    let r = resolver();
    let subfolder = std::env::current_dir().unwrap().join("crates/shared/src");
    if !subfolder.exists() {
        return;
    }
    let subfolder_str = subfolder.to_string_lossy().to_string();

    // Find a taxonomy file that is known to be imported by other crates
    // (e.g. taxonomy_severity_vo.rs is imported by orphan-detector)
    let taxonomy_file = subfolder.join("common/taxonomy_severity_vo.rs");
    if !taxonomy_file.exists() {
        return;
    }
    let taxonomy_str = taxonomy_file.to_string_lossy().to_string();

    let file_vo = OrphanFileListVO::new(vec![taxonomy_str.clone()]);
    let ctx = r.build_graph_context(&[file_vo], &subfolder_str);

    // Check if inbound_links includes importers from other crates
    if let Some(importers) = ctx.inbound_links.mapping.get(&taxonomy_str) {
        let has_cross_crate_importer = importers.iter().any(|i| !i.contains("/shared/"));
        // The taxonomy file should have importers from other crates
        // (e.g. orphan-detector, code-analysis, etc.)
        assert!(
            has_cross_crate_importer,
            "taxonomy_severity_vo.rs should have cross-crate importers. Found importers: {:?}",
            importers
        );
    }
}
