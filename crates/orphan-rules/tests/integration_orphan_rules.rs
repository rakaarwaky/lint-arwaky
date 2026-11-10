// Integration tests — OrphanContainer wiring and analyzer lifecycle.
#[allow(dead_code, unused_imports)]
#[path = "../../shared/tests/mock_filesystem.rs"]
mod mock_filesystem;

use mock_filesystem::{mock_filesystem, mock_workspace};
use orphan_rules_lint_arwaky::root_orphan_detector_container::OrphanContainer;
use shared_common::taxonomy_path_vo::FilePath;
use shared_common::taxonomy_severity_vo::Severity;
use shared_config_system::ArchitectureConfig;
use shared_orphan_rules::{OrphanFileListVO, OrphanRequest};

#[test]
fn container_creates_with_default_config() {
    let fs = mock_filesystem();
    let container = OrphanContainer::new(fs, mock_workspace());
    let analyzer = container.analyzer();
    let files = OrphanFileListVO::new(vec![]);
    let root = FilePath::new(".".to_string()).unwrap();
    let results = analyzer
        .execute(OrphanRequest::check(&files, &root))
        .into_violations();
    // No entry points → single NO_ENTRY_POINT finding (UX-S1-001).
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].code.code(), "AES501");
    assert_eq!(results[0].violation_name, "NO_ENTRY_POINT");
    assert_eq!(results[0].severity, Severity::HIGH);
    assert!(results[0].message.value().contains("No entry point found"));
    assert!(results[0].enclosing_scope.is_none());
    assert!(!results[0].why.is_empty());
    assert!(!results[0].fix.is_empty());
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
    // Empty file list and no configured entry points → NO_ENTRY_POINT finding.
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].code.code(), "AES501");
    assert_eq!(results[0].violation_name, "NO_ENTRY_POINT");
    assert_eq!(results[0].severity, Severity::HIGH);
    assert!(results[0].message.value().contains("No entry point found"));
    assert!(results[0].enclosing_scope.is_none());
    assert!(!results[0].why.is_empty());
    assert!(!results[0].fix.is_empty());
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
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].code.code(), "AES501");
    assert_eq!(results[0].violation_name, "NO_ENTRY_POINT");
    assert_eq!(results[0].severity, Severity::HIGH);
    assert!(results[0].message.value().contains("No entry point found"));
    assert!(results[0].enclosing_scope.is_none());
    assert!(!results[0].why.is_empty());
    assert!(!results[0].fix.is_empty());
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
    // Empty filesystem → no entry points → NO_ENTRY_POINT finding (UX-S1-001).
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].code.code(), "AES501");
    assert_eq!(results[0].violation_name, "NO_ENTRY_POINT");
    assert_eq!(results[0].severity, Severity::HIGH);
    assert!(results[0].message.value().contains("No entry point found"));
    assert!(results[0].enclosing_scope.is_none());
    assert!(!results[0].why.is_empty());
    assert!(!results[0].fix.is_empty());
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
    // Empty entry points → NO_ENTRY_POINT finding (UX-S1-001).
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].code.code(), "AES501");
    assert_eq!(results[0].violation_name, "NO_ENTRY_POINT");
    assert_eq!(results[0].severity, Severity::HIGH);
    assert!(results[0].message.value().contains("No entry point found"));
    assert!(results[0].enclosing_scope.is_none());
    assert!(!results[0].why.is_empty());
    assert!(!results[0].fix.is_empty());
}
