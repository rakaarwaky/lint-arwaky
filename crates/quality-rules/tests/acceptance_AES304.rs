// PURPOSE: Acceptance test AES304 — bypass detection
// Verify unwrap(), #[allow(...)], FIXME, HACK, XXX are flagged; unwrap_or_default() is NOT flagged.
use shared::cli_commands::LintResult;
use shared::common::PatternList;
use shared::quality_rules::IBypassCheckerProtocol;

use quality_rules_lint_arwaky::capabilities_check_bypass_checker::BypassChecker;

fn _checker() -> BypassChecker {
    BypassChecker::new()
}

fn checker_with_patterns(patterns: &[&str]) -> BypassChecker {
    BypassChecker::from_patterns(&PatternList {
        values: patterns.iter().map(|s| s.to_string()).collect(),
    })
}

// ── unwrap() detection ───────────────────────────────────────

#[test]
fn unwrap_detected() {
    let content = "let x = value.unwrap();\n";
    let mut violations: Vec<LintResult> = Vec::new();
    checker_with_patterns(&["unwrap"]).check_bypass_comments(
        "src/lib.rs",
        content,
        &mut violations,
    );
    assert_eq!(violations.len(), 1);
    assert!(violations[0].code.code().contains("AES304"));
    assert_eq!(violations[0].severity, shared::common::Severity::CRITICAL);
}

#[test]
fn expect_detected() {
    let content = "let x = value.expect(\"failed\");\n";
    let mut violations: Vec<LintResult> = Vec::new();
    checker_with_patterns(&["expect"]).check_bypass_comments(
        "src/lib.rs",
        content,
        &mut violations,
    );
    assert_eq!(violations.len(), 1);
    assert!(violations[0].code.code().contains("AES304"));
}

// ── unwrap_or_default() NOT flagged ──────────────────────────

#[test]
fn unwrap_or_default_not_flagged() {
    let content = "let x = value.unwrap_or_default();\n";
    let mut violations: Vec<LintResult> = Vec::new();
    checker_with_patterns(&["unwrap"]).check_bypass_comments(
        "src/lib.rs",
        content,
        &mut violations,
    );
    assert!(
        violations.is_empty(),
        "unwrap_or_default() should not be flagged"
    );
}

#[test]
fn unwrap_or_not_flagged() {
    let content = "let x = value.unwrap_or(0);\n";
    let mut violations: Vec<LintResult> = Vec::new();
    checker_with_patterns(&["unwrap"]).check_bypass_comments(
        "src/lib.rs",
        content,
        &mut violations,
    );
    assert!(violations.is_empty(), "unwrap_or() should not be flagged");
}

#[test]
fn unwrap_or_else_not_flagged() {
    let content = "let x = value.unwrap_or_else(|| 0);\n";
    let mut violations: Vec<LintResult> = Vec::new();
    checker_with_patterns(&["unwrap"]).check_bypass_comments(
        "src/lib.rs",
        content,
        &mut violations,
    );
    assert!(
        violations.is_empty(),
        "unwrap_or_else() should not be flagged"
    );
}

// ── #[allow(...)] detection ──────────────────────────────────

#[test]
fn allow_attribute_detected() {
    let content = "#[allow(dead_code)]\nfn unused() {}\n";
    let mut violations: Vec<LintResult> = Vec::new();
    checker_with_patterns(&["unwrap"]).check_bypass_comments(
        "src/lib.rs",
        content,
        &mut violations,
    );
    let has_allow = violations
        .iter()
        .any(|v| v.message.value.contains("bypass attribute"));
    assert!(has_allow, "Expected #[allow(...)] to be detected");
}

// ── FIXME / HACK / XXX detection ─────────────────────────────

#[test]
fn fixme_in_comment_detected() {
    let content = "// FIXME: this is broken\nlet x = 1;\n";
    let mut violations: Vec<LintResult> = Vec::new();
    checker_with_patterns(&["fixme"]).check_bypass_comments("src/lib.rs", content, &mut violations);
    assert_eq!(violations.len(), 1);
    assert!(violations[0].code.code().contains("AES304"));
}

#[test]
fn hack_in_comment_detected() {
    let content = "// HACK: temporary workaround\nlet x = 1;\n";
    let mut violations: Vec<LintResult> = Vec::new();
    checker_with_patterns(&["hack"]).check_bypass_comments("src/lib.rs", content, &mut violations);
    assert_eq!(violations.len(), 1);
}

#[test]
fn xxx_in_comment_detected() {
    let content = "// XXX: needs cleanup\nlet x = 1;\n";
    let mut violations: Vec<LintResult> = Vec::new();
    checker_with_patterns(&["xxx"]).check_bypass_comments("src/lib.rs", content, &mut violations);
    assert_eq!(violations.len(), 1);
}

// ── cfg(test) block skips ────────────────────────────────────

#[test]
fn unwrap_in_cfg_test_block_not_flagged() {
    let content =
        "#[cfg(test)]\nmod tests {\n    fn test_it() {\n        let x = foo.unwrap();\n    }\n}\n";
    let mut violations: Vec<LintResult> = Vec::new();
    checker_with_patterns(&["unwrap"]).check_bypass_comments(
        "src/lib.rs",
        content,
        &mut violations,
    );
    assert!(
        violations.is_empty(),
        "unwrap in #[cfg(test)] should not be flagged"
    );
}

// ── Noqa / type: ignore / ts-ignore ─────────────────────────

#[test]
fn noqa_detected() {
    let content = "x = 1  # noqa: E501\n";
    let mut violations: Vec<LintResult> = Vec::new();
    checker_with_patterns(&["noqa"]).check_bypass_comments("src/lib.py", content, &mut violations);
    assert_eq!(violations.len(), 1);
}

#[test]
fn ts_ignore_detected() {
    let content = "// @ts-ignore\nconst x: any = 1;\n";
    let mut violations: Vec<LintResult> = Vec::new();
    checker_with_patterns(&["@ts-ignore"]).check_bypass_comments(
        "src/lib.ts",
        content,
        &mut violations,
    );
    assert_eq!(violations.len(), 1);
}
