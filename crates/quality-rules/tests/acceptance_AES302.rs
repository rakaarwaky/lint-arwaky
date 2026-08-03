// PURPOSE: Acceptance test AES302 — min line count below threshold
// Create a file with <10 lines, verify violation when configured with min_lines > line count
use shared::cli_commands::LintResult;
use shared::common::{Count, LayerDefinition};
use shared::quality_rules::ILineCheckerProtocol;

use quality_rules_lint_arwaky::capabilities_line_checker::ArchLineChecker;

fn make_layer_def_with_min(min: i64) -> LayerDefinition {
    LayerDefinition {
        code_analysis: shared::quality_rules::CodeAnalysisRuleVO {
            min_lines: Count::new(min),
            max_lines: Count::new(0),
            ..Default::default()
        },
        exceptions: shared::common::PatternList { values: vec![] },
        ..Default::default()
    }
}

#[test]
fn file_below_min_lines_produces_aes302_violation() {
    let checker = ArchLineChecker::new();
    let def = make_layer_def_with_min(10);

    // Only 3 lines — below minimum of 10
    let content = "line1\nline2\nline3\n";
    let mut violations: Vec<LintResult> = Vec::new();

    checker.check_line_counts(
        "src/capabilities/tiny.rs",
        Some(&def),
        content,
        &mut violations,
    );

    assert_eq!(violations.len(), 1, "Expected exactly 1 AES302 violation");
    assert!(
        violations[0].code.code().contains("AES302"),
        "Expected AES302 code, got: {}",
        violations[0].code.code()
    );
    assert_eq!(violations[0].severity, shared::common::Severity::HIGH);
}

#[test]
fn file_exactly_at_min_lines_no_violation() {
    let checker = ArchLineChecker::new();
    let def = make_layer_def_with_min(5);

    // Exactly 5 lines
    let content = "line1\nline2\nline3\nline4\nline5\n";
    let mut violations: Vec<LintResult> = Vec::new();

    checker.check_line_counts(
        "src/capabilities/exact.rs",
        Some(&def),
        content,
        &mut violations,
    );

    assert!(violations.is_empty(), "No violation when exactly at min");
}

#[test]
fn file_above_min_lines_no_violation() {
    let checker = ArchLineChecker::new();
    let def = make_layer_def_with_min(5);

    let content = "line1\nline2\nline3\nline4\nline5\nline6\nline7\n";
    let mut violations: Vec<LintResult> = Vec::new();

    checker.check_line_counts(
        "src/capabilities/ok.rs",
        Some(&def),
        content,
        &mut violations,
    );

    assert!(violations.is_empty(), "No violation when above min");
}

#[test]
fn aes302_violation_message_contains_line_count() {
    let checker = ArchLineChecker::new();
    let def = make_layer_def_with_min(20);

    let content = "a\nb\nc\n";
    let mut violations: Vec<LintResult> = Vec::new();

    checker.check_line_counts(
        "src/capabilities/short.rs",
        Some(&def),
        content,
        &mut violations,
    );

    assert_eq!(violations.len(), 1);
    assert!(
        violations[0].message.value.contains("20"),
        "Message should reference min line count"
    );
}

#[test]
fn empty_file_below_min_produces_violation() {
    let checker = ArchLineChecker::new();
    let def = make_layer_def_with_min(10);

    let content = "";
    let mut violations: Vec<LintResult> = Vec::new();

    checker.check_line_counts(
        "src/capabilities/empty.rs",
        Some(&def),
        content,
        &mut violations,
    );

    assert_eq!(violations.len(), 1);
    assert!(violations[0].code.code().contains("AES302"));
}
