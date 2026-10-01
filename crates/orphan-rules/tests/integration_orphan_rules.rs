// Integration tests — OrphanContainer wiring and analyzer lifecycle.
#[allow(dead_code, unused_imports)]
#[path = "../../shared/tests/common/mock_filesystem.rs"]
mod mock_filesystem;

use mock_filesystem::{mock_filesystem, mock_workspace};
use orphan_rules_lint_arwaky::root_orphan_detector_container::OrphanContainer;
use shared_common::taxonomy_path_vo::FilePath;
use shared_config_system::ArchitectureConfig;
use shared_orphan_rules::{OrphanFileListVO, OrphanRequest};

#[test]
fn container_creates_with_default_config() {
    let fs = mock_filesystem();
    let container = OrphanContainer::new(fs, mock_workspace());
    let analyzer = container.analyzer();
    // analyzer() returns Arc<dyn IOrphanAggregate> — verify it's usable
    let files = OrphanFileListVO::new(vec![]);
    let root = FilePath::new(".".to_string()).unwrap();
    let results = analyzer
        .execute(OrphanRequest::check(&files, &root))
        .into_violations();
    assert!(results.is_empty());
}

#[test]
fn container_creates_with_custom_config() {
    let fs = mock_filesystem();
    let config = ArchitectureConfig::default();
    let container = OrphanContainer::new_with_config(config, fs, mock_workspace());
    let analyzer = container.analyzer();
    let files = OrphanFileListVO::new(vec![]);
    let root = FilePath::new(".".to_string()).unwrap();
    let results = analyzer
        .execute(OrphanRequest::check(&files, &root))
        .into_violations();
    // With empty file list and default config, no orphans
    assert!(results.is_empty());
}

#[test]
fn container_creates_with_ignored_paths() {
    let fs = mock_filesystem();
    let ignored = vec!["target".to_string(), ".git".to_string()];
    let container = OrphanContainer::new_with_ignored(ignored, fs, mock_workspace());
    let analyzer = container.analyzer();
    let files = OrphanFileListVO::new(vec![]);
    let root = FilePath::new(".".to_string()).unwrap();
    let results = analyzer
        .execute(OrphanRequest::check(&files, &root))
        .into_violations();
    assert!(results.is_empty());
}

#[test]
fn analyzer_scan_orphans_on_empty_dir() {
    let fs = mock_filesystem();
    let container = OrphanContainer::new(fs, mock_workspace());
    let analyzer = container.analyzer();
    let root = FilePath::new(".".to_string()).unwrap();
    let (context, results) = analyzer
        .execute(OrphanRequest::scan(
            &root,
            &shared_common::taxonomy_common_vo::PatternList::new(Vec::<String>::new()),
        ))
        .into_scan_outcome();
    // Empty filesystem returns empty results
    assert!(results.is_empty());
    assert!(context.all_workspace_files.is_empty());
}

#[test]
fn analyzer_returns_empty_for_disabled_config() {
    use shared_common::taxonomy_common_vo::BooleanVO;
    let fs = mock_filesystem();
    let config = ArchitectureConfig {
        enabled: BooleanVO::new(false),
        ..Default::default()
    };
    let container = OrphanContainer::new_with_config(config, fs, mock_workspace());
    let analyzer = container.analyzer();

    let files = OrphanFileListVO::new(vec!["src/taxonomy_color.rs".to_string()]);
    let root = FilePath::new(".".to_string()).unwrap();
    let results = analyzer
        .execute(OrphanRequest::check(&files, &root))
        .into_violations();
    // Config disabled → no results
    assert!(results.is_empty());
}

#[test]
fn analyzer_check_orphans_with_context_returns_empty_for_no_files() {
    use shared_filesystem::taxonomy_filesystem_vo::{
        GraphAnalysisContext, ImportGraph, InboundLinkMap, InheritanceMap,
    };
    use std::collections::HashMap;

    let fs = mock_filesystem();
    let container = OrphanContainer::new(fs, mock_workspace());
    let analyzer = container.analyzer();

    let files = OrphanFileListVO::new(vec![]);
    let root = FilePath::new(".".to_string()).unwrap();
    let context = GraphAnalysisContext::new(
        ImportGraph::new(HashMap::new()),
        InboundLinkMap::new(HashMap::new()),
        InheritanceMap::new(HashMap::new()),
        vec![],
    );
    let results = analyzer
        .execute(OrphanRequest::check_with_context(&files, &root, &context))
        .into_violations();
    assert!(results.is_empty());
}
