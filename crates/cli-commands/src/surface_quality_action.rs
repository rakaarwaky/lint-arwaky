use std::process::ExitCode;
use std::sync::Arc;

use shared::cli_commands::taxonomy_format_vo::Format;
use shared::report_formatter::contract_report_formatter_aggregate::IReportFormatterAggregate;
use shared::cli_commands::taxonomy_scan_report_vo::ScanReport;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::common::taxonomy_path_vo::FilePath;

pub fn handle_scan_quality(
    path: Option<FilePath>,
    code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
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
    let results = code_analysis_linter.run_code_analysis_path(&root_fp);
    let report = ScanReport::new(results.clone(), vec![]);
    let output = report_formatter.format(&report, Format::Text);
    println!("{output}");
    if results.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}
