// PURPOSE: Integration tests — DI container wiring, full pipeline through OrphanContainer.
// Layer: Integration
// Speed: ms–s

use orphan_detector_lint_arwaky::root_orphan_detector_container::OrphanContainer;

// ─── Container wiring ─────────────────────────────────────

#[test]
fn container_creates_valid_analyzer() {
    let container = OrphanContainer::new();
    let analyzer = container.analyzer();
    // Should be able to call check_orphans without panic
    let results = analyzer.check_orphans(&[], "/tmp/nonexistent");
    assert!(results.is_empty());
}

#[test]
fn container_with_ignored_paths_creates_valid_analyzer() {
    let container =
        OrphanContainer::new_with_ignored(vec!["target".to_string(), "node_modules".to_string()]);
    let analyzer = container.analyzer();
    let results = analyzer.check_orphans(&[], "/tmp/nonexistent");
    assert!(results.is_empty());
}

#[test]
fn container_default_creates_valid_analyzer() {
    let container = OrphanContainer::default();
    let analyzer = container.analyzer();
    let entries = analyzer.identify_orphan_entry_points(&[
        "src/main.rs".to_string(),
        "src/root_app_container.rs".to_string(),
    ]);
    assert!(entries.contains("src/main.rs"));
    assert!(entries.contains("src/root_app_container.rs"));
}

// ─── Graph context through container ──────────────────────

#[test]
fn container_build_graph_context_returns_valid_structure() {
    let container = OrphanContainer::new();
    let analyzer = container.analyzer();
    let ctx = analyzer.build_orphan_graph_context(
        &["crates/orphan-detector/src/lib.rs".to_string()],
        "/tmp/project",
    );
    // lib.rs should be a node in the graph
    assert!(ctx
        .import_graph
        .mapping
        .contains_key("crates/orphan-detector/src/lib.rs"));
}

// ─── Multiple analyzer calls are independent ──────────────

#[test]
fn container_analyzer_is_cloneable_and_independent() {
    let container = OrphanContainer::new();
    let a1 = container.analyzer();
    let a2 = container.analyzer();

    let r1 = a1.check_orphans(&[], "/tmp/a");
    let r2 = a2.check_orphans(&[], "/tmp/b");
    assert_eq!(r1.len(), r2.len());
}
