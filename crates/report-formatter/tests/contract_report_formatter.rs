// PURPOSE: Contract tests — verify all trait implementations for report-formatter types.
// Layer: Contract (trait verification).

use report_formatter_lint_arwaky::agent_report_formatter_orchestrator::ReportFormatterOrchestrator;
use report_formatter_lint_arwaky::capabilities_json_formatter::JsonFormatter;
use report_formatter_lint_arwaky::capabilities_junit_formatter::JunitFormatter;
use report_formatter_lint_arwaky::capabilities_sarif_formatter::SarifFormatter;
use report_formatter_lint_arwaky::capabilities_text_formatter::TextFormatter;
use shared::cli_commands::contract_report_formatter_aggregate::IReportFormatterAggregate;
use shared::cli_commands::contract_report_formatter_protocol::IReportFormatterProtocol;
use std::sync::Arc;

// ─── Verify JsonFormatter implements IReportFormatterProtocol ──

#[test]
fn json_formatter_implements_protocol() {
    let formatter = JsonFormatter::new();
    let _: &dyn IReportFormatterProtocol = &formatter;
}

// ─── Verify JunitFormatter implements IReportFormatterProtocol ──

#[test]
fn junit_formatter_implements_protocol() {
    let formatter = JunitFormatter::new();
    let _: &dyn IReportFormatterProtocol = &formatter;
}

// ─── Verify SarifFormatter implements IReportFormatterProtocol ──

#[test]
fn sarif_formatter_implements_protocol() {
    let formatter = SarifFormatter::new();
    let _: &dyn IReportFormatterProtocol = &formatter;
}

// ─── Verify TextFormatter implements IReportFormatterProtocol ──

#[test]
fn text_formatter_implements_protocol() {
    let code_analysis = Arc::new(
        shared::code_analysis::root_code_analysis_container::CodeAnalysisContainer::default(),
    );
    let formatter = TextFormatter::new(code_analysis);
    let _: &dyn IReportFormatterProtocol = &formatter;
}

// ─── Verify ReportFormatterOrchestrator implements IReportFormatterAggregate ──

#[test]
fn orchestrator_implements_aggregate() {
    let text = Arc::new(TextFormatter::new(Arc::new(
        shared::code_analysis::root_code_analysis_container::CodeAnalysisContainer::default(),
    )));
    let json = JsonFormatter::new();
    let sarif = SarifFormatter::new();
    let junit = JunitFormatter::new();

    let orchestrator = ReportFormatterOrchestrator::new(text, json, sarif, junit);
    let _: &dyn IReportFormatterAggregate = &orchestrator;
}

// ─── Verify supported_format returns correct enum variant ──

#[test]
fn json_formatter_supported_format_returns_json() {
    let formatter = JsonFormatter::new();
    assert_eq!(
        formatter.supported_format(),
        shared::cli_commands::taxonomy_format_vo::Format::Json
    );
}

#[test]
fn junit_formatter_supported_format_returns_junit() {
    let formatter = JunitFormatter::new();
    assert_eq!(
        formatter.supported_format(),
        shared::cli_commands::taxonomy_format_vo::Format::Junit
    );
}

#[test]
fn sarif_formatter_supported_format_returns_sarif() {
    let formatter = SarifFormatter::new();
    assert_eq!(
        formatter.supported_format(),
        shared::cli_commands::taxonomy_format_vo::Format::Sarif
    );
}

#[test]
fn text_formatter_supported_format_returns_text() {
    let code_analysis = Arc::new(
        shared::code_analysis::root_code_analysis_container::CodeAnalysisContainer::default(),
    );
    let formatter = TextFormatter::new(code_analysis);
    assert_eq!(
        formatter.supported_format(),
        shared::cli_commands::taxonomy_format_vo::Format::Text
    );
}
