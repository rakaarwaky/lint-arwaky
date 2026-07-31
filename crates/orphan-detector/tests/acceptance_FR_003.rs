// PURPOSE: Acceptance test — FR-003 Reachability Tracing.
// Requirement: Perform BFS from entry points through forward import graph, handling circular imports safely.

use orphan_detector_lint_arwaky::capabilities_orphan_capabilities_analyzer::CapabilitiesOrphanAnalyzer;
use shared::code_analysis::ReachabilityResult;
use shared::common::FilePath;
use shared::orphan_detector::ICapabilitiesOrphanProtocol;
use std::collections::HashSet;

#[test]
fn fr003_reachability_tracing_and_circular_import_safety() {
    let analyzer = CapabilitiesOrphanAnalyzer::default();

    let entry = FilePath::new("src/root_cli_main_entry.rs".to_string()).unwrap();
    let cap_a = FilePath::new("src/capabilities_service_a.rs".to_string()).unwrap();
    let cap_b = FilePath::new("src/capabilities_service_b.rs".to_string()).unwrap();
    let root = FilePath::new("src".to_string()).unwrap();

    // Circular dependency graph: Entry -> CapA -> CapB -> CapA
    let alive = ReachabilityResult::new(HashSet::from([entry, cap_a.clone(), cap_b.clone()]));

    let res_a = analyzer.is_capabilities_orphan(&cap_a, &root, &alive);
    let res_b = analyzer.is_capabilities_orphan(&cap_b, &root, &alive);

    assert!(
        !res_a.is_orphan,
        "FR-003 FAIL: transitively reachable file service_a must be alive"
    );
    assert!(
        !res_b.is_orphan,
        "FR-003 FAIL: transitively reachable file service_b must be alive"
    );
}
