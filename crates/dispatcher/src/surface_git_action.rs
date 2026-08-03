// PURPOSE: GitCommandsSurface — git-diff integration business logic, no formatting.
// Runs AES analysis only on files changed since the specified git base.
//
// AES406 NOTE: This surface uses std::process::Command for git operations because
// no aggregate exists for git subprocess execution. This is a known gap —
// a GitCommandsAggregate should be created in a git-operations crate to
// abstract subprocess calls behind a contract trait.
use shared::cli_commands::LintResult;
use shared::common::{FilePath, GitBranchName};
use shared::quality_rules::ICodeAnalysisAggregate;
use std::process::Command;

/// Git-diff scan outcome — formatted by CLI surfaces.
#[derive(Debug, Clone)]
pub struct GitDiffReport {
    pub base: String,
    pub files: Vec<FilePath>,
    pub results: Vec<LintResult>,
    pub total_violations: usize,
}

pub fn collect_git_diff(
    code_analysis_linter: std::sync::Arc<dyn ICodeAnalysisAggregate>,
    base: GitBranchName,
    project_path: Option<&str>,
    filter: Option<&str>,
) -> Result<GitDiffReport, String> {
    let project_path = FilePath::new(project_path.unwrap_or(".").to_string()).unwrap_or_default();

    // Get changed files via std::process::Command (sync git diff)
    let changed_files = get_changed_files_sync(&project_path, &base)?;

    // Filter to lintable files
    let files: Vec<FilePath> = changed_files
        .into_iter()
        .filter(|fp| {
            shared::common::utility_language_detector::is_lintable(fp)
                && filter.map(|f| fp.value.contains(f)).unwrap_or(true)
        })
        .collect();

    let mut results: Vec<LintResult> = Vec::new();
    for f in &files {
        let r = code_analysis_linter.run_code_analysis_path(f);
        results.extend(r);
    }

    Ok(GitDiffReport {
        base: base.value().to_string(),
        files,
        total_violations: results.len(),
        results,
    })
}

/// Get list of changed files from git diff using std::process::Command.
fn get_changed_files_sync(project_path: &FilePath, base: &GitBranchName) -> Result<Vec<FilePath>, String> {
    let output = Command::new("git")
        .args(["diff", "--name-only", &format!("{}...HEAD", base.value())])
        .current_dir(&project_path.value)
        .output();

    match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            Ok(stdout
                .lines()
                .filter(|line| !line.is_empty())
                .filter_map(|line| FilePath::new(line.to_string()).ok())
                .collect())
        }
        Err(e) => Err(format!("[error] git diff failed: {e}")),
    }
}
