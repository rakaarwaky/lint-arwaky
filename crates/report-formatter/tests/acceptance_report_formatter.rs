// PURPOSE: Acceptance tests — verify FRD requirements for report-formatter.
// Layer: Acceptance (FRD requirement validation).

use report_formatter_lint_arwaky::agent_report_formatter_orchestrator::ReportFormatterOrchestrator;
use report_formatter_lint_arwaky::capabilities_json_formatter::JsonFormatter;
use report_formatter_lint_arwaky::capabilities_junit_formatter::JunitFormatter;
use report_formatter_lint_arwaky::capabilities_sarif_formatter::SarifFormatter;
use report_formatter_lint_arwaky::capabilities_text_formatter::TextFormatter;
use shared::cli_commands::contract_report_formatter_aggregate::IReportFormatterAggregate;
use shared::cli_commands::taxonomy_format_vo::Format;
use shared::cli_commands::taxonomy_scan_report_vo::ScanReport;
use std::sync::Arc;

fn build_orchestrator() -> ReportFormatterOrchestrator {
    let text = Arc::new(TextFormatter::new(Arc::new(
        code_analysis::root_code_analysis_container::CodeAnalysisContainer::default(),
    )));
    let json = Arc::new(JsonFormatter::new());
    let sarif = Arc::new(SarifFormatter::new());
    let junit = Arc::new(JunitFormatter::new());
    ReportFormatterOrchestrator::new(text, json, sarif, junit)
}

// ─── Acceptance: Text formatter produces human-readable output ──

#[test]
fn acceptance_report_formatter_text_is_readable() {
    // FRD requirement: Text format should produce human-readable output
    let orch = build_orchestrator();
    let report = ScanReport::new(vec![], vec![]);

    let result = orch.format(&report, Format::Text);
    assert!(!result.value.is_empty());
    // Should contain readable text (not binary or encoded)
    assert!(result.value.chars().all(|c| c.is_ascii()));
}

// ─── Acceptance: JSON formatter produces valid JSON ──

#[test]
fn acceptance_report_formatter_json_is_valid() {
    // FRD requirement: JSON format should produce valid JSON
    let orch = build_orchestrator();
    let report = ScanReport::new(vec![], vec![]);

    let result = orch.format(&report, Format::Json);
    assert!(result.value.contains("["));
    assert!(result.value.contains("]"));
}

// ─── Acceptance: SARIF formatter produces SARIF 2.1.0 ──

#[test]
fn acceptance_report_formatter_sarif_schema() {
    // FRD requirement: SARIF format should produce valid SARIF 2.1.0 JSON
    let orch = build_orchestrator();
    let report = ScanReport::new(vec![], vec![]);

    let result = orch.format(&report, Format::Sarif);
    assert!(result.value.contains("\"@schema\""));
}

// ─── Acceptance: JUnit formatter produces valid XML ──

#[test]
fn acceptance_report_formatter_junit_xml() {
    // FRD requirement: JUnit format should produce valid XML
    let orch = build_orchestrator();
    let report = ScanReport::new(vec![], vec![]);

    let result = orch.format(&report, Format::Junit);
    assert!(result.value.contains("<testsuites"));
    assert!(result.value.contains("</testsuites>"));
}

// ─── Acceptance: Orchestrator delegates to correct formatter ──

#[test]
fn acceptance_report_formatter_routing_correct() {
    // FRD requirement: Orchestrator must route to the correct formatter based on format enum
    let orch = build_orchestrator();
    let report = ScanReport::new(vec![], vec![]);

    // Each format should produce distinct output
    let text = orch.format(&report, Format::Text).value;
    let json = orch.format(&report, Format::Json).value;
    let sarif = orch.format(&report, Format::Sarif).value;
    let junit = orch.format(&report, Format::Junit).value;

    // Verify each format produces unique content
    assert_ne!(text, json);
    assert_ne!(text, sarif);
    assert_ne!(text, junit);
    assert_ne!(json, sarif);
}

// ─── Acceptance: format_report_default utility function works ──

#[test]
fn acceptance_report_formatter_default_format() {
    // FRD requirement: Default format should produce a text summary
    let report = ScanReport::new(vec![], vec![]);
    let result =
        report_formatter_lint_arwaky::utility_report_format::format_report_default(&report);
    assert!(!result.is_empty());
}
