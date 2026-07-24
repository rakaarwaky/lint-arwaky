use std::process::ExitCode;
use std::sync::Arc;

use shared::cli_commands::taxonomy_format_vo::Format;
use shared::cli_commands::utility_path_resolver::is_member_path;
use shared::common::taxonomy_path_vo::FilePath;
use shared::role_rules::contract_role_runner_aggregate::IRoleRunnerAggregate;

use crate::surface_common_command;
use crate::surface_output_component::{output_violations, ViolationItem};

pub fn handle_scan_role(
    path: Option<FilePath>,
    format: Format,
    role_orchestrator: Arc<dyn IRoleRunnerAggregate>,
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
    let results = rt.block_on(role_orchestrator.run_audit(&root_fp));
    let violations: Vec<ViolationItem> = results.iter().map(ViolationItem::from_lint_result).collect();
    output_violations(&violations, &root, format, is_member_path(&root));
    if violations.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}
