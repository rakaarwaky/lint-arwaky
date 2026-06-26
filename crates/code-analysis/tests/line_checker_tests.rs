use code_analysis_lint_arwaky::capabilities_line_checker::ArchLineChecker;
use shared::code_analysis::contract_line_protocol::ILineCheckerProtocol;
use shared::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO;
use shared::taxonomy_common_vo::{Count, PatternList};
use shared::taxonomy_definition_vo::LayerDefinition;

fn make_def(min_lines: i64, max_lines: i64) -> Option<LayerDefinition> {
    Some(LayerDefinition {
        code_analysis: CodeAnalysisRuleVO {
            min_lines: Count::new(min_lines),
            max_lines: Count::new(max_lines),
            ..Default::default()
        },
        exceptions: PatternList::default(),
        ..Default::default()
    })
}

fn make_def_with_exception(
    min_lines: i64,
    max_lines: i64,
    exception: &str,
) -> Option<LayerDefinition> {
    Some(LayerDefinition {
        code_analysis: CodeAnalysisRuleVO {
            min_lines: Count::new(min_lines),
            max_lines: Count::new(max_lines),
            ..Default::default()
        },
        exceptions: PatternList::new(vec![exception.to_string()]),
        ..Default::default()
    })
}

#[test]
fn line_checker_skips_mod_rs() {
    let checker = ArchLineChecker::new();
    let mut violations = Vec::new();
    checker.check_line_counts(
        "src/mod.rs",
        make_def(5, 20).as_ref(),
        "a\nb\nc\nd\ne\nf\n",
        &mut violations,
    );
    assert!(violations.is_empty(), "mod.rs should be skipped");
}

#[test]
fn line_checker_skips_init_py() {
    let checker = ArchLineChecker::new();
    let mut violations = Vec::new();
    checker.check_line_counts(
        "src/__init__.py",
        make_def(5, 20).as_ref(),
        "a\nb\nc\nd\ne\nf\n",
        &mut violations,
    );
    assert!(violations.is_empty(), "__init__.py should be skipped");
}

#[test]
fn line_checker_within_limits() {
    let checker = ArchLineChecker::new();
    let mut violations = Vec::new();
    checker.check_line_counts(
        "src/test.rs",
        make_def(2, 10).as_ref(),
        "a\nb\nc\nd\ne\n",
        &mut violations,
    );
    assert!(
        violations.is_empty(),
        "5 lines within [2, 10] should be fine"
    );
}

#[test]
fn line_checker_too_large() {
    let checker = ArchLineChecker::new();
    let mut violations = Vec::new();
    checker.check_line_counts(
        "src/test.rs",
        make_def(2, 3).as_ref(),
        "a\nb\nc\nd\ne\nf\n",
        &mut violations,
    );
    assert_eq!(violations.len(), 1);
    assert!(violations[0].code.to_string().contains("AES301"));
}

#[test]
fn line_checker_too_short() {
    let checker = ArchLineChecker::new();
    let mut violations = Vec::new();
    checker.check_line_counts(
        "src/test.rs",
        make_def(5, 10).as_ref(),
        "a\nb\nc\n",
        &mut violations,
    );
    assert_eq!(violations.len(), 1);
    assert!(violations[0].code.to_string().contains("AES302"));
}

#[test]
fn line_checker_skips_exceptions() {
    let checker = ArchLineChecker::new();
    let mut violations = Vec::new();
    checker.check_line_counts(
        "generated.rs",
        make_def_with_exception(2, 3, "generated.rs").as_ref(),
        "a\nb\nc\nd\ne\nf\n",
        &mut violations,
    );
    assert!(
        violations.is_empty(),
        "exceptions should skip even if too large"
    );
}

#[test]
fn line_checker_no_definition_no_check() {
    let checker = ArchLineChecker::new();
    let mut violations = Vec::new();
    checker.check_line_counts("src/test.rs", None, "a\nb\nc\n", &mut violations);
    assert!(violations.is_empty(), "no definition means no check");
}
