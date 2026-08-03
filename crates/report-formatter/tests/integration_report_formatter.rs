// Integration tests — DI wiring of ReportFormatterDeps into the orchestrator (FR-005).
use report_formatter_lint_arwaky::agent_report_formatter_orchestrator::{
    ReportFormatterDeps, ReportFormatterOrchestrator,
};
use report_formatter_lint_arwaky::capabilities_json_formatter::JsonFormatter;
use report_formatter_lint_arwaky::capabilities_junit_formatter::JunitFormatter;
use report_formatter_lint_arwaky::capabilities_sarif_formatter::SarifFormatter;
use report_formatter_lint_arwaky::capabilities_text_formatter::TextFormatter;
use shared::cli_commands::{Format, LintResult, ScanReport};
use shared::common::{AdapterName, ErrorCode, FilePath, LineNumber, LintMessage, Severity};
use shared::report_formatter::IReportFormatterAggregate;
use std::sync::Arc;

fn make_orchestrator() -> ReportFormatterOrchestrator {
    ReportFormatterOrchestrator::new(ReportFormatterDeps {
        text: Arc::new(TextFormatter::new()),
        json: Arc::new(JsonFormatter::new()),
        sarif: Arc::new(SarifFormatter::new()),
        junit: Arc::new(JunitFormatter::new()),
    })
}

fn sample_report() -> ScanReport {
    ScanReport {
        results: vec![LintResult {
            file: FilePath::new("src/surface.rs").unwrap(),
            line: LineNumber::new(10),
            code: ErrorCode::raw("AES201"),
            message: LintMessage::new("import forbidden"),
            source: Some(AdapterName::raw("architecture")),
            severity: Severity::HIGH,
            ..Default::default()
        }],
        diagnostics: vec![],
        score: Some(shared::common::Score::new(70.0)),
    }
}

#[test]
fn orchestrator_routes_text_format() {
    let out = make_orchestrator().format(&sample_report(), Format::Text);
    assert!(out.value().contains("AES Violations: 1"));
    assert!(out.value().contains("Compliance score: 70.0/100"));
}

#[test]
fn orchestrator_routes_json_format() {
    let out = make_orchestrator().format(&sample_report(), Format::Json);
    let v: serde_json::Value = serde_json::from_str(out.value()).unwrap();
    assert_eq!(v["violations"][0]["code"], "AES201");
    assert_eq!(v["summary"]["total_violations"], 1);
    assert_eq!(v["summary"]["score"], 70.0);
}

#[test]
fn orchestrator_routes_sarif_format() {
    let out = make_orchestrator().format(&sample_report(), Format::Sarif);
    let v: serde_json::Value = serde_json::from_str(out.value()).unwrap();
    assert_eq!(v["version"], "2.1.0");
    assert_eq!(v["runs"][0]["results"][0]["level"], "error");
}

#[test]
fn orchestrator_routes_junit_format() {
    let out = make_orchestrator().format(&sample_report(), Format::Junit);
    assert!(out.value().contains("<testsuites"));
    assert!(out.value().contains("classname=\"AES201\""));
}

#[test]
fn all_four_formats_are_reachable_and_infallible() {
    let report = sample_report();
    let orch = make_orchestrator();
    for format in [Format::Text, Format::Json, Format::Sarif, Format::Junit] {
        let out = orch.format(&report, format);
        assert!(!out.value().is_empty(), "{format} output must not be empty");
    }
}
