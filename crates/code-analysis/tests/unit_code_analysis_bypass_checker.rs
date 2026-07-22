// PURPOSE: Unit tests for BypassChecker (AES304) — bypass comment detection,
// unwrap/expect/panic/todo detection, Cargo.toml clippy allow detection.

use code_analysis_lint_arwaky::BypassChecker;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::code_analysis::contract_bypass_checker_protocol::IBypassCheckerProtocol;
use shared::common::taxonomy_severity_vo::Severity;

fn checker() -> BypassChecker {
    BypassChecker::new()
}

fn scan(content: &str) -> Vec<LintResult> {
    let mut violations = Vec::new();
    checker().check_bypass_comments("test.rs", content, &mut violations);
    violations
}

fn scan_file(file: &str, content: &str) -> Vec<LintResult> {
    let mut violations = Vec::new();
    checker().check_bypass_comments(file, content, &mut violations);
    violations
}

// ─── Happy Path: Clean code produces no violations ───────────────────

#[test]
fn clean_code_no_violations() {
    let content = r#"
pub struct Foo {
    x: i32,
}

impl Foo {
    pub fn new() -> Self {
        Self { x: 0 }
    }
}
"#;
    assert!(scan(content).is_empty());
}

#[test]
fn safe_unwrap_or_not_flagged() {
    let content = r#"
fn example() -> i32 {
    let val = some_option.unwrap_or(42);
    let val2 = some_option.unwrap_or_else(|| compute());
    let val3 = some_option.unwrap_or_default();
    val + val2 + val3
}
"#;
    assert!(scan(content).is_empty());
}

// ─── AES304: unwrap() detection ──────────────────────────────────────

#[test]
fn detects_unwrap_call() {
    let content = "let x = some_result.unwrap();";
    let violations = scan(content);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES304");
    assert_eq!(violations[0].severity, Severity::CRITICAL);
}

#[test]
fn detects_expect_call() {
    let content = r#"let x = some_result.expect("should work");"#;
    let violations = scan(content);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES304");
}

// ─── AES304: panic! detection ────────────────────────────────────────

#[test]
fn detects_panic_macro() {
    let content = r#"panic!("something went wrong");"#;
    let violations = scan(content);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES304");
}

// ─── AES304: todo! detection ─────────────────────────────────────────

#[test]
fn detects_todo_macro() {
    let content = "todo!();";
    let violations = scan(content);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES304");
}

// ─── AES304: unimplemented! detection ────────────────────────────────

#[test]
fn detects_unimplemented_macro() {
    let content = "unimplemented!();";
    let violations = scan(content);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES304");
}

// ─── AES304: unreachable! detection ──────────────────────────────────

#[test]
fn detects_unreachable_macro() {
    let content = "unreachable!();";
    let violations = scan(content);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES304");
}

// ─── AES304: #[allow(...)] attribute detection ───────────────────────

#[test]
fn detects_allow_attribute() {
    let content = "#[allow(unused_variables)]\nfn foo() {}";
    let violations = scan(content);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES304");
}

#[test]
fn detects_clippy_allow_attribute() {
    let content = "#[clippy::allow(needless_return)]\nfn foo() {}";
    let violations = scan(content);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES304");
}

// ─── AES304: Comment bypass patterns ─────────────────────────────────

#[test]
fn detects_noqa_comment() {
    let content = "x = 1  # noqa";
    let violations = scan_file("test.py", content);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES304");
}

#[test]
fn detects_type_ignore_comment() {
    let content = "x: int = 'hello'  # type: ignore";
    let violations = scan_file("test.py", content);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES304");
}

#[test]
fn detects_eslint_disable_comment() {
    let content = "// eslint-disable-next-line no-console\nconsole.log('hi');";
    let violations = scan_file("test.js", content);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES304");
}

#[test]
fn detects_ts_ignore_comment() {
    let content = "// @ts-ignore\nconst x: number = 'hello';";
    let violations = scan_file("test.ts", content);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES304");
}

// ─── Edge Case: cfg(test) blocks are skipped ─────────────────────────

#[test]
fn skips_cfg_test_block() {
    let content = r#"
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let x = some_result.unwrap();
        panic!("expected in test");
    }
}
"#;
    assert!(scan(content).is_empty());
}

// ─── Edge Case: static Lazy<Regex> blocks are skipped ────────────────

#[test]
fn skips_static_lazy_regex_block() {
    let content = r#"
static RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"unwrap\(").unwrap()
});
"#;
    assert!(scan(content).is_empty());
}

// ─── Edge Case: unwrap inside string literal not flagged ─────────────

#[test]
fn unwrap_inside_string_not_flagged() {
    let content = r#"let msg = "call unwrap() to get value";"#;
    assert!(scan(content).is_empty());
}

// ─── Edge Case: Multiple violations in one file ──────────────────────

#[test]
fn multiple_violations_detected() {
    let content = r#"
fn a() { let x = opt.unwrap(); }
fn b() { panic!("oops"); }
fn c() { todo!(); }
"#;
    let violations = scan(content);
    assert_eq!(violations.len(), 3);
}

// ─── Cargo.toml clippy allow detection ───────────────────────────────

#[test]
fn detects_cargo_toml_clippy_allow() {
    let content = r#"
[workspace.lints.clippy]
all = "allow"
"#;
    let mut violations = Vec::new();
    checker().check_cargo_toml(content, &mut violations);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES304");
    assert_eq!(violations[0].severity, Severity::CRITICAL);
}

#[test]
fn detects_cargo_toml_clippy_allow_table_form() {
    let content = r#"
[lints.clippy]
warnings = { level = "allow", priority = -1 }
"#;
    let mut violations = Vec::new();
    checker().check_cargo_toml(content, &mut violations);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES304");
}

#[test]
fn cargo_toml_warn_level_not_flagged() {
    let content = r#"
[workspace.lints.clippy]
all = "warn"
"#;
    let mut violations = Vec::new();
    checker().check_cargo_toml(content, &mut violations);
    assert!(violations.is_empty());
}

// ─── Python: raise NotImplementedError ───────────────────────────────

#[test]
fn detects_python_raise_not_implemented() {
    let content = "raise NotImplementedError";
    let violations = scan_file("test.py", content);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES304");
}

// ─── JS/TS: throw new Error ──────────────────────────────────────────

#[test]
fn detects_js_throw_new_error() {
    let content = "throw new Error('fail');";
    let violations = scan_file("test.js", content);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES304");
}

// ─── Edge Case: Word boundary prevents false positives ───────────────

#[test]
fn no_false_positive_on_substring() {
    // "unwrapped" contains "unwrap" but is not a call
    let content = "let unwrapped_value = get_value();";
    assert!(scan(content).is_empty());
}

#[test]
fn no_false_positive_on_todo_in_comment_only() {
    // "todo" as a word token should not match inside a comment-only line
    // when it's not followed by ! or (
    let content = "// remember to do this later";
    assert!(scan(content).is_empty());
}
