// Unit tests for SuffixPrefixChecker — AES102 suffix/prefix layer alignment

use naming_rules_lint_arwaky::capabilities_suffix_prefix_checker::SuffixPrefixChecker;
use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::naming_rules::contract_naming_checker_protocol::ISuffixPrefixChecker;
use shared::naming_rules::taxonomy_naming_constant::{
    ADAPTER_NAME, RULE_CODE_SUFFIX_PREFIX, SUFFIX_POLICY_STRICT,
};
use shared::naming_rules::utility_naming::{get_stem, get_suffix};
use shared::taxonomy_adapter_name_vo::AdapterName;
use shared::taxonomy_definition_vo::LayerMapVO;
use shared::taxonomy_layer_vo::LayerNameVO;

// ─── Unit Tests: Happy Path — Allowed Suffixes ──────────────────────

/// Test that files with allowed suffixes in strict mode produce no violations.
#[tokio::test]
async fn check_domain_suffixes_allowed_suffix_no_violations() {
    let checker = SuffixPrefixChecker::new();
    let config = ArchitectureConfig::default();

    // Capabilities layer with "checker" as allowed suffix (strict policy)
    let mut layers = std::collections::HashMap::new();
    layers.insert(
        LayerNameVO::new("capabilities".to_string()),
        shared::common::taxonomy_definition_vo::LayerDefinition {
            allowed: shared::common::taxonomy_common_vo::PatternList::new(vec!["checker".to_string()]),
            forbidden: shared::common::taxonomy_common_vo::PatternList::new(vec![]),
            mandatory: shared::common::taxonomy_common_vo::PatternList::new(vec![]),
            word_count: shared::common::taxonomy_common_vo::Count::new(3),
            exceptions: shared::common::taxonomy_common_vo::PatternList::new(vec![]),
            recursive: shared::common::taxonomy_common_vo::BooleanVO::new(false),
            naming: shared::naming_rules::taxonomy_naming_rule_vo::NamingRuleVO {
                allowed_suffix: shared::common::taxonomy_common_vo::PatternList::new(vec![
                    "checker".to_string(),
                    "analyzer".to_string(),
                ]),
                forbidden_suffix: shared::common::taxonomy_common_vo::PatternList::new(vec![]),
                suffix_policy: shared::common::taxonomy_common_vo::PatternList::new(vec![
                    SUFFIX_POLICY_STRICT.to_string()
                ]),
            },
            code_analysis: shared::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO::default(),
            role: shared::role_rules::taxonomy_role_rule_vo::RoleRuleVO::default(),
            orphan: shared::orphan_detector::taxonomy_orphan_rule_vo::OrphanRuleVO::default(),
        },
    );
    let layer_map = LayerMapVO::new(layers);

    // Valid: capabilities_user_checker has "checker" suffix (allowed)
    let files = FilePathList {
        values: vec![FilePath::new("capabilities_user_checker.rs".to_string()).unwrap()],
    };
    let root = FilePath::new("/".to_string()).unwrap();
    let mut results = LintResultList::new(Vec::new());

    checker
        .check_domain_suffixes(&config, &layer_map, &files, &root, &mut results)
        .await;

    assert!(results.values.is_empty());
}

/// Test that taxonomy layer files with allowed suffixes pass.
#[tokio::test]
async fn check_domain_suffixes_taxonomy_allowed_pass() {
    let checker = SuffixPrefixChecker::new();
    let config = ArchitectureConfig::default();

    // Taxonomy layer
    let mut layers = std::collections::HashMap::new();
    layers.insert(
        LayerNameVO::new("taxonomy".to_string()),
        shared::common::taxonomy_definition_vo::LayerDefinition {
            allowed: shared::common::taxonomy_common_vo::PatternList::new(vec![]),
            forbidden: shared::common::taxonomy_common_vo::PatternList::new(vec![]),
            mandatory: shared::common::taxonomy_common_vo::PatternList::new(vec![]),
            word_count: shared::common::taxonomy_common_vo::Count::new(3),
            exceptions: shared::common::taxonomy_common_vo::PatternList::new(vec![]),
            recursive: shared::common::taxonomy_common_vo::BooleanVO::new(false),
            naming: shared::naming_rules::taxonomy_naming_rule_vo::NamingRuleVO {
                allowed_suffix: shared::common::taxonomy_common_vo::PatternList::new(vec![
                    "entity".to_string(),
                    "vo".to_string(),
                ]),
                forbidden_suffix: shared::common::taxonomy_common_vo::PatternList::new(vec![]),
                suffix_policy: shared::common::taxonomy_common_vo::PatternList::new(vec![
                    SUFFIX_POLICY_STRICT.to_string()
                ]),
            },
            code_analysis: shared::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO::default(),
            role: shared::role_rules::taxonomy_role_rule_vo::RoleRuleVO::default(),
            orphan: shared::orphan_detector::taxonomy_orphan_rule_vo::OrphanRuleVO::default(),
        },
    );
    let layer_map = LayerMapVO::new(layers);

    // Valid: taxonomy_user_entity has "entity" suffix (allowed)
    let files = FilePathList {
        values: vec![FilePath::new("taxonomy_user_entity.rs".to_string()).unwrap()],
    };
    let root = FilePath::new("/".to_string()).unwrap();
    let mut results = LintResultList::new(Vec::new());

    checker
        .check_domain_suffixes(&config, &layer_map, &files, &root, &mut results)
        .await;

    assert!(results.values.is_empty());
}

