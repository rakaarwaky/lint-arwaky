// PURPOSE: Git diff — CLI thin wrapper
// Calls dispatcher for git-diff business logic, only adds CLI output.
use shared::common::{ExitCode, GitBranchName, Severity};
use shared::quality_rules::ICodeAnalysisAggregate;
use std::sync::Arc;
use tracing::error;

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
                        let loc = match (r.line.value(), r.column.value()) {
                            (l, c) if l > 0 && c > 0 => {
                                format!("{}:{}:{}", r.file.value(), l, c)
                            }
                            (l, _) if l > 0 => format!("{}:{}", r.file.value(), l),
                            _ => r.file.value().to_string(),
                        };
                        let sev = match r.severity {
                            Severity::CRITICAL => "CRITICAL",
                            Severity::HIGH => "HIGH",
                            Severity::MEDIUM => "MEDIUM",
                            Severity::LOW => "LOW",
                            _ => "INFO",
                        };
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
