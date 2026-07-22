// PURPOSE: Acceptance test for FR-004: Bypass Detection (AES304)
// Detects and flags any attempt to suppress warnings/errors.

use code_analysis_lint_arwaky::BypassChecker;
use shared::code_analysis::contract_bypass_checker_protocol::IBypassCheckerProtocol;
use shared::common::taxonomy_severity_vo::Severity;

fn checker() -> BypassChecker {
    BypassChecker::new()
}

fn scan_rs(content: &str) -> Vec<shared::cli_commands::taxonomy_result_vo::LintResult> {
    let mut v = Vec::new();
    checker().check_bypass_comments("test.rs", content, &mut v);
    v
}

/// FR-004: unwrap() detected with AES304
#[test]
fn fr_004_unwrap_detected() {
    let violations = scan_rs("let x = opt.unwrap();");
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES304");
    assert_eq!(violations[0].severity, Severity::CRITICAL);
}

/// FR-004: expect() detected with AES304
#[test]
fn fr_004_expect_detected() {
    let violations = scan_rs(r#"let x = opt.expect("msg");"#);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES304");
}

/// FR-004: panic! detected with AES304
#[test]
fn fr_004_panic_detected() {
    let violations = scan_rs(r#"panic!("fatal");"#);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES304");
}

/// FR-004: todo! detected with AES304
#[test]
fn fr_004_todo_detected() {
    let violations = scan_rs("todo!();");
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES304");
}

/// FR-004: #[allow(...)] detected with AES304
#[test]
fn fr_004_allow_attribute_detected() {
    let violations = scan_rs("#[allow(unused)]\nfn foo() {}");
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES304");
}

/// FR-004: Safe variants NOT flagged: unwrap_or()
#[test]
fn fr_004_unwrap_or_not_flagged() {
    let violations = scan_rs("let x = opt.unwrap_or(0);");
    assert!(violations.is_empty());
}

/// FR-004: Safe variants NOT flagged: unwrap_or_else()
#[test]
fn fr_004_unwrap_or_else_not_flagged() {
    let violations = scan_rs("let x = opt.unwrap_or_else(|| 0);");
    assert!(violations.is_empty());
}

/// FR-004: noqa comment detected (Python)
#[test]
fn fr_004_noqa_detected() {
    let mut v = Vec::new();
    checker().check_bypass_comments("test.py", "x = 1  # noqa", &mut v);
    assert_eq!(v.len(), 1);
    assert_eq!(v[0].code.code(), "AES304");
}

/// FR-004: type: ignore detected (Python)
#[test]
fn fr_004_type_ignore_detected() {
    let mut v = Vec::new();
    checker().check_bypass_comments("test.py", "x = bad()  # type: ignore", &mut v);
    assert_eq!(v.len(), 1);
    assert_eq!(v[0].code.code(), "AES304");
}

/// FR-004: eslint-disable detected (JavaScript)
#[test]
fn fr_004_eslint_disable_detected() {
    let mut v = Vec::new();
    checker().check_bypass_comments("test.js", "// eslint-disable-next-line\nfoo();", &mut v);
    assert_eq!(v.len(), 1);
    assert_eq!(v[0].code.code(), "AES304");
}

/// FR-004: Cargo.toml clippy allow detected
#[test]
fn fr_004_cargo_toml_clippy_allow() {
    let content = "[workspace.lints.clippy]\nall = \"allow\"";
    let mut v = Vec::new();
    checker().check_cargo_toml(content, &mut v);
    assert_eq!(v.len(), 1);
    assert_eq!(v[0].code.code(), "AES304");
    assert_eq!(v[0].severity, Severity::CRITICAL);
}
