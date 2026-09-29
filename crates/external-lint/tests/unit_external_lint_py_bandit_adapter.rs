// Unit tests for BanditAdapter — bandit stdout JSON parsing.
use external_lint_lint_arwaky::capabilities_py_bandit_adapter::BanditAdapter;

#[allow(dead_code, unused_imports)]
#[path = "../../shared/tests/common/mock_filesystem.rs"]
mod mock_filesystem;

use mock_filesystem::MockFilesystem;
use shared::common::taxonomy_severity_vo::Severity;
use std::sync::Arc;

fn make_adapter() -> BanditAdapter {
    BanditAdapter::new(
        None,
        Arc::new(MockFilesystem::new()),
        Arc::new(MockFilesystem::new()),
    )
}

#[test]
fn high_confidence_high_severity_maps_to_critical() {
    let adapter = make_adapter();
    assert_eq!(adapter.map_severity("HIGH", "HIGH"), Severity::CRITICAL);
}

#[test]
fn high_severity_low_confidence_maps_to_high() {
    let adapter = make_adapter();
    assert_eq!(adapter.map_severity("HIGH", "LOW"), Severity::HIGH);
    assert_eq!(adapter.map_severity("HIGH", "MEDIUM"), Severity::HIGH);
}

#[test]
fn medium_severity_any_confidence_maps_to_medium() {
    let adapter = make_adapter();
    assert_eq!(adapter.map_severity("MEDIUM", "HIGH"), Severity::MEDIUM);
    assert_eq!(adapter.map_severity("MEDIUM", "LOW"), Severity::MEDIUM);
}

#[test]
fn low_severity_any_confidence_maps_to_low() {
    let adapter = make_adapter();
    assert_eq!(adapter.map_severity("LOW", "HIGH"), Severity::LOW);
    assert_eq!(adapter.map_severity("LOW", "LOW"), Severity::LOW);
}

#[test]
fn unknown_severity_defaults_to_medium() {
    let adapter = make_adapter();
    assert_eq!(adapter.map_severity("UNKNOWN", "HIGH"), Severity::MEDIUM);
}