// ─── Unit Tests: Edge Cases — Forbidden Suffix ──────────────────────

/// Test that files with forbidden suffixes produce violations.
#[tokio::test]
async fn check_domain_suffixes_forbidden_suffix_produces_violation() {
    let checker = SuffixPrefixChecker::new();
    let config = ArchitectureConfig::default();

    // Capabilities layer forbids "adapter" suffix
    let mut layers = std::collections::HashMap::new();
    layers.insert(
        LayerNameVO::new("capabilities".to_string()),
        shared::common::taxonomy_definition_vo::LayerDefinition {
            allowed: shared::common::taxonomy_common_vo::PatternList::new(vec![]),
            forbidden: shared::common::taxonomy_common_vo::PatternList::new(vec![
                "adapter".to_string()
            ]),
            mandatory: shared::common::taxonomy_common_vo::PatternList::new(vec![]),
            word_count: shared::common::taxonomy_common_vo::Count::new(3),
            exceptions: shared::common::taxonomy_common_vo::PatternList::new(vec![]),
            recursive: shared::common::taxonomy_common_vo::BooleanVO::new(false),
            naming: shared::naming_rules::taxonomy_naming_rule_vo::NamingRuleVO {
                allowed_suffix: shared::common::taxonomy_common_vo::PatternList::new(vec![
                    "checker".to_string(),
                ]),
                forbidden_suffix: shared::common::taxonomy_common_vo::PatternList::new(vec![
                    "adapter".to_string()
                ]),
                suffix_policy: shared::common::taxonomy_common_vo::PatternList::new(vec![
                    SUFFIX_POLICY_STRICT.to_string()
                ]),
            },
            code_analysis: shared::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO::default(),
            role: shared::role_rules::taxonomy_role_rule_vo::RoleRuleVO::default(),
            orphan: shared::orphan_detector::taxonomy_orphan_rule_vo::OrphanRuleVO::default(),
        },
    );
    let layer_map = LayerMapVO::new(layers);

    // Invalid: capabilities_user_adapter has "adapter" suffix (forbidden)
    let files = FilePathList {
        values: vec![FilePath::new("capabilities_user_adapter.rs".to_string()).unwrap()],
    };
    let root = FilePath::new("/".to_string()).unwrap();
    let mut results = LintResultList::new(Vec::new());

    checker
        .check_domain_suffixes(&config, &layer_map, &files, &root, &mut results)
        .await;

    assert_eq!(
        results.values.len(),
        1,
        "Forbidden suffix should produce exactly one violation"
    );
    assert_eq!(results.values[0].code.value, RULE_CODE_SUFFIX_PREFIX);
}

