// Unit tests — shared/config_system taxonomy types.
use shared_common::taxonomy_language_vo::ConfigLanguage;
use shared_config_system::taxonomy_config_system_error::ConfigError;
use shared_config_system::taxonomy_config_system_vo::ConfigKey;
use shared_config_system::taxonomy_config_system_vo::ValidationResult;
use shared_config_system::taxonomy_config_system_vo::WorkspaceInfo;
use shared_config_system::taxonomy_config_system_vo::{
    AdapterEntry, AdapterStatus, ProjectConfig, Thresholds,
};
use shared_config_system::taxonomy_config_system_vo::{
    ArchitectureConfig, ArchitectureRule, NamingRuleVO, RoleRuleVO,
};
use shared_config_system::taxonomy_config_system_vo::{ConfigResult, ConfigSource};
use shared_config_system::{
    ConfigKey as ConfigKeyRe, ConfigLanguage as ConfigLanguageRe, ProjectConfig as ProjectConfigRe,
};
use std::str::FromStr;

// ── ConfigLanguage ──────────────────────────────────────────
#[test]
fn config_language_as_str() {
    assert_eq!(ConfigLanguage::Rust.as_str(), "rust");
    assert_eq!(ConfigLanguage::Python.as_str(), "python");
    assert_eq!(ConfigLanguage::TypeScript.as_str(), "typescript");
}

#[test]
fn config_language_from_str_cases() {
    assert_eq!(
        ConfigLanguage::from_str("RUST").expect("parses"),
        ConfigLanguage::Rust
    );
    assert_eq!(
        ConfigLanguage::from_str(" ts ").expect("parses"),
        ConfigLanguage::TypeScript
    );
    assert_eq!(
        ConfigLanguage::from_str("js").expect("parses"),
        ConfigLanguage::TypeScript
    );
    assert!(ConfigLanguage::from_str("go").is_err());
}

#[test]
fn config_language_file_names_per_language() {
    assert_eq!(
        ConfigLanguage::Rust.config_file_names(),
        &["lint_arwaky.config.yaml"]
    );
    assert_eq!(
        ConfigLanguage::Python.config_file_names(),
        &["lint_arwaky.config.yaml"]
    );
    assert!(
        ConfigLanguage::TypeScript
            .config_file_names()
            .contains(&"lint_arwaky.config.yaml")
    );
}

// ── Thresholds / AdapterEntry / ProjectConfig ───────────────
#[test]
fn thresholds_default_values() {
    let t = Thresholds::default();
    assert!((t.score.value() - 80.0).abs() < 1e-9);
    assert_eq!(t.max_file_lines.value(), 500);
}

#[test]
fn adapter_status_variants() {
    assert_eq!(AdapterStatus::default(), AdapterStatus::Enabled);
    assert_eq!(AdapterStatus::Enabled.as_str(), "enabled");
    assert_eq!(AdapterStatus::Disabled.to_string(), "disabled");
    assert_eq!(AdapterStatus::NotInstalled.to_string(), "not_installed");
}

#[test]
fn adapter_entry_helpers() {
    let entry = AdapterEntry::enabled(shared_common::AdapterName::raw("ruff"));
    assert!(entry.is_active());
    assert!((entry.timeout - 60.0).abs() < 1e-9);
    let custom = AdapterEntry::with_timeout(
        shared_common::AdapterName::raw("mypy"),
        AdapterStatus::Disabled,
        2.0,
        30.0,
    );
    assert!(!custom.is_active());
    assert!((custom.timeout - 30.0).abs() < 1e-9);
    assert!((custom.weight - 2.0).abs() < 1e-9);
}

#[test]
fn project_config_defaults_has_adapters() {
    let config = ProjectConfig::defaults();
    assert_eq!(config.project_name.value, "lint-arwaky");
    assert_eq!(config.adapters.len(), 4);
    assert!(config.adapters.iter().all(|a| a.is_active()));
    assert_eq!(config.thresholds.score.value(), 80.0);
    assert_eq!(config.thresholds.max_file_lines.value(), 500);
}

#[test]
fn project_config_default_is_empty() {
    let config = ProjectConfig::default();
    assert!(config.adapters.is_empty());
    assert!(config.ignored_paths.is_empty());
}

// ── ArchitectureConfig / rules ──────────────────────────────
#[test]
fn architecture_config_defaults() {
    let config = ArchitectureConfig::default();
    assert!(config.enabled.value());
    assert!(config.layers.is_empty());
    assert!(config.rules.is_empty());
    assert_eq!(config.naming.word_count.value(), 3);
    assert!(!config.mandatory_class_definition.value());
}

#[test]
fn architecture_rule_default_flags() {
    let rule = ArchitectureRule::default();
    assert!(!rule.enabled.value());
    assert!(rule.allowed.is_empty());
    assert!(rule.exceptions.is_empty());
    assert_eq!(rule.name.value, "");
}

#[test]
fn naming_rule_vo_defaults() {
    let rule = NamingRuleVO::default();
    assert!(!rule.naming_convention.value());
    assert_eq!(rule.suffix_policy.value, "");
}

