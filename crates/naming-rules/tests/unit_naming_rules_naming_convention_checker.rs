// Unit tests for NamingConventionChecker — AES101 file naming validation.
use naming_rules_lint_arwaky::capabilities_naming_convention_checker::NamingConventionChecker;
use shared::common::LayerNameVO;

fn checker() -> NamingConventionChecker {
    NamingConventionChecker::new()
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
