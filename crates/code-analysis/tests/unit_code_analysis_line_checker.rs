// PURPOSE: Unit tests for ArchLineChecker (AES301/AES302) — file line count limits.

use code_analysis_lint_arwaky::ArchLineChecker;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::code_analysis::contract_line_protocol::ILineCheckerProtocol;
use shared::common::taxonomy_common_vo::{Count, PatternList};
use shared::common::taxonomy_definition_vo::LayerDefinition;

fn checker() -> ArchLineChecker {
    ArchLineChecker::new()
}

fn make_def(min: i64, max: i64) -> LayerDefinition {
    LayerDefinition {
        code_analysis: shared::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO {
            min_lines: Count::new(min),
            max_lines: Count::new(max),
            ..Default::default()
        },
        ..Default::default()
    }
}

fn check(file: &str, def: Option<&LayerDefinition>, content: &str) -> Vec<LintResult> {
    let mut violations = Vec::new();
    checker().check_line_counts(file, def, content, &mut violations);
    violations
}

// ─── Happy Path: File within limits ──────────────────────────────────

#[test]
fn file_within_limits_no_violation() {
    let def = make_def(5, 100);
    let content = "line1\nline2\nline3\nline4\nline5\nline6\nline7\nline8\nline9\nline10";
    let violations = check("capabilities_foo.rs", Some(&def), content);
    assert!(violations.is_empty());
}

// ─── AES301: File exceeds max lines ──────────────────────────────────

#[test]
fn file_exceeds_max_lines_aes301() {
    let def = make_def(5, 10);
    let content: String = (0..15)
        .map(|i| format!("line {}", i))
        .collect::<Vec<_>>()
        .join("\n");
    let violations = check("capabilities_foo.rs", Some(&def), &content);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES301");
    assert!(violations[0].message.value.contains("FILE_TOO_LARGE"));
}

// ─── AES302: File below min lines ────────────────────────────────────

#[test]
fn file_below_min_lines_aes302() {
    let def = make_def(10, 1000);
    let content = "line1\nline2\nline3";
    let violations = check("capabilities_foo.rs", Some(&def), content);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES302");
    assert!(violations[0].message.value.contains("FILE_TOO_SHORT"));
}

// ─── Edge Case: No definition provided → skip ────────────────────────

#[test]
fn no_definition_skips_check() {
    let content = "x";
    let violations = check("capabilities_foo.rs", None, content);
    assert!(violations.is_empty());
}

// ─── Edge Case: Barrel files (mod.rs) are skipped ────────────────────

#[test]
fn mod_rs_skipped() {
    let def = make_def(10, 1000);
    let content = "line1";
    let violations = check("mod.rs", Some(&def), content);
    assert!(violations.is_empty());
}

#[test]
fn init_py_skipped() {
    let def = make_def(10, 1000);
    let content = "line1";
    let violations = check("__init__.py", Some(&def), content);
    assert!(violations.is_empty());
}

// ─── Edge Case: Filename in exception list → skip ────────────────────

#[test]
fn exception_filename_skipped() {
    let mut def = make_def(10, 1000);
    def.exceptions = PatternList::new(vec!["special_file.rs".to_string()]);
    let content = "line1";
    let violations = check("special_file.rs", Some(&def), content);
    assert!(violations.is_empty());
}

// ─── Edge Case: min_lines = 0 disables min check ─────────────────────

#[test]
fn zero_min_lines_disables_min_check() {
    let def = make_def(0, 1000);
    let content = "line1";
    let violations = check("capabilities_foo.rs", Some(&def), content);
    assert!(violations.is_empty());
}

// ─── Edge Case: max_lines = 0 disables max check ─────────────────────

#[test]
fn zero_max_lines_disables_max_check() {
    let def = make_def(1, 0);
    let content: String = (0..5000)
        .map(|i| format!("line {}", i))
        .collect::<Vec<_>>()
        .join("\n");
    let violations = check("capabilities_foo.rs", Some(&def), &content);
    assert!(violations.is_empty());
}

// ─── Edge Case: Exactly at boundary (no violation) ───────────────────

#[test]
fn exactly_at_max_lines_no_violation() {
    let def = make_def(1, 10);
    let content: String = (0..10)
        .map(|i| format!("line {}", i))
        .collect::<Vec<_>>()
        .join("\n");
    let violations = check("capabilities_foo.rs", Some(&def), &content);
    assert!(violations.is_empty());
}

#[test]
fn exactly_at_min_lines_no_violation() {
    let def = make_def(5, 100);
    let content: String = (0..5)
        .map(|i| format!("line {}", i))
        .collect::<Vec<_>>()
        .join("\n");
    let violations = check("capabilities_foo.rs", Some(&def), &content);
    assert!(violations.is_empty());
}
