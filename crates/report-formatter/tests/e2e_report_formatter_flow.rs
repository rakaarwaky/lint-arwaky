// E2E tests — full lifecycle: orchestrator formats LintResult in each format,
// outputs persisted to a temp dir and read back for validation.
use report_formatter_lint_arwaky::agent_report_formatter_orchestrator::{
    ReportFormatterDeps, ReportFormatterOrchestrator,
};
use report_formatter_lint_arwaky::capabilities_json_formatter::JsonFormatter;
use report_formatter_lint_arwaky::capabilities_junit_formatter::JunitFormatter;
use report_formatter_lint_arwaky::capabilities_sarif_formatter::SarifFormatter;
use report_formatter_lint_arwaky::capabilities_text_formatter::TextFormatter;
use shared::cli_commands::DiagnosticSeverity;
use shared::cli_commands::{Format, LintResult, PipelineDiagnostic, ScanReport};
use shared::common::{AdapterName, ErrorCode, FilePath, LineNumber, LintMessage, Severity};
use shared::report_formatter::IReportFormatterAggregate;
use std::fs;
use std::sync::Arc;
use tempfile::TempDir;

fn make_orchestrator() -> ReportFormatterOrchestrator {
    ReportFormatterOrchestrator::new(ReportFormatterDeps {
        text: Arc::new(TextFormatter::new()),
        json: Arc::new(JsonFormatter::new()),
        sarif: Arc::new(SarifFormatter::new()),
        junit: Arc::new(JunitFormatter::new()),
    })
}

fn full_report() -> ScanReport {
    ScanReport {
        results: vec![
            LintResult {
                file: FilePath::new("src/surface.rs").unwrap(),
                line: LineNumber::new(14),
                code: ErrorCode::raw("AES201"),
                message: LintMessage::new("surface -> capabilities import forbidden"),
                source: Some(AdapterName::raw("architecture")),
                severity: Severity::CRITICAL,
                ..Default::default()
            },
            LintResult {
                file: FilePath::new("src/lib.rs").unwrap(),
                line: LineNumber::new(42),
                code: ErrorCode::raw("ruff::E501"),
                message: LintMessage::new("line too long"),
                source: Some(AdapterName::raw("ruff")),
                severity: Severity::MEDIUM,
                ..Default::default()
            },
        ],
        diagnostics: vec![PipelineDiagnostic::new(
            "parser".to_string(),
            "File skipped: parse failure".to_string(),
            DiagnosticSeverity::Warning,
        )],
        score: Some(shared::common::Score::new(61.5)),
    }
}

fn validate_structure(format: Format, content: &str) {
    match format {
        Format::Json => {
            let v: serde_json::Value = serde_json::from_str(content).expect("JSON must parse");
            assert!(v["violations"].is_array());
            assert!(v["external_results"].is_array());
            assert!(v["diagnostics"].is_array());
        }
        Format::Sarif => {
            let v: serde_json::Value = serde_json::from_str(content).expect("SARIF must parse");
            assert_eq!(v["version"], "2.1.0");
            assert!(v["runs"][0]["results"].is_array());
        }
        Format::Junit => {
            assert!(content.contains("<testsuites"));
            assert!(content.contains("<testsuite"));
            assert!(content.contains("</testsuites>"));
        }
        Format::Text => {
            assert!(content.contains("Lint Arwaky Report"));
            assert!(content.contains("Total violations:"));
        }
    }
}

fn validate_full_report(format: Format, content: &str) {
    validate_structure(format, content);
    match format {
        Format::Json => {
            let v: serde_json::Value = serde_json::from_str(content).expect("JSON must parse");
            assert_eq!(v["summary"]["total_violations"], 2);
            assert_eq!(v["summary"]["score"], 61.5);
            assert_eq!(v["diagnostics"][0]["source"], "parser");
        }
        Format::Sarif => {
            let v: serde_json::Value = serde_json::from_str(content).expect("SARIF must parse");
            assert_eq!(v["runs"][0]["results"].as_array().unwrap().len(), 3);
            assert_eq!(v["runs"][0]["results"][2]["rule_id"], "PARSE_WARN");
        }
        Format::Junit => {
            assert!(content.contains("tests=\"3\""));
            assert!(content.contains("classname=\"PARSE_WARN\""));
            assert!(content.contains("<skipped"));
        }
        Format::Text => {
            assert!(content.contains("AES Violations: 1"));
            assert!(content.contains("External Lint Results: 1"));
            assert!(content.contains("Diagnostics: 1"));
            assert!(content.contains("Compliance score: 61.5/100"));
        }
    }
}

fn validate_empty_report(format: Format, content: &str) {
    validate_structure(format, content);
    match format {
        Format::Json => {
            let v: serde_json::Value = serde_json::from_str(content).expect("JSON must parse");
            assert_eq!(v["summary"]["total_violations"], 0);
            assert_eq!(v["violations"].as_array().unwrap().len(), 0);
        }
        Format::Sarif => {
            let v: serde_json::Value = serde_json::from_str(content).expect("SARIF must parse");
            assert!(v["runs"][0]["results"].as_array().unwrap().is_empty());
        }
        Format::Junit => {
            assert!(content.contains("tests=\"0\" failures=\"0\""));
        }
        Format::Text => {
            assert!(content.contains("Total violations: 0"));
        }
    }
}

#[test]
fn full_report_round_trips_through_all_formats_to_disk() {
    let tmp = TempDir::new().unwrap();
    let report = full_report();
    let orch = make_orchestrator();
    for format in [Format::Text, Format::Json, Format::Sarif, Format::Junit] {
        let out = orch.format(&report, format);
        let path = tmp.path().join(format!("report.{format}"));
        fs::write(&path, out.value()).expect("persist report output");
        let content = fs::read_to_string(&path).expect("read back report output");
        validate_full_report(format, &content);
    }
}

#[test]
fn empty_report_round_trips_through_all_formats_to_disk() {
    let tmp = TempDir::new().unwrap();
    let report = ScanReport::new(vec![], vec![]);
    let orch = make_orchestrator();
    for format in [Format::Text, Format::Json, Format::Sarif, Format::Junit] {
        let out = orch.format(&report, format);
        let path = tmp.path().join(format!("empty.{format}"));
        fs::write(&path, out.value()).unwrap();
        let content = fs::read_to_string(&path).unwrap();
        validate_empty_report(format, &content);
    }
}
