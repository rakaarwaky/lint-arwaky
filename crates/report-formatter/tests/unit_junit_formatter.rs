// PURPOSE: Unit tests for JunitFormatter — JUnit XML report serialization.
// Layer: Capabilities (JunitFormatter)

use report_formatter_lint_arwaky::capabilities_junit_formatter::JunitFormatter;
use shared::cli_commands::taxonomy_format_vo::Format;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_scan_report_vo::ScanReport;
use shared::common::taxonomy_severity_vo::Severity;
use shared::report_formatter::contract_report_formatter_protocol::IReportFormatterProtocol;

fn formatter() -> JunitFormatter {
    JunitFormatter::new()
}

// ─── format: Empty report produces valid JUnit XML ──

#[test]
fn junit_formatter_formats_empty_report() {
    let formatter = formatter();
    let report = ScanReport::new(vec![], vec![]);

    let result = formatter.format(&report, Format::Junit);
    assert!(!result.value.is_empty());
    assert!(result.value.contains("<?xml"));
    assert!(result.value.contains("<testsuites"));
    assert!(result.value.contains("</testsuites>"));
}

// ─── format: Report with results generates testcases ──

#[test]
fn junit_formatter_formats_report_with_results() {
    let formatter = formatter();
    let results = vec![LintResult::new_arch_with_column(
        "test.rs",
        1,
        0,
        "TEST001",
        Severity::MEDIUM,
        "Test violation",
    )];
    let report = ScanReport::new(results, vec![]);

    let result = formatter.format(&report, Format::Junit);
    assert!(result.value.contains("<testcase"));
    assert!(result.value.contains("TEST001"));
}

// ─── format: Failure details included for high severity ──

#[test]
fn junit_formatter_includes_failure_element() {
    let formatter = formatter();
    let results = vec![LintResult::new_arch_with_column(
        "test.rs",
        10,
        5,
        "TEST001",
        Severity::HIGH,
        "Violation",
    )];
    let report = ScanReport::new(results, vec![]);

    let result = formatter.format(&report, Format::Junit);
    assert!(result.value.contains("<failure"));
}

// ─── format: Falls back to default for non-JUnit format ──

#[test]
fn junit_formatter_fallback_for_non_junit_format() {
    let formatter = formatter();
    let report = ScanReport::new(vec![], vec![]);

    let result = formatter.format(&report, Format::Text);
    assert!(!result.value.is_empty());
}
