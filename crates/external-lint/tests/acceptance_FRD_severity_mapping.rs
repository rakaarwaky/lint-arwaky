// PURPOSE: Acceptance test — FRD: "Error level translation — tool severities
// are correctly mapped to Lint Arwaky Severity."

use shared::common::taxonomy_severity_vo::Severity;

/// FRD-EXT-007: Severity enum has all four required levels.
#[test]
fn frd_007_severity_has_all_levels() {
    let levels = [
        Severity::CRITICAL,
        Severity::HIGH,
        Severity::MEDIUM,
        Severity::LOW,
    ];
    assert_eq!(levels.len(), 4);
}

/// FRD-EXT-008: Severity Display renders lowercase strings.
#[test]
fn frd_008_severity_display_is_lowercase() {
    assert_eq!(Severity::CRITICAL.to_string(), "critical");
    assert_eq!(Severity::HIGH.to_string(), "high");
    assert_eq!(Severity::MEDIUM.to_string(), "medium");
    assert_eq!(Severity::LOW.to_string(), "low");
    assert_eq!(Severity::INFO.to_string(), "info");
}

/// FRD-EXT-009: Severity score_impact is monotonically increasing.
#[test]
fn frd_009_severity_score_impact_ordering() {
    assert!(Severity::CRITICAL.score_impact() > Severity::HIGH.score_impact());
    assert!(Severity::HIGH.score_impact() > Severity::MEDIUM.score_impact());
    assert!(Severity::MEDIUM.score_impact() > Severity::LOW.score_impact());
    assert!(Severity::LOW.score_impact() > Severity::INFO.score_impact());
}

/// FRD-EXT-010: Severity serialization uses lowercase rename.
#[test]
fn frd_010_severity_serializes_correctly() {
    let json = serde_json::to_string(&Severity::HIGH).unwrap();
    assert_eq!(json, "\"high\"");

    let json = serde_json::to_string(&Severity::CRITICAL).unwrap();
    assert_eq!(json, "\"critical\"");
}

/// FRD-EXT-011: Severity deserialization roundtrips.
#[test]
fn frd_011_severity_deserialization_roundtrip() {
    let original = Severity::MEDIUM;
    let json = serde_json::to_string(&original).unwrap();
    let restored: Severity = serde_json::from_str(&json).unwrap();
    assert_eq!(original, restored);
}

/// FRD-EXT-012: Default severity is INFO.
#[test]
fn frd_012_default_severity_is_info() {
    let default = Severity::default();
    assert_eq!(default, Severity::INFO);
}
