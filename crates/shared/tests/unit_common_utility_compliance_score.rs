// PURPOSE: Test compliance score utility from shared::common::utility_compliance_score

use shared_lint_arwaky::common::taxonomy_adapter_name_vo::AdapterName;
use shared_lint_arwaky::common::taxonomy_common_vo::LineNumber;
use shared_lint_arwaky::common::taxonomy_error_vo::ErrorCode;
use shared_lint_arwaky::common::taxonomy_lint_result_vo::LintResult;
use shared_lint_arwaky::common::taxonomy_lint_vo::LocationList;
use shared_lint_arwaky::common::taxonomy_message_vo::LintMessage;
use shared_lint_arwaky::common::taxonomy_path_vo::FilePath;
use shared_lint_arwaky::common::taxonomy_severity_vo::Severity;
use shared_lint_arwaky::common::utility_compliance_score::compute_score;

/// Helper: build a minimal LintResult with the given severity.
fn make_result(severity: Severity) -> LintResult {
    LintResult {
        file: FilePath::new("test.rs").unwrap(),
        line: LineNumber::new(1),
        column: shared_lint_arwaky::common::taxonomy_common_vo::ColumnNumber::new(0),
        code: ErrorCode::raw("AES101"),
        message: LintMessage::new("test"),
        source: Some(AdapterName::raw("architecture")),
        severity,
        enclosing_scope: None,
        related_locations: LocationList::new(),
    }
}

// ── Empty results ───────────────────────────────────────────

#[test]
fn compute_score_empty_returns_100() {
    let score = compute_score(&[]);
    assert_eq!(score, 100.0);
}

// ── Single-severity deductions ──────────────────────────────

#[test]
fn compute_score_info_no_penalty() {
    let results = vec![make_result(Severity::INFO)];
    assert_eq!(compute_score(&results), 100.0);
}

#[test]
fn compute_score_low_deducts_1() {
    let results = vec![make_result(Severity::LOW)];
    assert_eq!(compute_score(&results), 99.0);
}

#[test]
fn compute_score_medium_deducts_2() {
    let results = vec![make_result(Severity::MEDIUM)];
    assert_eq!(compute_score(&results), 98.0);
}

#[test]
fn compute_score_high_deducts_3() {
    let results = vec![make_result(Severity::HIGH)];
    assert_eq!(compute_score(&results), 97.0);
}

#[test]
fn compute_score_critical_deducts_5() {
    let results = vec![make_result(Severity::CRITICAL)];
    assert_eq!(compute_score(&results), 95.0);
}

// ── Multiple violations ─────────────────────────────────────

#[test]
fn compute_score_multiple_violations_sum() {
    let results = vec![
        make_result(Severity::HIGH),   // 3
        make_result(Severity::MEDIUM), // 2
        make_result(Severity::LOW),    // 1
    ];
    // penalty = 3 + 2 + 1 = 6
    assert_eq!(compute_score(&results), 94.0);
}

#[test]
fn compute_score_many_violations_clamped_at_zero() {
    // 30 CRITICAL violations = 30 * 5 = 150 penalty → clamped to 0.0
    let results: Vec<LintResult> = (0..30).map(|_| make_result(Severity::CRITICAL)).collect();
    assert_eq!(compute_score(&results), 0.0);
}

// ── Mixed severities ────────────────────────────────────────

#[test]
fn compute_score_mixed_severities() {
    let results = vec![
        make_result(Severity::INFO),     // 0
        make_result(Severity::LOW),      // 1
        make_result(Severity::LOW),      // 1
        make_result(Severity::MEDIUM),   // 2
        make_result(Severity::HIGH),     // 3
        make_result(Severity::CRITICAL), // 5
    ];
    // penalty = 0 + 1 + 1 + 2 + 3 + 5 = 12
    assert_eq!(compute_score(&results), 88.0);
}

// ── Boundary ────────────────────────────────────────────────

#[test]
fn compute_score_exactly_100_penalty() {
    // 20 CRITICAL = 100 penalty → exactly 0.0
    let results: Vec<LintResult> = (0..20).map(|_| make_result(Severity::CRITICAL)).collect();
    assert_eq!(compute_score(&results), 0.0);
}
