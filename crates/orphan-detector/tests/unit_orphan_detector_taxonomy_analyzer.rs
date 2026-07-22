// PURPOSE: Unit tests for TaxonomyOrphanAnalyzer — AES501 taxonomy orphan detection.
// Layer: Capabilities (TaxonomyOrphanAnalyzer)
// Speed: ms

use orphan_detector_lint_arwaky::capabilities_orphan_taxonomy_analyzer::TaxonomyOrphanAnalyzer;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::taxonomy_analysis_vo::InboundLinkMap;
use shared::common::taxonomy_path_vo::FilePath;
use shared::orphan_detector::contract_orphan_protocol::ITaxonomyOrphanProtocol;
use std::collections::HashMap;

fn analyzer() -> TaxonomyOrphanAnalyzer {
    TaxonomyOrphanAnalyzer::new()
}

fn make_inbound_links(links: Vec<(&str, Vec<&str>)>) -> InboundLinkMap {
    let mut mapping = HashMap::new();
    for (file, importers) in links {
        mapping.insert(
            file.to_string(),
            importers.iter().map(|s| s.to_string()).collect(),
        );
    }
    InboundLinkMap::new(mapping)
}

// ─── Happy path: taxonomy imported by contract ────────────

#[test]
fn taxonomy_vo_imported_by_contract_is_not_orphan() {
    let a = analyzer();
    let f = FilePath::new("crates/shared/src/common/taxonomy_path_vo.rs".to_string()).unwrap();
    let root = FilePath::new("crates/shared".to_string()).unwrap();
    let inbound = make_inbound_links(vec![(
        "crates/shared/src/common/taxonomy_path_vo.rs",
        vec!["crates/shared/src/orphan-detector/contract_orphan_protocol.rs"],
    )]);
    let result = a.is_taxonomy_orphan(&f, &root, None, &inbound);
    assert!(!result.is_orphan);
}

// ─── Orphan: no inbound links ─────────────────────────────

#[test]
fn taxonomy_vo_with_no_importers_is_orphan() {
    let a = analyzer();
    let f = FilePath::new("crates/shared/src/common/taxonomy_orphan_vo.rs".to_string()).unwrap();
    let root = FilePath::new("crates/shared".to_string()).unwrap();
    let inbound = make_inbound_links(vec![]);
    let result = a.is_taxonomy_orphan(&f, &root, None, &inbound);
    assert!(result.is_orphan);
    assert_eq!(result.severity, Severity::LOW);
    assert!(result.reason.contains("taxonomy_orphan_vo"));
}

// ─── Orphan: only imported by other taxonomy files ────────

#[test]
fn taxonomy_vo_imported_only_by_taxonomy_is_orphan() {
    let a = analyzer();
    let f = FilePath::new("crates/shared/src/common/taxonomy_foo_vo.rs".to_string()).unwrap();
    let root = FilePath::new("crates/shared".to_string()).unwrap();
    let inbound = make_inbound_links(vec![(
        "crates/shared/src/common/taxonomy_foo_vo.rs",
        vec!["crates/shared/src/common/taxonomy_bar_vo.rs"],
    )]);
    let result = a.is_taxonomy_orphan(&f, &root, None, &inbound);
    assert!(result.is_orphan);
}

// ─── Not orphan: imported by capabilities ─────────────────

#[test]
fn taxonomy_vo_imported_by_capabilities_is_not_orphan() {
    let a = analyzer();
    let f = FilePath::new("crates/shared/src/common/taxonomy_severity_vo.rs".to_string()).unwrap();
    let root = FilePath::new("crates/shared".to_string()).unwrap();
    let inbound = make_inbound_links(vec![(
        "crates/shared/src/common/taxonomy_severity_vo.rs",
        vec!["crates/orphan-detector/src/capabilities_orphan_agent_analyzer.rs"],
    )]);
    let result = a.is_taxonomy_orphan(&f, &root, None, &inbound);
    assert!(!result.is_orphan);
}

// ─── Default trait ────────────────────────────────────────

#[test]
fn default_creates_valid_instance() {
    let a = TaxonomyOrphanAnalyzer::default();
    let f = FilePath::new("taxonomy_test_vo.rs".to_string()).unwrap();
    let root = FilePath::new("/tmp".to_string()).unwrap();
    let inbound = make_inbound_links(vec![]);
    let result = a.is_taxonomy_orphan(&f, &root, None, &inbound);
    // No importers → orphan
    assert!(result.is_orphan);
}
