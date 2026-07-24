use shared::common::taxonomy_common_error::ExitCode;
use std::sync::Arc;

use shared::cli_commands::taxonomy_format_vo::Format;
use shared::cli_commands::utility_path_resolver::is_member_path;
use shared::common::taxonomy_path_vo::FilePath;
use shared::external_lint::contract_external_lint_aggregate::IExternalLintAggregate;

use crate::surface_common_action;
use crate::surface_output_component::{output_violations, ViolationItem};

pub fn handle_scan_external(
    path: Option<FilePath>,
    format: Format,
    external_lint: Arc<dyn IExternalLintAggregate>,
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
    let root_fp = match FilePath::new(root) {
        Ok(fp) => fp,
        Err(_) => return ExitCode::RUNTIME_ERROR,
    };
    let rt = match surface_common_action::create_current_thread_runtime() {
        Ok(r) => r,
        Err(_) => return ExitCode::RUNTIME_ERROR,
    };
    let results = rt.block_on(external_lint.scan_all(&root_fp));
    let violations: Vec<ViolationItem> = results
        .values
        .iter()
        .map(ViolationItem::from_lint_result)
        .collect();
    let has_violations = !violations.is_empty();
    output_violations(
        &violations,
        &root_fp.value,
        format,
        is_member_path(&root_fp.value),
    );
    if has_violations {
        ExitCode::POLICY_FAIL
    } else {
        ExitCode::OK
    }
}
