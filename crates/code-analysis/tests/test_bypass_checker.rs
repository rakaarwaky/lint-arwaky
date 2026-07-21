// Integration tests for BypassChecker (AES304)
// Tests the bypass detection logic without triggering self-flagging

use code_analysis_lint_arwaky::capabilities_check_bypass_checker::BypassChecker;
use shared::code_analysis::contract_bypass_checker_protocol::IBypassCheckerProtocol;
use shared::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO;
use shared::code_analysis::utility_bypass::{is_inside_string_or_char, strip_trailing_comment};
use shared::common::taxonomy_common_vo::PatternList;

fn create_checker() -> BypassChecker {
    BypassChecker::new()
}

fn create_checker_with_patterns(patterns: Vec<String>) -> BypassChecker {
    let rule = CodeAnalysisRuleVO {
        forbidden_bypass: PatternList { values: patterns },
        ..Default::default()
    };
    BypassChecker::from_rule(rule)
}

fn create_checker_from_patterns(patterns: &[&str]) -> BypassChecker {
    let values: Vec<String> = patterns.iter().map(|s| s.to_string()).collect();
    BypassChecker::from_patterns(&PatternList { values })
}

// ─── Rust: unwrap / expect ─────────────────────────────────

#[test]
fn test_detects_unwrap_method_call() {
    let checker = create_checker();
    let content = "let x = some_option.unwrap();";
    let mut violations = Vec::new();
    checker.check_bypass_comments("test.rs", content, &mut violations);
    assert_eq!(
        violations.len(),
        1,
        "Should detect exactly one .unwrap() call"
    );
}

#[test]
fn test_detects_expect_method_call() {
    let checker = create_checker();
    let content = r#"
fn main() {
    let x = some_option.expect("failed");
}
"#;
    let mut violations = Vec::new();
    checker.check_bypass_comments("test.rs", content, &mut violations);
    assert!(!violations.is_empty(), "Should detect .expect() call");
}

#[test]
fn test_detects_unwrap_in_complex_expression() {
    let checker = create_checker();
    let content = r#"
fn main() {
    let x = map.get("key").unwrap();
}
"#;
    let mut violations = Vec::new();
    checker.check_bypass_comments("test.rs", content, &mut violations);
    assert!(
        !violations.is_empty(),
        "Should detect .unwrap() in chained call"
    );
}

#[test]
fn test_detects_expect_in_chained_call() {
    let checker = create_checker();
    let content = r#"
fn main() {
    let x = vec.first().expect("empty vec");
}
"#;
    let mut violations = Vec::new();
    checker.check_bypass_comments("test.rs", content, &mut violations);
    assert!(
        !violations.is_empty(),
        "Should detect .expect() in chained call"
    );
}

// ─── Rust: panic / todo / unimplemented / unreachable ──────

#[test]
fn test_detects_panic_macro() {
    let checker = create_checker();
    let content = r#"
fn main() {
    panic!("something went wrong");
}
"#;
    let mut violations = Vec::new();
    checker.check_bypass_comments("test.rs", content, &mut violations);
    assert!(!violations.is_empty(), "Should detect panic!() macro");
}

#[test]
fn test_detects_todo_macro() {
    let checker = create_checker();
    let content = r#"
fn main() {
    todo!("implement this");
}
"#;
    let mut violations = Vec::new();
    checker.check_bypass_comments("test.rs", content, &mut violations);
    assert!(!violations.is_empty(), "Should detect todo!() macro");
}

#[test]
fn test_detects_unimplemented_macro() {
    let checker = create_checker();
    let content = r#"
fn main() {
    unimplemented!("not yet");
}
"#;
    let mut violations = Vec::new();
    checker.check_bypass_comments("test.rs", content, &mut violations);
    assert!(
        !violations.is_empty(),
        "Should detect unimplemented!() macro"
    );
}

#[test]
fn test_detects_unreachable_macro() {
    let checker = create_checker();
    let content = r#"
fn main() {
    unreachable!("should not reach here");
}
"#;
    let mut violations = Vec::new();
    checker.check_bypass_comments("test.rs", content, &mut violations);
    assert!(!violations.is_empty(), "Should detect unreachable!() macro");
}

// ─── Rust: #[allow] attributes ─────────────────────────────

#[test]
fn test_detects_allow_attribute() {
    let checker = create_checker();
    let content = r#"
#[allow(clippy::unwrap_used)]
fn main() {
    let x = 5;
}
"#;
    let mut violations = Vec::new();
    checker.check_bypass_comments("test.rs", content, &mut violations);
    assert!(
        !violations.is_empty(),
        "Should detect #[allow(...)] attribute"
    );
}

