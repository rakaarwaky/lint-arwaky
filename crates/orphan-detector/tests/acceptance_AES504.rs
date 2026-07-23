// PURPOSE: Acceptance test — AES504 Utility Orphan Checker.
// Requirement: Utility files must be wired into root containers or imported by capabilities/agents.

use orphan_detector_lint_arwaky::capabilities_orphan_utility_analyzer::UtilityOrphanAnalyzer;
use shared::code_analysis::taxonomy_analysis_vo::InboundLinkMap;
use shared::common::taxonomy_path_vo::FilePath;
use shared::orphan_detector::contract_orphan_protocol::IUtilityOrphanProtocol;
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

/// AES504: Utility imported by a capabilities file is NOT orphan.
#[test]
fn aes504_utility_imported_by_capabilities_not_orphan() {
    let a = UtilityOrphanAnalyzer::new();
    let f = FilePath::new("shared/src/orphan-detector/utility_orphan.rs".to_string()).unwrap();
    let root = FilePath::new("shared".to_string()).unwrap();
    let all = vec![
        "shared/src/orphan-detector/utility_orphan.rs".to_string(),
        "orphan-detector/src/capabilities_orphan_capabilities_analyzer.rs".to_string(),
    ];
    let inbound = make_inbound(vec![(
        "shared/src/orphan-detector/utility_orphan.rs",
        vec!["orphan-detector/src/capabilities_orphan_capabilities_analyzer.rs"],
    )]);

    let result = a.is_utility_orphan(&f, &root, &all, &inbound);
    assert!(
        !result.is_orphan,
        "AES504 FAIL: utility imported by capabilities should not be orphan"
    );
}

/// AES504: Utility with zero importers IS orphan.
#[test]
fn aes504_utility_with_no_importers_is_orphan() {
    let a = UtilityOrphanAnalyzer::new();
    let f = FilePath::new("shared/src/orphan-detector/utility_dead.rs".to_string()).unwrap();
    let root = FilePath::new("shared".to_string()).unwrap();
    let all = vec!["shared/src/orphan-detector/utility_dead.rs".to_string()];
    let inbound = make_inbound(vec![]);

    let result = a.is_utility_orphan(&f, &root, &all, &inbound);
    assert!(
        result.is_orphan,
        "AES504 FAIL: utility with no importers must be flagged"
    );
}

/// AES504: Utility imported ONLY by other utilities IS dead code.
#[test]
fn aes504_utility_only_imported_by_utilities_is_dead_code() {
    let a = UtilityOrphanAnalyzer::new();
    let f = FilePath::new("shared/src/orphan-detector/utility_inner.rs".to_string()).unwrap();
    let root = FilePath::new("shared".to_string()).unwrap();
    let all = vec![
        "shared/src/orphan-detector/utility_inner.rs".to_string(),
        "shared/src/orphan-detector/utility_outer.rs".to_string(),
    ];
    let inbound = make_inbound(vec![(
        "shared/src/orphan-detector/utility_inner.rs",
        vec!["shared/src/orphan-detector/utility_outer.rs"],
    )]);

    let result = a.is_utility_orphan(&f, &root, &all, &inbound);
    assert!(
        result.is_orphan,
        "AES504 FAIL: utility only imported by utilities must be flagged as dead code"
    );
    assert!(result.reason.contains("only imported by other utility"));
}