#[test]
fn role_rule_vo_defaults() {
    let rule = RoleRuleVO::default();
    assert!(!rule.no_domain_logic.value());
    assert!(rule.forbidden_inheritance.is_empty());
}

// ── ConfigSource / ConfigResult ─────────────────────────────
#[test]
fn config_source_new_normalizes_path() {
    let source = ConfigSource::new("rust", "/tmp/proj/", "raw yaml");
    assert_eq!(source.language, "rust");
    assert_eq!(source.path.value(), "/tmp/proj");
    assert_eq!(source.raw_content, "raw yaml");
}

#[test]
fn config_result_new() {
    let source = ConfigSource::new("rust", "x", "y");
    let result = ConfigResult::new(
        ArchitectureConfig::default(),
        source,
        vec!["w1".to_string()],
    );
    assert_eq!(result.warnings.len(), 1);
    assert!(result.config.enabled.value());
}

// ── ValidationResult ────────────────────────────────────────
#[test]
fn validation_result_ok_and_fail() {
    let ok = ValidationResult::ok();
    assert!(ok.is_valid);
    assert!(ok.reason.is_none());
    let fail = ValidationResult::fail("threshold too low");
    assert!(!fail.is_valid);
    assert_eq!(fail.reason.as_deref(), Some("threshold too low"));
}

// ── ConfigError ─────────────────────────────────────────────
#[test]
fn config_error_display() {
    let error = ConfigError::new(
        ConfigKey::new("thresholds.score"),
        shared_common::ErrorMessage::new("below minimum"),
    );
    let rendered = error.to_string();
    assert!(rendered.contains("thresholds.score"));
    assert!(rendered.contains("below minimum"));
}

#[test]
fn config_error_default_fields() {
    let error = ConfigError::new(ConfigKey::new("x"), shared_common::ErrorMessage::new("m"));
    assert_eq!(error.config_file.value(), "");
}

// ── ConfigKey ───────────────────────────────────────────────
#[test]
fn config_key_parts_parent_leaf() {
    let key = ConfigKey::new("architecture.layers.taxonomy");
    assert_eq!(key.parts(), vec!["architecture", "layers", "taxonomy"]);
    assert_eq!(key.parent(), "architecture.layers");
    assert_eq!(key.leaf(), "taxonomy");
    let single = ConfigKey::new("root");
    assert_eq!(single.parent(), "");
    assert_eq!(single.leaf(), "root");
}

// ── WorkspaceInfo ───────────────────────────────────────────
#[test]
fn workspace_info_new() {
    let info = WorkspaceInfo::new(
        shared_common::FilePath::new("crates/core").expect("path"),
        "crate".to_string(),
        ArchitectureConfig::default(),
    );
    assert_eq!(info.workspace_type, "crate");
    assert_eq!(info.path.basename(), "core");
}

// ── Barrel re-exports resolve to the same types ─────────────
#[test]
fn barrel_reexports_are_same_types() {
    assert_eq!(ConfigLanguageRe::Rust, ConfigLanguage::Rust);
    assert_eq!(
        ProjectConfigRe::defaults().project_name.value,
        ProjectConfig::defaults().project_name.value
    );
    let key = ConfigKeyRe::new("a.b");
    assert_eq!(key.leaf(), "b");
}

// ── KNOWN whitelist pin: new CodeAnalysisRuleVO fields must not warn ───────

#[test]
fn unknown_rule_key_warnings_omit_code_analysis_rule_vo_fields() {
    use shared_config_system::utility_config_parser::parse_config_yaml_with_warnings;
    use shared_quality_rules::CodeAnalysisRuleVO;

    let yaml = r#"
architecture:
  enabled: true
  rules:
    - name: AES305
      enabled: true
      min_lines: 5
      duplication_threshold: 75.0
      identical_threshold: 80.0
      similar_threshold: 50.0
"#;

    let (_, warnings) = parse_config_yaml_with_warnings(yaml);
    let unknown: Vec<String> = warnings
        .iter()
        .filter(|w| w.contains("Unknown configuration key"))
        .cloned()
        .collect();
    assert!(
        unknown.is_empty(),
        "CodeAnalysisRuleVO fields must be in the KNOWN whitelist, got: {unknown:?}"
    );

    // Round-trip: the parsed config must carry the new fields.
    let (config, _) = parse_config_yaml_with_warnings(yaml);
    let rule = config
        .rules
        .iter()
        .find(|r| r.name.value == "AES305")
        .expect("AES305 rule present");
    let expected = CodeAnalysisRuleVO {
        identical_threshold: Some(80.0),
        similar_threshold: Some(50.0),
        duplication_threshold: Some(75.0),
        ..Default::default()
    };
    assert_eq!(
        rule.code_analysis.identical_threshold,
        expected.identical_threshold
    );
    assert_eq!(
        rule.code_analysis.similar_threshold,
        expected.similar_threshold
    );
    assert_eq!(
        rule.code_analysis.duplication_threshold,
        expected.duplication_threshold
    );
}