#[test]
fn test_detects_expect_attribute() {
    let checker = create_checker();
    let content = r#"
#[expect(clippy::unwrap_used)]
fn main() {
    let x = 5;
}
"#;
    let mut violations = Vec::new();
    checker.check_bypass_comments("test.rs", content, &mut violations);
    assert!(
        !violations.is_empty(),
        "Should detect #[expect(...)] attribute"
    );
}

#[test]
fn test_detects_inner_allow_attribute() {
    let checker = create_checker();
    let content = r#"#![allow(unused)]"#;
    let mut violations = Vec::new();
    checker.check_bypass_comments("test.rs", content, &mut violations);
    assert!(
        !violations.is_empty(),
        "Should detect #![allow(...)] inner attribute"
    );
}

// ─── Safe unwrap variants (should NOT be detected) ─────────

#[test]
fn test_safe_unwrap_or_default_not_detected() {
    let checker = create_checker();
    let content = r#"
fn main() {
    let x = some_option.unwrap_or_default();
}
"#;
    let mut violations = Vec::new();
    checker.check_bypass_comments("test.rs", content, &mut violations);
    assert!(
        violations.is_empty(),
        "Should not detect .unwrap_or_default()"
    );
}

#[test]
fn test_safe_unwrap_or_not_detected() {
    let checker = create_checker();
    let content = r#"
fn main() {
    let x = some_option.unwrap_or(0);
}
"#;
    let mut violations = Vec::new();
    checker.check_bypass_comments("test.rs", content, &mut violations);
    assert!(violations.is_empty(), "Should not detect .unwrap_or()");
}

#[test]
fn test_safe_unwrap_or_else_not_detected() {
    let checker = create_checker();
    let content = r#"
fn main() {
    let x = some_option.unwrap_or_else(|| 0);
}
"#;
    let mut violations = Vec::new();
    checker.check_bypass_comments("test.rs", content, &mut violations);
    assert!(violations.is_empty(), "Should not detect .unwrap_or_else()");
}

#[test]
fn test_mixed_safe_and_unsafe_unwrap() {
    let checker = create_checker();
    let content = r#"
fn main() {
    let x = some_option.unwrap_or_default();
    let y = other_option.unwrap();
}
"#;
    let mut violations = Vec::new();
    checker.check_bypass_comments("test.rs", content, &mut violations);
    assert_eq!(
        violations.len(),
        1,
        "Should detect only the unsafe .unwrap(), not .unwrap_or_default()"
    );
}

// ─── Skip logic: comments, doc comments, test modules ──────

#[test]
fn test_skips_line_comments() {
    let checker = create_checker();
    let content = r#"
fn main() {
    // This is a comment about unwrap
    let x = 5;
}
"#;
    let mut violations = Vec::new();
    checker.check_bypass_comments("test.rs", content, &mut violations);
    assert!(
        violations.is_empty(),
        "Should not detect patterns in line comments"
    );
}

#[test]
fn test_skips_block_comments() {
    let checker = create_checker();
    let content = r#"
fn main() {
    /* This is a block comment about unwrap */
    let x = 5;
}
"#;
    let mut violations = Vec::new();
    checker.check_bypass_comments("test.rs", content, &mut violations);
    assert!(
        violations.is_empty(),
        "Should not detect patterns in block comments"
    );
}

#[test]
fn test_skips_doc_comments() {
    let checker = create_checker();
    let content = r#"
/// This function uses unwrap internally
fn main() {
    let x = 5;
}
"#;
    let mut violations = Vec::new();
    checker.check_bypass_comments("test.rs", content, &mut violations);
    assert!(
        violations.is_empty(),
        "Should not detect patterns in doc comments"
    );
}

#[test]
fn test_skips_test_modules() {
    let checker = create_checker();
    let content = r#"
fn main() {
    let x = 5;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        let x = some_option.unwrap();
        assert_eq!(x, 5);
    }
}
"#;
    let mut violations = Vec::new();
    checker.check_bypass_comments("test.rs", content, &mut violations);
    assert!(
        violations.is_empty(),
        "Should not detect patterns in test modules"
    );
}

#[test]
fn test_skips_string_literals() {
    let checker = create_checker();
    let content = r#"
fn main() {
    let pattern = "unwrap";
    let x = 5;
}
"#;
    let mut violations = Vec::new();
    checker.check_bypass_comments("test.rs", content, &mut violations);
    assert!(
        violations.is_empty(),
        "Should not detect patterns in string literals"
    );
}

