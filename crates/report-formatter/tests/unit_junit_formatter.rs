// PURPOSE: Unit tests for JunitFormatter — JUnit XML report serialization.
// Layer: Capabilities (JunitFormatter)

use report_formatter_lint_arwaky::capabilities_junit_formatter::JunitFormatter;
use shared::cli_commands::taxonomy_scan_report_vo::{DiagnosticSeverity, PipelineDiagnostic};
use shared::cli_commands::{Format, LintResult, ScanReport};
use shared::common::Severity;
use shared::report_formatter::IReportFormatterProtocol;

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

// ─── format: INFO severity produces clean testcase without failure ──

#[test]
fn junit_formatter_info_severity_no_failure() {
    let formatter = formatter();
    let results = vec![LintResult::new_arch(
        "test.rs",
        1,
        "AES001",
        Severity::INFO,
        "Info only",
    )];
    let report = ScanReport::new(results, vec![]);

    let result = formatter.format(&report, Format::Junit);
    assert!(result.value.contains("<testcase"));
    assert!(!result.value.contains("<failure"));
}

// ─── format: PARSE_WARN diagnostics produce skipped testcases ──

#[test]
fn junit_formatter_parses_warn_as_skipped() {
    let formatter = formatter();
    let diagnostics = vec![PipelineDiagnostic::new(
        "filesystem".to_string(),
        "File skipped: parse error".to_string(),
        DiagnosticSeverity::Warning,
    )];
    let report = ScanReport::new(vec![], diagnostics);

    let result = formatter.format(&report, Format::Junit);
    assert!(result.value.contains("<skipped"));
    assert!(result.value.contains("File skipped: parse error"));
}

// ─── format: Test/failure/skip counts match ──

#[test]
fn junit_formatter_correct_counts() {
    let formatter = formatter();
    let results = vec![
        LintResult::new_arch("a.rs", 1, "AES101", Severity::HIGH, "fail1"),
        LintResult::new_arch("b.rs", 2, "AES201", Severity::INFO, "info1"),
    ];
    let diagnostics = vec![PipelineDiagnostic::new(
        "fs".to_string(),
        "warn1".to_string(),
        DiagnosticSeverity::Warning,
    )];
    let report = ScanReport::new(results, diagnostics);

    let result = formatter.format(&report, Format::Junit);
    // 2 results + 1 diagnostic = 3 tests, 1 failure (HIGH), 1 skipped
    assert!(result.value.contains("tests=\"3\""));
    assert!(result.value.contains("failures=\"1\""));
    assert!(result.value.contains("skipped=\"1\""));
}

// ─── format: XML escaping works ──

#[test]
fn junit_formatter_escapes_xml_special_chars() {
    let formatter = formatter();
    let results = vec![LintResult::new_arch(
        "test.rs",
        1,
        "AES101",
        Severity::MEDIUM,
        "Message with <special> & \"chars\"",
    )];
    let report = ScanReport::new(results, vec![]);

    let result = formatter.format(&report, Format::Junit);
    assert!(result.value.contains("&lt;special&gt;"));
    assert!(result.value.contains("&amp;"));
    assert!(result.value.contains("&quot;"));
}

// ─── format: Falls back to default for non-JUnit format ──

#[test]
fn junit_formatter_fallback_for_non_junit_format() {
    let formatter = formatter();
    let report = ScanReport::new(vec![], vec![]);

    let result = formatter.format(&report, Format::Text);
    assert!(!result.value.is_empty());
}
