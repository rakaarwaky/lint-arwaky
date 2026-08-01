use shared::common::ExitCode;
use std::sync::Arc;

use shared::cli_commands::Format;
use shared::cli_commands::utility_path_resolver::is_member_path;
use shared::common::FilePath;
use shared::import_rules::IImportRunnerAggregate;

use crate::surface_common_action;
use crate::surface_output_component::{ViolationItem, output_violations};

pub fn handle_scan_import(
    path: Option<FilePath>,
    format: Format,
    import_orchestrator: Arc<dyn IImportRunnerAggregate>,
    _report_formatter: Arc<dyn shared::report_formatter::IReportFormatterAggregate>,
    filter: Option<String>,
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
    let mut violations: Vec<ViolationItem> = results
        .iter()
        .map(ViolationItem::from_lint_result)
        .collect();

    if let Some(ref filter_str) = filter {
        let filter_upper = filter_str.to_uppercase();
        violations.retain(|v| v.code.code().contains(&filter_upper));
    }

    output_violations(&violations, &root, format, is_member_path(&root));
    if violations.is_empty() {
        ExitCode::OK
    } else {
        ExitCode::POLICY_FAIL
    }
}