// ─── Python patterns ───────────────────────────────────────

#[test]
fn test_python_raise_notimplementederror() {
    let checker = create_checker();
    let content = r#"
def main():
    raise NotImplementedError("not yet")
"#;
    let mut violations = Vec::new();
    checker.check_bypass_comments("test.py", content, &mut violations);
    assert!(
        !violations.is_empty(),
        "Should detect Python raise NotImplementedError"
    );
}

#[test]
fn test_python_raise_notimplemented() {
    let checker = create_checker();
    let content = r#"
def main():
    raise NotImplemented
"#;
    let mut violations = Vec::new();
    checker.check_bypass_comments("test.py", content, &mut violations);
    assert!(
        !violations.is_empty(),
        "Should detect Python raise NotImplemented"
    );
}

#[test]
fn test_python_assert_false() {
    let checker = create_checker();
    let content = r#"
def main():
    assert False
"#;
    let mut violations = Vec::new();
    checker.check_bypass_comments("test.py", content, &mut violations);
    assert!(!violations.is_empty(), "Should detect Python assert False");
}

#[test]
fn test_python_type_ignore() {
    let checker = create_checker();
    let content = r#"
x = 5  # type: ignore
"#;
    let mut violations = Vec::new();
    checker.check_bypass_comments("test.py", content, &mut violations);
    assert!(!violations.is_empty(), "Should detect Python type: ignore");
}

#[test]
fn test_python_noqa() {
    let checker = create_checker();
    let content = r#"
x = 5  # noqa: F841
"#;
    let mut violations = Vec::new();
    checker.check_bypass_comments("test.py", content, &mut violations);
    assert!(!violations.is_empty(), "Should detect Python noqa");
}

// ─── JavaScript / TypeScript patterns ──────────────────────

#[test]
fn test_javascript_throw_error() {
    let checker = create_checker();
    let content = r#"
function main() {
    throw new Error("not yet");
}
"#;
    let mut violations = Vec::new();
    checker.check_bypass_comments("test.js", content, &mut violations);
    assert!(
        !violations.is_empty(),
        "Should detect JavaScript throw new Error"
    );
}

#[test]
fn test_typescript_throw_typeerror() {
    let checker = create_checker();
    let content = r#"
function main() {
    throw new TypeError("bad type");
}
"#;
    let mut violations = Vec::new();
    checker.check_bypass_comments("test.ts", content, &mut violations);
    assert!(
        !violations.is_empty(),
        "Should detect TypeScript throw new TypeError"
    );
}

#[test]
fn test_ts_ignore_in_comment_detected() {
    let checker = create_checker();
    let content = r#"
// @ts-ignore
const x = 5;
"#;
    let mut violations = Vec::new();
    checker.check_bypass_comments("test.ts", content, &mut violations);
    assert!(
        !violations.is_empty(),
        "@ts-ignore in line comments should be detected (bypass-comment pattern)"
    );
}

#[test]
fn test_ts_expect_error_in_comment_detected() {
    let checker = create_checker();
    let content = r#"
// @ts-expect-error
const x = 5;
"#;
    let mut violations = Vec::new();
    checker.check_bypass_comments("test.ts", content, &mut violations);
    assert!(
        !violations.is_empty(),
        "@ts-expect-error in line comments should be detected (bypass-comment pattern)"
    );
}

#[test]
fn test_eslint_disable_in_comment_detected() {
    let checker = create_checker();
    let content = r#"
/* eslint-disable */
const x = 5;
"#;
    let mut violations = Vec::new();
    checker.check_bypass_comments("test.js", content, &mut violations);
    assert!(
        !violations.is_empty(),
        "eslint-disable in block comments should be detected (bypass-comment pattern)"
    );
}

// ─── Cargo.toml checks ─────────────────────────────────────

#[test]
fn test_cargo_toml_workspace_lints_clippy_allow() {
    let checker = create_checker();
    let content = r#"
[workspace.lints.clippy]
unwrap_used = "allow"
"#;
    let mut violations = Vec::new();
    checker.check_cargo_toml(content, &mut violations);
    assert!(
        !violations.is_empty(),
        "Should detect Cargo.toml workspace clippy allow"
    );
}

#[test]
fn test_cargo_toml_lints_clippy_allow() {
    let checker = create_checker();
    let content = r#"
[lints.clippy]
unwrap_used = "allow"
"#;
    let mut violations = Vec::new();
    checker.check_cargo_toml(content, &mut violations);
    assert!(
        !violations.is_empty(),
        "Should detect Cargo.toml [lints.clippy] allow"
    );
}

