// PURPOSE: GitCommandsSurface — CLI surface for git-diff integration
//
// Runs AES analysis only on files changed since the specified git base (e.g. HEAD).
// Filters changed files through the language detector to skip non-lintable files.
//
// Use-case: pre-commit hooks and CI workflows that want per-file diff analysis.
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::common::taxonomy_git_vo::GitBranchName;
use shared::common::taxonomy_path_vo::FilePath;
use shared::git_hooks::contract_git_hooks_aggregate::GitHooksAggregate;
use shared::common::taxonomy_common_error::ExitCode;
use std::sync::Arc;

pub struct GitCommandsSurface {}

impl Default for GitCommandsSurface {
    fn default() -> Self {
        Self::new()
    }
}

impl GitCommandsSurface {
    pub fn new() -> Self {
        Self {}
    }
}

pub async fn handle_git_diff(
    git_aggregate: Arc<dyn GitHooksAggregate>,
    code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    base: GitBranchName,
    project_path: Option<&str>,
    filter: Option<&str>,
) -> ExitCode {
    println!("Lint Arwaky v{} (Git-Diff Mode)", env!("CARGO_PKG_VERSION"));

    // P2.5: use user-provided path instead of hardcoded "."
    let project_path = FilePath::new(project_path.unwrap_or(".").to_string()).unwrap_or_default();

    let changed_files = git_aggregate
        .diff_protocol()
        .get_changed_files(&project_path, &base)
        .await;

    // P2.5: apply filter to changed files
    let files: Vec<&shared::common::taxonomy_path_vo::FilePath> = changed_files
        .values
        .iter()
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
            for r in results.iter().take(3) {
                println!(
                    "    {}:{} [{}] {}",
                    r.file.value(),
                    r.line.value(),
                    match r.severity {
                        shared::common::taxonomy_severity_vo::Severity::CRITICAL => "CRITICAL",
                        shared::common::taxonomy_severity_vo::Severity::HIGH => "HIGH",
                        shared::common::taxonomy_severity_vo::Severity::MEDIUM => "MEDIUM",
                        shared::common::taxonomy_severity_vo::Severity::LOW => "LOW",
                        _ => "INFO",
                    },
                    r.message.value()
                );
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
