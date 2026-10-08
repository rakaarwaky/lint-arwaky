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
    let mut results = Vec::<shared_common::ViolationItem>::new();
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
        !results.is_empty(),
        "10-word stem must violate when min_words is 11"
    );
}

fn run_min_words(min_words: u64, files: &[&str]) -> Vec<shared_common::ViolationItem> {
    let checker = checker();
    let mut config = shared_config_system::ArchitectureConfig::default();
    config.naming.word_count =
        shared_common::taxonomy_common_vo::Count::new(min_words.try_into().unwrap());
    let mut results = Vec::<shared_common::ViolationItem>::new();
    let path_list = shared_common::FilePathList::new(
        files
            .iter()
            .map(|f| shared_common::FilePath::new(*f).unwrap())
            .collect(),
    );
    checker.check_file_naming(
        &config,
        &layer_map(),
        &path_list,
        &shared_common::FilePath::new(".").unwrap(),
        &mut results,
    );
    results
}

#[test]
fn ten_word_stem_violates_when_min_words_is_15() {
    let violations = run_min_words(15, &["a_b_c_d_e_f_g_h_i_j"]);
    assert!(
        !violations.is_empty(),
        "10-word stem must violate when min_words is 15"
    );
}

#[test]
fn ten_word_stem_violates_when_min_words_is_100() {
    let violations = run_min_words(100, &["a_b_c_d_e_f_g_h_i_j"]);
    assert!(
        !violations.is_empty(),
        "10-word stem must violate when min_words is 100"
    );
}

#[test]
fn stem_with_enough_words_passes_above_cache_table() {
    // Positive control: a 15-word stem satisfies min_words=11/15 (no violation).
    let stem = "a_b_c_d_e_f_g_h_i_j_k_l_m_n_o";
    assert!(
        run_min_words(15, &[stem]).is_empty(),
        "15-word stem must pass when min_words is 15"
    );
    // min_words=11 is above the cache table too; the 15-word stem still passes.
    assert!(
        run_min_words(11, &[stem]).is_empty(),
        "15-word stem must pass when min_words is 11"
    );
}

#[test]
fn cache_table_slots_still_behave_identically_for_1_to_10() {
    // min_words=10 with a 9-word stem violates; a 10-word stem passes —
    // the last cached slot boundary must remain unchanged.
    let ten_word = "a_b_c_d_e_f_g_h_i_j";
    assert!(
        !run_min_words(10, &["a_b_c_d_e_f_g_h_i"]).is_empty(),
        "9-word stem must violate when min_words is 10"
    );
    assert!(
        run_min_words(10, &[ten_word]).is_empty(),
        "10-word stem must pass when min_words is 10"
    );
}