#[test]
fn test_cargo_toml_deny_not_detected() {
    let checker = create_checker();
    let content = r#"
[workspace.lints.clippy]
unwrap_used = "deny"
"#;
    let mut violations = Vec::new();
    checker.check_cargo_toml(content, &mut violations);
    assert!(
        violations.is_empty(),
        "Should not detect Cargo.toml clippy deny"
    );
}

#[test]
fn test_cargo_toml_warn_not_detected() {
    let checker = create_checker();
    let content = r#"
[workspace.lints.clippy]
unwrap_used = "warn"
"#;
    let mut violations = Vec::new();
    checker.check_cargo_toml(content, &mut violations);
    assert!(
        violations.is_empty(),
        "Should not detect Cargo.toml clippy warn"
    );
}

#[test]
fn test_cargo_toml_non_clippy_section_ignored() {
    let checker = create_checker();
    let content = r#"
[workspace.dependencies]
serde = "1.0"
"#;
    let mut violations = Vec::new();
    checker.check_cargo_toml(content, &mut violations);
    assert!(
        violations.is_empty(),
        "Should not detect non-clippy sections"
    );
}

// ─── Multiple violations ───────────────────────────────────

#[test]
fn test_multiple_violations_in_file() {
    let checker = create_checker();
    let content = r#"
fn main() {
    let x = some_option.unwrap();
    let y = some_option.expect("failed");
    panic!("error");
}
"#;
    let mut violations = Vec::new();
    checker.check_bypass_comments("test.rs", content, &mut violations);
    assert!(violations.len() >= 3, "Should detect at least 3 violations");
}

#[test]
fn test_violation_line_numbers_are_correct() {
    let checker = create_checker();
    let content = "line1\nline2\nline3\nline4\nlet x = foo.unwrap();";
    let mut violations = Vec::new();
    checker.check_bypass_comments("test.rs", content, &mut violations);
    assert_eq!(violations.len(), 1);
    assert_eq!(
        violations[0].line.value(),
        5,
        "Violation should be on line 5"
    );
}

// ─── Edge cases ────────────────────────────────────────────

#[test]
fn test_empty_content_no_violations() {
    let checker = create_checker();
    let content = "";
    let mut violations = Vec::new();
    checker.check_bypass_comments("test.rs", content, &mut violations);
    assert!(
        violations.is_empty(),
        "Empty content should have no violations"
    );
}

#[test]
fn test_clean_code_no_violations() {
    let checker = create_checker();
    let content = r#"
fn main() {
    let x = 5;
    let y = 10;
    println!("{}", x + y);
}
"#;
    let mut violations = Vec::new();
    checker.check_bypass_comments("test.rs", content, &mut violations);
    assert!(
        violations.is_empty(),
        "Clean code should have no violations"
    );
}

#[test]
fn test_whitespace_only_content_no_violations() {
    let checker = create_checker();
    let content = "   \n  \n   ";
    let mut violations = Vec::new();
    checker.check_bypass_comments("test.rs", content, &mut violations);
    assert!(
        violations.is_empty(),
        "Whitespace-only content should have no violations"
    );
}

// ─── Custom patterns via from_patterns ─────────────────────

#[test]
fn test_custom_patterns_detect_match() {
    let checker = create_checker_from_patterns(&["custom_bypass"]);
    let content = "let x = custom_bypass();";
    let mut violations = Vec::new();
    checker.check_bypass_comments("test.rs", content, &mut violations);
    assert!(!violations.is_empty(), "Should detect custom pattern match");
}

#[test]
fn test_custom_patterns_no_false_positive() {
    let checker = create_checker_from_patterns(&["custom_bypass"]);
    let content = "let x = unwrap();";
    let mut violations = Vec::new();
    checker.check_bypass_comments("test.rs", content, &mut violations);
    assert!(
        violations.is_empty(),
        "Custom patterns should not detect default patterns"
    );
}

#[test]
fn test_empty_patterns_uses_fallback() {
    let checker = create_checker_with_patterns(vec![]);
    let content = "let x = some_option.unwrap();";
    let mut violations = Vec::new();
    checker.check_bypass_comments("test.rs", content, &mut violations);
    assert!(
        !violations.is_empty(),
        "Empty patterns should fall back to default forbidden list"
    );
}

// ─── Non-word bypass patterns detected even in comments ────

