use naming_rules_lint_arwaky::capabilities_suffix_prefix_checker::SuffixPrefixChecker;
use shared::cli_commands::taxonomy_severity_vo::Severity;

#[test]
fn is_barrel_file_recognizes_mod_rs() {
    assert!(SuffixPrefixChecker::is_barrel_file("mod.rs"));
}

#[test]
fn is_barrel_file_recognizes_init_py() {
    assert!(SuffixPrefixChecker::is_barrel_file("__init__.py"));
}

#[test]
fn is_barrel_file_rejects_normal_file() {
    assert!(!SuffixPrefixChecker::is_barrel_file("checker.rs"));
}

#[test]
fn is_entry_point_recognizes_main_rs() {
    assert!(SuffixPrefixChecker::is_entry_point("main.rs"));
}

#[test]
fn is_entry_point_recognizes_lib_rs() {
    assert!(SuffixPrefixChecker::is_entry_point("lib.rs"));
}

#[test]
fn is_entry_point_rejects_regular_file() {
    assert!(!SuffixPrefixChecker::is_entry_point("service.rs"));
}

#[test]
fn get_stem_removes_extension() {
    assert_eq!(
        SuffixPrefixChecker::get_stem("checker.rs"),
        Some("checker".to_string())
    );
}

#[test]
fn get_stem_handles_no_extension() {
    assert_eq!(
        SuffixPrefixChecker::get_stem("checker"),
        Some("checker".to_string())
    );
}

#[test]
fn get_stem_handles_multiple_dots() {
    assert_eq!(
        SuffixPrefixChecker::get_stem("my.test.file.rs"),
        Some("my.test.file".to_string())
    );
}

#[test]
fn get_suffix_returns_last_underscore_part() {
    assert_eq!(
        SuffixPrefixChecker::get_suffix("capabilities_checker"),
        Some("checker".to_string())
    );
}

#[test]
fn get_suffix_no_underscore_returns_none() {
    assert_eq!(SuffixPrefixChecker::get_suffix("checker"), None);
}

#[test]
fn get_suffix_single_underscore() {
    assert_eq!(
        SuffixPrefixChecker::get_suffix("_checker"),
        Some("checker".to_string())
    );
}

#[test]
fn check_domain_suffixes_skips_barrel_file() {
    let checker = SuffixPrefixChecker::new();
    let mut violations = Vec::new();
    checker.check_domain_suffixes("mod.rs", "mod.rs", None, &None, &mut violations);
    assert!(violations.is_empty(), "barrel files should be skipped");
}

#[test]
fn check_domain_suffixes_skips_entry_point() {
    let checker = SuffixPrefixChecker::new();
    let mut violations = Vec::new();
    checker.check_domain_suffixes("main.rs", "main.rs", None, &None, &mut violations);
    assert!(violations.is_empty(), "entry points should be skipped");
}

#[test]
fn check_domain_suffixes_no_definition_no_op() {
    let checker = SuffixPrefixChecker::new();
    let mut violations = Vec::new();
    checker.check_domain_suffixes("random.rs", "random.rs", None, &None, &mut violations);
    assert!(violations.is_empty(), "no definition means no check");
}

#[test]
fn check_domain_suffixes_skips_exceptions() {
    use shared::taxonomy_common_vo::PatternList;
    use shared::taxonomy_definition_vo::LayerDefinition;

    let checker = SuffixPrefixChecker::new();
    let mut violations = Vec::new();
    let def = LayerDefinition {
        exceptions: PatternList::new(vec!["skip.rs".to_string()]),
        ..Default::default()
    };
    checker.check_domain_suffixes(
        "skip.rs",
        "skip.rs",
        Some(&def),
        &Some("capabilities".to_string()),
        &mut violations,
    );
    assert!(violations.is_empty(), "exceptions should be skipped");
}

#[test]
fn make_result_produces_lint_result_with_code() {
    let result = SuffixPrefixChecker::make_result("test.rs", "AES102", "msg", Severity::HIGH);
    assert_eq!(result.code.to_string(), "AES102");
    assert_eq!(result.message.to_string(), "msg");
    assert_eq!(result.severity, Severity::HIGH);
}
