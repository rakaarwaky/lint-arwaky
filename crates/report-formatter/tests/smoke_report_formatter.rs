// PURPOSE: Smoke test — verify the report-formatter crate boots and responds within 5 seconds.
// Layer: Smoke (must complete < 5s).

use report_formatter_lint_arwaky::agent_report_formatter_orchestrator::{ReportFormatterDeps, ReportFormatterOrchestrator};
use report_formatter_lint_arwaky::capabilities_json_formatter::JsonFormatter;
use report_formatter_lint_arwaky::capabilities_junit_formatter::JunitFormatter;
use report_formatter_lint_arwaky::capabilities_sarif_formatter::SarifFormatter;
use report_formatter_lint_arwaky::capabilities_text_formatter::TextFormatter;
use shared::cli_commands::contract_report_formatter_aggregate::IReportFormatterAggregate;
use std::sync::Arc;

#[test]
fn smoke_report_formatter_crate_boots_and_responds() {
    let code_analysis =
        code_analysis::root_code_analysis_container::CodeAnalysisContainer::default()
            .code_analysis_linter();

    // 2. Orchestrator instantiates
    let orch = ReportFormatterOrchestrator::new(ReportFormatterDeps {
        text: Arc::new(TextFormatter::new(code_analysis)),
        json: Arc::new(JsonFormatter::new()),
        sarif: Arc::new(SarifFormatter::new()),
        junit: Arc::new(JunitFormatter::new()),
    });

    // 3. Format method responds
    let report = shared::cli_commands::taxonomy_scan_report_vo::ScanReport::new(vec![], vec![]);
    let result = orch.format(
        &report,
        shared::cli_commands::taxonomy_format_vo::Format::Text,
    );

    assert!(!result.value.is_empty());
}
