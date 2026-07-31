// PURPOSE: Unit tests for SurfacesOrphanAnalyzer — AES506 surface orphan detection.
// Layer: Capabilities (SurfacesOrphanAnalyzer)
// Speed: ms

use orphan_detector_lint_arwaky::capabilities_orphan_surfaces_analyzer::SurfacesOrphanAnalyzer;
use shared::code_analysis::{InboundLinkMap, ReachabilityResult};
use shared::common::{FilePath, Severity};

use shared::orphan_detector::ISurfacesOrphanProtocol;
use std::collections::{HashMap, HashSet};

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
    let inbound = InboundLinkMap::new(HashMap::new());

    let result = a.is_surface_orphan(&f, &root, &alive, &inbound, None);
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
    let inbound = InboundLinkMap::new(HashMap::new());

    let result = a.is_surface_orphan(&f, &root, &alive, &inbound, None);
    assert!(result.is_orphan);
    assert_eq!(result.severity, Severity::HIGH);
}

// ─── Chain validation: passive surface imported only by passive surface ─

#[test]
fn passive_surface_imported_only_by_passive_is_orphan() {
    let a = analyzer();
    let f = FilePath::new("crates/app/src/surface_card_component.rs".to_string()).unwrap();
    let root = FilePath::new("crates/app".to_string()).unwrap();
    let alive = ReachabilityResult::new(HashSet::from([f.clone()]));

    let mut map = HashMap::new();
    map.insert(
        "crates/app/src/surface_card_component.rs".to_string(),
        vec!["crates/app/src/surface_list_view.rs".to_string()],
    );
    let inbound = InboundLinkMap::new(map);

    let result = a.is_surface_orphan(&f, &root, &alive, &inbound, None);
    assert!(result.is_orphan);
    assert_eq!(result.severity, Severity::LOW);
}

// ─── Default trait ────────────────────────────────────────

#[test]
fn default_creates_valid_instance() {
    let _a = SurfacesOrphanAnalyzer::default();
}
