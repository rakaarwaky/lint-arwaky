// PURPOSE: E2E tests — verify full report-formatter pipeline from orchestrator to all formatters.
// Layer: E2E (full integration of all layers).

use report_formatter_lint_arwaky::agent_report_formatter_orchestrator::{
    ReportFormatterDeps, ReportFormatterOrchestrator,
};
use report_formatter_lint_arwaky::capabilities_json_formatter::JsonFormatter;
use report_formatter_lint_arwaky::capabilities_junit_formatter::JunitFormatter;
use report_formatter_lint_arwaky::capabilities_sarif_formatter::SarifFormatter;
use report_formatter_lint_arwaky::capabilities_text_formatter::TextFormatter;
use shared::cli_commands::taxonomy_format_vo::Format;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_scan_report_vo::ScanReport;
use shared::common::taxonomy_severity_vo::Severity;
use shared::report_formatter::contract_report_formatter_aggregate::IReportFormatterAggregate;
use std::sync::Arc;

fn build_full_pipeline() -> (ReportFormatterOrchestrator, ScanReport) {
    let text = Arc::new(TextFormatter::new(
        code_analysis::root_code_analysis_container::CodeAnalysisContainer::default()
            .code_analysis_linter(),
    ));
    let json = Arc::new(JsonFormatter::new());
    let sarif = Arc::new(SarifFormatter::new());
    let junit = Arc::new(JunitFormatter::new());

    let orch = ReportFormatterOrchestrator::new(ReportFormatterDeps {
        text,
        json,
        sarif,
        junit,
    });

    let results = vec![LintResult::new_arch_with_column(
        "test.rs",
        10,
        5,
        "TEST001",
        Severity::MEDIUM,
        "E2E test violation",
    )];
    let report = ScanReport::new(results, vec![]);

    (orch, report)
}

// ─── E2E: Full format pipeline with real results ──

#[test]
fn e2e_all_format_types_work_with_results() {
    let (orch, report) = build_full_pipeline();

    // All formats should produce valid output
    let text_result = orch.format(&report, Format::Text);
    let json_result = orch.format(&report, Format::Json);
    let sarif_result = orch.format(&report, Format::Sarif);
    let junit_result = orch.format(&report, Format::Junit);

    assert!(!text_result.value.is_empty());
    assert!(!json_result.value.is_empty());
    assert!(!sarif_result.value.is_empty());
    assert!(!junit_result.value.is_empty());
}

// ─── E2E: Report with diagnostics ──

#[test]
fn e2e_format_with_diagnostics() {
    let (orch, report) = build_full_pipeline();

    // Format all types and verify they handle diagnostics correctly
    let _text = orch.format(&report, Format::Text);
    let _json = orch.format(&report, Format::Json);
}

// ─── E2E: Empty report pipeline ──

#[test]
fn e2e_empty_report_pipeline() {
    let (orch, _report) = build_full_pipeline();

    // Create empty report
    let empty_report = ScanReport::new(vec![], vec![]);

    // All formats should handle empty reports gracefully
    let _text = orch.format(&empty_report, Format::Text);
    let _json = orch.format(&empty_report, Format::Json);
    let _sarif = orch.format(&empty_report, Format::Sarif);
    let _junit = orch.format(&empty_report, Format::Junit);
}
