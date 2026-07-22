// PURPOSE: Unit tests for JunitFormatter — JUnit XML report generation.
// Layer: Capabilities (JunitFormatter)

use report_formatter_lint_arwaky::capabilities_junit_formatter::JunitFormatter;
use shared::cli_commands::contract_report_formatter_protocol::IReportFormatterProtocol;
use shared::cli_commands::taxonomy_format_vo::Format;
use shared::cli_commands::taxonomy_lint_result_vo::{LintResult, Severity};
use shared::cli_commands::taxonomy_scan_report_vo::ScanReport;

fn formatter() -> JunitFormatter {
    JunitFormatter::new()
}

// ─── format: JUnit XML for empty report ──

#[test]
fn junit_formatter_formats_empty_report() {
    let formatter = formatter();
    let report = ScanReport::new(vec![], vec![]);

    let result = formatter.format(&report, Format::Junit);
    assert!(result.value.contains("<testsuites"));
    assert!(result.value.contains("</testsuites>"));
}

// ─── format: JUnit XML with test cases ──

#[test]
fn junit_formatter_formats_report_with_results() {
    let formatter = formatter();
    let results = vec![LintResult::new(
        shared::common::taxonomy_path_vo::FilePath::new("test.rs".to_string()).unwrap(),
        10,
        5,
        shared::cli_commands::taxonomy_result_vo::LintResultCode::new("TEST001"),
        shared::cli_commands::taxonomy_result_vo::LintResultMessage::new(
            "Test violation".to_string(),
        ),
        Severity::new(shared::common::taxonomy_severity_vo::SeverityLevel::Medium),
    )];
    let report = ScanReport::new(results, vec![]);

    let result = formatter.format(&report, Format::Junit);
    assert!(result.value.contains("<testcase"));
    assert!(result.value.contains("test.rs:10"));
}

// ─── format: Failure element for non-INFO severity ──

#[test]
fn junit_formatter_includes_failure_for_violations() {
    let formatter = formatter();
    let results = vec![LintResult::new(
        shared::common::taxonomy_path_vo::FilePath::new("test.rs".to_string()).unwrap(),
        10,
        5,
        shared::cli_commands::taxonomy_result_vo::LintResultCode::new("TEST001"),
        shared::cli_commands::taxonomy_result_vo::LintResultMessage::new("Violation".to_string()),
        Severity::new(shared::common::taxonomy_severity_vo::SeverityLevel::High),
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

// ─── xml_escape: Special characters ──

#[test]
fn junit_formatter_escapes_xml_special_chars() {
    let formatter = formatter();
    // The xml_escape helper is private, but we can verify through format output
    let results = vec![LintResult::new(
        shared::common::taxonomy_path_vo::FilePath::new("test.rs".to_string()).unwrap(),
        1,
        0,
        shared::cli_commands::taxonomy_result_vo::LintResultCode::new("TEST001"),
        shared::cli_commands::taxonomy_result_vo::LintResultMessage::new(
            "<script>alert('xss')</script>".to_string(),
        ),
        Severity::new(shared::common::taxonomy_severity_vo::SeverityLevel::Info),
    )];
    let report = ScanReport::new(results, vec![]);

    let result = formatter.format(&report, Format::Junit);
    assert!(result.value.contains("&lt;script&gt;"));
}

// ─── Default trait ──

#[test]
fn junit_formatter_default_creates_valid_instance() {
    let _ = JunitFormatter::default();
}
