// Unit tests — TextFormatter (FR-001): human-readable output with severity badges.
use report_formatter_lint_arwaky::capabilities_text_formatter::TextFormatter;
use shared::cli_commands::DiagnosticSeverity;
use shared::cli_commands::{Format, LintResult, PipelineDiagnostic, ScanReport};
use shared::common::{AdapterName, ErrorCode, FilePath, LineNumber, LintMessage, Severity};
use shared::report_formatter::IReportFormatterProtocol;

fn aes_violation(code: &str, sev: Severity) -> LintResult {
    LintResult {
        file: FilePath::new("src/surface.rs").unwrap(),
        line: LineNumber::new(14),
        code: ErrorCode::raw(code),
        message: LintMessage::new(format!("violation {code}")),
        source: Some(AdapterName::raw("architecture")),
        severity: sev,
        ..Default::default()
    }
}

fn external_result(tool: &str, code: &str, sev: Severity) -> LintResult {
    LintResult {
        file: FilePath::new("src/lib.rs").unwrap(),
        line: LineNumber::new(42),
        code: ErrorCode::raw(code),
        message: LintMessage::new(format!("external {code}")),
        source: Some(AdapterName::raw(tool)),
        severity: sev,
        ..Default::default()
    }
}

#[test]
fn empty_report_produces_clean_output() {
    let out = TextFormatter::new().format_text(&ScanReport::new(vec![], vec![]));
    let text = out.value();
    assert!(text.contains("Lint Arwaky Report"));
    assert!(text.contains("AES Violations: 0"));
    assert!(text.contains("External Lint Results: 0"));
    assert!(text.contains("Total violations: 0"));
}

#[test]
fn aes_violations_show_badges_and_details() {
    let report = ScanReport {
        results: vec![
            aes_violation("AES201", Severity::CRITICAL),
            aes_violation("AES201", Severity::HIGH),
            aes_violation("AES301", Severity::MEDIUM),
        ],
        diagnostics: vec![],
        score: None,
    };
    let text = TextFormatter::new()
        .format_text(&report)
        .value()
        .to_string();
    assert!(text.contains("AES Violations: 3"));
    assert!(text.contains("[!!!]"));
    assert!(text.contains("[!! ]"));
    assert!(text.contains("[!  ]"));
    assert!(text.contains("AES201 src/surface.rs:14"));
    assert!(text.contains("AES301"));
    assert!(text.contains("Severity breakdown:"));
    assert!(text.contains("CRITICAL: 1"));
    assert!(text.contains("HIGH:     1"));
    assert!(text.contains("MEDIUM:   1"));
}

#[test]
fn violations_grouped_by_code_descending() {
    let report = ScanReport {
        results: vec![
            aes_violation("AES101", Severity::LOW),
            aes_violation("AES201", Severity::HIGH),
            aes_violation("AES201", Severity::MEDIUM),
            aes_violation("AES201", Severity::LOW),
        ],
        diagnostics: vec![],
        score: None,
    };
    let text = TextFormatter::new()
        .format_text(&report)
        .value()
        .to_string();
    let by_code = text
        .find("Violations by rule code:")
        .expect("section present");
    let section = &text[by_code..];
    let aes201 = section.find("AES201: 3").expect("AES201 grouped first");
    let aes101 = section.find("AES101: 1").expect("AES101 present");
    assert!(aes201 < aes101, "AES201 (3) must sort before AES101 (1)");
}

#[test]
fn external_results_grouped_by_tool() {
    let report = ScanReport {
        results: vec![
            external_result("clippy", "clippy::needless_return", Severity::MEDIUM),
            external_result("ruff", "ruff::E501", Severity::LOW),
            external_result("clippy", "clippy::collapsible_if", Severity::LOW),
        ],
        diagnostics: vec![],
        score: None,
    };
    let text = TextFormatter::new()
        .format_text(&report)
        .value()
        .to_string();
    assert!(text.contains("AES Violations: 0"));
    assert!(text.contains("External Lint Results: 3"));
    assert!(text.contains("[clippy]"));
    assert!(text.contains("[ruff]"));
    assert!(text.contains("ruff::E501"));
    assert!(text.contains("clippy::needless_return"));
}

#[test]
fn parse_warn_diagnostics_section_rendered() {
    let report = ScanReport {
        results: vec![],
        diagnostics: vec![
            PipelineDiagnostic::new(
                "parser".to_string(),
                "File skipped: parse failure".to_string(),
                DiagnosticSeverity::Warning,
            ),
            PipelineDiagnostic::new(
                "parser".to_string(),
                "bad file".to_string(),
                DiagnosticSeverity::Error,
            ),
        ],
        score: None,
    };
    let text = TextFormatter::new()
        .format_text(&report)
        .value()
        .to_string();
    assert!(text.contains("Diagnostics: 2"));
    assert!(text.contains("[WARNING]"));
    assert!(text.contains("[ERROR]"));
    assert!(text.contains("File skipped: parse failure"));
}

#[test]
fn score_line_included_when_present() {
    let report = ScanReport::new(vec![], vec![]).with_score(shared::common::Score::new(92.5));
    let text = TextFormatter::new()
        .format_text(&report)
        .value()
        .to_string();
    assert!(text.contains("Compliance score: 92.5/100"));
}

#[test]
fn mismatched_format_falls_back_to_default() {
    let report = ScanReport::new(vec![], vec![]);
    let out = TextFormatter::new().format(&report, Format::Json);
    assert!(out.value().contains("Violations: 0"));
}

#[test]
fn text_formatter_supports_text_format() {
    assert_eq!(TextFormatter::new().supported_format(), Format::Text);
}
