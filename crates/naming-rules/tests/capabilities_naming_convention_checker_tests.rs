use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::common::taxonomy_path_vo::FilePath;
use shared::taxonomy_layer_vo::LayerNameVO;

// ---------------------------------------------------------------------------
// is_barrel_file
// ---------------------------------------------------------------------------

#[test]
fn is_barrel_file_recognizes_mod_rs() {
    let fp = FilePath::new("mod.rs".to_string()).unwrap();
    assert!(fp.is_barrel_file());
}

#[test]
fn is_barrel_file_recognizes_init_py() {
    let fp = FilePath::new("__init__.py".to_string()).unwrap();
    assert!(fp.is_barrel_file());
}

#[test]
fn is_barrel_file_recognizes_index_ts() {
    let fp = FilePath::new("index.ts".to_string()).unwrap();
    assert!(fp.is_barrel_file());
}

#[test]
fn is_barrel_file_recognizes_index_tsx() {
    let fp = FilePath::new("index.tsx".to_string()).unwrap();
    assert!(fp.is_barrel_file());
}

#[test]
fn is_barrel_file_recognizes_index_jsx() {
    let fp = FilePath::new("index.jsx".to_string()).unwrap();
    assert!(fp.is_barrel_file());
}

#[test]
fn is_barrel_file_rejects_normal_file() {
    let fp = FilePath::new("checker.rs".to_string()).unwrap();
    assert!(!fp.is_barrel_file());
}

// ---------------------------------------------------------------------------
// is_entry_point
// ---------------------------------------------------------------------------

#[test]
fn is_entry_point_recognizes_main_rs() {
    let fp = FilePath::new("main.rs".to_string()).unwrap();
    assert!(fp.is_entry_point());
}

#[test]
fn is_entry_point_recognizes_lib_rs() {
    let fp = FilePath::new("lib.rs".to_string()).unwrap();
    assert!(fp.is_entry_point());
}

#[test]
fn is_entry_point_recognizes_app_py() {
    let fp = FilePath::new("app.py".to_string()).unwrap();
    assert!(fp.is_entry_point());
}

#[test]
fn is_entry_point_recognizes_main_ts() {
    let fp = FilePath::new("main.ts".to_string()).unwrap();
    assert!(fp.is_entry_point());
}

#[test]
fn is_entry_point_recognizes_app_js() {
    let fp = FilePath::new("app.js".to_string()).unwrap();
    assert!(fp.is_entry_point());
}

#[test]
fn is_entry_point_rejects_regular_file() {
    let fp = FilePath::new("service.rs".to_string()).unwrap();
    assert!(!fp.is_entry_point());
}

// ---------------------------------------------------------------------------
// make_result
// ---------------------------------------------------------------------------

#[test]
fn make_result_produces_lint_result() {
    use naming_rules_lint_arwaky::capabilities_naming_convention_checker::NamingConventionChecker;
    let result = NamingConventionChecker::make_result(
        "test.rs",
        "AES101",
        "naming violation",
        Severity::HIGH,
    );
    assert_eq!(result.code.to_string(), "AES101");
    assert_eq!(result.message.value(), "naming violation");
    assert_eq!(result.severity as i32, Severity::HIGH as i32);
}

#[test]
fn make_result_different_codes() {
    use naming_rules_lint_arwaky::capabilities_naming_convention_checker::NamingConventionChecker;
    let r1 = NamingConventionChecker::make_result("f.rs", "AES101", "msg", Severity::MEDIUM);
    let r2 = NamingConventionChecker::make_result("f.rs", "AES102", "msg", Severity::LOW);
    assert_eq!(r1.code.to_string(), "AES101");
    assert_eq!(r2.code.to_string(), "AES102");
}

// ---------------------------------------------------------------------------
// check_file_naming — barrel file and entry point skipping
// ---------------------------------------------------------------------------

#[test]
fn check_file_naming_skips_barrel_files() {
    use naming_rules_lint_arwaky::capabilities_naming_convention_checker::NamingConventionChecker;
    let checker = NamingConventionChecker::new();
    let mut violations = vec![];
    checker.check_file_naming(
        "mod.rs",
        "mod.rs",
        &None,
        None,
        &Default::default(),
        &mut violations,
    );
    assert!(violations.is_empty(), "barrel files should be skipped");
}

#[test]
fn check_file_naming_skips_entry_points() {
    use naming_rules_lint_arwaky::capabilities_naming_convention_checker::NamingConventionChecker;
    let checker = NamingConventionChecker::new();
    let mut violations = vec![];
    checker.check_file_naming(
        "main.rs",
        "main.rs",
        &None,
        None,
        &Default::default(),
        &mut violations,
    );
    assert!(violations.is_empty(), "entry points should be skipped");
}

#[test]
fn check_file_naming_reports_unknown_prefix() {
    use naming_rules_lint_arwaky::capabilities_naming_convention_checker::NamingConventionChecker;
    let checker = NamingConventionChecker::new();
    let mut violations = vec![];
    checker.check_file_naming(
        "bad_prefix_checker.rs",
        "bad_prefix_checker.rs",
        &None,
        None,
        &Default::default(),
        &mut violations,
    );
    assert!(violations.len() >= 1, "unknown prefix should be flagged");
    assert!(violations[0].code.to_string().contains("AES102"));
}

#[test]
fn check_file_naming_reports_no_layer() {
    use naming_rules_lint_arwaky::capabilities_naming_convention_checker::NamingConventionChecker;
    let checker = NamingConventionChecker::new();
    let mut violations = vec![];
    checker.check_file_naming(
        "file.rs",
        "file.rs",
        &None,
        None,
        &Default::default(),
        &mut violations,
    );
    assert!(
        !violations.is_empty(),
        "file without layer prefix should be flagged"
    );
}

#[test]
fn check_file_naming_valid_layer_file() {
    use naming_rules_lint_arwaky::capabilities_naming_convention_checker::NamingConventionChecker;
    use shared::taxonomy_definition_vo::LayerDefinition;
    let checker = NamingConventionChecker::new();
    let mut violations = vec![];
    let def = LayerDefinition::default();
    checker.check_file_naming(
        "capabilities_some_checker.rs",
        "capabilities_some_checker.rs",
        &Some(LayerNameVO::new("capabilities")),
        Some(&def),
        &Default::default(),
        &mut violations,
    );
    // Should not report violation for valid naming
    assert!(
        violations.is_empty(),
        "valid layer file should not be flagged"
    );
}
