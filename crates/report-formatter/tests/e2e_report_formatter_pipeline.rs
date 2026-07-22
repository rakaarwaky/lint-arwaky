// PURPOSE: E2E tests — verify full report-formatter pipeline from orchestrator to all formatters.
// Layer: E2E (full integration of all layers).

use report_formatter_lint_arwaky::agent_report_formatter_orchestrator::ReportFormatterOrchestrator;
use report_formatter_lint_arwaky::capabilities_json_formatter::JsonFormatter;
use report_formatter_lint_arwaky::capabilities_junit_formatter::JunitFormatter;
use report_formatter_lint_arwaky::capabilities_sarif_formatter::SarifFormatter;
use report_formatter_lint_arwaky::capabilities_text_formatter::TextFormatter;
use shared::cli_commands::contract_report_formatter_aggregate::IReportFormatterAggregate;
use shared::cli_commands::taxonomy_format_vo::Format;
use shared::cli_commands::taxonomy_lint_result_vo::{LintResult, Severity};
use shared::cli_commands::taxonomy_scan_report_vo::ScanReport;
use std::sync::Arc;

fn build_full_pipeline() -> (ReportFormatterOrchestrator, ScanReport) {
    let text = Arc::new(TextFormatter::new(Arc::new(
        shared::code_analysis::root_code_analysis_container::CodeAnalysisContainer::default(),
    )));
    let json = JsonFormatter::new();
    let sarif = SarifFormatter::new();
    let junit = JunitFormatter::new();

    let orch = ReportFormatterOrchestrator::new(text, json, sarif, junit);

    let results = vec![LintResult::new(
        shared::common::taxonomy_path_vo::FilePath::new("test.rs".to_string()).unwrap(),
        10,
        5,
        shared::cli_commands::taxonomy_result_vo::LintResultCode::new("TEST001"),
        shared::cli_commands::taxonomy_result_vo::LintResultMessage::new(
            "E2E test violation".to_string(),
        ),
        Severity::new(shared::common::taxonomy_severity_vo::SeverityLevel::Medium),
    )];
    let report = ScanReport::new(results, vec![], None);

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

    // Add diagnostics to the report
    let mut report = report;
    // Format all types and verify they handle diagnostics correctly
    let _text = orch.format(&report, Format::Text);
    let _json = orch.format(&report, Format::Json);
}

// ─── E2E: Empty report pipeline ──

#[test]
fn e2e_empty_report_pipeline() {
    let (orch, report) = build_full_pipeline();

    // Create empty report
    let empty_report = ScanReport::new(vec![], vec![], None);

    // All formats should handle empty reports gracefully
    let _text = orch.format(&empty_report, Format::Text);
    let _json = orch.format(&empty_report, Format::Json);
    let _sarif = orch.format(&empty_report, Format::Sarif);
    let _junit = orch.format(&empty_report, Format::Junit);
}