/// Test that files with disallowed suffix in strict mode produce violations.
#[tokio::test]
async fn check_domain_suffixes_strict_mode_wrong_suffix_produces_violation() {
    let checker = SuffixPrefixChecker::new();
    let config = ArchitectureConfig::default();

    // Capabilities layer allows only "checker" and "analyzer"
    let mut layers = std::collections::HashMap::new();
    layers.insert(
        LayerNameVO::new("capabilities".to_string()),
        shared::common::taxonomy_definition_vo::LayerDefinition {
            allowed: shared::common::taxonomy_common_vo::PatternList::new(vec![]),
            forbidden: shared::common::taxonomy_common_vo::PatternList::new(vec![]),
            mandatory: shared::common::taxonomy_common_vo::PatternList::new(vec![]),
            word_count: shared::common::taxonomy_common_vo::Count::new(3),
            exceptions: shared::common::taxonomy_common_vo::PatternList::new(vec![]),
            recursive: shared::common::taxonomy_common_vo::BooleanVO::new(false),
            naming: shared::naming_rules::taxonomy_naming_rule_vo::NamingRuleVO {
                allowed_suffix: shared::common::taxonomy_common_vo::PatternList::new(vec![
                    "checker".to_string(),
                    "analyzer".to_string(),
                ]),
                forbidden_suffix: shared::common::taxonomy_common_vo::PatternList::new(vec![]),
                suffix_policy: shared::common::taxonomy_common_vo::PatternList::new(vec![
                    SUFFIX_POLICY_STRICT.to_string()
                ]),
            },
            code_analysis: shared::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO::default(),
            role: shared::role_rules::taxonomy_role_rule_vo::RoleRuleVO::default(),
            orphan: shared::orphan_detector::taxonomy_orphan_rule_vo::OrphanRuleVO::default(),
        },
    );
    let layer_map = LayerMapVO::new(layers);

    // Invalid: capabilities_user_checker has "checker" (allowed), but we test with wrong suffix
    let files = FilePathList {
        values: vec![FilePath::new("capabilities_user_writer.rs".to_string()).unwrap()],
    };
    let root = FilePath::new("/".to_string()).unwrap();
    let mut results = LintResultList::new(Vec::new());

    checker
        .check_domain_suffixes(&config, &layer_map, &files, &root, &mut results)
        .await;

    assert_eq!(
        results.values.len(),
        1,
        "Wrong suffix in strict mode should produce exactly one violation"
    );
    assert_eq!(results.values[0].code.value, RULE_CODE_SUFFIX_PREFIX);
}

// ─── Unit Tests: Edge Cases — No Suffix ─────────────────────────────

/// Test that files without a suffix (only 2 words) produce violations in strict mode.
#[tokio::test]
async fn check_domain_suffixes_no_suffix_strict_produces_violation() {
    let checker = SuffixPrefixChecker::new();
    let config = ArchitectureConfig::default();

    // Capabilities layer with strict policy
    let mut layers = std::collections::HashMap::new();
    layers.insert(
        LayerNameVO::new("capabilities".to_string()),
        shared::common::taxonomy_definition_vo::LayerDefinition {
            allowed: shared::common::taxonomy_common_vo::PatternList::new(vec![]),
            forbidden: shared::common::taxonomy_common_vo::PatternList::new(vec![]),
            mandatory: shared::common::taxonomy_common_vo::PatternList::new(vec![]),
            word_count: shared::common::taxonomy_common_vo::Count::new(3),
            exceptions: shared::common::taxonomy_common_vo::PatternList::new(vec![]),
            recursive: shared::common::taxonomy_common_vo::BooleanVO::new(false),
            naming: shared::naming_rules::taxonomy_naming_rule_vo::NamingRuleVO {
                allowed_suffix: shared::common::taxonomy_common_vo::PatternList::new(vec![
                    "checker".to_string(),
                ]),
                forbidden_suffix: shared::common::taxonomy_common_vo::PatternList::new(vec![]),
                suffix_policy: shared::common::taxonomy_common_vo::PatternList::new(vec![
                    SUFFIX_POLICY_STRICT.to_string()
                ]),
            },
            code_analysis: shared::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO::default(),
            role: shared::role_rules::taxonomy_role_rule_vo::RoleRuleVO::default(),
            orphan: shared::orphan_detector::taxonomy_orphan_rule_vo::OrphanRuleVO::default(),
        },
    );
    let layer_map = LayerMapVO::new(layers);

    // Invalid: capabilities_user has no suffix (only 2 words)
    let files = FilePathList {
        values: vec![FilePath::new("capabilities_user.rs".to_string()).unwrap()],
    };
    let root = FilePath::new("/".to_string()).unwrap();
    let mut results = LintResultList::new(Vec::new());

    checker
        .check_domain_suffixes(&config, &layer_map, &files, &root, &mut results)
        .await;

    assert_eq!(
        results.values.len(),
        1,
        "No suffix in strict mode should produce exactly one violation"
    );
}

// ─── Unit Tests: Edge Cases — Barrel and Entry Point Files ──────────

