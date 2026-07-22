// Unit tests for NamingConventionChecker — AES101 naming convention validation

use naming_rules_lint_arwaky::capabilities_naming_convention_checker::NamingConventionChecker;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::common::taxonomy_definition_vo::LayerDefinition;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::common::taxonomy_severity_vo::Severity;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::naming_rules::contract_naming_checker_protocol::INamingConventionChecker;
use shared::naming_rules::taxonomy_naming_constant::{
    ADAPTER_NAME, LAYER_PREFIXES, RULE_CODE_NAMING_CONVENTION, RULE_CODE_SUFFIX_PREFIX,
};
use shared::taxonomy_definition_vo::LayerMapVO;
use shared::taxonomy_layer_vo::LayerNameVO;

// ─── Unit Tests: Happy Path — Valid File Naming ─────────────────────

/// Test that valid naming convention files produce no violations.
#[tokio::test]
async fn check_file_naming_valid_convention_no_violations() {
    let checker = NamingConventionChecker::new();
    let config = ArchitectureConfig::default();

    // Build a minimal layer map with "capabilities" layer
    let mut layers = std::collections::HashMap::new();
    layers.insert(
        LayerNameVO::new("capabilities".to_string()),
        LayerDefinition {
            word_count: shared::common::taxonomy_common_vo::Count::new(3),
            ..LayerDefinition::default()
        },
    );
    let layer_map = LayerMapVO::new(layers);

    // Valid file: capabilities_user_checker.rs (3 words, lowercase, underscore)
    let files = FilePathList {
        values: vec![FilePath::new("capabilities_user_checker.rs".to_string()).unwrap()],
    };
    let root = FilePath::new("/".to_string()).unwrap();
    let mut results = LintResultList::new(Vec::new());

    checker
        .check_file_naming(&config, &layer_map, &files, &root, &mut results)
        .await;

    // Should have zero violations for valid naming
    assert!(
        results.values.is_empty(),
        "Valid file naming should produce zero violations, got: {:?}",
        results.values
    );
}

/// Test that taxonomy layer files with valid naming pass.
#[tokio::test]
async fn check_file_naming_valid_taxonomy_no_violations() {
    let checker = NamingConventionChecker::new();
    let config = ArchitectureConfig::default();

    let mut layers = std::collections::HashMap::new();
    layers.insert(
        LayerNameVO::new("taxonomy".to_string()),
        LayerDefinition {
            word_count: shared::common::taxonomy_common_vo::Count::new(3),
            ..LayerDefinition::default()
        },
    );
    let layer_map = LayerMapVO::new(layers);

    // Valid taxonomy file
    let files = FilePathList {
        values: vec![FilePath::new("taxonomy_user_entity.rs".to_string()).unwrap()],
    };
    let root = FilePath::new("/".to_string()).unwrap();
    let mut results = LintResultList::new(Vec::new());

    checker
        .check_file_naming(&config, &layer_map, &files, &root, &mut results)
        .await;

    assert!(results.values.is_empty());
}

/// Test that agent layer files with valid naming pass.
#[tokio::test]
async fn check_file_naming_valid_agent_no_violations() {
    let checker = NamingConventionChecker::new();
    let config = ArchitectureConfig::default();

    let mut layers = std::collections::HashMap::new();
    layers.insert(
        LayerNameVO::new("agent".to_string()),
        LayerDefinition {
            word_count: shared::common::taxonomy_common_vo::Count::new(3),
            ..LayerDefinition::default()
        },
    );
    let layer_map = LayerMapVO::new(layers);

    let files = FilePathList {
        values: vec![FilePath::new("agent_pipeline_orchestrator.rs".to_string()).unwrap()],
    };
    let root = FilePath::new("/".to_string()).unwrap();
    let mut results = LintResultList::new(Vec::new());

    checker
        .check_file_naming(&config, &layer_map, &files, &root, &mut results)
        .await;

    assert!(results.values.is_empty());
}

// ─── Unit Tests: Edge Cases — Invalid Naming Convention ─────────────

