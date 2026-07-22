// PURPOSE: Unit tests for TextFormatter — human-readable text report generation.
// Layer: Capabilities (TextFormatter)

use report_formatter_lint_arwaky::capabilities_text_formatter::TextFormatter;
use shared::cli_commands::contract_report_formatter_protocol::IReportFormatterProtocol;
use shared::cli_commands::taxonomy_format_vo::Format;
use shared::cli_commands::taxonomy_lint_result_vo::{LintResult, Severity};
use shared::cli_commands::taxonomy_scan_report_vo::ScanReport;

fn formatter() -> TextFormatter {
    let code_analysis = Arc::new(
        code_analysis_lint_arwaky::root_code_analysis_container::CodeAnalysisContainer::default(),
    );
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
    let results = vec![LintResult::new(
        shared::common::taxonomy_path_vo::FilePath::new("test.rs".to_string()).unwrap(),
        10,
        5,
        shared::cli_commands::taxonomy_result_vo::LintResultCode::new("TEST001"),
        shared::cli_commands::taxonomy_result_vo::LintResultMessage::new(
            "Test violation message".to_string(),
        ),
        Severity::new(shared::common::taxonomy_severity_vo::SeverityLevel::Medium),
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
    let code_analysis = Arc::new(
        code_analysis_lint_arwaky::root_code_analysis_container::CodeAnalysisContainer::default(),
    );
    let _ = TextFormatter::default(code_analysis);
}
