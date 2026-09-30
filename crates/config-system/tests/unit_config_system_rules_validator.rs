// Unit tests for config system validation utilities.
use shared::common::AdapterName;
use shared::common::{Count, Score};
use shared::config_system::Thresholds;
use shared::config_system::taxonomy_config_system_vo::{
    AdapterEntry, AdapterStatus, ProjectConfig, ValidationResult,
};
use shared::config_system::utility_config_parser::{is_adapter_enabled, validate_thresholds};

fn make_config_with_adapters(adapters: Vec<AdapterEntry>) -> ProjectConfig {
    ProjectConfig {
        adapters,
        ..Default::default()
    }
}

#[test]
fn adapter_enabled_returns_true_when_status_enabled() {
    let config = make_config_with_adapters(vec![AdapterEntry::new(
        AdapterName::raw("ruff"),
        AdapterStatus::Enabled,
        1.0,
    )]);
    assert!(is_adapter_enabled(&config, &AdapterName::raw("ruff")));
}

#[test]
fn adapter_enabled_returns_false_when_status_disabled() {
    let config = make_config_with_adapters(vec![AdapterEntry::new(
        AdapterName::raw("mypy"),
        AdapterStatus::Disabled,
        1.0,
    )]);
    assert!(!is_adapter_enabled(&config, &AdapterName::raw("mypy")));
}

#[test]
fn adapter_enabled_returns_false_when_status_not_installed() {
    let config = make_config_with_adapters(vec![AdapterEntry::new(
        AdapterName::raw("bandit"),
        AdapterStatus::NotInstalled,
        1.0,
    )]);
    assert!(!is_adapter_enabled(&config, &AdapterName::raw("bandit")));
}

#[test]
fn adapter_enabled_defaults_true_when_adapter_not_in_list() {
    let config = make_config_with_adapters(vec![]);
    assert!(is_adapter_enabled(
        &config,
        &AdapterName::raw("unknown_adapter")
    ));
}

#[test]
fn adapter_enabled_matches_first_occurrence() {
    let config = make_config_with_adapters(vec![
        AdapterEntry::new(AdapterName::raw("ruff"), AdapterStatus::Disabled, 1.0),
        AdapterEntry::new(AdapterName::raw("ruff"), AdapterStatus::Enabled, 2.0),
    ]);
    assert!(!is_adapter_enabled(&config, &AdapterName::raw("ruff")));
}

#[test]
fn validate_thresholds_ok_with_valid_values() {
    let config = ProjectConfig {
        thresholds: Thresholds::new(Score::new(80.0), Count::new(10), Count::new(500)),
        ..Default::default()
    };
    let result = validate_thresholds(&config);
    assert!(result.is_valid);
    assert!(result.reason.is_none());
}

#[test]
fn validate_thresholds_fails_when_score_above_100() {
    let config = ProjectConfig {
        thresholds: Thresholds::new(Score::new(101.0), Count::new(10), Count::new(500)),
        ..Default::default()
    };
    let result = validate_thresholds(&config);
    assert!(!result.is_valid);
    assert!(
        result
            .reason
            .as_deref()
            .unwrap_or("")
            .contains("Score threshold")
    );
}

#[test]
fn validate_thresholds_fails_when_score_negative() {
    let config = ProjectConfig {
        thresholds: Thresholds::new(Score::new(-1.0), Count::new(10), Count::new(500)),
        ..Default::default()
    };
    let result = validate_thresholds(&config);
    assert!(!result.is_valid);
}

#[test]
fn validate_thresholds_fails_when_complexity_zero() {
    let config = ProjectConfig {
        thresholds: Thresholds::new(Score::new(80.0), Count::new(0), Count::new(500)),
        ..Default::default()
    };
    let result = validate_thresholds(&config);
    assert!(!result.is_valid);
    assert!(
        result
            .reason
            .as_deref()
            .unwrap_or("")
            .contains("Complexity")
    );
}

#[test]
fn validate_thresholds_fails_when_max_file_lines_zero() {
    let config = ProjectConfig {
        thresholds: Thresholds::new(Score::new(80.0), Count::new(10), Count::new(0)),
        ..Default::default()
    };
    let result = validate_thresholds(&config);
    assert!(!result.is_valid);
    assert!(
        result
            .reason
            .as_deref()
            .unwrap_or("")
            .contains("max_file_lines")
    );
}

#[test]
fn validate_thresholds_accumulates_multiple_errors() {
    let config = ProjectConfig {
        thresholds: Thresholds::new(Score::new(200.0), Count::new(0), Count::new(-1)),
        ..Default::default()
    };
    let result = validate_thresholds(&config);
    assert!(!result.is_valid);
    let reason = result.reason.as_deref().unwrap_or("");
    assert!(reason.contains("Score threshold"));
    assert!(reason.contains("Complexity"));
    assert!(reason.contains("max_file_lines"));
}

#[test]
fn validate_thresholds_boundary_score_0_is_valid() {
    let config = ProjectConfig {
        thresholds: Thresholds::new(Score::new(0.0), Count::new(1), Count::new(1)),
        ..Default::default()
    };
    assert!(validate_thresholds(&config).is_valid);
}

#[test]
fn validate_thresholds_boundary_score_100_is_valid() {
    let config = ProjectConfig {
        thresholds: Thresholds::new(Score::new(100.0), Count::new(1), Count::new(1)),
        ..Default::default()
    };
    assert!(validate_thresholds(&config).is_valid);
}
