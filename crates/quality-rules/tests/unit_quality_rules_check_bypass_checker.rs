// Unit tests for BypassChecker — AES304 bypass comment/attribute detection.
use quality_rules_lint_arwaky::capabilities_check_bypass_checker::BypassChecker;
use shared::common::PatternList;
use shared::quality_rules::IBypassCheckerProtocol;

fn checker() -> BypassChecker {
    BypassChecker::new()
}

fn checker_with_patterns(patterns: &[&str]) -> BypassChecker {
    BypassChecker::from_patterns(&PatternList {
        values: patterns.iter().map(|s| s.to_string()).collect(),
    })
}

#[test]
fn construction_succeeds() {
    let _ = checker();
}

#[test]
fn cargo_toml_with_allow_produces_violation() {
    let mut violations = Vec::new();
    let content = "[workspace.lints.clippy]\nunwrap_used = \"allow\"\n";
    checker().check_cargo_toml(content, &mut violations);
    assert_eq!(violations.len(), 1);
    assert!(violations[0].code.code().contains("AES304"));
}

#[test]
fn cargo_toml_without_allow_no_violation() {
    let mut violations = Vec::new();
    let content = "[workspace.lints.clippy]\nunwrap_used = \"deny\"\n";
    checker().check_cargo_toml(content, &mut violations);
    assert!(violations.is_empty());
}

#[test]
fn cargo_toml_non_clippy_section_no_violation() {
    let mut violations = Vec::new();
    let content = "[dependencies]\nfoo = \"1.0\"\n";
    checker().check_cargo_toml(content, &mut violations);
    assert!(violations.is_empty());
}

#[test]
fn rust_unwrap_detected() {
    let mut violations = Vec::new();
    let content = "let x = foo.unwrap();\n";
    checker_with_patterns(&["unwrap"]).check_bypass_comments(
        "src/lib.rs",
        content,
        &mut violations,
    );
    assert_eq!(violations.len(), 1);
}

#[test]
fn rust_unwrap_or_safe_no_violation() {
    let mut violations = Vec::new();
    let content = "let x = foo.unwrap_or(0);\n";
    checker_with_patterns(&["unwrap"]).check_bypass_comments(
        "src/lib.rs",
        content,
        &mut violations,
    );
    assert!(violations.is_empty());
}

#[test]
fn rust_todo_detected() {
    let mut violations = Vec::new();
    let content = "fn foo() {\n    todo!()\n}\n";
    checker_with_patterns(&["todo"]).check_bypass_comments("src/lib.rs", content, &mut violations);
    assert_eq!(violations.len(), 1);
}

#[test]
fn rust_panic_detected() {
    let mut violations = Vec::new();
    let content = "panic!(\"oh no\");\n";
    checker_with_patterns(&["panic"]).check_bypass_comments("src/lib.rs", content, &mut violations);
    assert_eq!(violations.len(), 1);
}

#[test]
fn cfg_test_block_skipped() {
    let mut violations = Vec::new();
    let content = "#[cfg(test)]\nmod tests {\n    use super::*;\n    #[test]\n    fn foo() { let x = bar.unwrap(); }\n}\n";
    checker_with_patterns(&["unwrap"]).check_bypass_comments(
        "src/lib.rs",
        content,
        &mut violations,
    );
    assert!(violations.is_empty());
}

#[test]
fn bypass_pattern_in_comment_detected() {
    let mut violations = Vec::new();
    let content = "// FIXME: this is broken\nlet x = 1;\n";
    checker_with_patterns(&["fixme"]).check_bypass_comments("src/lib.rs", content, &mut violations);
    assert_eq!(violations.len(), 1);
}
