use std::process::ExitCode;
use std::sync::Arc;

use shared::cli_commands::taxonomy_format_vo::Format;
use shared::report_formatter::contract_report_formatter_aggregate::IReportFormatterAggregate;
use shared::cli_commands::taxonomy_scan_report_vo::ScanReport;
use shared::common::taxonomy_path_vo::FilePath;
use shared::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;

use crate::surface_common_command;

pub fn handle_scan_naming(
    path: Option<FilePath>,
    naming_orchestrator: Arc<dyn INamingRunnerAggregate>,
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
    let results = match rt.block_on(naming_orchestrator.run_audit(&root_fp)) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("[error] naming rules failed: {e}");
            return ExitCode::from(2);
        }
    };
    let report = ScanReport::new(results.clone(), vec![]);
    let output = report_formatter.format(&report, Format::Text);
    println!("{output}");
    if results.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}
