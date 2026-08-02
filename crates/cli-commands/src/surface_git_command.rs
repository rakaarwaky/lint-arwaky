// PURPOSE: GitCommandsSurface — CLI surface for git-diff integration
// Runs AES analysis only on files changed since the specified git base.
// Adapted: uses std::process::Command for git operations (no async, no tokio).
use shared::common::{ExitCode, FilePath, GitBranchName, Severity};
use shared::quality_rules::ICodeAnalysisAggregate;
use std::process::Command;

pub fn handle_git_diff(
    code_analysis_linter: std::sync::Arc<dyn ICodeAnalysisAggregate>,
    base: GitBranchName,
    project_path: Option<&str>,
    filter: Option<&str>,
) -> ExitCode {
    println!("Lint Arwaky v{} (Git-Diff Mode)", env!("CARGO_PKG_VERSION"));

    let project_path = FilePath::new(project_path.unwrap_or(".").to_string()).unwrap_or_default();

    // Get changed files via std::process::Command (sync git diff)
    let changed_files = get_changed_files_sync(&project_path, &base);

    // Filter to lintable files
    let files: Vec<FilePath> = changed_files
        .into_iter()
        .filter(|fp| {
            shared::common::utility_language_detector::is_lintable(fp)
                && filter.map(|f| fp.value.contains(f)).unwrap_or(true)
        })
        .collect();

    println!("Base: {} (changed files)", base.value());
    println!("Files changed: {}", files.len());
    println!();

    let mut total_violations = 0;
    for f in &files {
        let results = code_analysis_linter.run_code_analysis_path(f);
        let fv = results.len();
        total_violations += fv;
        if fv > 0 {
            println!("  {}  -> {} violation(s)", f.value, fv);
            for r in &results {
                let loc = match (r.line.value(), r.column.value()) {
                    (l, c) if l > 0 && c > 0 => format!("{}:{}:{}", r.file.value(), l, c),
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
        total_violations,
        files.len()
    );
    if total_violations > 0 {
        ExitCode::POLICY_FAIL
    } else {
        ExitCode::OK
    }
}

/// Get list of changed files from git diff using std::process::Command.
fn get_changed_files_sync(project_path: &FilePath, base: &GitBranchName) -> Vec<FilePath> {
    let output = Command::new("git")
        .args(["diff", "--name-only", &format!("{}...HEAD", base.value())])
        .current_dir(&project_path.value)
        .output();

    match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            stdout
                .lines()
                .filter(|line| !line.is_empty())
                .filter_map(|line| FilePath::new(line.to_string()).ok())
                .collect()
        }
        Err(e) => {
            eprintln!("[error] git diff failed: {e}");
            Vec::new()
        }
    }
}