/// Test that files with uppercase characters produce violations.
#[tokio::test]
async fn check_file_naming_uppercase_produces_violation() {
    let checker = NamingConventionChecker::new();
    let config = ArchitectureConfig::default();

    let mut layers = std::collections::HashMap::new();
    layers.insert(
        LayerNameVO::new("capabilities".to_string()),
        LayerDefinition {
            word_count: shared::common::taxonomy_common_vo::Count::new(3),
            ..LayerDefinition::default()
        },
    );
    let layer_map = LayerMapVO::new(layers);

    // Invalid: uppercase characters
    let files = FilePathList {
        values: vec![FilePath::new("capabilities_User_Checker.rs".to_string()).unwrap()],
    };
    let root = FilePath::new("/".to_string()).unwrap();
    let mut results = LintResultList::new(Vec::new());

    checker
        .check_file_naming(&config, &layer_map, &files, &root, &mut results)
        .await;

    assert_eq!(
        results.values.len(),
        1,
        "Uppercase naming should produce exactly one violation"
    );
    assert_eq!(results.values[0].code.code(), RULE_CODE_NAMING_CONVENTION);
}

/// Test that files with wrong separator (hyphens instead of underscores) produce violations.
#[tokio::test]
async fn check_file_naming_hyphen_separator_produces_violation() {
    let checker = NamingConventionChecker::new();
    let config = ArchitectureConfig::default();

    let mut layers = std::collections::HashMap::new();
    layers.insert(
        LayerNameVO::new("capabilities".to_string()),
        LayerDefinition {
            word_count: shared::common::taxonomy_common_vo::Count::new(3),
            ..LayerDefinition::default()
        },
    );
    let layer_map = LayerMapVO::new(layers);

    // Invalid: hyphens instead of underscores — produces unknown prefix violation (AES102)
    // because the regex doesn't match, so no layer can be detected
    let files = FilePathList {
        values: vec![FilePath::new("capabilities-user-checker.rs".to_string()).unwrap()],
    };
    let root = FilePath::new("/".to_string()).unwrap();
    let mut results = LintResultList::new(Vec::new());

    checker
        .check_file_naming(&config, &layer_map, &files, &root, &mut results)
        .await;

    assert_eq!(
        results.values.len(),
        1,
        "Hyphen separator should produce exactly one violation"
    );
    // Hyphens don't match underscore pattern, so no layer prefix is detected → AES102 (UnknownPrefix)
    assert_eq!(results.values[0].code.code(), RULE_CODE_SUFFIX_PREFIX);
}

/// Test that files with fewer than minimum words produce violations.
#[tokio::test]
async fn check_file_naming_too_few_words_produces_violation() {
    let checker = NamingConventionChecker::new();
    let config = ArchitectureConfig::default();

    let mut layers = std::collections::HashMap::new();
    layers.insert(
        LayerNameVO::new("capabilities".to_string()),
        LayerDefinition {
            word_count: shared::common::taxonomy_common_vo::Count::new(3),
            ..LayerDefinition::default()
        },
    );
    let layer_map = LayerMapVO::new(layers);

    // Invalid: only 1 word (capabilities, no concept or suffix) — should fail with default word_count=2
    let files = FilePathList {
        values: vec![FilePath::new("capabilities.rs".to_string()).unwrap()],
    };
    let root = FilePath::new("/".to_string()).unwrap();
    let mut results = LintResultList::new(Vec::new());

    checker
        .check_file_naming(&config, &layer_map, &files, &root, &mut results)
        .await;

    // With word_count=2 in default config, "capabilities" (1 word) should produce a violation
    assert_eq!(
        results.values.len(),
        1,
        "File with fewer than min words should produce exactly one violation"
    );
}

// ─── Unit Tests: Edge Cases — Unknown Layer Prefix ──────────────────

