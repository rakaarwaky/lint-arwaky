// PURPOSE: Unit tests for CapabilitiesOrphanAnalyzer — AES503 capabilities orphan detection.
// Layer: Capabilities (CapabilitiesOrphanAnalyzer)
// Speed: ms

use orphan_detector_lint_arwaky::capabilities_orphan_capabilities_analyzer::CapabilitiesOrphanAnalyzer;
use shared::common::taxonomy_severity_vo::Severity;
use shared::code_analysis::taxonomy_analysis_vo::ReachabilityResult;
use shared::common::taxonomy_path_vo::FilePath;
use shared::orphan_detector::contract_orphan_protocol::ICapabilitiesOrphanProtocol;
use std::collections::HashSet;

fn analyzer() -> CapabilitiesOrphanAnalyzer {
    CapabilitiesOrphanAnalyzer::new()
}

// ─── Happy path: reachable capabilities file ──────────────

#[test]
fn capabilities_file_reachable_from_entry_is_not_orphan() {
    let a = analyzer();
    let f = FilePath::new(
        "crates/orphan-detector/src/capabilities_orphan_agent_analyzer.rs".to_string(),
    )
    .unwrap();
    let root = FilePath::new("crates/orphan-detector".to_string()).unwrap();
    let alive = ReachabilityResult::new(HashSet::from([f.clone()]));

    let result = a.is_capabilities_orphan(&f, &root, &alive);
    assert!(!result.is_orphan);
}

// ─── Orphan: unreachable capabilities file ────────────────

#[test]
fn capabilities_file_not_reachable_is_orphan() {
    let a = analyzer();
    let f = FilePath::new("crates/orphan-detector/src/capabilities_dead_analyzer.rs".to_string())
        .unwrap();
    let root = FilePath::new("crates/orphan-detector".to_string()).unwrap();
    let alive = ReachabilityResult::new(HashSet::new());

    let result = a.is_capabilities_orphan(&f, &root, &alive);
    assert!(result.is_orphan);
    assert_eq!(result.severity, Severity::MEDIUM);
    assert!(result.reason.contains("capabilities_dead_analyzer"));
}

// ─── Default trait ────────────────────────────────────────

#[test]
fn default_creates_valid_instance() {
    let _a = CapabilitiesOrphanAnalyzer::default();
}
