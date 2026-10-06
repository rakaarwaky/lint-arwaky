// PURPOSE: FixCommandsSurface — auto-fix business logic, no formatting.
// Runs lint → apply auto-fixes → re-lint to measure improvement.
// Supports dry-run mode (preview only) via the fix_orchestrator_factory closure.
// Adapted: sync (no async_trait, no tokio).
use shared_auto_fix::{FixRequest, IFixAggregate};
use shared_cli_commands::LintResult;
use shared_common::FilePath;
use shared_filesystem::taxonomy_filesystem_vo::{FileEntry, Language};
use shared_quality_rules::ICodeAnalysisAggregate;

use shared_quality_rules::CodeAnalysisRequest;
use std::sync::Arc;

/// Build `FileEntry`s for lintable files under *root* so `run_analysis` gets
/// the real file list (#907). Reads content directly; skips unreadable files.
fn build_fix_entries(root: &FilePath) -> Vec<FileEntry> {
    let base = std::path::Path::new(&root.value);
    if !base.exists() {
        return Vec::new();
    }
    let mut entries: Vec<FileEntry> = Vec::new();
    let mut stack = vec![base.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let Ok(read_dir) = std::fs::read_dir(&dir) else {
            continue;
        };
        for entry in read_dir.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                // Skip generated/vendored trees; source lives in member dirs.
                if matches!(name, "target" | ".git" | "node_modules" | "vendor") {
                    continue;
                }
                stack.push(path);
            } else if is_lintable_path(&path) {
                if let Ok(content) = std::fs::read_to_string(&path) {
                    let size = content.len() as u64;
                    entries.push(FileEntry {
                        path: path.clone(),
                        extension: path
                            .extension()
                            .and_then(|e| e.to_str())
                            .unwrap_or("")
                            .to_string(),
                        language: Language::from_extension(
                            path.extension().and_then(|e| e.to_str()).unwrap_or(""),
                        )
                        .unwrap_or(Language::Unknown),
                        size,
                        content,
                        parse_ok: size > 0,
                        parse_metadata: None,
                    });
                }
            }
        }
    }
    entries
}

/// Whether the file extension maps to a lintable language.
fn is_lintable_path(path: &std::path::Path) -> bool {
    matches!(
        path.extension().and_then(|e| e.to_str()),
        Some("rs") | Some("py") | Some("ts") | Some("tsx") | Some("js") | Some("jsx")
    )
}

fn run_analysis_with_fix_entries(
    code_analysis_linter: &Arc<dyn ICodeAnalysisAggregate>,
    root: &FilePath,
) -> Vec<LintResult> {
    let entries = build_fix_entries(root);
    code_analysis_linter
        .execute(CodeAnalysisRequest::run_analysis(&entries))
        .into_violations()
}

/// Auto-fix outcome — formatted by CLI/MCP surfaces.
#[derive(Debug, Clone)]
pub struct FixReport {
    pub project_path: String,
    pub dry_run: bool,
    pub before_count: usize,
    pub after_count: usize,
    pub fixed_count: usize,
    pub output: String,
    pub success: bool,
    /// True when any per-item fix returned `Failed(reason)`. Per the
    /// PRD Exit Code Contract this means the command is a runtime error (2),
    /// not a policy failure.
    pub has_failed: bool,
    /// Violations matching fixable rules (AES101/203/304) — rendered in dry-run preview.
    pub fixable: Vec<LintResult>,
}

pub fn collect_fix(
    path: Option<FilePath>,
    dry_run: bool,
    code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    fix_orchestrator_factory: Arc<dyn Fn(bool) -> Arc<dyn IFixAggregate> + Send + Sync>,
) -> Result<FixReport, String> {
    let project_path = match path {
        Some(p) => p,
        None => FilePath::new(".").unwrap_or_default(),
    };

    let results = run_analysis_with_fix_entries(&code_analysis_linter, &project_path);

    let fixable: Vec<LintResult> = results
        .iter()
        .filter(|r| {
            let code_str = r.code.code();
            code_str == "AES101" || code_str == "AES203" || code_str == "AES304"
        })
        .cloned()
        .collect();

    let fix_orch = (fix_orchestrator_factory)(dry_run);
    let fix_result = fix_orch.execute(FixRequest::execute(&project_path, dry_run));
    let fix_result = fix_result.into_fix_result();
    let has_failed = fix_result.error.is_some();

    let (after_count, fixed_count, success) = if dry_run {
        (results.len(), 0usize, true)
    } else {
        let after_results = run_analysis_with_fix_entries(&code_analysis_linter, &project_path);
        let fixed_count = results.len().saturating_sub(after_results.len());
        (after_results.len(), fixed_count, after_results.is_empty())
    };

    Ok(FixReport {
        project_path: project_path.value,
        dry_run,
        before_count: results.len(),
        after_count,
        fixed_count,
        output: fix_result.output.value,
        success,
        has_failed,
        fixable,
    })
}

/// Direct fix — takes the orchestrator directly instead of a factory closure.
/// For surfaces that hold a single pre-built orchestrator (e.g. TUI).
pub fn collect_fix_direct(
    path: Option<FilePath>,
    dry_run: bool,
    code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    fix_orchestrator: Arc<dyn IFixAggregate>,
) -> Result<FixReport, String> {
    let project_path = match path {
        Some(p) => p,
        None => FilePath::new(".").unwrap_or_default(),
    };

    let results = run_analysis_with_fix_entries(&code_analysis_linter, &project_path);

    let fixable: Vec<LintResult> = results
        .iter()
        .filter(|r| {
            let code_str = r.code.code();
            code_str == "AES101" || code_str == "AES203" || code_str == "AES304"
        })
        .cloned()
        .collect();

    let fix_result = fix_orchestrator.execute(FixRequest::execute(&project_path, dry_run));
    let fix_result = fix_result.into_fix_result();
    let has_failed = fix_result.error.is_some();

    let (after_count, fixed_count, success) = if dry_run {
        (results.len(), 0usize, true)
    } else {
        let after_results = run_analysis_with_fix_entries(&code_analysis_linter, &project_path);
        let fixed_count = results.len().saturating_sub(after_results.len());
        (after_results.len(), fixed_count, after_results.is_empty())
    };

    Ok(FixReport {
        project_path: project_path.value,
        dry_run,
        before_count: results.len(),
        after_count,
        fixed_count,
        output: fix_result.output.value,
        success,
        has_failed,
        fixable,
    })
}