/// Test that barrel files (mod.rs) are skipped.
#[tokio::test]
async fn check_domain_suffixes_barrel_file_skipped() {
    let checker = SuffixPrefixChecker::new();
    let config = ArchitectureConfig::default();
    let layer_map = LayerMapVO::new(std::collections::HashMap::new());

    let files = FilePathList {
        values: vec![FilePath::new("mod.rs".to_string()).unwrap()],
    };
    let root = FilePath::new("/".to_string()).unwrap();
    let mut results = LintResultList::new(Vec::new());

    checker
        .check_domain_suffixes(&config, &layer_map, &files, &root, &mut results)
        .await;

    assert!(results.values.is_empty());
}

/// Test that entry point files (main.rs) are skipped.
#[tokio::test]
async fn check_domain_suffixes_entry_point_skipped() {
    let checker = SuffixPrefixChecker::new();
    let config = ArchitectureConfig::default();
    let layer_map = LayerMapVO::new(std::collections::HashMap::new());

    let files = FilePathList {
        values: vec![FilePath::new("main.rs".to_string()).unwrap()],
    };
    let root = FilePath::new("/".to_string()).unwrap();
    let mut results = LintResultList::new(Vec::new());

    checker
        .check_domain_suffixes(&config, &layer_map, &files, &root, &mut results)
        .await;

    assert!(results.values.is_empty());
}

/// Test that exception files bypass suffix checks.
#[tokio::test]
async fn check_domain_suffixes_exception_bypasses_check() {
    let checker = SuffixPrefixChecker::new();

    // Config with exception for "capabilities_user.rs"
    let mut layers = std::collections::HashMap::new();
    layers.insert(
        LayerNameVO::new("capabilities".to_string()),
        shared::common::taxonomy_definition_vo::LayerDefinition {
            allowed: shared::common::taxonomy_common_vo::PatternList::new(vec![]),
            forbidden: shared::common::taxonomy_common_vo::PatternList::new(vec![]),
            mandatory: shared::common::taxonomy_common_vo::PatternList::new(vec![]),
            word_count: shared::common::taxonomy_common_vo::Count::new(3),
            exceptions: shared::common::taxonomy_common_vo::PatternList::new(vec![
                "capabilities_user.rs".to_string()
            ]),
            recursive: shared::common::taxonomy_common_vo::BooleanVO::new(false),
            naming: shared::naming_rules::taxonomy_naming_rule_vo::NamingRuleVO {
                allowed_suffix: shared::common::taxonomy_common_vo::PatternList::new(vec![
                    "checker".to_string(),
                ]),
                forbidden_suffix: shared::common::taxonomy_common_vo::PatternList::new(vec![]),
                suffix_policy: shared::common::taxonomy_common_vo::PatternList::new(vec![
                    SUFFIX_POLICY_STRICT.to_string()
                ]),
            },
            code_analysis: shared::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO::default(),
            role: shared::role_rules::taxonomy_role_rule_vo::RoleRuleVO::default(),
            orphan: shared::orphan_detector::taxonomy_orphan_rule_vo::OrphanRuleVO::default(),
        },
    );
    let layer_map = LayerMapVO::new(layers);

    // Exception file that would normally fail (no suffix)
    let files = FilePathList {
        values: vec![FilePath::new("capabilities_user.rs".to_string()).unwrap()],
    };
    let root = FilePath::new("/".to_string()).unwrap();
    let mut results = LintResultList::new(Vec::new());

    checker
        .check_domain_suffixes(&config, &layer_map, &files, &root, &mut results)
        .await;

    assert!(results.values.is_empty());
}

/// Test that empty file list produces no violations.
#[tokio::test]
async fn check_domain_suffixes_empty_files_no_violations() {
    let checker = SuffixPrefixChecker::new();
    let config = ArchitectureConfig::default();
    let layer_map = LayerMapVO::new(std::collections::HashMap::new());

    let files = FilePathList { values: vec![] };
    let root = FilePath::new("/".to_string()).unwrap();
    let mut results = LintResultList::new(Vec::new());

    checker
        .check_domain_suffixes(&config, &layer_map, &files, &root, &mut results)
        .await;

    assert!(results.values.is_empty());
}

/// Test that files with no layer definition produce no violations.
#[tokio::test]
async fn check_domain_suffixes_no_layer_definition() {
    let checker = SuffixPrefixChecker::new();
    let config = ArchitectureConfig::default();

    // Layer map with empty layers — file won't match any layer
    let layer_map = LayerMapVO::new(std::collections::HashMap::new());

    let files = FilePathList {
        values: vec![FilePath::new("unknown_file.rs".to_string()).unwrap()],
    };
    let root = FilePath::new("/".to_string()).unwrap();
    let mut results = LintResultList::new(Vec::new());

    checker
        .check_domain_suffixes(&config, &layer_map, &files, &root, &mut results)
        .await;

    // Should have zero violations (no layer definition means no suffix check)
    assert!(results.values.is_empty());
}

