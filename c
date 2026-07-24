use std::process::ExitCode;
use std::sync::Arc;

use shared::cli_commands::taxonomy_format_vo::Format;
use shared::common::taxonomy_path_vo::FilePath;
use shared::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;

use crate::surface_common_command;
use crate::surface_output_component::{output_violations, ViolationItem};
use shared::cli_commands::utility_path_resolver::is_member_path;

pub fn handle_scan_naming(
    path: Option<FilePath>,
    format: Format,
    naming_orchestrator: Arc<dyn INamingRunnerAggregate>,
    _report_formatter: Arc<dyn shared::report_formatter::contract_report_formatter_aggregate::IReportFormatterAggregate>,
) -> ExitCode {
    let root = match &path {
        Some(p) => p.value().to_string(),
        None => ".".to_string(),
    };
    if !std::path::Path::new(&root).exists() {
        eprintln!("Error: path '{}' does not exist", root);
        return ExitCode::from(2);
    }
    let root_fp = match FilePath::new(root.clone()) {
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
    let violations: Vec<ViolationItem> = results.iter().map(ViolationItem::from_lint_result).collect();
    output_violations(&violations, &root, format, is_member_path(&root));
    if violations.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}
