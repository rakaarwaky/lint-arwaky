// PURPOSE: Unit tests for SurfacesOrphanAnalyzer — AES506 surface orphan detection.
// Layer: Capabilities (SurfacesOrphanAnalyzer)
// Speed: ms

use orphan_detector_lint_arwaky::capabilities_orphan_surfaces_analyzer::SurfacesOrphanAnalyzer;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::taxonomy_analysis_vo::ReachabilityResult;
use shared::common::taxonomy_path_vo::FilePath;
use shared::orphan_detector::contract_orphan_protocol::ISurfacesOrphanProtocol;
use std::collections::HashSet;

fn analyzer() -> SurfacesOrphanAnalyzer {
    SurfacesOrphanAnalyzer::new()
}

// ─── Happy path: reachable surface ────────────────────────

#[test]
fn surface_reachable_from_entry_is_not_orphan() {
    let a = analyzer();
    let f = FilePath::new("crates/app/src/surface_scan_command.rs".to_string()).unwrap();
    let root = FilePath::new("crates/app".to_string()).unwrap();
    let alive = ReachabilityResult::new(HashSet::from([f.clone()]));

    let result = a.is_surface_orphan(&f, &root, &alive, None);
    assert!(!result.is_orphan);
}

// ─── Orphan: unreachable surface ──────────────────────────

#[test]
fn surface_not_reachable_is_orphan() {
    let a = analyzer();
    let dir = tempfile::tempdir().unwrap();
    let surface_path = dir.path().join("surface_dead_command.rs");
    std::fs::write(&surface_path, "pub struct DeadCommand;\n").unwrap();

    let f = FilePath::new(surface_path.to_str().unwrap().to_string()).unwrap();
    let root = FilePath::new(dir.path().to_str().unwrap().to_string()).unwrap();
    let alive = ReachabilityResult::new(HashSet::new());

    let result = a.is_surface_orphan(&f, &root, &alive, None);
    assert!(result.is_orphan);
    assert_eq!(result.severity, Severity::HIGH);
}

// ─── Default trait ────────────────────────────────────────

#[test]
fn default_creates_valid_instance() {
    let _a = SurfacesOrphanAnalyzer::default();
}