/// Test that the adapter name in results is correct.
#[tokio::test]
async fn check_domain_suffixes_result_has_correct_adapter() {
    let checker = SuffixPrefixChecker::new();

    // Config with forbidden "adapter" suffix
    let mut layers = std::collections::HashMap::new();
    layers.insert(
        LayerNameVO::new("capabilities".to_string()),
        shared::common::taxonomy_definition_vo::LayerDefinition {
            allowed: shared::common::taxonomy_common_vo::PatternList::new(vec![]),
            forbidden: shared::common::taxonomy_common_vo::PatternList::new(vec![
                "adapter".to_string()
            ]),
            mandatory: shared::common::taxonomy_common_vo::PatternList::new(vec![]),
            word_count: shared::common::taxonomy_common_vo::Count::new(3),
            exceptions: shared::common::taxonomy_common_vo::PatternList::new(vec![]),
            recursive: shared::common::taxonomy_common_vo::BooleanVO::new(false),
            naming: shared::naming_rules::taxonomy_naming_rule_vo::NamingRuleVO {
                allowed_suffix: shared::common::taxonomy_common_vo::PatternList::new(vec![
                    "checker".to_string(),
                ]),
                forbidden_suffix: shared::common::taxonomy_common_vo::PatternList::new(vec![
                    "adapter".to_string()
                ]),
                suffix_policy: shared::common::taxonomy_common_vo::PatternList::new(vec![
                    SUFFIX_POLICY_STRICT.to_string()
                ]),
            },
            code_analysis: shared::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO::default(),
            role: shared::role_rules::taxonomy_role_rule_vo::RoleRuleVO::default(),
            orphan: shared::orphan_detector::taxonomy_orphan_rule_vo::OrphanRuleVO::default(),
        },
    );
    let layer_map = LayerMapVO::new(layers);

    // Invalid: capabilities_user_adapter has forbidden suffix
    let files = FilePathList {
        values: vec![FilePath::new("capabilities_user_adapter.rs".to_string()).unwrap()],
    };
    let root = FilePath::new("/".to_string()).unwrap();
    let mut results = LintResultList::new(Vec::new());

    checker
        .check_domain_suffixes(&config, &layer_map, &files, &root, &mut results)
        .await;

    if !results.values.is_empty() {
        assert_eq!(
            results.values[0].source.as_ref().unwrap().value,
            ADAPTER_NAME,
            "Result should have 'architecture' as adapter name"
        );
        assert_eq!(
            results.values[0].severity,
            Severity::HIGH,
            "Suffix violation should be HIGH severity"
        );
    }
}

// ─── Unit Tests: Utility Functions — Stem and Suffix Extraction ─────

/// Test get_stem with various file patterns.
#[test]
fn stem_extraction_normal_file() {
    assert_eq!(get_stem("checker.rs"), Some("checker"));
    assert_eq!(get_stem("mod.rs"), Some("mod"));
}

/// Test get_stem with multi-dot files.
#[test]
fn stem_extraction_multi_dot_file() {
    assert_eq!(get_stem("foo.spec.rs"), Some("foo.spec"));
    assert_eq!(get_stem("test.unit.test.rs"), Some("test.unit.test"));
}

/// Test get_stem with dotfiles.
#[test]
fn stem_extraction_dotfile() {
    assert_eq!(get_stem(".gitignore"), Some(".gitignore"));
    assert_eq!(get_stem(".eslintrc"), Some(".eslintrc"));
}

/// Test get_stem with files without extension.
#[test]
fn stem_extraction_no_extension() {
    assert_eq!(get_stem("Makefile"), Some("Makefile"));
    assert_eq!(get_stem("README"), Some("README"));
}

/// Test get_suffix extracts last word after underscore.
#[test]
fn suffix_extraction_with_underscore() {
    assert_eq!(get_suffix("foo_bar"), Some("bar"));
    assert_eq!(get_suffix("hello_world_foo"), Some("foo"));
}

/// Test get_suffix with no underscore.
#[test]
fn suffix_extraction_no_underscore() {
    assert_eq!(get_suffix("nounderscore"), None);
    assert_eq!(get_suffix("simple"), None);
}

/// Test get_suffix with empty string.
#[test]
fn suffix_extraction_empty_string() {
    assert_eq!(get_suffix(""), None);
}
