// PURPOSE: Acceptance tests — verify FRD requirements for report-formatter.
// Layer: Acceptance (FRD requirement validation).

use report_formatter_lint_arwaky::agent_report_formatter_orchestrator::{
    ReportFormatterDeps, ReportFormatterOrchestrator,
};
use report_formatter_lint_arwaky::capabilities_json_formatter::JsonFormatter;
use report_formatter_lint_arwaky::capabilities_junit_formatter::JunitFormatter;
use report_formatter_lint_arwaky::capabilities_sarif_formatter::SarifFormatter;
use report_formatter_lint_arwaky::capabilities_text_formatter::TextFormatter;
use shared::cli_commands::{Format, ScanReport};
use shared::report_formatter::IReportFormatterAggregate;
use std::sync::Arc;

fn build_orchestrator() -> ReportFormatterOrchestrator {
    ReportFormatterOrchestrator::new(ReportFormatterDeps {
        text: Arc::new(TextFormatter::new()),
        json: Arc::new(JsonFormatter::new()),
        sarif: Arc::new(SarifFormatter::new()),
        junit: Arc::new(JunitFormatter::new()),
    })
}

// ─── Acceptance: Text formatter produces human-readable output ──

#[test]
fn acceptance_report_formatter_text_is_readable() {
    let orch = build_orchestrator();
    let report = ScanReport::new(vec![], vec![]);

    let result = orch.format(&report, Format::Text);
    assert!(!result.value.is_empty());
    assert!(result.value.is_ascii());
}

// ─── Acceptance: JSON formatter produces valid JSON ──

#[test]
fn acceptance_report_formatter_json_is_valid() {
    let orch = build_orchestrator();
    let report = ScanReport::new(vec![], vec![]);

    let result = orch.format(&report, Format::Json);
    assert!(result.value.contains("\"violations\""));
    assert!(result.value.contains("\"summary\""));
}

// ─── Acceptance: SARIF formatter produces SARIF 2.1.0 ──

#[test]
fn acceptance_report_formatter_sarif_schema() {
    let orch = build_orchestrator();
    let report = ScanReport::new(vec![], vec![]);

    let result = orch.format(&report, Format::Sarif);
    assert!(result.value.contains("\"$schema\""));
}

// ─── Acceptance: JUnit formatter produces valid XML ──

#[test]
fn acceptance_report_formatter_junit_xml() {
    let orch = build_orchestrator();
    let report = ScanReport::new(vec![], vec![]);

    let result = orch.format(&report, Format::Junit);
    assert!(result.value.contains("<testsuites"));
    assert!(result.value.contains("</testsuites>"));
}

// ─── Acceptance: Orchestrator delegates to correct formatter ──

#[test]
fn acceptance_report_formatter_routing_correct() {
    let orch = build_orchestrator();
    let report = ScanReport::new(vec![], vec![]);

    let text = orch.format(&report, Format::Text).value;
    let json = orch.format(&report, Format::Json).value;
    let sarif = orch.format(&report, Format::Sarif).value;
    let junit = orch.format(&report, Format::Junit).value;

    assert_ne!(text, json);
    assert_ne!(text, sarif);
    assert_ne!(text, junit);
    assert_ne!(json, sarif);
}

// ─── Acceptance: format_report_default utility function works ──

#[test]
fn acceptance_report_formatter_default_format() {
    let report = ScanReport::new(vec![], vec![]);
    let result = report_formatter_lint_arwaky::format_report_default(&report);
    assert!(!result.is_empty());
}
