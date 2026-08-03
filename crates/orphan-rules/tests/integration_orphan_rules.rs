// Integration tests — OrphanContainer wiring and analyzer lifecycle.
#[path = "mock_filesystem.rs"]
mod mock_filesystem;

use mock_filesystem::mock_filesystem;
use orphan_rules_lint_arwaky::root_orphan_detector_container::OrphanContainer;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::ArchitectureConfig;
use shared::orphan_rules::OrphanFileListVO;

#[test]
fn container_creates_with_default_config() {
    let fs = mock_filesystem();
    let container = OrphanContainer::new(fs);
    let analyzer = container.analyzer();
    // analyzer() returns Arc<dyn IOrphanAggregate> — verify it's usable
    let files = OrphanFileListVO::new(vec![]);
    let root = FilePath::new(".".to_string()).unwrap();
    let _results = analyzer.check_orphans(&files, &root);
}

#[test]
fn container_creates_with_custom_config() {
    let fs = mock_filesystem();
    let config = ArchitectureConfig::default();
    let container = OrphanContainer::new_with_config(config, fs);
    let analyzer = container.analyzer();
    let files = OrphanFileListVO::new(vec![]);
    let root = FilePath::new(".".to_string()).unwrap();
    let results = analyzer.check_orphans(&files, &root);
    // With empty file list and default config, no orphans
    assert!(results.is_empty());
}

#[test]
fn container_creates_with_ignored_paths() {
    let fs = mock_filesystem();
    let ignored = vec!["target".to_string(), ".git".to_string()];
    let container = OrphanContainer::new_with_ignored(ignored, fs);
    let analyzer = container.analyzer();
    let files = OrphanFileListVO::new(vec![]);
    let root = FilePath::new(".".to_string()).unwrap();
    let results = analyzer.check_orphans(&files, &root);
    assert!(results.is_empty());
}

#[test]
fn analyzer_scan_orphans_on_empty_dir() {
    let fs = mock_filesystem();
    let container = OrphanContainer::new(fs);
    let analyzer = container.analyzer();
    let root = FilePath::new(".".to_string()).unwrap();
    let (_context, results) = analyzer.scan_orphans(&root, &[]);
    // Empty filesystem returns empty results
    assert!(results.is_empty());
}

#[test]
fn analyzer_returns_empty_for_disabled_config() {
    use shared::common::taxonomy_common_vo::BooleanVO;
    let fs = mock_filesystem();
    let config = ArchitectureConfig {
        enabled: BooleanVO::new(false),
        ..Default::default()
    };
    let container = OrphanContainer::new_with_config(config, fs);
    let analyzer = container.analyzer();

    let files = OrphanFileListVO::new(vec!["src/taxonomy_color.rs".to_string()]);
    let root = FilePath::new(".".to_string()).unwrap();
    let results = analyzer.check_orphans(&files, &root);
    // Config disabled → no results
    assert!(results.is_empty());
}

#[test]
fn analyzer_check_orphans_with_context_returns_empty_for_no_files() {
    use shared::filesystem::taxonomy_filesystem_vo::{
        GraphAnalysisContext, ImportGraph, InboundLinkMap, InheritanceMap,
    };
    use std::collections::HashMap;

    let fs = mock_filesystem();
    let container = OrphanContainer::new(fs);
    let analyzer = container.analyzer();

    let files = OrphanFileListVO::new(vec![]);
    let root = FilePath::new(".".to_string()).unwrap();
    let context = GraphAnalysisContext::new(
        ImportGraph::new(HashMap::new()),
        InboundLinkMap::new(HashMap::new()),
        InheritanceMap::new(HashMap::new()),
        vec![],
    );
    let results = analyzer.check_orphans_with_context(&files, &root, &context);
    assert!(results.is_empty());
}
