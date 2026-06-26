use code_analysis_lint_arwaky::capabilities_check_bypass_checker::BypassChecker;
use shared::code_analysis::contract_bypass_checker_protocol::IBypassCheckerProtocol;
use shared::common::taxonomy_common_vo::PatternList;

fn empty_violations() -> Vec<shared::cli_commands::taxonomy_result_vo::LintResult> {
    Vec::new()
}

fn count_code(violations: &[shared::cli_commands::taxonomy_result_vo::LintResult], code: &str) -> usize {
    violations.iter().filter(|v| v.code.code() == code).count()
}

#[test]
fn detects_bare_unwrap() {
    let checker = BypassChecker::new();
    let mut v = empty_violations();
    checker.check_bypass_comments("f.rs", "let x = Some(5).unwrap();\n", &mut v);
    assert_eq!(count_code(&v, "AES304"), 1);
}

#[test]
fn does_not_detect_unwrap_or_default() {
    let checker = BypassChecker::new();
    let mut v = empty_violations();
    checker.check_bypass_comments("f.rs", "let x = fs::read(p).unwrap_or_default();\n", &mut v);
    assert_eq!(count_code(&v, "AES304"), 0);
}

#[test]
fn does_not_detect_unwrap_or() {
    let checker = BypassChecker::new();
    let mut v = empty_violations();
    checker.check_bypass_comments("f.rs", "let x = opt.unwrap_or(0);\n", &mut v);
    assert_eq!(count_code(&v, "AES304"), 0);
}

#[test]
fn does_not_detect_unwrap_or_else() {
    let checker = BypassChecker::new();
    let mut v = empty_violations();
    checker.check_bypass_comments("f.rs", "let x = opt.unwrap_or_else(|| 0);\n", &mut v);
    assert_eq!(count_code(&v, "AES304"), 0);
}

#[test]
fn detects_bare_expect() {
    let checker = BypassChecker::new();
    let mut v = empty_violations();
    checker.check_bypass_comments("f.rs", "let x = Some(5).expect(\"msg\");\n", &mut v);
    assert_eq!(count_code(&v, "AES304"), 1);
}

#[test]
fn detects_panic_macro() {
    let checker = BypassChecker::new();
    let mut v = empty_violations();
    checker.check_bypass_comments("f.rs", "panic!(\"oops\");\n", &mut v);
    assert_eq!(count_code(&v, "AES304"), 1);
}

#[test]
fn detects_todo_macro() {
    let checker = BypassChecker::new();
    let mut v = empty_violations();
    checker.check_bypass_comments("f.rs", "todo!();\n", &mut v);
    assert_eq!(count_code(&v, "AES304"), 1);
}

#[test]
fn detects_unimplemented_macro() {
    let checker = BypassChecker::new();
    let mut v = empty_violations();
    checker.check_bypass_comments("f.rs", "unimplemented!();\n", &mut v);
    assert_eq!(count_code(&v, "AES304"), 1);
}

#[test]
fn detects_unreachable_macro() {
    let checker = BypassChecker::new();
    let mut v = empty_violations();
    checker.check_bypass_comments("f.rs", "unreachable!();\n", &mut v);
    assert_eq!(count_code(&v, "AES304"), 1);
}

#[test]
fn detects_allow_attribute() {
    let checker = BypassChecker::new();
    let mut v = empty_violations();
    checker.check_bypass_comments("f.rs", "#[allow(unused)]\nfn x() {}\n", &mut v);
    assert_eq!(count_code(&v, "AES304"), 1);
}

#[test]
fn skips_test_modules() {
    let checker = BypassChecker::new();
    let mut v = empty_violations();
    let src = "#[cfg(test)]\nmod tests {\n    let x = Some(5).unwrap();\n}\n";
    checker.check_bypass_comments("f.rs", src, &mut v);
    assert_eq!(count_code(&v, "AES304"), 0);
}

#[test]
fn does_not_match_substring_of_identifier() {
    let checker = BypassChecker::new();
    let mut v = empty_violations();
    checker.check_bypass_comments("f.rs", "let expectation = 5;\n", &mut v);
    assert_eq!(count_code(&v, "AES304"), 0);
}

#[test]
fn does_not_match_unwrap_in_identifier_name() {
    let checker = BypassChecker::new();
    let mut v = empty_violations();
    checker.check_bypass_comments("f.rs", "let unwrap_helper = Some(5);\nlet _x = unwrap_helper;\n", &mut v);
    assert_eq!(count_code(&v, "AES304"), 0);
}

