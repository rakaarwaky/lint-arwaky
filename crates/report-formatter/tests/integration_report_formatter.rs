// PURPOSE: Integration tests — verify DI container wiring and cross-layer collaboration.
// Layer: Integration (uses real orchestrator with all formatters).

use report_formatter_lint_arwaky::agent_report_formatter_orchestrator::{
    ReportFormatterDeps, ReportFormatterOrchestrator,
};
use report_formatter_lint_arwaky::capabilities_json_formatter::JsonFormatter;
use report_formatter_lint_arwaky::capabilities_junit_formatter::JunitFormatter;
use report_formatter_lint_arwaky::capabilities_sarif_formatter::SarifFormatter;
use report_formatter_lint_arwaky::capabilities_text_formatter::TextFormatter;
use shared::cli_commands::taxonomy_format_vo::Format;
use shared::cli_commands::taxonomy_scan_report_vo::ScanReport;
use shared::report_formatter::contract_report_formatter_aggregate::IReportFormatterAggregate;
use shared::report_formatter::contract_report_formatter_protocol::IReportFormatterProtocol;
use std::sync::Arc;

fn build_full_orchestrator() -> ReportFormatterOrchestrator {
    let text = Arc::new(TextFormatter::new(
        code_analysis::root_code_analysis_container::CodeAnalysisContainer::default()
            .code_analysis_linter(),
    ));
    let json = Arc::new(JsonFormatter::new());
    let sarif = Arc::new(SarifFormatter::new());
    let junit = Arc::new(JunitFormatter::new());
    ReportFormatterOrchestrator::new(ReportFormatterDeps {
        text,
        json,
        sarif,
        junit,
    })
}

// ─── Container wiring tests ──

#[test]
fn orchestrator_creates_successfully() {
    let _orch = build_full_orchestrator();
}

#[test]
fn orchestrator_implements_aggregate_trait() {
    let orch = build_full_orchestrator();
    let _: &dyn IReportFormatterAggregate = &orch;
}

// ─── Cross-layer collaboration test ──

#[test]
fn full_format_pipeline_works() {
    let orch = build_full_orchestrator();
    let report = ScanReport::new(vec![], vec![]);

    // All formats should produce non-empty output
    let text_result = orch.format(&report, Format::Text);
    let json_result = orch.format(&report, Format::Json);
    let sarif_result = orch.format(&report, Format::Sarif);
    let junit_result = orch.format(&report, Format::Junit);

    assert!(!text_result.value.is_empty());
    assert!(!json_result.value.is_empty());
    assert!(!sarif_result.value.is_empty());
    assert!(!junit_result.value.is_empty());
}

// ─── Verify all formatters are wired correctly ──

#[test]
fn all_formatters_accessible() {
    let text = Arc::new(TextFormatter::new(
        code_analysis::root_code_analysis_container::CodeAnalysisContainer::default()
            .code_analysis_linter(),
    ));
    let json = Arc::new(JsonFormatter::new());
    let sarif = Arc::new(SarifFormatter::new());
    let junit = Arc::new(JunitFormatter::new());

    // Verify all implement IReportFormatterProtocol
    let _: &dyn IReportFormatterProtocol = &*text;
    let _: &dyn IReportFormatterProtocol = &*json;
    let _: &dyn IReportFormatterProtocol = &*sarif;
    let _: &dyn IReportFormatterProtocol = &*junit;
}
