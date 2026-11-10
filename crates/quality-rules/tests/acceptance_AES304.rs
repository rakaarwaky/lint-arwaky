// PURPOSE: Acceptance test AES304 — bypass detection
// Verify unwrap(), #[allow(...)], FIXME, HACK, XXX are flagged; unwrap_or_default() is NOT flagged.
use shared_cli_commands::LintResult;
use shared_common::PatternList;
use shared_quality_rules::IBypassCheckerProtocol;

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
    assert_eq!(violations[0].severity, shared_common::Severity::CRITICAL);
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

// ── Python branch (UX-S4-002) ───────────────────────────────
// The Python branch must use Python vocabulary — no `unimplemented!()`
// (Rust macro) and no `Result::Err` (Rust type) in the message or fix.

#[test]
fn python_not_implemented_message_is_python_vocabulary() {
    let content = "def f():\n    raise NotImplementedError\n";
    let mut violations: Vec<LintResult> = Vec::new();
    checker_with_patterns(&["unwrap"]).check_bypass_comments(
        "src/lib.py",
        content,
        &mut violations,
    );
    assert_eq!(
        violations.len(),
        1,
        "Python `raise NotImplementedError` must produce one AES304 finding"
    );
    let v = &violations[0];
    assert!(v.code.code().contains("AES304"));
    // UX-S4-002: no Rust-only vocabulary in the Python branch.
    assert!(
        !v.message.value.contains("unimplemented!()") && !v.message.value.contains("Result::Err"),
        "Python message must not use Rust vocabulary, got: {}",
        v.message.value
    );
    assert!(
        !v.fix.contains("Result::Err") && !v.fix.contains("unimplemented!()"),
        "Python fix must not use Rust vocabulary, got: {}",
        v.fix
    );
    // The message names the Python token it detected.
    assert!(
        v.message.value.contains("NotImplementedError"),
        "Python message must name the matched token, got: {}",
        v.message.value
    );
}

#[test]
fn python_assert_false_message_is_python_vocabulary() {
    let content = "def f():\n    assert False, 'bad'\n";
    let mut violations: Vec<LintResult> = Vec::new();
    checker_with_patterns(&["unwrap"]).check_bypass_comments(
        "src/lib.py",
        content,
        &mut violations,
    );
    assert_eq!(violations.len(), 1);
    let v = &violations[0];
    assert!(
        !v.message.value.contains("unimplemented!()")
            && !v.message.value.contains("Result::Err")
            && !v.fix.contains("Result::Err"),
        "Python assert-False finding must not use Rust vocabulary, got: {} / fix: {}",
        v.message.value,
        v.fix
    );
    assert!(
        v.message.value.contains("assert False"),
        "Python message must name the matched token, got: {}",
        v.message.value
    );
}

// ── Kind naming (UX-S2-004 / UX-S3-003) ────────────────────
// Every AES304 finding's message must open with its kind word so the kind is
// visible in the copy, not just in the subtype field.

#[test]
fn unwrap_message_names_its_kind() {
    let content = "let x = value.unwrap();\n";
    let mut violations: Vec<LintResult> = Vec::new();
    checker_with_patterns(&["unwrap"]).check_bypass_comments(
        "src/lib.rs",
        content,
        &mut violations,
    );
    assert_eq!(violations.len(), 1);
    let v = &violations[0];
    assert!(
        v.message.value.starts_with("UNWRAP_EXPECT"),
        "unwrap finding must open with its kind, got: {}",
        v.message.value
    );
    assert!(
        v.message.value.contains("unwrap"),
        "unwrap finding must name the matched token, got: {}",
        v.message.value
    );
}

#[test]
fn panic_message_names_its_kind() {
    let content = "fn f() { panic!(\"bad\"); }\n";
    let mut violations: Vec<LintResult> = Vec::new();
    checker_with_patterns(&["panic"]).check_bypass_comments("src/lib.rs", content, &mut violations);
    assert_eq!(violations.len(), 1);
    let v = &violations[0];
    assert!(
        v.message.value.starts_with("PANIC"),
        "panic finding must open with its kind, got: {}",
        v.message.value
    );
    assert!(
        v.message.value.contains("panic"),
        "panic finding must name the matched token, got: {}",
        v.message.value
    );
}

#[test]
fn todo_message_names_its_kind() {
    let content = "fn f() { todo!(); }\n";
    let mut violations: Vec<LintResult> = Vec::new();
    checker_with_patterns(&["todo"]).check_bypass_comments("src/lib.rs", content, &mut violations);
    assert_eq!(violations.len(), 1);
    let v = &violations[0];
    assert!(
        v.message.value.starts_with("TODO"),
        "todo finding must open with its kind, got: {}",
        v.message.value
    );
    assert!(
        v.message.value.contains("todo"),
        "todo finding must name the matched token, got: {}",
        v.message.value
    );
}

#[test]
fn unimplemented_message_names_its_kind() {
    let content = "fn f() { unimplemented!(); }\n";
    let mut violations: Vec<LintResult> = Vec::new();
    checker_with_patterns(&["unimplemented"]).check_bypass_comments(
        "src/lib.rs",
        content,
        &mut violations,
    );
    assert_eq!(violations.len(), 1);
    let v = &violations[0];
    assert!(
        v.message.value.starts_with("UNIMPLEMENTED"),
        "unimplemented finding must open with its kind, got: {}",
        v.message.value
    );
    assert!(
        v.message.value.contains("unimplemented"),
        "unimplemented finding must name the matched token, got: {}",
        v.message.value
    );
}

#[test]
fn bypass_comment_message_names_its_kind() {
    let content = "// FIXME: this is broken\nlet x = 1;\n";
    let mut violations: Vec<LintResult> = Vec::new();
    checker_with_patterns(&["fixme"]).check_bypass_comments("src/lib.rs", content, &mut violations);
    assert_eq!(violations.len(), 1);
    let v = &violations[0];
    assert!(
        v.message.value.starts_with("BYPASS_COMMENT"),
        "bypass comment finding must open with its kind, got: {}",
        v.message.value
    );
    assert!(
        v.message.value.contains("fixme"),
        "bypass comment finding must name the matched token, got: {}",
        v.message.value
    );
}