#[test]
fn honors_config_patterns() {
    let patterns = PatternList::new(vec!["panic".to_string()]);
    let checker = BypassChecker::from_patterns(&patterns);
    let mut v = empty_violations();
    checker.check_bypass_comments("f.rs", "let x = Some(5).unwrap();\npanic!(\"oops\");\n", &mut v);
    assert_eq!(count_code(&v, "AES304"), 1);
}

#[test]
fn empty_patterns_falls_back_to_defaults() {
    let checker = BypassChecker::from_patterns(&PatternList::default());
    let mut v = empty_violations();
    checker.check_bypass_comments("f.rs", "let x = Some(5).unwrap();\n", &mut v);
    assert_eq!(count_code(&v, "AES304"), 1);
}

#[test]
fn detects_python_raise_not_implemented_error() {
    let checker = BypassChecker::new();
    let mut v = empty_violations();
    checker.check_bypass_comments("f.py", "def foo():\n    raise NotImplementedError\n", &mut v);
    assert_eq!(count_code(&v, "AES304"), 1);
}

#[test]
fn detects_python_raise_not_implemented() {
    let checker = BypassChecker::new();
    let mut v = empty_violations();
    checker.check_bypass_comments("f.py", "def foo():\n    raise NotImplemented\n", &mut v);
    assert_eq!(count_code(&v, "AES304"), 1);
}

#[test]
fn detects_python_assert_false() {
    let checker = BypassChecker::new();
    let mut v = empty_violations();
    checker.check_bypass_comments("f.py", "def foo():\n    assert False, \"unreachable\"\n", &mut v);
    assert_eq!(count_code(&v, "AES304"), 1);
}

#[test]
fn detects_python_noqa_bypass_comment() {
    let checker = BypassChecker::new();
    let mut v = empty_violations();
    checker.check_bypass_comments("f.py", "x = foo()  # noqa\n", &mut v);
    assert_eq!(count_code(&v, "AES304"), 1);
}

#[test]
fn detects_js_throw_new_error() {
    let checker = BypassChecker::new();
    let mut v = empty_violations();
    checker.check_bypass_comments("f.js", "function foo() {\n    throw new Error(\"oops\");\n}\n", &mut v);
    assert_eq!(count_code(&v, "AES304"), 1);
}

#[test]
fn detects_js_throw_new_type_error() {
    let checker = BypassChecker::new();
    let mut v = empty_violations();
    checker.check_bypass_comments("f.js", "function foo(x) {\n    if (typeof x !== \"number\") throw new TypeError(\"not a number\");\n}\n", &mut v);
    assert_eq!(count_code(&v, "AES304"), 1);
}

#[test]
fn detects_ts_throw_new_error() {
    let checker = BypassChecker::new();
    let mut v = empty_violations();
    checker.check_bypass_comments("f.ts", "export function foo(): never {\n    throw new Error(\"oops\");\n}\n", &mut v);
    assert_eq!(count_code(&v, "AES304"), 1);
}

#[test]
fn detects_eslint_disable_js() {
    let checker = BypassChecker::new();
    let mut v = empty_violations();
    checker.check_bypass_comments("f.js", "// eslint-disable-next-line\nvar x = 1;\n", &mut v);
    assert_eq!(count_code(&v, "AES304"), 1);
}

#[test]
fn jest_expect_does_not_match_word_pattern() {
    let checker = BypassChecker::new();
    let mut v = empty_violations();
    checker.check_bypass_comments("f.js", "expect(value).toBe(5);\n", &mut v);
    assert_eq!(count_code(&v, "AES304"), 0);
}

#[test]
fn detects_cargo_toml_clippy_allow() {
    let checker = BypassChecker::new();
    let mut v = empty_violations();
    let cargo = "[workspace.lints.clippy]\nmanual_unwrap_or_default = \"allow\"\nmanual_unwrap_or = \"allow\"\n";
    checker.check_cargo_toml(cargo, &mut v);
    assert_eq!(v.len(), 2);
}

#[test]
fn skips_warn_or_deny_in_cargo_toml() {
    let checker = BypassChecker::new();
    let mut v = empty_violations();
    let cargo = "[workspace.lints.clippy]\nresult_large_err = \"warn\"\nunsafe_code = \"deny\"\n";
    checker.check_cargo_toml(cargo, &mut v);
    assert_eq!(v.len(), 0);
}

#[test]
fn skips_non_clippy_sections_in_cargo_toml() {
    let checker = BypassChecker::new();
    let mut v = empty_violations();
    let cargo = "[workspace.dependencies]\nserde = \"1.0\"\n\n[profile.release]\nopt-level = 3\n";
    checker.check_cargo_toml(cargo, &mut v);
    assert_eq!(v.len(), 0);
}
