// PURPOSE: Acceptance test — AES503 Capabilities Orphan Checker.
// Requirement: Capability files must be instantiated or imported by orchestrators or other capability files.

use orphan_detector_lint_arwaky::capabilities_orphan_capabilities_analyzer::CapabilitiesOrphanAnalyzer;
use shared::code_analysis::taxonomy_analysis_vo::ReachabilityResult;
use shared::common::taxonomy_path_vo::FilePath;
use shared::orphan_detector::contract_orphan_protocol::ICapabilitiesOrphanProtocol;
use std::collections::HashSet;

/// AES503: Capability reachable from entry point is NOT orphan.
#[test]
fn aes503_reachable_capability_not_orphan() {
    let a = CapabilitiesOrphanAnalyzer::new();
    let f = FilePath::new("src/capabilities_greeter_analyzer.rs".to_string()).unwrap();
    let root = FilePath::new("src".to_string()).unwrap();
    let alive = ReachabilityResult::new(HashSet::from([f.clone()]));

    let result = a.is_capabilities_orphan(&f, &root, &alive);
    assert!(
        !result.is_orphan,
        "AES503 FAIL: reachable capability should not be orphan"
    );
}

/// AES503: Capability NOT reachable from any entry point IS orphan.
#[test]
fn aes503_unreachable_capability_is_orphan() {
    let a = CapabilitiesOrphanAnalyzer::new();
    let f = FilePath::new("src/capabilities_dead_analyzer.rs".to_string()).unwrap();
    let root = FilePath::new("src".to_string()).unwrap();
    let alive = ReachabilityResult::new(HashSet::new());

    let result = a.is_capabilities_orphan(&f, &root, &alive);
    assert!(
        result.is_orphan,
        "AES503 FAIL: unreachable capability must be flagged"
    );
}