/// Test that files with unknown layer prefix produce violations.
#[tokio::test]
async fn check_file_naming_unknown_prefix_produces_violation() {
    let checker = NamingConventionChecker::new();
    let config = ArchitectureConfig::default();

    // No layers configured — file won't match any known prefix
    let layer_map = LayerMapVO::new(std::collections::HashMap::new());

    // Unknown prefix "foo" is not in LAYER_PREFIXES
    let files = FilePathList {
        values: vec![FilePath::new("foo_bar_baz.rs".to_string()).unwrap()],
    };
    let root = FilePath::new("/".to_string()).unwrap();
    let mut results = LintResultList::new(Vec::new());

    checker
        .check_file_naming(&config, &layer_map, &files, &root, &mut results)
        .await;

    assert_eq!(
        results.values.len(),
        1,
        "Unknown prefix should produce exactly one violation"
    );
    // Should be the UnknownPrefix violation
    assert_eq!(results.values[0].code.code(), RULE_CODE_SUFFIX_PREFIX);
}

/// Test that all valid layer prefixes are recognized.
#[tokio::test]
async fn check_file_naming_all_valid_prefixes() {
    let checker = NamingConventionChecker::new();
    let config = ArchitectureConfig::default();

    for prefix in LAYER_PREFIXES {
        // Build a layer map with this prefix's layer
        let base_name = prefix.trim_end_matches('_');
        let mut layers = std::collections::HashMap::new();
        layers.insert(
            LayerNameVO::new(base_name.to_string()),
            LayerDefinition {
                word_count: shared::common::taxonomy_common_vo::Count::new(3),
                ..LayerDefinition::default()
            },
        );
        let layer_map = LayerMapVO::new(layers);

        // Create a valid file with this prefix (prefix + concept + suffix = 3 words)
        let filename = format!("{}_concept_checker.rs", base_name);
        let files = FilePathList {
            values: vec![FilePath::new(filename).unwrap()],
        };
        let root = FilePath::new("/".to_string()).unwrap();
        let mut results = LintResultList::new(Vec::new());

        checker
            .check_file_naming(&config, &layer_map, &files, &root, &mut results)
            .await;

        // Should have zero violations for valid naming with known prefix
        assert!(
            results.values.is_empty(),
            "Valid file with prefix '{}' should produce zero violations",
            base_name
        );
    }
}

// ─── Unit Tests: Edge Cases — Empty and Special Files ───────────────

/// Test that empty file list produces no violations.
#[tokio::test]
async fn check_file_naming_empty_files_no_violations() {
    let checker = NamingConventionChecker::new();
    let config = ArchitectureConfig::default();
    let layer_map = LayerMapVO::new(std::collections::HashMap::new());

    let files = FilePathList { values: vec![] };
    let root = FilePath::new("/".to_string()).unwrap();
    let mut results = LintResultList::new(Vec::new());

    checker
        .check_file_naming(&config, &layer_map, &files, &root, &mut results)
        .await;

    assert!(results.values.is_empty());
}

/// Test that barrel files (mod.rs) are skipped.
#[tokio::test]
async fn check_file_naming_barrel_file_skipped() {
    let checker = NamingConventionChecker::new();
    let config = ArchitectureConfig::default();
    let layer_map = LayerMapVO::new(std::collections::HashMap::new());

    // mod.rs is a barrel file — should be skipped
    let files = FilePathList {
        values: vec![FilePath::new("mod.rs".to_string()).unwrap()],
    };
    let root = FilePath::new("/".to_string()).unwrap();
    let mut results = LintResultList::new(Vec::new());

    checker
        .check_file_naming(&config, &layer_map, &files, &root, &mut results)
        .await;

    assert!(results.values.is_empty());
}

/// Test that entry point files (main.rs, app.rs) are skipped.
#[tokio::test]
async fn check_file_naming_entry_point_skipped() {
    let checker = NamingConventionChecker::new();
    let config = ArchitectureConfig::default();
    let layer_map = LayerMapVO::new(std::collections::HashMap::new());

    // main.rs is an entry point — should be skipped
    let files = FilePathList {
        values: vec![FilePath::new("main.rs".to_string()).unwrap()],
    };
    let root = FilePath::new("/".to_string()).unwrap();
    let mut results = LintResultList::new(Vec::new());

    checker
        .check_file_naming(&config, &layer_map, &files, &root, &mut results)
        .await;

    assert!(results.values.is_empty());
}