#[test]
fn test_fixme_in_comment_detected() {
    let checker = create_checker();
    let content = r#"
// FIXME: this is broken
fn main() {}
"#;
    let mut violations = Vec::new();
    checker.check_bypass_comments("test.rs", content, &mut violations);
    assert!(
        !violations.is_empty(),
        "FIXME in comments should be detected (bypass-comment pattern)"
    );
}

#[test]
fn test_hack_in_comment_detected() {
    let checker = create_checker();
    let content = r#"
// HACK: temporary workaround
fn main() {}
"#;
    let mut violations = Vec::new();
    checker.check_bypass_comments("test.rs", content, &mut violations);
    assert!(
        !violations.is_empty(),
        "HACK in comments should be detected (bypass-comment pattern)"
    );
}

#[test]
fn test_xxx_in_comment_detected() {
    let checker = create_checker();
    let content = r#"
// XXX: needs review
fn main() {}
"#;
    let mut violations = Vec::new();
    checker.check_bypass_comments("test.rs", content, &mut violations);
    assert!(
        !violations.is_empty(),
        "XXX in comments should be detected (bypass-comment pattern)"
    );
}

// ─── False positive fixes: trailing comments ───────────────

#[test]
fn test_trailing_comment_allow_not_detected() {
    let checker = create_checker();
    // Line starts with code, not // — trailing comment should be ignored
    let content = r#"mk(&['#', '[', 'a', 'l', 'l', 'o', 'w', '(']), // #[allow("#;
    let mut violations = Vec::new();
    checker.check_bypass_comments("test.rs", content, &mut violations);
    assert!(
        violations.is_empty(),
        "Trailing // #[allow( in comment should not be detected"
    );
}

#[test]
fn test_code_with_trailing_comment_unwrap_detected() {
    let checker = create_checker();
    // Real unwrap in code, trailing comment is irrelevant
    let content = "let x = foo.unwrap(); // some comment";
    let mut violations = Vec::new();
    checker.check_bypass_comments("test.rs", content, &mut violations);
    assert_eq!(
        violations.len(),
        1,
        "Real .unwrap() should still be detected"
    );
}

// ─── False positive fixes: string literals ─────────────────

#[test]
fn test_string_literal_todo_not_detected() {
    let checker = create_checker();
    let content = r#"inner.starts_with("todo!(")"#;
    let mut violations = Vec::new();
    checker.check_bypass_comments("test.rs", content, &mut violations);
    assert!(
        violations.is_empty(),
        "todo! inside a string literal should not be detected"
    );
}

#[test]
fn test_string_literal_unwrap_not_detected() {
    let checker = create_checker();
    let content = r#"let pattern = "unwrap";"#;
    let mut violations = Vec::new();
    checker.check_bypass_comments("test.rs", content, &mut violations);
    assert!(
        violations.is_empty(),
        "unwrap inside a string literal should not be detected"
    );
}

#[test]
fn test_string_literal_panic_not_detected() {
    let checker = create_checker();
    let content = r#"msg.contains("panic!")"#;
    let mut violations = Vec::new();
    checker.check_bypass_comments("test.rs", content, &mut violations);
    assert!(
        violations.is_empty(),
        "panic! inside a string literal should not be detected"
    );
}

#[test]
fn test_string_literal_expect_not_detected() {
    let checker = create_checker();
    let content = r#"inner.starts_with("unimplemented!(")"#;
    let mut violations = Vec::new();
    checker.check_bypass_comments("test.rs", content, &mut violations);
    assert!(
        violations.is_empty(),
        "unimplemented! inside a string literal should not be detected"
    );
}

// ─── Utility function tests ────────────────────────────────

#[test]
fn test_strip_trailing_comment_basic() {
    assert_eq!(strip_trailing_comment("code // comment"), "code ");
    assert_eq!(strip_trailing_comment("no comment here"), "no comment here");
    assert_eq!(strip_trailing_comment("// full comment"), "");
}

#[test]
fn test_strip_trailing_comment_in_string() {
    // // inside a string should NOT be treated as a comment
    assert_eq!(
        strip_trailing_comment(r#"s.contains("//")"#),
        r#"s.contains("//")"#
    );
}

#[test]
fn test_is_inside_string_or_char() {
    // Position 9 is inside the string "unwrap"
    assert!(is_inside_string_or_char(r#"foo("unwrap")"#, 9));
    // Position 3 is outside the string
    assert!(!is_inside_string_or_char(r#"foo("unwrap")"#, 3));
    // Position 5 is inside the char literal '\''
    assert!(is_inside_string_or_char("c = '\\''", 5));
    // Position 4 is the start of the char literal, not inside
    assert!(!is_inside_string_or_char("c = '\\''", 4));
}
