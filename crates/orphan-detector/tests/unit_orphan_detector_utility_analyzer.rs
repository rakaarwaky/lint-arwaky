// PURPOSE: Unit tests for UtilityOrphanAnalyzer — AES504 utility orphan detection.
// Layer: Capabilities (UtilityOrphanAnalyzer)
// Speed: ms

use orphan_detector_lint_arwaky::capabilities_orphan_utility_analyzer::UtilityOrphanAnalyzer;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::taxonomy_analysis_vo::InboundLinkMap;
use shared::common::taxonomy_path_vo::FilePath;
use shared::orphan_detector::contract_orphan_protocol::IUtilityOrphanProtocol;
use std::collections::HashMap;

fn analyzer() -> UtilityOrphanAnalyzer {
    UtilityOrphanAnalyzer::new()
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

// ─── Happy path: utility imported by capabilities ─────────

#[test]
fn utility_imported_by_capabilities_is_not_orphan() {
    let a = analyzer();
    let f =
        FilePath::new("crates/shared/src/orphan-detector/utility_orphan.rs".to_string()).unwrap();
    let root = FilePath::new("crates/shared".to_string()).unwrap();
    let all_files = vec![
        "crates/shared/src/orphan-detector/utility_orphan.rs".to_string(),
        "crates/orphan-detector/src/capabilities_orphan_capabilities_analyzer.rs".to_string(),
    ];
    let inbound = make_inbound_links(vec![(
        "crates/shared/src/orphan-detector/utility_orphan.rs",
        vec!["crates/orphan-detector/src/capabilities_orphan_capabilities_analyzer.rs"],
    )]);

    let result = a.is_utility_orphan(&f, &root, &all_files, &inbound);
    assert!(!result.is_orphan);
}

// ─── Orphan: utility not imported by anyone ───────────────

#[test]
fn utility_not_imported_is_orphan() {
    let a = analyzer();
    let f = FilePath::new("crates/shared/src/orphan-detector/utility_dead.rs".to_string()).unwrap();
    let root = FilePath::new("crates/shared".to_string()).unwrap();
    let all_files = vec!["crates/shared/src/orphan-detector/utility_dead.rs".to_string()];
    let inbound = make_inbound_links(vec![]);

    let result = a.is_utility_orphan(&f, &root, &all_files, &inbound);
    assert!(result.is_orphan);
    assert_eq!(result.severity, Severity::MEDIUM);
}

// ─── Dead code: utility only imported by other utilities ──

#[test]
fn utility_imported_only_by_utilities_is_dead_code() {
    let a = analyzer();
    let f =
        FilePath::new("crates/shared/src/orphan-detector/utility_inner.rs".to_string()).unwrap();
    let root = FilePath::new("crates/shared".to_string()).unwrap();
    let all_files = vec![
        "crates/shared/src/orphan-detector/utility_inner.rs".to_string(),
        "crates/shared/src/orphan-detector/utility_outer.rs".to_string(),
    ];
    let inbound = make_inbound_links(vec![(
        "crates/shared/src/orphan-detector/utility_inner.rs",
        vec!["crates/shared/src/orphan-detector/utility_outer.rs"],
    )]);

    let result = a.is_utility_orphan(&f, &root, &all_files, &inbound);
    assert!(result.is_orphan);
    assert!(result.reason.contains("only imported by other utility"));
}

// ─── Default trait ────────────────────────────────────────

#[test]
fn default_creates_valid_instance() {
    let _a = UtilityOrphanAnalyzer::default();
}
