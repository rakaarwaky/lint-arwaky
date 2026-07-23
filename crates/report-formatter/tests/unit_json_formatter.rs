// PURPOSE: Unit tests for JsonFormatter — JSON report serialization.
// Layer: Capabilities (JsonFormatter)

use report_formatter_lint_arwaky::capabilities_json_formatter::JsonFormatter;
use shared::cli_commands::taxonomy_format_vo::Format;
use shared::report_formatter::contract_report_formatter_protocol::IReportFormatterProtocol;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_scan_report_vo::ScanReport;
use shared::common::taxonomy_severity_vo::Severity;

fn formatter() -> JsonFormatter {
    JsonFormatter::new()
}

// ─── format: JSON output for empty report ──

#[test]
fn json_formatter_formats_empty_report() {
    let formatter = formatter();
    let report = ScanReport::new(vec![], vec![]);

    let result = formatter.format(&report, Format::Json);
    assert!(!result.value.is_empty());
    assert!(result.value.contains("["));
    assert!(result.value.contains("]"));
}

// ─── format: JSON output for report with results ──

#[test]
fn json_formatter_formats_report_with_results() {
    let formatter = formatter();
    let results = vec![LintResult::new_arch_with_column(
        "test.rs",
        1,
        0,
        "TEST001",
        Severity::MEDIUM,
        "Test message",
    )];
    let report = ScanReport::new(results, vec![]);

    let result = formatter.format(&report, Format::Json);
    assert!(result.value.contains("Test message"));
    assert!(result.value.contains("TEST001"));
}

// ─── format: Falls back to default for non-JSON format ──

#[test]
fn json_formatter_fallback_for_non_json_format() {
    let formatter = formatter();
    let report = ScanReport::new(vec![], vec![]);

    // When format is Text, JsonFormatter should fall back to default formatting
    let result = formatter.format(&report, Format::Text);
    assert!(!result.value.is_empty());
}
