// PURPOSE: Acceptance test — AES501 Taxonomy Orphan Checker.
// Requirement: Taxonomy layer files must be reachable from contracts, capabilities, or orchestrators.

use orphan_detector_lint_arwaky::capabilities_orphan_taxonomy_analyzer::TaxonomyOrphanAnalyzer;
use shared::code_analysis::taxonomy_analysis_vo::InboundLinkMap;
use shared::common::taxonomy_path_vo::FilePath;
use shared::orphan_detector::contract_orphan_protocol::ITaxonomyOrphanProtocol;
use std::collections::HashMap;

fn make_inbound(links: Vec<(&str, Vec<&str>)>) -> InboundLinkMap {
    let mut m = HashMap::new();
    for (f, importers) in links {
        m.insert(
            f.to_string(),
            importers.iter().map(|s| s.to_string()).collect(),
        );
    }
    InboundLinkMap::new(m)
}

/// AES501: Taxonomy file imported by a contract is NOT orphan.
#[test]
fn aes501_taxonomy_imported_by_contract_not_orphan() {
    let a = TaxonomyOrphanAnalyzer::new();
    let f = FilePath::new("shared/src/common/taxonomy_path_vo.rs".to_string()).unwrap();
    let root = FilePath::new("shared".to_string()).unwrap();
    let inbound = make_inbound(vec![(
        "shared/src/common/taxonomy_path_vo.rs",
        vec!["shared/src/orphan-detector/contract_orphan_protocol.rs"],
    )]);
    let result = a.is_taxonomy_orphan(&f, &root, None, &inbound);
    assert!(
        !result.is_orphan,
        "AES501 FAIL: taxonomy imported by contract should not be orphan"
    );
}

/// AES501: Taxonomy file with zero importers IS orphan.
#[test]
fn aes501_taxonomy_with_no_importers_is_orphan() {
    let a = TaxonomyOrphanAnalyzer::new();
    let f = FilePath::new("shared/src/common/taxonomy_dead_vo.rs".to_string()).unwrap();
    let root = FilePath::new("shared".to_string()).unwrap();
    let inbound = make_inbound(vec![]);
    let result = a.is_taxonomy_orphan(&f, &root, None, &inbound);
    assert!(
        result.is_orphan,
        "AES501 FAIL: taxonomy with no importers must be flagged"
    );
    assert!(result.reason.contains("taxonomy_dead_vo"));
}

/// AES501: Taxonomy file imported only by another taxonomy file IS orphan.
#[test]
fn aes501_taxonomy_imported_only_by_taxonomy_is_orphan() {
    let a = TaxonomyOrphanAnalyzer::new();
    let f = FilePath::new("shared/src/common/taxonomy_inner_vo.rs".to_string()).unwrap();
    let root = FilePath::new("shared".to_string()).unwrap();
    let inbound = make_inbound(vec![(
        "shared/src/common/taxonomy_inner_vo.rs",
        vec!["shared/src/common/taxonomy_outer_vo.rs"],
    )]);
    let result = a.is_taxonomy_orphan(&f, &root, None, &inbound);
    assert!(
        result.is_orphan,
        "AES501 FAIL: taxonomy imported only by taxonomy must be flagged"
    );
}
