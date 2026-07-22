// PURPOSE: Benchmark — measure report-formatter throughput.
// Layer: Benchmark (performance validation).

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
use std::time::Instant;

fn build_orchestrator() -> ReportFormatterOrchestrator {
    let text = Arc::new(TextFormatter::new(Arc::new(
        shared::code_analysis::root_code_analysis_container::CodeAnalysisContainer::default(),
    )));
    let json = Arc::new(JsonFormatter::new());
    let sarif = Arc::new(SarifFormatter::new());
    let junit = Arc::new(JunitFormatter::new());
    ReportFormatterOrchestrator::new(text, json, sarif, junit)
}

// ─── Benchmark: Formatter instantiation throughput ──

#[test]
fn bench_formatter_instantiation() {
    let start = Instant::now();
    for _ in 0..1000 {
        let _text = TextFormatter::new(Arc::new(
            shared::code_analysis::root_code_analysis_container::CodeAnalysisContainer::default(),
        ));
        let _json = JsonFormatter::new();
        let _sarif = SarifFormatter::new();
        let _junit = JunitFormatter::new();
    }
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_millis() < 1000,
        "1000 formatter instantiations took {}ms",
        elapsed.as_millis()
    );
}

// ─── Benchmark: Text formatting throughput ──

#[test]
fn bench_text_formatting() {
    let text = TextFormatter::new(Arc::new(
        shared::code_analysis::root_code_analysis_container::CodeAnalysisContainer::default(),
    ));
    let report = ScanReport::new(vec![], vec![]);

    let start = Instant::now();
    for _ in 0..1000 {
        let _ = text.format(&report, Format::Text);
    }
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_millis() < 5000,
        "Text formatting took {}ms",
        elapsed.as_millis()
    );
}

// ─── Benchmark: JSON serialization throughput ──

#[test]
fn bench_json_serialization() {
    let json = JsonFormatter::new();
    let report = ScanReport::new(vec![], vec![]);

    let start = Instant::now();
    for _ in 0..1000 {
        let _ = json.format(&report, Format::Json);
    }
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_millis() < 5000,
        "JSON serialization took {}ms",
        elapsed.as_millis()
    );
}

// ─── Benchmark: SARIF generation throughput ──

#[test]
fn bench_sarif_generation() {
    let sarif = SarifFormatter::new();
    let report = ScanReport::new(vec![], vec![]);

    let start = Instant::now();
    for _ in 0..1000 {
        let _ = sarif.format(&report, Format::Sarif);
    }
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_millis() < 5000,
        "SARIF generation took {}ms",
        elapsed.as_millis()
    );
}

// ─── Benchmark: JUnit XML generation throughput ──

#[test]
fn bench_junit_generation() {
    let junit = JunitFormatter::new();
    let report = ScanReport::new(vec![], vec![]);

    let start = Instant::now();
    for _ in 0..1000 {
        let _ = junit.format(&report, Format::Junit);
    }
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_millis() < 5000,
        "JUnit generation took {}ms",
        elapsed.as_millis()
    );
}

// ─── Benchmark: Full orchestrator pipeline throughput ──

#[test]
fn bench_full_orchestrator_pipeline() {
    let orch = build_orchestrator();
    let report = ScanReport::new(vec![], vec![]);

    let start = Instant::now();
    for _ in 0..100 {
        let _text = orch.format(&report, Format::Text);
        let _json = orch.format(&report, Format::Json);
        let _sarif = orch.format(&report, Format::Sarif);
        let _junit = orch.format(&report, Format::Junit);
    }
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_millis() < 10000,
        "Full pipeline took {}ms",
        elapsed.as_millis()
    );
}
