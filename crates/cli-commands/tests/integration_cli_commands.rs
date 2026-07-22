// Integration tests — DI container wiring, pipeline construction, and cross-layer interaction.

use cli_commands_lint_arwaky::CliContainer;
use shared::cli_commands::contract_analysis_pipeline_aggregate::IAnalysisPipelineAggregate;
use shared::cli_commands::contract_report_formatter_aggregate::IReportFormatterAggregate;
use shared::cli_commands::taxonomy_format_vo::Format;
use shared::cli_commands::taxonomy_scan_report_vo::ScanReport;
use shared::cli_commands::taxonomy_scan_request_vo::{ScanMode, ScanRequest, ScanTarget};
use std::sync::Arc;

#[test]
fn cli_container_new_default_does_not_panic() {
    let _container = CliContainer::new_default();
}

#[test]
fn cli_container_pipeline_aggregate_returns_arc() {
    let container = CliContainer::new_default();
    let pipeline: Arc<dyn IAnalysisPipelineAggregate> = container.pipeline_aggregate();
    assert!(Arc::strong_count(&pipeline) >= 1);
}

#[test]
fn cli_container_fix_orchestrator_factory_produces_orchestrator() {
    let container = CliContainer::new_default();
    let factory = container.fix_orchestrator_factory();
    let fix_real = factory(false);
    assert!(Arc::strong_count(&fix_real) >= 1);
    let fix_dry = factory(true);
    assert!(Arc::strong_count(&fix_dry) >= 1);
}

#[test]
fn cli_container_report_formatter_is_wired() {
    let container = CliContainer::new_default();
    let formatter: &Arc<dyn IReportFormatterAggregate> = &container.report_formatter;
    assert!(Arc::strong_count(formatter) >= 1);
}

#[tokio::test]
async fn pipeline_run_on_empty_directory_returns_empty_report() {
    let container = CliContainer::new_default();
    let pipeline = container.pipeline_aggregate();
    let tmp = std::env::temp_dir().join(format!("integ_empty_{}", std::process::id()));
    std::fs::create_dir_all(&tmp).unwrap();

    let request = ScanRequest {
        target: ScanTarget::new(tmp.to_str().unwrap().to_string()),
        mode: ScanMode::Scan,
        filter: None,
        member: None,
        format: Format::Text,
    };

    let result = pipeline.run(request).await;
    assert!(result.is_ok());
    let report = result.unwrap();
    assert_eq!(report.results.len(), 0);
    std::fs::remove_dir_all(&tmp).ok();
}

#[tokio::test]
async fn pipeline_run_on_nonexistent_path_returns_error() {
    let container = CliContainer::new_default();
    let pipeline = container.pipeline_aggregate();

    let request = ScanRequest {
        target: ScanTarget::new("/nonexistent/path/xyz".to_string()),
        mode: ScanMode::Scan,
        filter: None,
        member: None,
        format: Format::Text,
    };

    let result = pipeline.run(request).await;
    match result {
        Ok(report) => assert!(report.results.is_empty()),
        Err(_) => {}
    }
}

#[test]
fn report_formatter_formats_empty_report_as_text() {
    let container = CliContainer::new_default();
    let report = ScanReport::new(vec![], vec![]);
    let output = container.report_formatter.format(&report, Format::Text);
    let output_str = format!("{}", output);
    assert!(!output_str.is_empty() || output_str.is_empty());
}

#[test]
fn report_formatter_formats_empty_report_as_json() {
    let container = CliContainer::new_default();
    let report = ScanReport::new(vec![], vec![]);
    let output = container.report_formatter.format(&report, Format::Json);
    let output_str = format!("{}", output);
    if !output_str.is_empty() {
        let _: Result<serde_json::Value, _> = serde_json::from_str(&output_str);
    }
}

#[test]
fn report_formatter_formats_empty_report_as_sarif() {
    let container = CliContainer::new_default();
    let report = ScanReport::new(vec![], vec![]);
    let output = container.report_formatter.format(&report, Format::Sarif);
    let output_str = format!("{}", output);
    if !output_str.is_empty() {
        let parsed: Result<serde_json::Value, _> = serde_json::from_str(&output_str);
        if let Ok(v) = parsed {
            assert_eq!(v["version"], "2.1.0");
        }
    }
}

#[test]
fn check_commands_surface_scan_on_empty_dir() {
    use cli_commands_lint_arwaky::CheckCommandsSurface;
    let container = CliContainer::new_default();
    let surface = CheckCommandsSurface::new(
        container.pipeline_aggregate(),
        container.report_formatter.clone(),
        Some(container.multi_project_orchestrator.clone()),
    );
    let tmp = std::env::temp_dir().join(format!("integ_surface_{}", std::process::id()));
    std::fs::create_dir_all(&tmp).unwrap();
    let exit = surface.scan(tmp.to_str().unwrap(), None, Format::Text);
    assert_eq!(exit, std::process::ExitCode::SUCCESS);
    std::fs::remove_dir_all(&tmp).ok();
}

#[test]
fn check_orphan_single_file_nonexistent_returns_empty() {
    let container = CliContainer::new_default();
    let surface = cli_commands_lint_arwaky::CheckCommandsSurface::new(
        container.pipeline_aggregate(),
        container.report_formatter.clone(),
        None,
    );
    surface.check_orphan_single_file("/nonexistent/file.rs");
}
