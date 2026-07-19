use naming_rules_lint_arwaky::capabilities_suffix_prefix_checker::SuffixPrefixChecker;
use naming_rules_lint_arwaky::taxonomy_naming_utility::{get_stem, get_suffix};
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::common::taxonomy_path_vo::FilePath;
use shared::taxonomy_layer_vo::LayerNameVO;

// ---------------------------------------------------------------------------
// is_barrel_file (via FilePath)
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
// is_entry_point (via FilePath)
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
// get_stem / get_suffix (via taxonomy_naming_utility)
// ---------------------------------------------------------------------------

#[test]
fn get_stem_removes_extension() {
    assert_eq!(get_stem("checker.rs"), Some("checker"));
}

#[test]
fn get_stem_handles_no_extension() {
    assert_eq!(get_stem("checker"), Some("checker"));
}

#[test]
fn get_stem_handles_multiple_dots() {
    assert_eq!(get_stem("my.test.file.rs"), Some("my.test.file"));
}

#[test]
fn get_suffix_returns_last_underscore_part() {
    assert_eq!(get_suffix("capabilities_checker"), Some("checker"));
}

#[test]
fn get_suffix_no_underscore_returns_none() {
    assert_eq!(get_suffix("checker"), None);
}

#[test]
fn get_suffix_single_underscore() {
    assert_eq!(get_suffix("_checker"), Some("checker"));
}

// ---------------------------------------------------------------------------
// check_domain_suffixes — barrel file and entry point skipping
// ---------------------------------------------------------------------------

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
        &Some(LayerNameVO::new("capabilities")),
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
