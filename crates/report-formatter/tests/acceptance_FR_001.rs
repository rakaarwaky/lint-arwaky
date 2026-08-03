// FR-001 — Text Format Output
use report_formatter_lint_arwaky::capabilities_text_formatter::TextFormatter;
use shared::cli_commands::DiagnosticSeverity;
use shared::cli_commands::{LintResult, PipelineDiagnostic, ScanReport};
use shared::common::{AdapterName, ErrorCode, FilePath, LineNumber, LintMessage, Severity};

fn result(code: &str, sev: Severity, tool: &str) -> LintResult {
    LintResult {
        file: FilePath::new("src/surface.rs").unwrap(),
        line: LineNumber::new(14),
        code: ErrorCode::raw(code),
        message: LintMessage::new(format!("message {code}")),
        source: Some(AdapterName::raw(tool)),
        severity: sev,
        ..Default::default()
    }
}

fn report_with_aes_violations() -> ScanReport {
    ScanReport {
        results: vec![
            result("AES201", Severity::CRITICAL, "architecture"),
            result("AES301", Severity::HIGH, "architecture"),
        ],
        diagnostics: vec![],
        score: Some(shared::common::Score::new(80.0)),
    }
}

#[test]
fn us1_report_with_aes_violations_has_severity_badges() {
    let text = TextFormatter::new()
        .format_text(&report_with_aes_violations())
        .value()
        .to_string();
    assert!(text.contains("AES Violations: 2"));
    assert!(text.contains("[!!!] AES201 src/surface.rs:14"));
    assert!(text.contains("[!! ] AES301 src/surface.rs:14"));
}

#[test]
fn us2_report_with_external_lint_results_has_tool_section() {
    let report = ScanReport {
        results: vec![
            result("ruff::E501", Severity::MEDIUM, "ruff"),
            result("clippy::needless_return", Severity::LOW, "clippy"),
        ],
        diagnostics: vec![],
        score: None,
    };
    let text = TextFormatter::new()
        .format_text(&report)
        .value()
        .to_string();
    assert!(text.contains("External Lint Results: 2"));
    assert!(text.contains("[ruff]"));
    assert!(text.contains("[clippy]"));
}

#[test]
fn us3_report_with_parse_warn_diagnostics_is_distinct() {
    let report = ScanReport {
        results: vec![],
        diagnostics: vec![PipelineDiagnostic::new(
            "parser".to_string(),
            "File skipped: parse failure".to_string(),
            DiagnosticSeverity::Warning,
        )],
        score: None,
    };
    let text = TextFormatter::new()
        .format_text(&report)
        .value()
        .to_string();
    assert!(text.contains("Diagnostics: 1"));
    assert!(text.contains("[WARNING]"));
    assert!(text.contains("AES Violations: 0"));
}

#[test]
fn us4_empty_report_is_clean() {
    let text = TextFormatter::new()
        .format_text(&ScanReport::new(vec![], vec![]))
        .value()
        .to_string();
    assert!(text.contains("Total violations: 0"));
    assert!(text.contains("AES Violations: 0"));
    assert!(text.contains("External Lint Results: 0"));
}

#[test]
fn us5_only_parse_warn_report_shows_zero_violations() {
    let report = ScanReport {
        results: vec![],
        diagnostics: vec![PipelineDiagnostic::new(
            "parser".to_string(),
            "File skipped".to_string(),
            DiagnosticSeverity::Info,
        )],
        score: None,
    };
    let text = TextFormatter::new()
        .format_text(&report)
        .value()
        .to_string();
    assert!(text.contains("Diagnostics: 1"));
    assert!(text.contains("Total violations: 0"));
}
