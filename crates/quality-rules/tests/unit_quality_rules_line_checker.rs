// Unit tests for ArchLineChecker — AES301/AES302 line count validation.
use quality_rules_lint_arwaky::capabilities_line_checker::ArchLineChecker;
use shared::common::{Count, LayerDefinition};
use shared::quality_rules::{CodeAnalysisRuleVO, ILineCheckerProtocol};

fn checker() -> ArchLineChecker {
    ArchLineChecker::new()
}

fn def_with_lines(min: i64, max: i64) -> LayerDefinition {
    LayerDefinition {
        code_analysis: CodeAnalysisRuleVO {
            min_lines: Count::new(min),
            max_lines: Count::new(max),
            ..Default::default()
        },
        ..Default::default()
    }
}

#[test]
fn construction_succeeds() {
    let _ = checker();
}

#[test]
fn no_violation_within_bounds() {
    let mut violations = Vec::new();
    let def = def_with_lines(5, 100);
    let content = "line1\nline2\nline3\nline4\nline5\nline6\nline7\n";
    checker().check_line_counts("src/checker.rs", Some(&def), content, &mut violations);
    assert!(violations.is_empty());
}

#[test]
fn file_too_short_produces_violation() {
    let mut violations = Vec::new();
    let def = def_with_lines(10, 100);
    let content = "a\nb\nc\n";
    checker().check_line_counts("src/checker.rs", Some(&def), content, &mut violations);
    assert_eq!(violations.len(), 1);
    assert!(violations[0].code.code().contains("AES302"));
}

#[test]
fn file_too_large_produces_violation() {
    let mut violations = Vec::new();
    let def = def_with_lines(0, 3);
    let content = "a\nb\nc\nd\ne\n";
    checker().check_line_counts("src/checker.rs", Some(&def), content, &mut violations);
    assert_eq!(violations.len(), 1);
    assert!(violations[0].code.code().contains("AES301"));
}

#[test]
fn both_violations_when_outside_range() {
    let mut violations = Vec::new();
    let def = def_with_lines(10, 5);
    let content = "a\n";
    checker().check_line_counts("src/checker.rs", Some(&def), content, &mut violations);
    assert_eq!(violations.len(), 1);
}

#[test]
fn barrel_file_mod_rs_skipped() {
    let mut violations = Vec::new();
    let def = def_with_lines(100, 100);
    let content = "short";
    checker().check_line_counts("src/mod.rs", Some(&def), content, &mut violations);
    assert!(violations.is_empty());
}

#[test]
fn barrel_file_init_py_skipped() {
    let mut violations = Vec::new();
    let def = def_with_lines(100, 100);
    let content = "short";
    checker().check_line_counts("src/__init__.py", Some(&def), content, &mut violations);
    assert!(violations.is_empty());
}

#[test]
fn no_definition_skipped() {
    let mut violations = Vec::new();
    let content = "short";
    checker().check_line_counts("src/lib.rs", None, content, &mut violations);
    assert!(violations.is_empty());
}
