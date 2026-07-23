// PURPOSE: ExternalCommandsSurface — CLI surface for external lint commands
//
// Thin CLI surface that delegates all external lint logic to the agent layer.
// Handles path resolution, request construction, and output formatting.
use std::process::ExitCode;
use std::sync::Arc;

use shared::cli_commands::taxonomy_format_vo::Format;
use shared::report_formatter::contract_report_formatter_aggregate::IReportFormatterAggregate;
use shared::cli_commands::taxonomy_scan_report_vo::ScanReport;
use shared::common::taxonomy_path_vo::FilePath;
use shared::external_lint::contract_external_lint_aggregate::IExternalLintAggregate;

use crate::surface_common_command;

pub fn handle_scan_external(
    path: Option<FilePath>,
    external_lint: Arc<dyn IExternalLintAggregate>,
    report_formatter: Arc<dyn IReportFormatterAggregate>,
) -> ExitCode {
    let root = match &path {
        Some(p) => p.value().to_string(),
        None => ".".to_string(),
    };
    if !std::path::Path::new(&root).exists() {
        eprintln!("Error: path '{}' does not exist", root);
        return ExitCode::from(2);
    }
    let root_fp = match FilePath::new(root) {
        Ok(fp) => fp,
        Err(_) => return ExitCode::from(2),
    };
    let rt = match surface_common_command::create_current_thread_runtime() {
        Ok(r) => r,
        Err(_) => return ExitCode::from(2),
    };
    let results = rt.block_on(external_lint.scan_all(&root_fp));
    let report = ScanReport::new(results.values, vec![]);
    let output = report_formatter.format(&report, Format::Text);
    println!("{output}");
    if report.violation_count() > 0 {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    }
}
