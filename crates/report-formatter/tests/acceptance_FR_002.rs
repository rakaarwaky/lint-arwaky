// FR-002 — JSON Format Output
use report_formatter_lint_arwaky::capabilities_json_formatter::JsonFormatter;
use shared::cli_commands::DiagnosticSeverity;
use shared::cli_commands::{LintResult, PipelineDiagnostic, ScanReport};
use shared::common::{AdapterName, ErrorCode, FilePath, LineNumber, LintMessage, Severity};

fn parse(out: &str) -> serde_json::Value {
    serde_json::from_str(out).expect("output must be valid JSON")
}

#[test]
fn us1_normal_report_is_valid_pretty_json() {
    let report = ScanReport {
        results: vec![LintResult {
            file: FilePath::new("src/surface.rs").unwrap(),
            line: LineNumber::new(14),
            code: ErrorCode::raw("AES201"),
            message: LintMessage::new("import forbidden"),
            source: Some(AdapterName::raw("architecture")),
            severity: Severity::CRITICAL,
            ..Default::default()
        }],
        diagnostics: vec![],
        score: Some(shared::common::Score::new(85.0)),
    };
    let out = JsonFormatter::new().format_json(&report);
    let v = parse(out.value());
    assert_eq!(v["violations"][0]["file"], "src/surface.rs");
    assert_eq!(v["violations"][0]["line"], 14);
    assert_eq!(v["violations"][0]["code"], "AES201");
    assert_eq!(v["violations"][0]["severity"], "critical");
    assert_eq!(v["summary"]["total_violations"], 1);
    assert_eq!(v["summary"]["critical"], 1);
    assert_eq!(v["summary"]["score"], 85.0);
}

#[test]
fn us2_empty_results_have_empty_arrays_and_zero_summary() {
    let out = JsonFormatter::new().format_json(&ScanReport::new(vec![], vec![]));
    let v = parse(out.value());
    assert_eq!(v["violations"].as_array().unwrap().len(), 0);
    assert_eq!(v["external_results"].as_array().unwrap().len(), 0);
    assert_eq!(v["diagnostics"].as_array().unwrap().len(), 0);
    assert_eq!(v["summary"]["total_violations"], 0);
    assert_eq!(v["summary"]["critical"], 0);
}

#[test]
fn us3_external_results_populate_external_array() {
    let report = ScanReport {
        results: vec![LintResult {
            file: FilePath::new("src/lib.rs").unwrap(),
            line: LineNumber::new(42),
            code: ErrorCode::raw("clippy::needless_return"),
            message: LintMessage::new("needless return"),
            source: Some(AdapterName::raw("clippy")),
            severity: Severity::MEDIUM,
            ..Default::default()
        }],
        diagnostics: vec![],
        score: None,
    };
    let v = parse(JsonFormatter::new().format_json(&report).value());
    assert_eq!(v["external_results"].as_array().unwrap().len(), 1);
    assert_eq!(v["external_results"][0]["code"], "clippy::needless_return");
    assert_eq!(v["violations"].as_array().unwrap().len(), 0);
}

#[test]
fn us4_parse_warn_diagnostics_populate_diagnostics_array() {
    let report = ScanReport {
        results: vec![],
        diagnostics: vec![PipelineDiagnostic::new(
            "parser".to_string(),
            "File skipped: parse failure".to_string(),
            DiagnosticSeverity::Warning,
        )],
        score: None,
    };
    let v = parse(JsonFormatter::new().format_json(&report).value());
    assert_eq!(v["diagnostics"].as_array().unwrap().len(), 1);
    assert_eq!(v["diagnostics"][0]["source"], "parser");
    assert_eq!(v["diagnostics"][0]["severity"], "Warning");
    assert_eq!(
        v["diagnostics"][0]["message"],
        "File skipped: parse failure"
    );
}
