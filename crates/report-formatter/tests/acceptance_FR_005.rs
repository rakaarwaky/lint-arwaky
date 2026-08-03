// FR-005 — Format Delegation (orchestrator), FR-006 — Default Fallback, FR-007 — XML Escape.
use report_formatter_lint_arwaky::agent_report_formatter_orchestrator::{
    ReportFormatterDeps, ReportFormatterOrchestrator,
};
use report_formatter_lint_arwaky::capabilities_json_formatter::JsonFormatter;
use report_formatter_lint_arwaky::capabilities_junit_formatter::JunitFormatter;
use report_formatter_lint_arwaky::capabilities_junit_formatter::xml_escape;
use report_formatter_lint_arwaky::capabilities_sarif_formatter::SarifFormatter;
use report_formatter_lint_arwaky::capabilities_text_formatter::TextFormatter;
use report_formatter_lint_arwaky::utility_report_format::format_report_default;
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

fn report() -> ScanReport {
    ScanReport {
        results: vec![LintResult {
            file: FilePath::new("src/surface.rs").unwrap(),
            line: LineNumber::new(14),
            code: ErrorCode::raw("AES201"),
            message: LintMessage::new("forbidden import"),
            source: Some(AdapterName::raw("architecture")),
            severity: Severity::HIGH,
            ..Default::default()
        }],
        diagnostics: vec![],
        score: Some(shared::common::Score::new(75.0)),
    }
}

#[test]
fn us1_orchestrator_routes_text() {
    let out = make_orchestrator().format(&report(), Format::Text);
    assert!(out.value().contains("AES Violations: 1"));
}

#[test]
fn us2_orchestrator_routes_json() {
    let out = make_orchestrator().format(&report(), Format::Json);
    let v: serde_json::Value = serde_json::from_str(out.value()).unwrap();
    assert_eq!(v["summary"]["total_violations"], 1);
}

#[test]
fn us3_orchestrator_routes_sarif() {
    let out = make_orchestrator().format(&report(), Format::Sarif);
    let v: serde_json::Value = serde_json::from_str(out.value()).unwrap();
    assert_eq!(v["version"], "2.1.0");
}

#[test]
fn us4_orchestrator_routes_junit() {
    let out = make_orchestrator().format(&report(), Format::Junit);
    assert!(out.value().contains("<testsuites"));
}

#[test]
fn us5_default_fallback_counts_by_code_descending() {
    let r = ScanReport {
        results: vec![
            LintResult {
                code: ErrorCode::raw("AES101"),
                ..Default::default()
            },
            LintResult {
                code: ErrorCode::raw("AES201"),
                ..Default::default()
            },
            LintResult {
                code: ErrorCode::raw("AES201"),
                ..Default::default()
            },
        ],
        diagnostics: vec![],
        score: None,
    };
    let out = format_report_default(&r);
    let section = &out[out.find("Violations by code:").expect("section present")..];
    let aes201 = section.find("AES201: 2").expect("AES201 first");
    let aes101 = section.find("AES101: 1").expect("AES101 present");
    assert!(aes201 < aes101, "sorted by count descending");
}

#[test]
fn us6_default_fallback_includes_score_line() {
    let r = ScanReport::new(vec![], vec![]).with_score(shared::common::Score::new(88.0));
    let out = format_report_default(&r);
    assert!(out.contains("Score: 88.0/100"));
}

#[test]
fn us7_default_fallback_empty_shows_violations_zero() {
    let out = format_report_default(&ScanReport::new(vec![], vec![]));
    assert!(out.contains("Violations: 0"));
}

#[test]
fn us8_xml_escape_all_five_characters() {
    assert_eq!(xml_escape("&<>\"'"), "&amp;&lt;&gt;&quot;&apos;");
}

#[test]
fn us9_xml_escape_normal_text_unchanged() {
    assert_eq!(xml_escape("normal text 123"), "normal text 123");
}
