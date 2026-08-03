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
    let result = checker()._check_file_naming(
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
    let result = checker()._check_file_naming(
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
    let result = checker()._check_file_naming(
        "src/capabilities/mod.rs",
        "mod.rs",
        &Some(LayerNameVO::new("capabilities")),
        None,
        3,
    );
    assert!(result.is_none());
}
