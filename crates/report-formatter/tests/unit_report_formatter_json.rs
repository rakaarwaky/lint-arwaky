// Unit tests — JsonFormatter (FR-002): structured pretty-printed JSON output.
use report_formatter_lint_arwaky::capabilities_json_formatter::JsonFormatter;
use shared::cli_commands::DiagnosticSeverity;
use shared::cli_commands::{Format, LintResult, PipelineDiagnostic, ScanReport};
use shared::common::{AdapterName, ErrorCode, FilePath, LineNumber, LintMessage, Severity};
use shared::report_formatter::IReportFormatterProtocol;

fn report_with_mixed_results() -> ScanReport {
    let aes = LintResult {
        file: FilePath::new("src/surface.rs").unwrap(),
        line: LineNumber::new(14),
        code: ErrorCode::raw("AES201"),
        message: LintMessage::new("surface -> capabilities import forbidden"),
        source: Some(AdapterName::raw("architecture")),
        severity: Severity::CRITICAL,
        ..Default::default()
    };
    let external = LintResult {
        file: FilePath::new("src/lib.rs").unwrap(),
        line: LineNumber::new(42),
        code: ErrorCode::raw("clippy::needless_return"),
        message: LintMessage::new("needless return"),
        source: Some(AdapterName::raw("clippy")),
        severity: Severity::MEDIUM,
        ..Default::default()
    };
    let info = LintResult {
        file: FilePath::new("src/lib.rs").unwrap(),
        line: LineNumber::new(7),
        code: ErrorCode::raw("AES101"),
        message: LintMessage::new("convention ok"),
        source: Some(AdapterName::raw("architecture")),
        severity: Severity::INFO,
        ..Default::default()
    };
    ScanReport {
        results: vec![aes, external, info],
        diagnostics: vec![PipelineDiagnostic::new(
            "parser".to_string(),
            "File skipped: parse failure".to_string(),
            DiagnosticSeverity::Warning,
        )],
        score: None,
    }
}

#[test]
fn empty_report_is_valid_json_with_zero_summary() {
    let out = JsonFormatter::new().format_json(&ScanReport::new(vec![], vec![]));
    let v: serde_json::Value = serde_json::from_str(out.value()).expect("must be valid JSON");
    assert_eq!(v["violations"].as_array().unwrap().len(), 0);
    assert_eq!(v["external_results"].as_array().unwrap().len(), 0);
    assert_eq!(v["diagnostics"].as_array().unwrap().len(), 0);
    assert_eq!(v["summary"]["total_violations"], 0);
    assert_eq!(v["summary"]["critical"], 0);
    assert_eq!(v["summary"]["score"], serde_json::Value::Null);
}

#[test]
fn mixed_report_separates_aes_from_external() {
    let out = JsonFormatter::new().format_json(&report_with_mixed_results());
    let v: serde_json::Value = serde_json::from_str(out.value()).expect("must be valid JSON");
    let violations = v["violations"].as_array().unwrap();
    let external = v["external_results"].as_array().unwrap();
    assert_eq!(violations.len(), 2);
    assert_eq!(external.len(), 1);
    assert_eq!(violations[0]["code"], "AES201");
    assert_eq!(violations[0]["severity"], "critical");
    assert_eq!(violations[0]["file"], "src/surface.rs");
    assert_eq!(violations[0]["line"], 14);
    assert_eq!(external[0]["code"], "clippy::needless_return");
}

#[test]
fn summary_counts_severities() {
    let out = JsonFormatter::new().format_json(&report_with_mixed_results());
    let v: serde_json::Value = serde_json::from_str(out.value()).unwrap();
    let summary = &v["summary"];
    // total_violations excludes INFO-severity results (consistent with ScanReport::violation_count)
    assert_eq!(summary["total_violations"], 2);
    assert_eq!(summary["critical"], 1);
    assert_eq!(summary["medium"], 1);
    assert_eq!(summary["low"], 0);
    assert_eq!(summary["high"], 0);
}

#[test]
fn diagnostics_are_serialized() {
    let out = JsonFormatter::new().format_json(&report_with_mixed_results());
    let v: serde_json::Value = serde_json::from_str(out.value()).unwrap();
    let diags = v["diagnostics"].as_array().unwrap();
    assert_eq!(diags.len(), 1);
    assert_eq!(diags[0]["source"], "parser");
    assert_eq!(diags[0]["severity"], "Warning");
}

#[test]
fn score_is_embedded_when_present() {
    let report = ScanReport::new(vec![], vec![]).with_score(shared::common::Score::new(85.0));
    let out = JsonFormatter::new().format_json(&report);
    let v: serde_json::Value = serde_json::from_str(out.value()).unwrap();
    assert_eq!(v["summary"]["score"], 85.0);
}

#[test]
fn mismatched_format_falls_back_to_default_text() {
    let report = ScanReport::new(vec![], vec![]);
    let out = JsonFormatter::new().format(&report, Format::Text);
    assert!(out.value().contains("Lint Arwaky Report"));
    assert!(out.value().contains("Violations: 0"));
}

#[test]
fn json_formatter_supports_json_format() {
    assert_eq!(JsonFormatter::new().supported_format(), Format::Json);
}
