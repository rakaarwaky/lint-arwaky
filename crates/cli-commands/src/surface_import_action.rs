use shared::common::taxonomy_common_error::ExitCode;
use std::sync::Arc;

use shared::cli_commands::taxonomy_format_vo::Format;
use shared::cli_commands::utility_path_resolver::is_member_path;
use shared::common::taxonomy_path_vo::FilePath;
use shared::import_rules::contract_import_runner_aggregate::IImportRunnerAggregate;

use crate::surface_common_action;
use crate::surface_output_component::{output_violations, ViolationItem};

pub fn handle_scan_import(
    path: Option<FilePath>,
    format: Format,
    import_orchestrator: Arc<dyn IImportRunnerAggregate>,
    _report_formatter: Arc<dyn shared::report_formatter::contract_report_formatter_aggregate::IReportFormatterAggregate>,
) -> ExitCode {
    let root = match &path {
        Some(p) => p.value().to_string(),
        None => ".".to_string(),
    };
    if !std::path::Path::new(&root).exists() {
        eprintln!("Error: path '{}' does not exist", root);
        return ExitCode::RUNTIME_ERROR;
    }
    let root_fp = match FilePath::new(root.clone()) {
        Ok(fp) => fp,
        Err(_) => return ExitCode::RUNTIME_ERROR,
    };
    let rt = match surface_common_action::create_current_thread_runtime() {
        Ok(r) => r,
        Err(_) => return ExitCode::RUNTIME_ERROR,
    };
    let results = match rt.block_on(import_orchestrator.run_audit(&root_fp)) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("[error] import rules failed: {e}");
            return ExitCode::RUNTIME_ERROR;
        }
    };
    let violations: Vec<ViolationItem> = results
        .iter()
        .map(ViolationItem::from_lint_result)
        .collect();
    output_violations(&violations, &root, format, is_member_path(&root));
    if violations.is_empty() {
        ExitCode::OK
    } else {
        ExitCode::POLICY_FAIL
    }
}
