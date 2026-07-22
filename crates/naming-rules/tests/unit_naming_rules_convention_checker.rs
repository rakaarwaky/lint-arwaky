// PURPOSE: Unit tests for NamingConventionChecker (AES101 — naming convention)
// Covers: happy path, edge cases, error paths
// Coverage target: Capabilities ≥ 70%

use naming_rules_lint_arwaky::capabilities_naming_convention_checker::NamingConventionChecker;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::common::taxonomy_definition_vo::LayerDefinition;
use shared::common::taxonomy_definition_vo::LayerMapVO;
use shared::common::taxonomy_layer_vo::LayerNameVO;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::naming_rules::contract_naming_checker_protocol::INamingConventionChecker;
use std::collections::HashMap;
use std::sync::Arc;

fn default_config() -> ArchitectureConfig {
    ArchitectureConfig::default()
}

fn layer_map_with_capabilities() -> LayerMapVO {
    let mut map = HashMap::new();
    map.insert(LayerNameVO::new("capabilities"), LayerDefinition::default());
    map.insert(LayerNameVO::new("taxonomy"), LayerDefinition::default());
    map.insert(LayerNameVO::new("utility"), LayerDefinition::default());
    LayerMapVO::new(map)
}

fn make_files(paths: &[&str]) -> FilePathList {
    FilePathList::new(
        paths
            .iter()
            .map(|p| FilePath::new(p.to_string()).unwrap())
            .collect(),
    )
}

async fn run_check(
    files: &[&str],
    config: &ArchitectureConfig,
    layer_map: &LayerMapVO,
) -> LintResultList {
    let checker = NamingConventionChecker::new();
    let mut results = LintResultList::new(Vec::new());
    let root = FilePath::new("/project".to_string()).unwrap();
    checker
        .check_file_naming(config, layer_map, &make_files(files), &root, &mut results)
        .await;
    results
}

// ─── Happy Path ───────────────────────────────────────────

#[tokio::test]
async fn valid_three_word_snake_case_passes() {
    let config = default_config();
    let lm = layer_map_with_capabilities();
    let results = run_check(&["capabilities_user_checker.rs"], &config, &lm).await;
    assert!(
        results.is_empty(),
        "Expected no violations, got: {:?}",
        results.values
    );
}

#[tokio::test]
async fn valid_four_word_snake_case_passes() {
    let config = default_config();
    let lm = layer_map_with_capabilities();
    let results = run_check(&["capabilities_db_user_adapter.rs"], &config, &lm).await;
    assert!(results.is_empty());
}

#[tokio::test]
async fn valid_taxonomy_vo_passes() {
    let config = default_config();
    let lm = layer_map_with_capabilities();
    let results = run_check(&["taxonomy_user_vo.rs"], &config, &lm).await;
    assert!(results.is_empty());
}

#[tokio::test]
async fn valid_utility_parser_passes() {
    let config = default_config();
    let lm = layer_map_with_capabilities();
    let results = run_check(&["utility_json_parser.rs"], &config, &lm).await;
    assert!(results.is_empty());
}

// ─── AES101 Violations ────────────────────────────────────

#[tokio::test]
async fn uppercase_in_stem_fails_aes101() {
    let config = default_config();
    let lm = layer_map_with_capabilities();
    let results = run_check(&["capabilities_User_Checker.rs"], &config, &lm).await;
    assert!(!results.is_empty(), "Uppercase should trigger AES101");
    assert_eq!(results.values[0].code.code(), "AES101");
}

#[tokio::test]
async fn hyphen_separator_fails_aes101() {
    let config = default_config();
    let lm = layer_map_with_capabilities();
    let results = run_check(&["capabilities-user-checker.rs"], &config, &lm).await;
    assert!(!results.is_empty(), "Hyphens should trigger AES101");
}

