use orphan_detector_lint_arwaky::root_orphan_detector_container::OrphanContainer;

#[test]
fn orphan_container_constructs() {
    let container = OrphanContainer::new();
    let analyzer = container.analyzer();
    let _layer_detector = container.layer_detector();
    assert!(analyzer
        .build_orphan_graph_context(&[], ".")
        .import_graph
        .mapping
        .is_empty());
}

#[test]
fn orphan_container_default() {
    let container = OrphanContainer::default();
    let analyzer = container.analyzer();
    // Should not panic
    let _ = analyzer.check_orphans(&*container.layer_detector(), &[], ".");
}

#[test]
fn orphan_container_detect_layer() {
    let container = OrphanContainer::new();
    let detector = container.layer_detector();
    assert_eq!(
        detector.detect_layer("src/taxonomy_config_vo.rs", "."),
        Some("taxonomy".to_string())
    );
    assert_eq!(
        detector.detect_layer("src/capabilities_checker.rs", "."),
        Some("capabilities".to_string())
    );
    assert_eq!(detector.detect_layer("src/random.rs", "."), None);
}

#[test]
fn orphan_container_get_orphan_entry_points() {
    let container = OrphanContainer::new();
    let detector = container.layer_detector();
    let entries = detector.get_orphan_entry_points();
    assert!(entries.contains(&"_container.rs".to_string()));
    assert!(entries.contains(&"main.rs".to_string()));
    assert!(entries.contains(&"lib.rs".to_string()));
    assert!(entries.contains(&"index.ts".to_string()));
}
