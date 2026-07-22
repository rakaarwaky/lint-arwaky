// Unit tests for ConfigRulesValidator — adapter enablement and threshold validation.
use config_system_lint_arwaky::capabilities_rules_validator::ConfigRulesValidator;
use shared::common::taxonomy_adapter_name_vo::AdapterName;
use shared::common::taxonomy_common_vo::{Count, Score};
use shared::config_system::contract_validator_protocol::IConfigValidatorProtocol;
use shared::config_system::taxonomy_setting_vo::{
    AdapterEntry, AdapterStatus, ProjectConfig, Thresholds,
};

fn make_validator() -> ConfigRulesValidator {
    ConfigRulesValidator::new()
}

fn make_config_with_adapters(adapters: Vec<AdapterEntry>) -> ProjectConfig {
    let mut config = ProjectConfig::default();
    config.adapters = adapters;
    config
}

#[test]
fn adapter_enabled_returns_true_when_status_enabled() {
    let sut = make_validator();
    let config = make_config_with_adapters(vec![AdapterEntry::new(
        AdapterName::raw("ruff"),
        AdapterStatus::Enabled,
        1.0,
    )]);
    assert!(sut.is_adapter_enabled(&config, &AdapterName::raw("ruff")));
}

#[test]
fn adapter_enabled_returns_false_when_status_disabled() {
    let sut = make_validator();
    let config = make_config_with_adapters(vec![AdapterEntry::new(
        AdapterName::raw("mypy"),
        AdapterStatus::Disabled,
        1.0,
    )]);
    assert!(!sut.is_adapter_enabled(&config, &AdapterName::raw("mypy")));
}

#[test]
fn adapter_enabled_returns_false_when_status_not_installed() {
    let sut = make_validator();
    let config = make_config_with_adapters(vec![AdapterEntry::new(
        AdapterName::raw("bandit"),
        AdapterStatus::NotInstalled,
        1.0,
    )]);
    assert!(!sut.is_adapter_enabled(&config, &AdapterName::raw("bandit")));
}

#[test]
fn adapter_enabled_defaults_true_when_adapter_not_in_list() {
    let sut = make_validator();
    let config = make_config_with_adapters(vec![]);
    assert!(sut.is_adapter_enabled(&config, &AdapterName::raw("unknown_adapter")));
}

#[test]
fn adapter_enabled_matches_first_occurrence() {
    let sut = make_validator();
    let config = make_config_with_adapters(vec![
        AdapterEntry::new(AdapterName::raw("ruff"), AdapterStatus::Disabled, 1.0),
        AdapterEntry::new(AdapterName::raw("ruff"), AdapterStatus::Enabled, 2.0),
    ]);
    assert!(!sut.is_adapter_enabled(&config, &AdapterName::raw("ruff")));
}

#[test]
fn validate_thresholds_ok_with_valid_values() {
    let sut = make_validator();
    let mut config = ProjectConfig::default();
    config.thresholds = Thresholds::new(Score::new(80.0), Count::new(10), Count::new(500));
    let result = sut.validate_thresholds(&config);
    assert!(result.is_valid);
    assert!(result.reason.is_none());
}

#[test]
fn validate_thresholds_fails_when_score_above_100() {
    let sut = make_validator();
    let mut config = ProjectConfig::default();
    config.thresholds = Thresholds::new(Score::new(101.0), Count::new(10), Count::new(500));
    let result = sut.validate_thresholds(&config);
    assert!(!result.is_valid);
    assert!(result.reason.unwrap().contains("Score threshold"));
}

#[test]
fn validate_thresholds_fails_when_score_negative() {
    let sut = make_validator();
    let mut config = ProjectConfig::default();
    config.thresholds = Thresholds::new(Score::new(-1.0), Count::new(10), Count::new(500));
    let result = sut.validate_thresholds(&config);
    assert!(!result.is_valid);
}

#[test]
fn validate_thresholds_fails_when_complexity_zero() {
    let sut = make_validator();
    let mut config = ProjectConfig::default();
    config.thresholds = Thresholds::new(Score::new(80.0), Count::new(0), Count::new(500));
    let result = sut.validate_thresholds(&config);
    assert!(!result.is_valid);
    assert!(result.reason.unwrap().contains("Complexity"));
}

#[test]
fn validate_thresholds_fails_when_max_file_lines_zero() {
    let sut = make_validator();
    let mut config = ProjectConfig::default();
    config.thresholds = Thresholds::new(Score::new(80.0), Count::new(10), Count::new(0));
    let result = sut.validate_thresholds(&config);
    assert!(!result.is_valid);
    assert!(result.reason.unwrap().contains("max_file_lines"));
}

#[test]
fn validate_thresholds_accumulates_multiple_errors() {
    let sut = make_validator();
    let mut config = ProjectConfig::default();
    config.thresholds = Thresholds::new(Score::new(200.0), Count::new(0), Count::new(-1));
    let result = sut.validate_thresholds(&config);
    assert!(!result.is_valid);
    let reason = result.reason.unwrap();
    assert!(reason.contains("Score threshold"));
    assert!(reason.contains("Complexity"));
    assert!(reason.contains("max_file_lines"));
}

#[test]
fn validate_thresholds_boundary_score_0_is_valid() {
    let sut = make_validator();
    let mut config = ProjectConfig::default();
    config.thresholds = Thresholds::new(Score::new(0.0), Count::new(1), Count::new(1));
    assert!(sut.validate_thresholds(&config).is_valid);
}

#[test]
fn validate_thresholds_boundary_score_100_is_valid() {
    let sut = make_validator();
    let mut config = ProjectConfig::default();
    config.thresholds = Thresholds::new(Score::new(100.0), Count::new(1), Count::new(1));
    assert!(sut.validate_thresholds(&config).is_valid);
}

#[test]
fn default_and_new_produce_equivalent_instances() {
    let a = ConfigRulesValidator::new();
    let b = ConfigRulesValidator::default();
    let config = ProjectConfig::default();
    assert_eq!(
        a.validate_thresholds(&config),
        b.validate_thresholds(&config)
    );
}