#[tokio::test]
async fn two_words_fails_aes101() {
    let config = default_config();
    let lm = layer_map_with_capabilities();
    // "capabilities_checker" has only 2 words
    let results = run_check(&["capabilities_checker.rs"], &config, &lm).await;
    assert!(!results.is_empty(), "Two words should trigger AES101");
    assert_eq!(results.values[0].code.code(), "AES101");
}

#[tokio::test]
async fn single_word_fails_aes101() {
    let config = default_config();
    let lm = layer_map_with_capabilities();
    let results = run_check(&["checker.rs"], &config, &lm).await;
    assert!(!results.is_empty(), "Single word should trigger AES101");
}

// ─── Exception / Barrel Files ─────────────────────────────

#[tokio::test]
async fn barrel_file_mod_rs_passes() {
    let config = default_config();
    let lm = layer_map_with_capabilities();
    let results = run_check(&["mod.rs"], &config, &lm).await;
    assert!(results.is_empty(), "mod.rs is a barrel file exception");
}

#[tokio::test]
async fn entry_point_lib_rs_passes() {
    let config = default_config();
    let lm = layer_map_with_capabilities();
    let results = run_check(&["lib.rs"], &config, &lm).await;
    assert!(results.is_empty(), "lib.rs is an entry point exception");
}

#[tokio::test]
async fn entry_point_main_rs_passes() {
    let config = default_config();
    let lm = layer_map_with_capabilities();
    let results = run_check(&["main.rs"], &config, &lm).await;
    assert!(results.is_empty(), "main.rs is an entry point exception");
}

#[tokio::test]
async fn barrel_file_init_py_passes() {
    let config = default_config();
    let lm = layer_map_with_capabilities();
    let results = run_check(&["__init__.py"], &config, &lm).await;
    assert!(results.is_empty(), "__init__.py is a barrel file exception");
}

// ─── Unknown Prefix ───────────────────────────────────────

#[tokio::test]
async fn unknown_prefix_emits_aes102() {
    let config = default_config();
    let lm = layer_map_with_capabilities();
    let results = run_check(&["helpers_user_parser.rs"], &config, &lm).await;
    assert!(
        !results.is_empty(),
        "Unknown prefix should trigger violation"
    );
    assert_eq!(results.values[0].code.code(), "AES102");
}

// ─── Configurable min_words ───────────────────────────────

#[tokio::test]
async fn min_words_from_config_respected() {
    let mut config = default_config();
    config.naming.word_count = shared::common::taxonomy_common_vo::Count::new(4);
    let lm = layer_map_with_capabilities();
    // 3 words should now fail
    let results = run_check(&["capabilities_user_checker.rs"], &config, &lm).await;
    assert!(!results.is_empty(), "3 words should fail when min_words=4");
}

#[tokio::test]
async fn min_words_zero_defaults_to_three() {
    let mut config = default_config();
    config.naming.word_count = shared::common::taxonomy_common_vo::Count::new(0);
    let lm = layer_map_with_capabilities();
    // 3 words should pass (default fallback)
    let results = run_check(&["capabilities_user_checker.rs"], &config, &lm).await;
    assert!(results.is_empty(), "min_words=0 should default to 3");
}

// ─── Edge Cases ───────────────────────────────────────────

#[tokio::test]
async fn digits_in_words_pass() {
    let config = default_config();
    let lm = layer_map_with_capabilities();
    let results = run_check(&["capabilities_v2_adapter.rs"], &config, &lm).await;
    assert!(results.is_empty(), "Digits in words are valid");
}

#[tokio::test]
async fn multiple_files_mixed_results() {
    let config = default_config();
    let lm = layer_map_with_capabilities();
    let results = run_check(
        &[
            "capabilities_user_checker.rs", // valid
            "capabilities_Bad_Name.rs",     // invalid (uppercase)
            "taxonomy_item_vo.rs",          // valid
        ],
        &config,
        &lm,
    )
    .await;
    assert_eq!(results.len(), 1, "Only one file should violate");
}
