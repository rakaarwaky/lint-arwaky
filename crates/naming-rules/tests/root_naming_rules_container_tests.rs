use naming_rules_lint_arwaky::root_naming_rules_container::NamingContainer;

#[test]
fn naming_container_constructs() {
    let container = NamingContainer::new_default();
    let _ = container.naming_convention_checker();
    let _ = container.suffix_prefix_checker();
    let _ = container.analyzer();
}

#[test]
fn naming_container_orchestrator_creates() {
    let container = NamingContainer::new_default();
    let orchestrator = container.orchestrator();
    assert_eq!(orchestrator.name(), "naming-rules");
}

#[test]
fn naming_container_analyzer_detect_layer() {
    let container = NamingContainer::new_default();
    let analyzer = container.analyzer();
    // Default analyzer returns None for all paths
    use shared::common::taxonomy_path_vo::FilePath;
    let fp = FilePath::new("test.rs").unwrap_or_default();
    let root = FilePath::new(".").unwrap_or_default();
    assert!(analyzer.detect_layer(&fp, &root).is_none());
    assert!(analyzer.config().ignored_paths.values.is_empty());
}
