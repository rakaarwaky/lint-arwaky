use config_system_lint_arwaky::capabilities_rules_validator::ConfigRulesValidator;
use shared::config_system::contract_validator_protocol::IConfigValidatorProtocol;
use shared::config_system::taxonomy_setting_vo::{
    AdapterEntry, AdapterStatus, ProjectConfig, Thresholds,
};
use shared::common::taxonomy_adapter_name_vo::AdapterName;
use shared::common::taxonomy_common_vo::{Count, Score};

// ─── is_adapter_enabled ─────────────────────────────────────────────────────

#[test]
fn enabled_adapter_returns_true() {
    let config = ProjectConfig {
        adapters: vec![
            AdapterEntry::enabled(AdapterName::raw("ruff")),
        ],
        ..ProjectConfig::defaults()
    };
    let validator = ConfigRulesValidator::new(config);
    assert!(validator.is_adapter_enabled(&AdapterName::raw("ruff")));
}

#[test]
fn disabled_adapter_returns_false() {
    let config = ProjectConfig {
        adapters: vec![
            AdapterEntry::new(AdapterName::raw("mypy"), AdapterStatus::Disabled, 1.0),
        ],
        ..ProjectConfig::defaults()
    };
    let validator = ConfigRulesValidator::new(config);
    assert!(!validator.is_adapter_enabled(&AdapterName::raw("mypy")));
}

#[test]
fn unlisted_adapter_defaults_to_enabled() {
    let config = ProjectConfig::defaults();
    let validator = ConfigRulesValidator::new(config);
    assert!(validator.is_adapter_enabled(&AdapterName::raw("unknown_tool")));
}

#[test]
fn multiple_adapters_checked_independently() {
    let config = ProjectConfig {
        adapters: vec![
            AdapterEntry::enabled(AdapterName::raw("ruff")),
            AdapterEntry::new(AdapterName::raw("mypy"), AdapterStatus::Disabled, 1.0),
            AdapterEntry::new(AdapterName::raw("bandit"), AdapterStatus::NotInstalled, 1.0),
        ],
        ..ProjectConfig::defaults()
    };
    let validator = ConfigRulesValidator::new(config);
    assert!(validator.is_adapter_enabled(&AdapterName::raw("ruff")));
    assert!(!validator.is_adapter_enabled(&AdapterName::raw("mypy")));
    assert!(!validator.is_adapter_enabled(&AdapterName::raw("bandit")));
}

// ─── validate_thresholds ───────────────────────────────────────────────────

#[test]
fn default_thresholds_are_valid() {
    let config = ProjectConfig::defaults();
    let validator = ConfigRulesValidator::new(config);
    let result = validator.validate_thresholds();
    assert!(result.is_valid);
}

#[test]
fn score_below_zero_is_invalid() {
    let config = ProjectConfig {
        thresholds: Thresholds::new(Score::new(-1.0), Count::new(10), Count::new(500)),
        ..ProjectConfig::defaults()
    };
    let validator = ConfigRulesValidator::new(config);
    let result = validator.validate_thresholds();
    assert!(!result.is_valid);
}

#[test]
fn score_above_100_is_invalid() {
    let config = ProjectConfig {
        thresholds: Thresholds::new(Score::new(150.0), Count::new(10), Count::new(500)),
        ..ProjectConfig::defaults()
    };
    let validator = ConfigRulesValidator::new(config);
    let result = validator.validate_thresholds();
    assert!(!result.is_valid);
}

#[test]
fn zero_complexity_is_invalid() {
    let config = ProjectConfig {
        thresholds: Thresholds::new(Score::new(80.0), Count::new(0), Count::new(500)),
        ..ProjectConfig::defaults()
    };
    let validator = ConfigRulesValidator::new(config);
    let result = validator.validate_thresholds();
    assert!(!result.is_valid);
}

#[test]
fn zero_max_file_lines_is_invalid() {
    let config = ProjectConfig {
        thresholds: Thresholds::new(Score::new(80.0), Count::new(10), Count::new(0)),
        ..ProjectConfig::defaults()
    };
    let validator = ConfigRulesValidator::new(config);
    let result = validator.validate_thresholds();
    assert!(!result.is_valid);
}

#[test]
fn boundary_values_are_valid() {
    let config = ProjectConfig {
        thresholds: Thresholds::new(Score::new(0.0), Count::new(1), Count::new(1)),
        ..ProjectConfig::defaults()
    };
    let validator = ConfigRulesValidator::new(config);
    let result = validator.validate_thresholds();
    assert!(result.is_valid);
}

#[test]
fn max_score_is_valid() {
    let config = ProjectConfig {
        thresholds: Thresholds::new(Score::new(100.0), Count::new(10), Count::new(500)),
        ..ProjectConfig::defaults()
    };
    let validator = ConfigRulesValidator::new(config);
    let result = validator.validate_thresholds();
    assert!(result.is_valid);
}
