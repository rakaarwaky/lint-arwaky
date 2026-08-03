// Acceptance tests — AES501: Taxonomy orphan detection.
use orphan_rules_lint_arwaky::capabilities_orphan_taxonomy_analyzer::TaxonomyOrphanAnalyzer;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_severity_vo::Severity;
use shared::orphan_rules::ITaxonomyOrphanProtocol;
use shared::quality_rules::taxonomy_analysis_vo::InboundLinkMap;
use std::collections::HashMap;

fn taxonomy_analyzer() -> TaxonomyOrphanAnalyzer {
    TaxonomyOrphanAnalyzer::new()
}

#[test]
fn aes501_taxonomy_file_not_imported_is_orphan() {
    let analyzer = taxonomy_analyzer();
    let fp = FilePath::new("crates/shared/src/taxonomy_color.rs".to_string()).unwrap();
    let root = FilePath::new(".".to_string()).unwrap();
    let empty_map = InboundLinkMap::new(HashMap::new());

    let result = analyzer.is_taxonomy_orphan(&fp, &root, None, &empty_map);
    assert!(
        result.is_orphan,
        "taxonomy_color.rs with no importers should be orphan"
    );
    assert_eq!(result.severity, Severity::LOW);
    assert!(
        result.reason.contains("taxonomy_color"),
        "Reason should mention the file stem"
    );
}

#[test]
fn aes501_taxonomy_file_imported_by_capabilities_is_not_orphan() {
    let analyzer = taxonomy_analyzer();
    let target = "crates/shared/src/taxonomy_color.rs";
    let fp = FilePath::new(target.to_string()).unwrap();
    let root = FilePath::new(".".to_string()).unwrap();

    let mut mapping = HashMap::new();
    mapping.insert(
        target.to_string(),
        vec!["crates/orphan-rules/src/capabilities_orphan_taxonomy_analyzer.rs".to_string()],
    );
    let inbound = InboundLinkMap::new(mapping);

    let result = analyzer.is_taxonomy_orphan(&fp, &root, None, &inbound);
    assert!(
        !result.is_orphan,
        "taxonomy_color.rs imported by capabilities should NOT be orphan"
    );
}

#[test]
fn aes501_taxonomy_file_imported_by_agent_is_not_orphan() {
    let analyzer = taxonomy_analyzer();
    let target = "crates/shared/src/taxonomy_config_vo.rs";
    let fp = FilePath::new(target.to_string()).unwrap();
    let root = FilePath::new(".".to_string()).unwrap();

    let mut mapping = HashMap::new();
    mapping.insert(
        target.to_string(),
        vec!["crates/orphan-rules/src/agent_orphan_orchestrator.rs".to_string()],
    );
    let inbound = InboundLinkMap::new(mapping);

    let result = analyzer.is_taxonomy_orphan(&fp, &root, None, &inbound);
    assert!(!result.is_orphan);
}

#[test]
fn aes501_taxonomy_file_only_imported_by_other_taxonomy_is_orphan() {
    let analyzer = taxonomy_analyzer();
    let target = "crates/shared/src/taxonomy_size.rs";
    let fp = FilePath::new(target.to_string()).unwrap();
    let root = FilePath::new(".".to_string()).unwrap();

    let mut mapping = HashMap::new();
    mapping.insert(
        target.to_string(),
        vec!["crates/shared/src/taxonomy_color.rs".to_string()],
    );
    let inbound = InboundLinkMap::new(mapping);

    let result = analyzer.is_taxonomy_orphan(&fp, &root, None, &inbound);
    assert!(
        result.is_orphan,
        "taxonomy file only imported by taxonomy should be orphan"
    );
    assert!(result.reason.contains("taxonomy_size"));
}

#[test]
fn aes501_taxonomy_helper_suffix_categorized_as_utility() {
    let analyzer = taxonomy_analyzer();
    let fp = FilePath::new("crates/shared/src/taxonomy_helper.rs".to_string()).unwrap();
    let root = FilePath::new(".".to_string()).unwrap();
    let empty_map = InboundLinkMap::new(HashMap::new());

    let result = analyzer.is_taxonomy_orphan(&fp, &root, None, &empty_map);
    assert!(result.is_orphan);
    // The reason should still mention the stem
    assert!(result.reason.contains("taxonomy_helper"));
}

#[test]
fn aes501_taxonomy_utility_suffix_also_flagged() {
    let analyzer = taxonomy_analyzer();
    let fp = FilePath::new("crates/shared/src/taxonomy_utility.rs".to_string()).unwrap();
    let root = FilePath::new(".".to_string()).unwrap();
    let empty_map = InboundLinkMap::new(HashMap::new());

    let result = analyzer.is_taxonomy_orphan(&fp, &root, None, &empty_map);
    assert!(result.is_orphan);
}

#[test]
fn aes501_taxonomy_file_imported_by_surface_is_not_orphan() {
    let analyzer = taxonomy_analyzer();
    let target = "crates/shared/src/taxonomy_entry_vo.rs";
    let fp = FilePath::new(target.to_string()).unwrap();
    let root = FilePath::new(".".to_string()).unwrap();

    let mut mapping = HashMap::new();
    mapping.insert(
        target.to_string(),
        vec!["crates/tui/src/surface_main_screen.rs".to_string()],
    );
    let inbound = InboundLinkMap::new(mapping);

    let result = analyzer.is_taxonomy_orphan(&fp, &root, None, &inbound);
    assert!(
        !result.is_orphan,
        "taxonomy file imported by surface layer should NOT be orphan"
    );
}

#[test]
fn aes501_taxonomy_file_with_self_import_only_is_orphan() {
    let analyzer = taxonomy_analyzer();
    let target = "crates/shared/src/taxonomy_self_ref.rs";
    let fp = FilePath::new(target.to_string()).unwrap();
    let root = FilePath::new(".".to_string()).unwrap();

    // Self-references should not count as "imported by another layer"
    let mut mapping = HashMap::new();
    mapping.insert(target.to_string(), vec![target.to_string()]);
    let inbound = InboundLinkMap::new(mapping);

    let result = analyzer.is_taxonomy_orphan(&fp, &root, None, &inbound);
    assert!(result.is_orphan, "Self-import should still be orphan");
}
