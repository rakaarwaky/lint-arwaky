// Smoke test — report-formatter boots and all formats respond well under 5s.
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

#[test]
fn report_formatter_boots_and_formats_all_formats_under_5s() {
    let start = std::time::Instant::now();
    let orch = ReportFormatterOrchestrator::new(ReportFormatterDeps {
        text: Arc::new(TextFormatter::new()),
        json: Arc::new(JsonFormatter::new()),
        sarif: Arc::new(SarifFormatter::new()),
        junit: Arc::new(JunitFormatter::new()),
    });
    let report = ScanReport::new(vec![], vec![]);
    for format in [Format::Text, Format::Json, Format::Sarif, Format::Junit] {
        let out = orch.format(&report, format);
        assert!(!out.value().is_empty());
    }
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "Smoke test exceeded 5s: {:?}",
        elapsed
    );
}
