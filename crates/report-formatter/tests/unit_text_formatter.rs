// PURPOSE: Unit tests for TextFormatter — human-readable text report generation.
// Layer: Capabilities (TextFormatter)

use report_formatter_lint_arwaky::capabilities_text_formatter::TextFormatter;
use shared::cli_commands::taxonomy_format_vo::Format;
use shared::report_formatter::contract_report_formatter_protocol::IReportFormatterProtocol;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_scan_report_vo::ScanReport;
use shared::common::taxonomy_severity_vo::Severity;

fn formatter() -> TextFormatter {
    let code_analysis =
        code_analysis::root_code_analysis_container::CodeAnalysisContainer::default()
            .code_analysis_linter();
    TextFormatter::new(code_analysis)
}

// ─── format: Text output for empty report ──

#[test]
fn text_formatter_formats_empty_report() {
    let formatter = formatter();
    let report = ScanReport::new(vec![], vec![]);

    let result = formatter.format(&report, Format::Text);
    assert!(!result.value.is_empty());
}

// ─── format: Text output with results ──

#[test]
fn text_formatter_formats_report_with_results() {
    let formatter = formatter();
    let results = vec![LintResult::new_arch_with_column(
        "test.rs",
        10,
        5,
        "TEST001",
        Severity::MEDIUM,
        "Test violation message",
    )];
    let report = ScanReport::new(results, vec![]);

    let result = formatter.format(&report, Format::Text);
    assert!(result.value.contains("TEST001"));
    assert!(result.value.contains("test.rs"));
}

// ─── format: Falls back to default for non-Text format ──

#[test]
fn text_formatter_fallback_for_non_text_format() {
    let formatter = formatter();
    let report = ScanReport::new(vec![], vec![]);

    let result = formatter.format(&report, Format::Json);
    assert!(!result.value.is_empty());
}

// ─── Default trait ──

#[test]
fn text_formatter_default_creates_valid_instance() {
    let code_analysis =
        code_analysis::root_code_analysis_container::CodeAnalysisContainer::default()
            .code_analysis_linter();
    let _ = TextFormatter::new(code_analysis);
}
