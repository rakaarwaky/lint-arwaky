// PURPOSE: Unit tests for JsonFormatter — JSON report serialization.
// Layer: Capabilities (JsonFormatter)

use report_formatter_lint_arwaky::capabilities_json_formatter::JsonFormatter;
use shared::cli_commands::taxonomy_scan_report_vo::{DiagnosticSeverity, PipelineDiagnostic};
use shared::cli_commands::{Format, LintResult, ScanReport};
use shared::common::Severity;
use shared::report_formatter::IReportFormatterProtocol;

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
    // Full structure: violations, external_results, diagnostics, summary
    assert!(result.value.contains("\"violations\""));
    assert!(result.value.contains("\"external_results\""));
    assert!(result.value.contains("\"diagnostics\""));
    assert!(result.value.contains("\"summary\""));
}

// ─── format: JSON output for report with results ──

#[test]
fn json_formatter_formats_report_with_results() {
    let formatter = formatter();
    let results = vec![LintResult::new_arch_with_column(
        "test.rs",
        1,
        0,
        "AES101",
        Severity::MEDIUM,
        "Test message",
    )];
    let report = ScanReport::new(results, vec![]);

    let result = formatter.format(&report, Format::Json);
    assert!(result.value.contains("Test message"));
    assert!(result.value.contains("AES101"));
    assert!(result.value.contains("\"violations\""));
}

// ─── format: External lint results separate from AES violations ──

#[test]
fn json_formatter_separates_external_results() {
    let formatter = formatter();
    let results = vec![
        LintResult::new_arch("a.rs", 1, "AES201", Severity::HIGH, "AES issue"),
        LintResult::new_arch(
            "b.rs",
            2,
            "clippy::needless_return",
            Severity::MEDIUM,
            "clippy issue",
        ),
    ];
    let report = ScanReport::new(results, vec![]);

    let result = formatter.format(&report, Format::Json);
    assert!(result.value.contains("\"violations\""));
    assert!(result.value.contains("\"external_results\""));
    assert!(result.value.contains("AES201"));
    assert!(result.value.contains("clippy::needless_return"));
}

// ─── format: Diagnostics included in JSON output ──

#[test]
fn json_formatter_includes_diagnostics() {
    let formatter = formatter();
    let diagnostics = vec![PipelineDiagnostic::new(
        "filesystem".to_string(),
        "File skipped: parse error".to_string(),
        DiagnosticSeverity::Warning,
    )];
    let report = ScanReport::new(vec![], diagnostics);

    let result = formatter.format(&report, Format::Json);
    assert!(result.value.contains("\"diagnostics\""));
    assert!(result.value.contains("File skipped: parse error"));
}

// ─── format: Summary section with counts ──

#[test]
fn json_formatter_includes_summary() {
    let formatter = formatter();
    let results = vec![
        LintResult::new_arch("a.rs", 1, "AES201", Severity::CRITICAL, "crit"),
        LintResult::new_arch("b.rs", 2, "AES101", Severity::MEDIUM, "med"),
    ];
    let report = ScanReport::new(results, vec![]);

    let result = formatter.format(&report, Format::Json);
    assert!(result.value.contains("\"summary\""));
    assert!(result.value.contains("\"total_violations\": 2"));
    assert!(result.value.contains("\"critical\": 1"));
    assert!(result.value.contains("\"medium\": 1"));
}

// ─── format: Falls back to default for non-JSON format ──

#[test]
fn json_formatter_fallback_for_non_json_format() {
    let formatter = formatter();
    let report = ScanReport::new(vec![], vec![]);

    let result = formatter.format(&report, Format::Text);
    assert!(!result.value.is_empty());
}
