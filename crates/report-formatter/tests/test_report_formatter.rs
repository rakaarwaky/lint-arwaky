use std::sync::Arc;

use report_formatter_lint_arwaky::{
    JsonFormatter, JunitFormatter, ReportFormatterDeps, ReportFormatterOrchestrator,
    SarifFormatter, TextFormatter, xml_escape,
};
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_scan_report_vo::{DiagnosticSeverity, PipelineDiagnostic};
use shared::cli_commands::{Format, ScanReport};
use shared::common::Severity;
use shared::common::taxonomy_common_vo::Score;
use shared::report_formatter::IReportFormatterAggregate;

fn sample_report() -> ScanReport {
    let violation = LintResult::new_arch(
        "src/main.rs",
        14,
        "AES201",
        Severity::CRITICAL,
        "surface -> capabilities import forbidden",
    );

    let diagnostic = PipelineDiagnostic::new(
        "filesystem".to_string(),
        "File skipped: parse failure".to_string(),
        DiagnosticSeverity::Warning,
    );

    ScanReport {
        results: vec![violation],
        diagnostics: vec![diagnostic],
        score: Some(Score::new(85.0)),
    }
}

#[test]
fn test_text_formatter_empty() {
    let formatter = TextFormatter::new();
    let report = ScanReport {
        results: vec![],
        diagnostics: vec![],
        score: None,
    };
    let content = formatter.format_text(&report);
    assert!(content.value.contains("AES Violations: 0"));
}

#[test]
fn test_text_formatter_full() {
    let formatter = TextFormatter::new();
    let report = sample_report();
    let content = formatter.format_text(&report);

    assert!(content.value.contains("AES Violations"));
    assert!(content.value.contains("AES201"));
    assert!(content.value.contains("Diagnostics"));
    assert!(content.value.contains("Compliance score: 85.0/100"));
}

#[test]
fn test_json_formatter_schema() {
    let formatter = JsonFormatter::new();
    let report = sample_report();
    let content = formatter.format_json(&report);

    let json: serde_json::Value =
        serde_json::from_str(&content.value).expect("Valid JSON expected");
    assert!(json.get("violations").is_some());
    assert!(json.get("diagnostics").is_some());
    assert_eq!(json["summary"]["total_violations"], 1);
    assert_eq!(json["summary"]["score"], 85.0);
}

#[test]
fn test_sarif_formatter_structure() {
    let formatter = SarifFormatter::new();
    let report = sample_report();
    let content = formatter.format_sarif_report(&report);

    let json: serde_json::Value =
        serde_json::from_str(&content.value).expect("Valid SARIF JSON expected");
    assert_eq!(json["version"], "2.1.0");
    let runs = json["runs"].as_array().unwrap();
    assert_eq!(runs.len(), 1);
    let rules = runs[0]["rules"].as_array().unwrap();
    assert!(!rules.is_empty());
    let results = runs[0]["results"].as_array().unwrap();
    assert_eq!(results.len(), 2); // 1 violation + 1 diagnostic
}

#[test]
fn test_junit_formatter_xml() {
    let formatter = JunitFormatter::new();
    let report = sample_report();
    let content = formatter.format_junit_report(&report);

    assert!(content.value.contains("<testsuites"));
    assert!(content.value.contains("classname=\"AES201\""));
    assert!(content.value.contains("<failure message=\"critical:"));
    assert!(
        content
            .value
            .contains("<skipped message=\"File skipped: parse failure\" />")
    );
}

#[test]
fn test_xml_escape_utility() {
    let input = "<tag & \"quote\" 'single' >";
    let escaped = xml_escape(input);
    assert_eq!(
        escaped,
        "&lt;tag &amp; &quot;quote&quot; &apos;single&apos; &gt;"
    );
}

#[test]
fn test_orchestrator_routing() {
    let orchestrator = ReportFormatterOrchestrator::new(ReportFormatterDeps {
        text: Arc::new(TextFormatter::new()),
        json: Arc::new(JsonFormatter::new()),
        sarif: Arc::new(SarifFormatter::new()),
        junit: Arc::new(JunitFormatter::new()),
    });

    let report = sample_report();

    let text_res = orchestrator.format(&report, Format::Text);
    assert!(text_res.value.contains("Lint Arwaky Report"));

    let json_res = orchestrator.format(&report, Format::Json);
    assert!(json_res.value.contains("\"violations\""));

    let sarif_res = orchestrator.format(&report, Format::Sarif);
    assert!(sarif_res.value.contains("sarif-schema-2.1.0.json"));

    let junit_res = orchestrator.format(&report, Format::Junit);
    assert!(junit_res.value.contains("<testsuites"));
}
