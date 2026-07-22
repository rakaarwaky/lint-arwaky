// PURPOSE: Contract tests — verify ReportFormatterOrchestrator satisfies IReportFormatterAggregate.
// Layer: Contract (contract compliance validation).

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
    let text: Arc<
        dyn shared::cli_commands::contract_report_formatter_protocol::IReportFormatterProtocol,
    > = Arc::new(TextFormatter::new(
        code_analysis::root_code_analysis_container::CodeAnalysisContainer::default()
            .code_analysis_linter(),
    ));
    let json: Arc<
        dyn shared::cli_commands::contract_report_formatter_protocol::IReportFormatterProtocol,
    > = Arc::new(JsonFormatter::new());
    let sarif: Arc<
        dyn shared::cli_commands::contract_report_formatter_protocol::IReportFormatterProtocol,
    > = Arc::new(SarifFormatter::new());
    let junit: Arc<
        dyn shared::cli_commands::contract_report_formatter_protocol::IReportFormatterProtocol,
    > = Arc::new(JunitFormatter::new());
    ReportFormatterOrchestrator::new(text, json, sarif, junit)
}

#[test]
fn contract_report_formatter_orchestrator_implements_aggregate() {
    let orch = build_orchestrator();
    let report = ScanReport::new(vec![], vec![]);

    let result = orch.format(&report, Format::Text);
    assert!(!result.value.is_empty());
}
