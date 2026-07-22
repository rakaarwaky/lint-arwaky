// PURPOSE: Unit tests for ReportFormatterOrchestrator — agent layer delegation.
// Layer: Agent (ReportFormatterOrchestrator)

use report_formatter_lint_arwaky::agent_report_formatter_orchestrator::ReportFormatterOrchestrator;
use report_formatter_lint_arwaky::capabilities_json_formatter::JsonFormatter;
use report_formatter_lint_arwaky::capabilities_junit_formatter::JunitFormatter;
use report_formatter_lint_arwaky::capabilities_sarif_formatter::SarifFormatter;
use report_formatter_lint_arwaky::capabilities_text_formatter::TextFormatter;
use shared::cli_commands::contract_report_formatter_aggregate::IReportFormatterAggregate;
use shared::cli_commands::contract_report_formatter_protocol::IReportFormatterProtocol;
use shared::cli_commands::taxonomy_format_vo::Format;
use shared::cli_commands::taxonomy_scan_report_vo::ScanReport;
use std::sync::Arc;

fn build_orchestrator() -> ReportFormatterOrchestrator {
    let text = Arc::new(TextFormatter::new(Arc::new(
        shared::code_analysis::root_code_analysis_container::CodeAnalysisContainer::default(),
    )));
    let json = JsonFormatter::new();
    let sarif = SarifFormatter::new();
    let junit = JunitFormatter::new();
    ReportFormatterOrchestrator::new(text, json, sarif, junit)
}

// ─── Verify orchestrator implements IReportFormatterAggregate ──

#[test]
fn orchestrator_implements_aggregate() {
    let orch = build_orchestrator();
    let _: &dyn IReportFormatterAggregate = &orch;
}

// ─── format: Routes to correct formatter (Text) ──

#[test]
fn orchestrator_routes_to_text_formatter() {
    let orch = build_orchestrator();
    let report = ScanReport::new(vec![], vec![]);

    let result = orch.format(&report, Format::Text);
    assert!(!result.value.is_empty());
}

// ─── format: Routes to correct formatter (JSON) ──

#[test]
fn orchestrator_routes_to_json_formatter() {
    let orch = build_orchestrator();
    let report = ScanReport::new(vec![], vec![]);

    let result = orch.format(&report, Format::Json);
    assert!(result.value.contains("["));
}

// ─── format: Routes to correct formatter (SARIF) ──

#[test]
fn orchestrator_routes_to_sarif_formatter() {
    let orch = build_orchestrator();
    let report = ScanReport::new(vec![], vec![]);

    let result = orch.format(&report, Format::Sarif);
    assert!(result.value.contains("\"@schema\""));
}

// ─── format: Routes to correct formatter (JUnit) ──

#[test]
fn orchestrator_routes_to_junit_formatter() {
    let orch = build_orchestrator();
    let report = ScanReport::new(vec![], vec![]);

    let result = orch.format(&report, Format::Junit);
    assert!(result.value.contains("<testsuites"));
}

// ─── Default trait ──

#[test]
fn orchestrator_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<ReportFormatterOrchestrator>();
}
