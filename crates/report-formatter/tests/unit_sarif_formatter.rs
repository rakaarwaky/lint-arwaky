// PURPOSE: Unit tests for SarifFormatter — SARIF 2.1.0 output formatting.
// Layer: Capabilities (SarifFormatter)

use report_formatter_lint_arwaky::capabilities_sarif_formatter::SarifFormatter;
use shared::cli_commands::contract_report_formatter_protocol::IReportFormatterProtocol;
use shared::cli_commands::taxonomy_format_vo::Format;
use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultCode, LintResultMessage};
use shared::cli_commands::taxonomy_scan_report_vo::ScanReport;
use shared::common::taxonomy_severity_vo::Severity;

fn formatter() -> SarifFormatter {
    SarifFormatter::new()
}

// ─── format: Valid SARIF output structure ──

#[test]
fn sarif_formatter_formats_empty_report() {
    let formatter = formatter();
    let report = ScanReport::new(vec![], vec![]);

    let result = formatter.format(&report, Format::Sarif);
    assert!(!result.value.is_empty());
    assert!(result.value.contains("\"$schema\""));
    assert!(result.value.contains("\"version\" : \"2.1.0\""));
    assert!(result.value.contains("\"runs\""));
}

// ─── format: Correct tool driver name ──

#[test]
fn sarif_formatter_includes_tool_name() {
    let formatter = formatter();
    let report = ScanReport::new(vec![], vec![]);

    let result = formatter.format(&report, Format::Sarif);
    assert!(result.value.contains("\"name\" : \"lint-arwaky\""));
}

// ─── format: Severity mapping (High/Critical -> error) ──

#[test]
fn sarif_formatter_maps_high_to_error() {
    let formatter = formatter();
    let results = vec![LintResult::new(
        shared::common::taxonomy_path_vo::FilePath::new("test.rs".to_string()).unwrap(),
        1,
        0,
        LintResultCode { value: "HIGH001".to_string() },
        LintResultMessage { value: "High severity".to_string() },
        Severity::HIGH,
    )];
    let report = ScanReport::new(results, vec![]);

    let result = formatter.format(&report, Format::Sarif);
    assert!(result.value.contains("\"level\" : \"error\""));
}

// ─── format: Severity mapping (Medium -> warning) ──

#[test]
fn sarif_formatter_maps_medium_to_warning() {
    let formatter = formatter();
    let results = vec![LintResult::new(
        shared::common::taxonomy_path_vo::FilePath::new("test.rs".to_string()).unwrap(),
        1,
        0,
        LintResultCode { value: "MED001".to_string() },
        LintResultMessage { value: "Medium severity".to_string() },
        Severity::MEDIUM,
    )];
    let report = ScanReport::new(results, vec![]);

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
        LintResultCode { value: "LOW001".to_string() },
        LintResultMessage { value: "Low severity".to_string() },
        Severity::INFO,
    )];
    let report = ScanReport::new(results, vec![]);

    let result = formatter.format(&report, Format::Sarif);
    assert!(result.value.contains("\"level\" : \"note\""));
}

// ─── format: Falls back to default for non-SARIF format ──

#[test]
fn sarif_formatter_fallback_for_non_sarif_format() {
    let formatter = formatter();
    let report = ScanReport::new(vec![], vec![]);

    let result = formatter.format(&report, Format::Text);
    assert!(!result.value.is_empty());
}
