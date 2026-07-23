// PURPOSE: Integration tests — DI container wiring, full pipeline through OrphanContainer.
// Layer: Integration
// Speed: ms–s

use orphan_detector_lint_arwaky::root_orphan_detector_container::OrphanContainer;
use shared::common::taxonomy_path_vo::FilePath;
use shared::orphan_detector::taxonomy_orphan_contract_vo::OrphanFileListVO;

// ─── Container wiring ─────────────────────────────────────

#[test]
fn container_creates_valid_analyzer() {
    let container = OrphanContainer::new();
    let analyzer = container.analyzer();
    let files = OrphanFileListVO::new(vec![]);
    let root = FilePath::new("/tmp/nonexistent".to_string()).unwrap();
    let results = analyzer.check_orphans(&files, &root);
    assert!(results.is_empty());
}

#[test]
fn container_with_ignored_paths_creates_valid_analyzer() {
    let container =
        OrphanContainer::new_with_ignored(vec!["target".to_string(), "node_modules".to_string()]);
    let analyzer = container.analyzer();
    let files = OrphanFileListVO::new(vec![]);
    let root = FilePath::new("/tmp/nonexistent".to_string()).unwrap();
    let results = analyzer.check_orphans(&files, &root);
    assert!(results.is_empty());
}

#[test]
fn container_default_creates_valid_analyzer() {
    let container = OrphanContainer::default();
    let analyzer = container.analyzer();
    let files = OrphanFileListVO::new(vec![
        "src/main.rs".to_string(),
        "src/root_app_container.rs".to_string(),
    ]);
    let entries = analyzer.identify_orphan_entry_points(&files);
    assert!(entries.values.contains(&"src/main.rs".to_string()));
    assert!(entries
        .values
        .contains(&"src/root_app_container.rs".to_string()));
}

// ─── Graph context through container ──────────────────────

#[test]
fn container_build_graph_context_returns_valid_structure() {
    let container = OrphanContainer::new();
    let analyzer = container.analyzer();
    let files = OrphanFileListVO::new(vec!["crates/orphan-detector/src/lib.rs".to_string()]);
    let root = FilePath::new("/tmp/project".to_string()).unwrap();
    let ctx = analyzer.build_orphan_graph_context(&files, &root);
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

    let files = OrphanFileListVO::new(vec![]);
    let root_a = FilePath::new("/tmp/a".to_string()).unwrap();
    let root_b = FilePath::new("/tmp/b".to_string()).unwrap();
    let r1 = a1.check_orphans(&files, &root_a);
    let r2 = a2.check_orphans(&files, &root_b);
    assert_eq!(r1.len(), r2.len());
}
