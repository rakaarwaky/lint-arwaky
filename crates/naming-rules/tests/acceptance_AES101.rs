// Acceptance tests — AES101 naming convention (map to FRD user stories).
use naming_rules_lint_arwaky::capabilities_naming_convention_checker::NamingConventionChecker;
use shared::common::taxonomy_definition_vo::LayerDefinition;
use shared::common::taxonomy_definition_vo::LayerMapVO;
use shared::common::taxonomy_layer_vo::LayerNameVO;
use shared::common::taxonomy_lint_result_vo::LintResultList;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::naming_rules::RULE_CODE_NAMING_CONVENTION;
use std::collections::HashMap;

fn checker() -> NamingConventionChecker {
    NamingConventionChecker::new()
}

fn layer_map() -> LayerMapVO {
    LayerMapVO::new(HashMap::new())
}

// ── FR-AES101-01: Uppercase characters in filename ────────

#[test]
fn uppercase_single_word_produces_violation() {
    let result = checker().check_file_naming_internal(
        "src/capabilities_User.rs",
        "capabilities_User.rs",
        &Some(LayerNameVO::new("capabilities")),
        None,
        3,
    );
    assert!(result.is_some(), "single uppercase word must fail AES101");
    assert_eq!(result.unwrap().code.code(), RULE_CODE_NAMING_CONVENTION);
}

#[test]
fn mixed_case_produces_violation() {
    let result = checker().check_file_naming_internal(
        "src/capabilities_user_Checker.rs",
        "capabilities_user_Checker.rs",
        &Some(LayerNameVO::new("capabilities")),
        None,
        3,
    );
    assert!(result.is_some(), "mixed case must fail AES101");
}

#[test]
fn all_uppercase_produces_violation() {
    let result = checker().check_file_naming_internal(
        "src/capabilities_USER_CHECKER.rs",
        "capabilities_USER_CHECKER.rs",
        &Some(LayerNameVO::new("capabilities")),
        None,
        3,
    );
    assert!(result.is_some(), "all uppercase must fail AES101");
}

// ── FR-AES101-02: Hyphen separator instead of underscore ──

#[test]
fn hyphen_separator_produces_violation() {
    let result = checker().check_file_naming_internal(
        "src/capabilities-user-checker.rs",
        "capabilities-user-checker.rs",
        &Some(LayerNameVO::new("capabilities")),
        None,
        3,
    );
    assert!(result.is_some(), "hyphens must fail AES101");
}

#[test]
fn mixed_separator_produces_violation() {
    let result = checker().check_file_naming_internal(
        "src/capabilities_user-checker.rs",
        "capabilities_user-checker.rs",
        &Some(LayerNameVO::new("capabilities")),
        None,
        3,
    );
    assert!(result.is_some(), "mixed separators must fail AES101");
}

// ── FR-AES101-03: Too few words (below min_words) ─────────

#[test]
fn single_word_produces_violation() {
    let result = checker().check_file_naming_internal(
        "src/capabilities.rs",
        "capabilities.rs",
        &Some(LayerNameVO::new("capabilities")),
        None,
        3,
    );
    assert!(result.is_some(), "single word must fail AES101 with min 3");
}

#[test]
fn two_words_produces_violation_when_min_is_3() {
    let result = checker().check_file_naming_internal(
        "src/capabilities_user.rs",
        "capabilities_user.rs",
        &Some(LayerNameVO::new("capabilities")),
        None,
        3,
    );
    assert!(result.is_some(), "two words must fail when min_words is 3");
}

// ── FR-AES101-04: Clean underscore files pass ─────────────

#[test]
fn three_words_clean_underscore_passes() {
    let result = checker().check_file_naming_internal(
        "src/capabilities_user_checker.rs",
        "capabilities_user_checker.rs",
        &Some(LayerNameVO::new("capabilities")),
        None,
        3,
    );
    assert!(result.is_none(), "three-word clean file should pass");
}

#[test]
fn four_words_clean_underscore_passes() {
    let result = checker().check_file_naming_internal(
        "src/capabilities_user_permission_checker.rs",
        "capabilities_user_permission_checker.rs",
        &Some(LayerNameVO::new("capabilities")),
        None,
        3,
    );
    assert!(result.is_none(), "four-word clean file should pass");
}

#[test]
fn numeric_words_clean_underscore_passes() {
    let result = checker().check_file_naming_internal(
        "src/capabilities_v2_handler.rs",
        "capabilities_v2_handler.rs",
        &Some(LayerNameVO::new("capabilities")),
        None,
        3,
    );
    assert!(result.is_none(), "numeric words should be allowed");
}

// ── FR-AES101-05: Barrel and entry files are skipped ──────

#[test]
fn mod_rs_is_skipped() {
    let result = checker().check_file_naming_internal(
        "src/capabilities/mod.rs",
        "mod.rs",
        &Some(LayerNameVO::new("capabilities")),
        None,
        3,
    );
    assert!(result.is_none(), "mod.rs must be skipped");
}

#[test]
fn lib_rs_is_skipped() {
    let result = checker().check_file_naming_internal(
        "src/lib.rs",
        "lib.rs",
        &Some(LayerNameVO::new("capabilities")),
        None,
        3,
    );
    assert!(result.is_none(), "lib.rs must be skipped");
}

#[test]
fn main_rs_is_skipped() {
    let result = checker().check_file_naming_internal(
        "src/main.rs",
        "main.rs",
        &Some(LayerNameVO::new("capabilities")),
        None,
        3,
    );
    assert!(result.is_none(), "main.rs must be skipped");
}

// ── FR-AES101-06: Exception list bypasses naming ─────────

#[test]
fn excepted_filename_passes() {
    let def = LayerDefinition {
        exceptions: shared::common::PatternList::new(vec!["special_file.rs".to_string()]),
        ..Default::default()
    };
    let result = checker().check_file_naming_internal(
        "src/special_file.rs",
        "special_file.rs",
        &Some(LayerNameVO::new("capabilities")),
        Some(&def),
        3,
    );
    assert!(
        result.is_none(),
        "excepted file must pass regardless of name"
    );
}

// ── FR-AES101-07: Convention check via public trait API ───

#[test]
fn check_file_naming_via_trait_api() {
    use shared::naming_rules::INamingConventionChecker;

    let config = shared::config_system::taxonomy_config_vo::ArchitectureConfig::default();
    let layer_map = layer_map();
    let files = FilePathList::new(vec![
        FilePath::new("src/capabilities_BadFile.rs".to_string()).unwrap(),
        FilePath::new("src/capabilities_good_file.rs".to_string()).unwrap(),
    ]);
    let root = FilePath::new(".".to_string()).unwrap();
    let mut results = LintResultList::new(Vec::new());

    checker().check_file_naming(&config, &layer_map, &files, &root, &mut results);

    assert_eq!(
        results.len(),
        1,
        "only the bad file should produce a violation"
    );
    assert_eq!(results.values[0].code.code(), RULE_CODE_NAMING_CONVENTION);
}
