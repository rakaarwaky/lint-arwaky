// PURPOSE: Acceptance test AES301 — max line count exceeded
// Create a file with >1000 lines, verify violation when configured with max_lines < line count
use shared::cli_commands::LintResult;
use shared::common::{Count, LayerDefinition};
use shared::quality_rules::ILineCheckerProtocol;

use quality_rules_lint_arwaky::capabilities_line_checker::ArchLineChecker;

fn make_layer_def_with_max(max: i64) -> LayerDefinition {
    LayerDefinition {
        code_analysis: shared::quality_rules::CodeAnalysisRuleVO {
            max_lines: Count::new(max),
            min_lines: Count::new(0),
            ..Default::default()
        },
        exceptions: shared::common::PatternList { values: vec![] },
        ..Default::default()
    }
}

#[test]
fn file_exceeding_max_lines_produces_aes301_violation() {
    let checker = ArchLineChecker::new();
    let def = make_layer_def_with_max(1000);

    // Generate 1001 lines
    let content: String = (0..1001)
        .map(|i| format!("line_{}", i))
        .collect::<Vec<_>>()
        .join("\n");
    let mut violations: Vec<LintResult> = Vec::new();

    checker.check_line_counts(
        "src/capabilities/my_file.rs",
        Some(&def),
        &content,
        &mut violations,
    );

    assert_eq!(violations.len(), 1, "Expected exactly 1 AES301 violation");
    assert!(
        violations[0].code.code().contains("AES301"),
        "Expected AES301 code, got: {}",
        violations[0].code.code()
    );
    assert_eq!(violations[0].severity, shared::common::Severity::HIGH);
}

#[test]
fn file_exactly_at_max_lines_no_violation() {
    let checker = ArchLineChecker::new();
    let def = make_layer_def_with_max(100);

    // Generate exactly 100 lines
    let content: String = (0..100)
        .map(|i| format!("line_{}", i))
        .collect::<Vec<_>>()
        .join("\n");
    let mut violations: Vec<LintResult> = Vec::new();

    checker.check_line_counts(
        "src/capabilities/my_file.rs",
        Some(&def),
        &content,
        &mut violations,
    );

    assert!(violations.is_empty(), "No violation when exactly at max");
}

#[test]
fn file_within_max_lines_no_violation() {
    let checker = ArchLineChecker::new();
    let def = make_layer_def_with_max(1000);

    let content: String = (0..50)
        .map(|i| format!("line_{}", i))
        .collect::<Vec<_>>()
        .join("\n");
    let mut violations: Vec<LintResult> = Vec::new();

    checker.check_line_counts(
        "src/capabilities/my_file.rs",
        Some(&def),
        &content,
        &mut violations,
    );

    assert!(violations.is_empty(), "No violation when within max");
}

#[test]
fn aes301_violation_message_contains_details() {
    let checker = ArchLineChecker::new();
    let def = make_layer_def_with_max(10);

    let content: String = (0..20)
        .map(|i| format!("line_{}", i))
        .collect::<Vec<_>>()
        .join("\n");
    let mut violations: Vec<LintResult> = Vec::new();

    checker.check_line_counts(
        "src/capabilities/big_file.rs",
        Some(&def),
        &content,
        &mut violations,
    );

    assert_eq!(violations.len(), 1);
    assert!(
        violations[0].message.value.contains("10"),
        "Message should reference max line count"
    );
}
