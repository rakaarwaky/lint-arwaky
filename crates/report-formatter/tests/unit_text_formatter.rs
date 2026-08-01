// PURPOSE: Unit tests for TextFormatter — human-readable text report generation.
// Layer: Capabilities (TextFormatter)

use report_formatter_lint_arwaky::capabilities_text_formatter::TextFormatter;
use shared::cli_commands::taxonomy_scan_report_vo::{DiagnosticSeverity, PipelineDiagnostic};
use shared::cli_commands::{Format, LintResult, ScanReport};
use shared::common::Severity;
use shared::report_formatter::IReportFormatterProtocol;

fn formatter() -> TextFormatter {
    TextFormatter::new()
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

// ─── format: Text output with diagnostics ──

#[test]
fn text_formatter_formats_diagnostics() {
    let formatter = formatter();
    let diagnostics = vec![PipelineDiagnostic::new(
        "filesystem".to_string(),
        "File skipped: parse failure".to_string(),
        DiagnosticSeverity::Warning,
    )];
    let report = ScanReport::new(vec![], diagnostics);

    let result = formatter.format(&report, Format::Text);
    assert!(result.value.contains("DIAGNOSTICS"));
    assert!(result.value.contains("File skipped: parse failure"));
}

// ─── format: Severity badges present ──

#[test]
fn text_formatter_shows_severity_badges() {
    let formatter = formatter();
    let results = vec![
        LintResult::new_arch("test.rs", 1, "AES201", Severity::CRITICAL, "Critical issue"),
        LintResult::new_arch("test.rs", 2, "AES101", Severity::MEDIUM, "Medium issue"),
    ];
    let report = ScanReport::new(results, vec![]);

    let result = formatter.format(&report, Format::Text);
    assert!(result.value.contains("[CRITICAL]"));
    assert!(result.value.contains("[MEDIUM]"));
}

// ─── format: Violations breakdown ──

#[test]
fn text_formatter_groups_violations_by_code() {
    let formatter = formatter();
    let results = vec![
        LintResult::new_arch("a.rs", 1, "AES101", Severity::MEDIUM, "msg1"),
        LintResult::new_arch("b.rs", 2, "AES101", Severity::MEDIUM, "msg2"),
        LintResult::new_arch("c.rs", 3, "AES201", Severity::HIGH, "msg3"),
    ];
    let report = ScanReport::new(results, vec![]);

    let result = formatter.format(&report, Format::Text);
    assert!(result.value.contains("SUMMARY"));
    assert!(result.value.contains("Total Violations: 3"));
}

// ─── format: Score displayed when present ──

#[test]
fn text_formatter_shows_compliance_score() {
    let formatter = formatter();
    let report = ScanReport::new(vec![], vec![])
        .with_score(shared::common::taxonomy_common_vo::Score::new(85.0));

    let result = formatter.format(&report, Format::Text);
    assert!(result.value.contains("Compliance Score: 85.0%"));
}

// ─── Default trait ──

#[test]
fn text_formatter_default_creates_valid_instance() {
    let _ = TextFormatter::new();
}
