// FR-006 — Config Validation
// Tests threshold validation and adapter enablement per FR-006 spec.
mod common;

use config_system_lint_arwaky::capabilities_rules_validator::ConfigRulesValidator;
use shared::common::AdapterName;
use shared::common::{Count, Score};
use shared::config_system::{
    AdapterEntry, AdapterStatus, IConfigValidatorProtocol, ProjectConfig, Thresholds,
};

fn make_validator() -> ConfigRulesValidator {
    ConfigRulesValidator::new()
}

// FR-006 Scenario 1: Score threshold 50.0 → Valid
#[test]
fn us6_score_threshold_50_is_valid() {
    let config = ProjectConfig {
        thresholds: Thresholds::new(Score::new(50.0), Count::new(10), Count::new(500)),
        ..Default::default()
    };
    assert!(make_validator().validate_thresholds(&config).is_valid);
}

// FR-006 Scenario 2: Score threshold 0.0 → Valid (boundary)
#[test]
fn us6_score_threshold_0_is_valid() {
    let config = ProjectConfig {
        thresholds: Thresholds::new(Score::new(0.0), Count::new(1), Count::new(1)),
        ..Default::default()
    };
    assert!(make_validator().validate_thresholds(&config).is_valid);
}

// FR-006 Scenario 3: Score threshold 100.0 → Valid (boundary)
#[test]
fn us6_score_threshold_100_is_valid() {
    let config = ProjectConfig {
        thresholds: Thresholds::new(Score::new(100.0), Count::new(1), Count::new(1)),
        ..Default::default()
    };
    assert!(make_validator().validate_thresholds(&config).is_valid);
}

// FR-006 Scenario 4: Score threshold -1.0 → Invalid
#[test]
fn us6_score_threshold_negative_is_invalid() {
    let config = ProjectConfig {
        thresholds: Thresholds::new(Score::new(-1.0), Count::new(10), Count::new(500)),
        ..Default::default()
    };
    let result = make_validator().validate_thresholds(&config);
    assert!(!result.is_valid);
    assert!(result.reason.unwrap().contains("Score threshold"));
}

// FR-006 Scenario 5: Score threshold 101.0 → Invalid
#[test]
fn us6_score_threshold_above_100_is_invalid() {
    let config = ProjectConfig {
        thresholds: Thresholds::new(Score::new(101.0), Count::new(10), Count::new(500)),
        ..Default::default()
    };
    let result = make_validator().validate_thresholds(&config);
    assert!(!result.is_valid);
    assert!(result.reason.unwrap().contains("Score threshold"));
}

// FR-006 Scenario 6: Unknown adapter → Enabled (default true)
#[test]
fn us6_unknown_adapter_defaults_to_enabled() {
    let config = ProjectConfig::default();
    assert!(make_validator().is_adapter_enabled(&config, &AdapterName::raw("unknown_adapter")));
}

// Additional: Complexity threshold zero → Invalid
#[test]
fn us6_complexity_zero_is_invalid() {
    let config = ProjectConfig {
        thresholds: Thresholds::new(Score::new(80.0), Count::new(0), Count::new(500)),
        ..Default::default()
    };
    assert!(!make_validator().validate_thresholds(&config).is_valid);
}

// Additional: max_file_lines zero → Invalid
#[test]
fn us6_max_file_lines_zero_is_invalid() {
    let config = ProjectConfig {
        thresholds: Thresholds::new(Score::new(80.0), Count::new(10), Count::new(0)),
        ..Default::default()
    };
    assert!(!make_validator().validate_thresholds(&config).is_valid);
}

// Additional: Multiple invalid thresholds accumulate errors
#[test]
fn us6_multiple_invalid_thresholds_accumulate_errors() {
    let config = ProjectConfig {
        thresholds: Thresholds::new(Score::new(200.0), Count::new(0), Count::new(-1)),
        ..Default::default()
    };
    let result = make_validator().validate_thresholds(&config);
    assert!(!result.is_valid);
    let reason = result.reason.unwrap();
    assert!(reason.contains("Score threshold"));
    assert!(reason.contains("Complexity"));
    assert!(reason.contains("max_file_lines"));
}

// Additional: Adapter enabled/disabled via status field
#[test]
fn us6_adapter_disabled_via_status_field() {
    let config = ProjectConfig {
        adapters: vec![AdapterEntry::new(
            AdapterName::raw("mypy"),
            AdapterStatus::Disabled,
            1.0,
        )],
        ..Default::default()
    };
    assert!(!make_validator().is_adapter_enabled(&config, &AdapterName::raw("mypy")));
}