/// Test that dotfiles produce violations (they are not skipped).
#[tokio::test]
async fn check_file_naming_dotfile_produces_violation() {
    let checker = NamingConventionChecker::new();
    let config = ArchitectureConfig::default();
    let layer_map = LayerMapVO::new(std::collections::HashMap::new());

    // .gitignore is a dotfile — get_stem returns full name, produces violation (no valid prefix)
    let files = FilePathList {
        values: vec![FilePath::new(".gitignore".to_string()).unwrap()],
    };
    let root = FilePath::new("/".to_string()).unwrap();
    let mut results = LintResultList::new(Vec::new());

    checker
        .check_file_naming(&config, &layer_map, &files, &root, &mut results)
        .await;

    // Dotfiles without valid layer prefix should produce violations
    assert!(
        !results.values.is_empty(),
        "Dotfile without valid prefix should produce violations"
    );
}

/// Test that multi-dot files (foo.spec.rs) are handled correctly.
#[tokio::test]
async fn check_file_naming_multi_dot_file() {
    let checker = NamingConventionChecker::new();
    let config = ArchitectureConfig::default();

    let mut layers = std::collections::HashMap::new();
    layers.insert(
        LayerNameVO::new("capabilities".to_string()),
        LayerDefinition {
            word_count: shared::common::taxonomy_common_vo::Count::new(3),
            ..LayerDefinition::default()
        },
    );
    let layer_map = LayerMapVO::new(layers);

    // foo.spec.rs — stem is "foo.spec", which has 2 words (too few)
    let files = FilePathList {
        values: vec![FilePath::new("capabilities_user_checker.spec.rs".to_string()).unwrap()],
    };
    let root = FilePath::new("/".to_string()).unwrap();
    let mut results = LintResultList::new(Vec::new());

    checker
        .check_file_naming(&config, &layer_map, &files, &root, &mut results)
        .await;

    // Should pass because stem "capabilities_user_checker.spec" has 3+ words
    assert!(results.values.is_empty());
}

// ─── Unit Tests: Error Handling ─────────────────────────────────────

/// Test that checker handles invalid FilePath gracefully.
#[tokio::test]
async fn check_file_naming_handles_invalid_filepath() {
    let checker = NamingConventionChecker::new();
    let config = ArchitectureConfig::default();
    let layer_map = LayerMapVO::new(std::collections::HashMap::new());

    // Use an empty string as filepath (edge case)
    let files = FilePathList {
        values: vec![FilePath::new("".to_string()).unwrap_or_default()],
    };
    let root = FilePath::new("/".to_string()).unwrap();
    let mut results = LintResultList::new(Vec::new());

    // Should not panic
    checker
        .check_file_naming(&config, &layer_map, &files, &root, &mut results)
        .await;

    // Empty filepath may produce a violation or none — both are acceptable
}

/// Test that the adapter name in results is correct.
#[tokio::test]
async fn check_file_naming_result_has_correct_adapter() {
    let checker = NamingConventionChecker::new();
    let config = ArchitectureConfig::default();

    let mut layers = std::collections::HashMap::new();
    layers.insert(
        LayerNameVO::new("capabilities".to_string()),
        LayerDefinition {
            word_count: shared::common::taxonomy_common_vo::Count::new(3),
            ..LayerDefinition::default()
        },
    );
    let layer_map = LayerMapVO::new(layers);

    // Invalid naming to force a violation
    let files = FilePathList {
        values: vec![FilePath::new("Capabilities_User_Checker.rs".to_string()).unwrap()],
    };
    let root = FilePath::new("/".to_string()).unwrap();
    let mut results = LintResultList::new(Vec::new());

    checker
        .check_file_naming(&config, &layer_map, &files, &root, &mut results)
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
            "Naming violation should be HIGH severity"
        );
    }
}
