// PURPOSE: Import rules scan surface action
// Adapted: sync — IImportRunnerAggregate::run_audit is now sync. No tokio runtime.
use shared::common::ExitCode;
use std::sync::Arc;

use shared::cli_commands::Format;
use shared::common::FilePath;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::import_rules::IImportRunnerAggregate;

use crate::surface_output_component::{ViolationItem, output_violations};

pub fn handle_scan_import(
    path: Option<FilePath>,
    format: Format,
    import_orchestrator: Arc<dyn IImportRunnerAggregate>,
    _report_formatter: Arc<dyn shared::report_formatter::IReportFormatterAggregate>,
    filter: Option<String>,
    fs_agg: Arc<dyn IFilesystemAggregate>,
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

    // run_audit is sync in new API — call directly
    let results = match import_orchestrator.run_audit(&root_fp) {
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

    output_violations(&violations, &root, format, fs_agg.is_member_path(&root_fp));
    if violations.is_empty() {
        ExitCode::OK
    } else {
        ExitCode::POLICY_FAIL
    }
}
