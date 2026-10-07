// Unit tests for NamingConventionChecker — AES101 file naming validation.
use naming_rules_lint_arwaky::capabilities_naming_convention_checker::NamingConventionChecker;
use shared_common::{LayerMapVO, LayerNameVO};
use shared_naming_rules::INamingConventionProtocol;

fn checker() -> NamingConventionChecker {
    NamingConventionChecker::new()
}

fn layer_map() -> LayerMapVO {
    let mut map = LayerMapVO::default();
    map.values.insert(
        LayerNameVO::new("taxonomy"),
        shared_common::LayerDefinition::default(),
    );
    map
}

#[test]
fn construction_succeeds() {
    let _ = checker();
}

#[test]
fn valid_snake_case_no_violation() {
    let result = checker().check_file_naming_internal(
        "src/capabilities_user_checker.rs",
        "capabilities_user_checker.rs",
        &Some(LayerNameVO::new("capabilities")),
        None,
        3,
    );
    assert!(result.is_none());
}

#[test]
fn uppercase_in_name_produces_violation() {
    let result = checker().check_file_naming_internal(
        "src/capabilities_User_Checker.rs",
        "capabilities_User_Checker.rs",
        &Some(LayerNameVO::new("capabilities")),
        None,
        3,
    );
    assert!(result.is_some());
}

#[test]
fn barrel_file_skipped() {
    let result = checker().check_file_naming_internal(
        "src/capabilities/mod.rs",
        "mod.rs",
        &Some(LayerNameVO::new("capabilities")),
        None,
        3,
    );
    assert!(result.is_none());
}

#[test]
fn hyphens_in_name_produces_violation() {
    let result = checker().check_file_naming_internal(
        "src/capabilities-user-checker.rs",
        "capabilities-user-checker.rs",
        &Some(LayerNameVO::new("capabilities")),
        None,
        3,
    );
    assert!(result.is_some(), "hyphens must produce AES101 violation");
}

#[test]
fn dots_in_name_produces_violation() {
    let result = checker().check_file_naming_internal(
        "src/taxonomy.user.vo.rs",
        "taxonomy.user.vo.rs",
        &Some(LayerNameVO::new("taxonomy")),
        None,
        3,
    );
    assert!(result.is_some(), "dots must produce AES101 violation");
}

#[test]
fn too_few_words_produces_violation() {
    let result = checker().check_file_naming_internal(
        "src/taxonomy_user.rs",
        "taxonomy_user.rs",
        &Some(LayerNameVO::new("taxonomy")),
        None,
        3,
    );
    assert!(
        result.is_some(),
        "2 words must produce AES101 violation when min is 3"
    );
}

#[test]
fn digits_in_segment_no_violation() {
    let result = checker().check_file_naming_internal(
        "src/taxonomy_v2_vo.rs",
        "taxonomy_v2_vo.rs",
        &Some(LayerNameVO::new("taxonomy")),
        None,
        3,
    );
    assert!(result.is_none(), "digits in segments should be allowed");
}

#[test]
fn config_min_words_5_three_word_file_violates() {
    // Simulate min_words=5 via direct parameter
    let result = checker().check_file_naming_internal(
        "src/capabilities_user_checker.rs",
        "capabilities_user_checker.rs",
        &Some(LayerNameVO::new("capabilities")),
        None,
        5,
    );
    assert!(
        result.is_some(),
        "3-word file must fail when min_words is 5"
    );
}

#[test]
fn min_words_above_cache_table_is_honored_exactly() {
    // 10-word stem with min_words=11 must violate: the old code clamped to
    // the 10-word regex slot, letting a 10-word stem through when the
    // operator configured a floor of 11. Drive through the public trait
    // method so `min_words_from_config` + `naming_regex` are exercised.
    let checker = checker();
    let mut config = shared_config_system::ArchitectureConfig::default();
    config.naming.word_count = shared_common::taxonomy_common_vo::Count::new(11);
    let mut results = shared_common::LintResultList::new(Vec::new());
    let files = shared_common::FilePathList::new(vec![
        shared_common::FilePath::new("a_b_c_d_e_f_g_h_i_j").unwrap(),
    ]);
    checker.check_file_naming(
        &config,
        &layer_map(),
        &files,
        &shared_common::FilePath::new(".").unwrap(),
        &mut results,
    );
    assert!(
        !results.values.is_empty(),
        "10-word stem must violate when min_words is 11"
    );
}
