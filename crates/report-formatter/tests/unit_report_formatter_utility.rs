// Unit tests — format_report_default utility (FR-006): simple text summary fallback.
use report_formatter_lint_arwaky::utility_report_format::format_report_default;
use shared::cli_commands::DiagnosticSeverity;
use shared::cli_commands::{LintResult, PipelineDiagnostic, ScanReport};
use shared::common::{ErrorCode, Severity};

#[test]
fn empty_report_shows_zero_counts() {
    let out = format_report_default(&ScanReport::new(vec![], vec![]));
    assert!(out.contains("Violations: 0"));
    assert!(out.contains("Diagnostics: 0"));
}

#[test]
fn violations_grouped_by_code_descending() {
    let report = ScanReport {
        results: vec![
            LintResult {
                code: ErrorCode::raw("AES101"),
                severity: Severity::LOW,
                ..Default::default()
            },
            LintResult {
                code: ErrorCode::raw("AES201"),
                severity: Severity::HIGH,
                ..Default::default()
            },
            LintResult {
                code: ErrorCode::raw("AES201"),
                severity: Severity::MEDIUM,
                ..Default::default()
            },
        ],
        diagnostics: vec![],
        score: None,
    };
    let out = format_report_default(&report);
    assert!(out.contains("Violations: 3"));
    let section = &out[out.find("Violations by code:").expect("section present")..];
    let aes201 = section.find("AES201: 2").expect("AES201 first");
    let aes101 = section.find("AES101: 1").expect("AES101 present");
    assert!(aes201 < aes101, "descending sort by count");
}

#[test]
fn score_line_omitted_when_no_score() {
    let out = format_report_default(&ScanReport::new(vec![], vec![]));
    assert!(!out.contains("Score:"));
}

#[test]
fn score_line_included_when_present() {
    let report = ScanReport::new(vec![], vec![]).with_score(shared::common::Score::new(88.0));
    let out = format_report_default(&report);
    assert!(out.contains("Score: 88.0/100"));
}

#[test]
fn diagnostics_section_rendered() {
    let report = ScanReport {
        results: vec![],
        diagnostics: vec![PipelineDiagnostic::new(
            "parser".to_string(),
            "File skipped".to_string(),
            DiagnosticSeverity::Warning,
        )],
        score: None,
    };
    let out = format_report_default(&report);
    assert!(out.contains("Diagnostics: 1"));
    assert!(out.contains("parser"));
    assert!(out.contains("File skipped"));
}

#[test]
fn diagnostics_section_omitted_when_empty() {
    let out = format_report_default(&ScanReport::new(vec![], vec![]));
    assert!(!out.contains("Diagnostics:\n"));
}
