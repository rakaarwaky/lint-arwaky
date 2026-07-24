use std::process::ExitCode;
use std::sync::Arc;

use shared::cli_commands::taxonomy_format_vo::Format;
use shared::cli_commands::utility_path_resolver::is_member_path;
use shared::report_formatter::contract_report_formatter_aggregate::IReportFormatterAggregate;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::common::taxonomy_path_vo::FilePath;

use crate::surface_output_component::{output_violations, ViolationItem};

pub fn handle_scan_quality(
    path: Option<FilePath>,
    format: Format,
    code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    _report_formatter: Arc<dyn IReportFormatterAggregate>,
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
    let results = code_analysis_linter.run_code_analysis_path(&root_fp);
    let violations: Vec<ViolationItem> = results.iter().map(ViolationItem::from_lint_result).collect();
    output_violations(&violations, &root, format, is_member_path(&root));
    if violations.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}
