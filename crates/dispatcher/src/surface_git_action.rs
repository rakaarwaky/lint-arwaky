// PURPOSE: GitCommandsSurface — git-diff integration business logic, no formatting.
// Runs AES analysis only on files changed since the specified git base.
//
// AES406 NOTE: This surface uses std::process::Command for git operations because
// no aggregate exists for git subprocess execution. This is a known gap —
// a GitCommandsAggregate should be created in a git-operations crate to
// abstract subprocess calls behind a contract trait.
use shared_cli_commands::LintResult;
use shared_common::{FilePath, GitBranchName};
use shared_git_hooks::{GitHooksRequest, IGitHooksAggregate};
use shared_quality_rules::CodeAnalysisRequest;
use shared_quality_rules::ICodeAnalysisAggregate;
use std::process::Command;
use std::sync::Arc;

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
    let project_path = FilePath::new(project_path.unwrap_or(".").to_string())
        .map_err(|_| "[error] invalid project path".to_string())?;

    // Get changed files via std::process::Command (sync git diff)
    let changed_files = get_changed_files_sync(&project_path, &base)?;

    // Filter to lintable files
    let files: Vec<FilePath> = changed_files
        .into_iter()
        .filter(|fp| {
            shared_filesystem::utility_language_detector::is_lintable(fp)
                && filter.map(|f| fp.value.contains(f)).unwrap_or(true)
        })
        .collect();

    let mut results: Vec<LintResult> = Vec::new();
    if !files.is_empty() {
        // Build FileEntry list from changed files (matching capabilities_diff_checker.rs)
        let mut entries: Vec<shared_filesystem::taxonomy_filesystem_vo::FileEntry> = Vec::new();
        for fp in &files {
            let joined = std::path::Path::new(&project_path.value).join(&fp.value);
            if let Ok(content) = std::fs::read_to_string(&joined) {
                let size = content.len() as u64;
                entries.push(shared_filesystem::taxonomy_filesystem_vo::FileEntry {
                    path: joined,
                    extension: fp.extension(),
                    language: shared_filesystem::utility_language_detector::detect_language(fp),
                    size,
                    content,
                    parse_ok: size > 0,
                    parse_metadata: None,
                });
            }
        }
        let r = code_analysis_linter
            .execute(CodeAnalysisRequest::RunAnalysis { files: entries })
            .into_violations();
        results.extend(r);
    }

    Ok(GitDiffReport {
        base: base.value().to_string(),
        files,
        total_violations: results.len(),
        results,
    })
}

/// Hook install/uninstall outcome — formatted by CLI/MCP/TUI surfaces.
#[derive(Debug, Clone)]
pub struct HookReport {
    pub action: String,
    pub success: bool,
    pub message: String,
}

/// Install pre-commit hook via IGitHooksAggregate.
pub fn collect_install_hook(
    git_hooks: Arc<dyn IGitHooksAggregate>,
    executable_path: &FilePath,
) -> Result<HookReport, String> {
    match git_hooks
        .execute(GitHooksRequest::install(executable_path))
        .into_status()
    {
        Ok(status) => Ok(HookReport {
            action: "install".to_string(),
            success: status.value(),
            message: if status.value() {
                "Pre-commit hook installed successfully".to_string()
            } else {
                "Pre-commit hook installation reported failure".to_string()
            },
        }),
        Err(e) => Ok(HookReport {
            action: "install".to_string(),
            success: false,
            message: format!("Failed to install hook: {e}"),
        }),
    }
}

/// Uninstall pre-commit hook via IGitHooksAggregate.
pub fn collect_uninstall_hook(
    git_hooks: Arc<dyn IGitHooksAggregate>,
) -> Result<HookReport, String> {
    match git_hooks
        .execute(GitHooksRequest::uninstall())
        .into_status()
    {
        Ok(status) => Ok(HookReport {
            action: "uninstall".to_string(),
            success: status.value(),
            message: if status.value() {
                "Pre-commit hook removed successfully".to_string()
            } else {
                "Pre-commit hook removal reported failure".to_string()
            },
        }),
        Err(e) => Ok(HookReport {
            action: "uninstall".to_string(),
            success: false,
            message: format!("Failed to uninstall hook: {e}"),
        }),
    }
}

/// Get list of changed files from git diff using std::process::Command.
fn get_changed_files_sync(
    project_path: &FilePath,
    base: &GitBranchName,
) -> Result<Vec<FilePath>, String> {
    let base = base.value();
    validate_git_base_ref(base)?;
    let output = Command::new("git")
        .args(["diff", "--name-only", &format!("{base}...HEAD")])
        .current_dir(&project_path.value)
        .output();

    match output {
        Ok(out) => {
            // The exit status used to be ignored. `git diff` exits 128 for an
            // unknown base ref with an empty stdout, which read as "no files
            // changed" — the scan then reported zero violations and a clean
            // exit code for a ref that does not exist.
            if !out.status.success() {
                let stderr = String::from_utf8_lossy(&out.stderr);
                return Err(format!(
                    "[error] git diff against '{base}' failed ({}): {}",
                    out.status,
                    stderr.trim()
                ));
            }
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

/// Validate a git base ref, returning the reason when it is rejected.
///
/// Public so the check is testable: it was a private helper with no test, and
/// it is the only thing standing between the MCP `base` argument and the `git`
/// argument list.
pub fn validate_git_base_ref(s: &str) -> Result<(), String> {
    if is_valid_ref(s) {
        Ok(())
    } else if s.is_empty() {
        Err("[error] git base ref must not be empty".to_string())
    } else if s.starts_with('-') {
        Err(format!(
            "[error] invalid git base ref '{s}': a ref may not start with '-'"
        ))
    } else {
        Err(format!(
            "[error] invalid git base ref '{s}': only alphanumerics, '.', '_', '/' and '-' are allowed"
        ))
    }
}

/// Reject git refs that could inject command-line options (e.g. `--upload-pack`).
fn is_valid_ref(s: &str) -> bool {
    !s.is_empty()
        && !s.starts_with('-')
        && s.chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '/' | '-'))
}
