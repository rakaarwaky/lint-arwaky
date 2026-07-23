// PURPOSE: Acceptance test — AES506 Surface Orphan Checker.
// Requirement: Surface layer files must be registered in the routing system or called from main entries.

use orphan_detector_lint_arwaky::capabilities_orphan_surfaces_analyzer::SurfacesOrphanAnalyzer;
use shared::code_analysis::taxonomy_analysis_vo::ReachabilityResult;
use shared::common::taxonomy_path_vo::FilePath;
use shared::orphan_detector::contract_orphan_protocol::ISurfacesOrphanProtocol;
use std::collections::HashSet;
use std::fs;

/// AES506: Surface reachable from entry point is NOT orphan.
#[test]
fn aes506_reachable_surface_not_orphan() {
    let a = SurfacesOrphanAnalyzer::new();
    let f = FilePath::new("src/surface_scan_command.rs".to_string()).unwrap();
    let root = FilePath::new("src".to_string()).unwrap();
    let alive = ReachabilityResult::new(HashSet::from([f.clone()]));

    let result = a.is_surface_orphan(&f, &root, &alive, None);
    assert!(
        !result.is_orphan,
        "AES506 FAIL: reachable surface should not be orphan"
    );
}

/// AES506: Surface NOT reachable from any entry IS orphan.
#[test]
fn aes506_unreachable_surface_is_orphan() {
    let a = SurfacesOrphanAnalyzer::new();
    let dir = tempfile::tempdir().unwrap();
    let surface = dir.path().join("surface_dead_command.rs");
    fs::write(&surface, "pub struct DeadCommand;\n").unwrap();

    let f = FilePath::new(surface.to_str().unwrap().to_string()).unwrap();
    let root = FilePath::new(dir.path().to_str().unwrap().to_string()).unwrap();
    let alive = ReachabilityResult::new(HashSet::new());

    let result = a.is_surface_orphan(&f, &root, &alive, None);
    assert!(
        result.is_orphan,
        "AES506 FAIL: unreachable surface must be flagged"
    );
}
