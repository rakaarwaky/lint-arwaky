use config_system_lint_arwaky::capabilities_rules_validator::ConfigRulesValidator;
use shared::common::taxonomy_adapter_name_vo::AdapterName;
use shared::common::taxonomy_common_vo::{Count, Score};
use shared::config_system::contract_validator_protocol::IConfigValidatorProtocol;
use shared::config_system::taxonomy_setting_vo::{
    AdapterEntry, AdapterStatus, ProjectConfig, Thresholds,
};

// ─── is_adapter_enabled ─────────────────────────────────────────────────────

#[test]
fn enabled_adapter_returns_true() {
    let config = ProjectConfig {
        adapters: vec![AdapterEntry::enabled(AdapterName::raw("ruff"))],
        ..ProjectConfig::defaults()
    };
    let validator = ConfigRulesValidator::new();
    assert!(validator.is_adapter_enabled(&config, &AdapterName::raw("ruff")));
}

#[test]
fn disabled_adapter_returns_false() {
    let config = ProjectConfig {
        adapters: vec![AdapterEntry::new(
            AdapterName::raw("mypy"),
            AdapterStatus::Disabled,
            1.0,
        )],
        ..ProjectConfig::defaults()
    };
    let validator = ConfigRulesValidator::new();
    assert!(!validator.is_adapter_enabled(&config, &AdapterName::raw("mypy")));
}

#[test]
fn unlisted_adapter_defaults_to_enabled() {
    let config = ProjectConfig::defaults();
    let validator = ConfigRulesValidator::new();
    assert!(validator.is_adapter_enabled(&config, &AdapterName::raw("unknown_tool")));
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
    let validator = ConfigRulesValidator::new();
    assert!(validator.is_adapter_enabled(&config, &AdapterName::raw("ruff")));
    assert!(!validator.is_adapter_enabled(&config, &AdapterName::raw("mypy")));
    assert!(!validator.is_adapter_enabled(&config, &AdapterName::raw("bandit")));
}

// ─── validate_thresholds ───────────────────────────────────────────────────

#[test]
fn default_thresholds_are_valid() {
    let config = ProjectConfig::defaults();
    let validator = ConfigRulesValidator::new();
    let result = validator.validate_thresholds(&config);
    assert!(result.is_valid);
}

#[test]
fn score_below_zero_is_invalid() {
    let config = ProjectConfig {
        thresholds: Thresholds::new(Score::new(-1.0), Count::new(10), Count::new(500)),
        ..ProjectConfig::defaults()
    };
    let validator = ConfigRulesValidator::new();
    let result = validator.validate_thresholds(&config);
    assert!(!result.is_valid);
}

#[test]
fn score_above_100_is_invalid() {
    let config = ProjectConfig {
        thresholds: Thresholds::new(Score::new(150.0), Count::new(10), Count::new(500)),
        ..ProjectConfig::defaults()
    };
    let validator = ConfigRulesValidator::new();
    let result = validator.validate_thresholds(&config);
    assert!(!result.is_valid);
}

#[test]
fn zero_complexity_is_invalid() {
    let config = ProjectConfig {
        thresholds: Thresholds::new(Score::new(80.0), Count::new(0), Count::new(500)),
        ..ProjectConfig::defaults()
    };
    let validator = ConfigRulesValidator::new();
    let result = validator.validate_thresholds(&config);
    assert!(!result.is_valid);
}

#[test]
fn zero_max_file_lines_is_invalid() {
    let config = ProjectConfig {
        thresholds: Thresholds::new(Score::new(80.0), Count::new(10), Count::new(0)),
        ..ProjectConfig::defaults()
    };
    let validator = ConfigRulesValidator::new();
    let result = validator.validate_thresholds(&config);
    assert!(!result.is_valid);
}

#[test]
fn boundary_values_are_valid() {
    let config = ProjectConfig {
        thresholds: Thresholds::new(Score::new(0.0), Count::new(1), Count::new(1)),
        ..ProjectConfig::defaults()
    };
    let validator = ConfigRulesValidator::new();
    let result = validator.validate_thresholds(&config);
    assert!(result.is_valid);
}

#[test]
fn max_score_is_valid() {
    let config = ProjectConfig {
        thresholds: Thresholds::new(Score::new(100.0), Count::new(10), Count::new(500)),
        ..ProjectConfig::defaults()
    };
    let validator = ConfigRulesValidator::new();
    let result = validator.validate_thresholds(&config);
    assert!(result.is_valid);
}

#[test]
fn multiple_invalid_thresholds_reported() {
    let config = ProjectConfig {
        thresholds: Thresholds::new(Score::new(-1.0), Count::new(0), Count::new(0)),
        ..ProjectConfig::defaults()
    };
    let validator = ConfigRulesValidator::new();
    let result = validator.validate_thresholds(&config);
    assert!(!result.is_valid);
    let reason = result.reason.unwrap_or_default();
    assert!(reason.contains("Score threshold"));
    assert!(reason.contains("Complexity threshold"));
    assert!(reason.contains("max_file_lines threshold"));
}
