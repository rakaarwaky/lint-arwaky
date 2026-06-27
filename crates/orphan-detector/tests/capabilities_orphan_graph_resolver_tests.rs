use orphan_detector_lint_arwaky::capabilities_orphan_graph_resolver::OrphanGraphResolver;
use shared::orphan_detector::contract_orphan_graph_resolver_protocol::IOrphanGraphResolverProtocol;
use shared::orphan_detector::taxonomy_orphan_contract_vo::{
    OrphanEntryPatternListVO, OrphanFileListVO,
};

#[test]
fn identify_entry_points_empty_config_auto_detects() {
    let resolver = OrphanGraphResolver::new();
    // When no patterns configured, auto-detects main.rs / lib.rs / _entry / root_
    let files = vec![OrphanFileListVO::new(vec!["src/main.rs".to_string()])];
    let entries = resolver.identify_entry_points(&files, &[]);
    assert!(
        !entries.values.is_empty(),
        "auto-detect should find main.rs as entry point"
    );
    // Non-matching file should NOT be detected
    let files2 = vec![OrphanFileListVO::new(vec![
        "src/taxonomy_foo_vo.rs".to_string()
    ])];
    let entries2 = resolver.identify_entry_points(&files2, &[]);
    assert!(
        entries2.values.is_empty(),
        "taxonomy file should not be auto-detected as entry"
    );
}

#[test]
fn identify_entry_points_matches_ends_with() {
    let resolver = OrphanGraphResolver::new();
    let files = vec![OrphanFileListVO::new(vec![
        "src/root_container.rs".to_string(),
        "src/surface_command.rs".to_string(),
    ])];
    let patterns = vec![OrphanEntryPatternListVO::new(vec![
        "_container.rs".to_string()
    ])];
    let entries = resolver.identify_entry_points(&files, &patterns);
    assert_eq!(entries.values.len(), 1);
    assert!(entries.values[0].ends_with("_container.rs"));
}

#[test]
fn identify_entry_points_matches_contains() {
    let resolver = OrphanGraphResolver::new();
    let files = vec![OrphanFileListVO::new(vec![
        "src/root_entry.rs".to_string(),
        "src/lib.rs".to_string(),
    ])];
    let patterns = vec![OrphanEntryPatternListVO::new(vec!["_entry".to_string()])];
    let entries = resolver.identify_entry_points(&files, &patterns);
    assert_eq!(entries.values.len(), 1);
    assert!(entries.values[0].contains("_entry"));
}

#[test]
fn identify_entry_points_all_config_empty_returns_empty() {
    let resolver = OrphanGraphResolver::new();
    let files = vec![OrphanFileListVO::new(vec!["f.rs".to_string()])];
    let patterns = vec![OrphanEntryPatternListVO::new(vec![])];
    let entries = resolver.identify_entry_points(&files, &patterns);
    assert!(entries.values.is_empty());
}

#[test]
fn identify_entry_points_no_match() {
    let resolver = OrphanGraphResolver::new();
    let files = vec![OrphanFileListVO::new(vec!["src/helper.rs".to_string()])];
    let patterns = vec![OrphanEntryPatternListVO::new(
        vec!["_container".to_string()],
    )];
    let entries = resolver.identify_entry_points(&files, &patterns);
    assert!(entries.values.is_empty());
}

#[test]
fn identify_entry_points_multiple_matches() {
    let resolver = OrphanGraphResolver::new();
    let files = vec![OrphanFileListVO::new(vec![
        "src/root_container.rs".to_string(),
        "src/root_entry.rs".to_string(),
        "src/main.rs".to_string(),
        "src/lib.rs".to_string(),
    ])];
    let patterns = vec![OrphanEntryPatternListVO::new(vec![
        "_container.rs".to_string(),
        "_entry.rs".to_string(),
    ])];
    let entries = resolver.identify_entry_points(&files, &patterns);
    assert_eq!(entries.values.len(), 2);
}

#[test]
fn build_graph_context_with_empty_files() {
    let resolver = OrphanGraphResolver::new();
    let files = vec![OrphanFileListVO::new(vec![])];
    let context = resolver.build_graph_context(&files, ".");
    assert!(context.import_graph.mapping.is_empty());
    assert!(context.inbound_links.mapping.is_empty());
}
