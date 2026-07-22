// PURPOSE: Unit tests for SarifFormatter — SARIF 2.1.0 JSON report generation.
// Layer: Capabilities (SarifFormatter)

use report_formatter_lint_arwaky::capabilities_sarif_formatter::SarifFormatter;
use shared::cli_commands::contract_report_formatter_protocol::IReportFormatterProtocol;
use shared::cli_commands::taxonomy_format_vo::Format;
use shared::cli_commands::taxonomy_lint_result_vo::{LintResult, Severity};
use shared::cli_commands::taxonomy_scan_report_vo::ScanReport;

fn formatter() -> SarifFormatter {
    SarifFormatter::new()
}

// ─── format: SARIF JSON for empty report ──

#[test]
fn sarif_formatter_formats_empty_report() {
    let formatter = formatter();
    let report = ScanReport::new(vec![], vec![], None);

    let result = formatter.format(&report, Format::Sarif);
    assert!(result.value.contains("\"@schema\""));
    assert!(result.value.contains("sarif-schema"));
}

// ─── format: SARIF JSON with results ──

#[test]
fn sarif_formatter_formats_report_with_results() {
    let formatter = formatter();
    let results = vec![LintResult::new(
        shared::common::taxonomy_path_vo::FilePath::new("test.rs".to_string()).unwrap(),
        10,
        5,
        shared::cli_commands::taxonomy_result_vo::LintResultCode::new("TEST001"),
        shared::cli_commands::taxonomy_result_vo::LintResultMessage::new(
            "Test violation".to_string(),
        ),
        Severity::new(shared::cli_commands::taxonomy_severity_vo::SeverityLevel::Medium),
    )];
    let report = ScanReport::new(results, vec![], None);

    let result = formatter.format(&report, Format::Sarif);
    assert!(result.value.contains("test.rs"));
    assert!(result.value.contains("TEST001"));
}

// ─── format: Severity-to-SARIF mapping ──

#[test]
fn sarif_formatter_maps_high_to_error() {
    let formatter = formatter();
    let results = vec![LintResult::new(
        shared::common::taxonomy_path_vo::FilePath::new("test.rs".to_string()).unwrap(),
        1,
        0,
        shared::cli_commands::taxonomy_result_vo::LintResultCode::new("HIGH001"),
        shared::cli_commands::taxonomy_result_vo::LintResultMessage::new(
            "High severity".to_string(),
        ),
        Severity::new(shared::cli_commands::taxonomy_severity_vo::SeverityLevel::High),
    )];
    let report = ScanReport::new(results, vec![], None);

    let result = formatter.format(&report, Format::Sarif);
    assert!(result.value.contains("\"level\" : \"error\""));
}

#[test]
fn sarif_formatter_maps_medium_to_warning() {
    let formatter = formatter();
    let results = vec![LintResult::new(
        shared::common::taxonomy_path_vo::FilePath::new("test.rs".to_string()).unwrap(),
        1,
        0,
        shared::cli_commands::taxonomy_result_vo::LintResultCode::new("MED001"),
        shared::cli_commands::taxonomy_result_vo::LintResultMessage::new(
            "Medium severity".to_string(),
        ),
        Severity::new(shared::cli_commands::taxonomy_severity_vo::SeverityLevel::Medium),
    )];
    let report = ScanReport::new(results, vec![], None);

    let result = formatter.format(&report, Format::Sarif);
    assert!(result.value.contains("\"level\" : \"warning\""));
}

#[test]
fn sarif_formatter_maps_low_to_note() {
    let formatter = formatter();
    let results = vec![LintResult::new(
        shared::common::taxonomy_path_vo::FilePath::new("test.rs".to_string()).unwrap(),
        1,
        0,
        shared::cli_commands::taxonomy_result_vo::LintResultCode::new("LOW001"),
        shared::cli_commands::taxonomy_result_vo::LintResultMessage::new(
            "Low severity".to_string(),
        ),
        Severity::new(shared::cli_commands::taxonomy_severity_vo::SeverityLevel::Info),
    )];
    let report = ScanReport::new(results, vec![], None);

    let result = formatter.format(&report, Format::Sarif);
    assert!(result.value.contains("\"level\" : \"note\""));
}

// ─── format: Falls back to default for non-SARIF format ──

#[test]
fn sarif_formatter_fallback_for_non_sarif_format() {
    let formatter = formatter();
    let report = ScanReport::new(vec![], vec![], None);

    let result = formatter.format(&report, Format::Text);
    assert!(!result.value.is_empty());
}

// ─── Default trait ──

#[test]
fn sarif_formatter_default_creates_valid_instance() {
    let _ = SarifFormatter::default();
}
