// PURPOSE: Git diff — CLI thin wrapper
// Calls dispatcher for git-diff business logic, only adds CLI output.
use shared::common::{ExitCode, GitBranchName};
use shared::quality_rules::ICodeAnalysisAggregate;
use std::sync::Arc;
use tracing::error;

use crate::utility_output_text_formatter::format_location;

pub fn handle_git_diff(
    code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    base: GitBranchName,
    project_path: Option<&str>,
    filter: Option<&str>,
) -> ExitCode {
    match dispatcher::surface_git_action::collect_git_diff(
        code_analysis_linter,
        base,
        project_path,
        filter,
    ) {
        Ok(report) => {
            println!("Lint Arwaky v{} (Git-Diff Mode)", env!("CARGO_PKG_VERSION"));
            println!("Base: {} (changed files)", report.base);
            println!("Files changed: {}", report.files.len());
            println!();

            let mut per_file: std::collections::BTreeMap<
                String,
                Vec<&shared::cli_commands::LintResult>,
            > = std::collections::BTreeMap::new();
            for r in &report.results {
                per_file.entry(r.file.value.clone()).or_default().push(r);
            }
            for f in &report.files {
                let results = per_file.get(&f.value).map(|v| v.len()).unwrap_or(0);
                if results > 0 {
                    println!("  {}  -> {} violation(s)", f.value, results);
                    for r in per_file.get(&f.value).into_iter().flatten() {
                        let loc = format_location(r.file.value(), r.line.value(), r.column.value());
                        let sev = format!("{}", r.severity).to_uppercase();
                        println!("    {} [{}] {}", loc, sev, r.message.value());
                    }
                } else {
                    println!("  {}  -> clean", f.value);
                }
            }

            println!();
            println!(
                "{} violations across {} changed files",
                report.total_violations,
                report.files.len()
            );
            if report.total_violations > 0 {
                ExitCode::POLICY_FAIL
            } else {
                ExitCode::OK
            }
        }
        Err(e) => {
            error!(error = %e, "operation failed");
            ExitCode::RUNTIME_ERROR
        }
    }
}
