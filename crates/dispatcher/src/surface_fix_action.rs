// PURPOSE: FixCommandsSurface — auto-fix business logic, no formatting.
// Runs lint → apply auto-fixes → re-lint to measure improvement.
// Supports dry-run mode (preview only) via the fix_orchestrator_factory closure.
// Adapted: sync (no async_trait, no tokio).
use shared_auto_fix::{FixRequest, IFixAggregate};
use shared_cli_commands::FixDelta;
use shared_cli_commands::LintResult;
use shared_common::ExitCode;
use shared_common::FilePath;
use shared_filesystem::FilesystemRequest;
use shared_filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared_filesystem::taxonomy_filesystem_vo::{FileEntry, Language};
use shared_quality_rules::ICodeAnalysisAggregate;

use shared_quality_rules::CodeAnalysisRequest;
use std::sync::Arc;

/// Whether the auto-fixer owns this rule code.
///
/// Reads the aggregate's own list rather than a local copy: this file used to
/// carry a three-entry duplicate of `FIXABLE_CODES`, so AES403/AES405 were
/// fixable in the fixer but invisible to the verdict.
fn is_fixable(code: &str) -> bool {
    shared_auto_fix::FIXABLE_CODES
        .iter()
        .any(|c| c.code() == code)
}

/// Build `FileEntry`s for lintable files under *root*.
///
/// Discovery goes through the filesystem aggregate so the configured
/// `ignored_paths` apply and `read_lintable_file` bounds every read. The
/// previous implementation walked with `std::fs::read_dir` on a hard-coded
/// skip list, which meant an ignored `fixtures/` tree was linted, "fixed", and
/// written back.
///
/// Public so the ignored-paths contract is testable without a live fix run.
pub fn build_fix_entries(
    filesystem: &Arc<dyn IFilesystemAggregate>,
    root: &FilePath,
    ignored: &[String],
) -> Vec<FileEntry> {
    let base = std::path::Path::new(&root.value);
    if !base.exists() {
        return Vec::new();
    }
    let paths = filesystem
        .execute(FilesystemRequest::discover_source_files(base, ignored))
        .into_paths();

    let mut entries: Vec<FileEntry> = Vec::new();
    for path in paths {
        let Some(content) = filesystem
            .execute(FilesystemRequest::read_lintable_file(&path))
            .into_content_opt()
        else {
            continue;
        };
        let file_path = std::path::Path::new(&path);
        let extension = file_path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_string();
        let size = content.len() as u64;
        entries.push(FileEntry {
            path: file_path.to_path_buf(),
            language: Language::from_extension(&extension).unwrap_or(Language::Unknown),
            extension,
            size,
            content,
            // Reading succeeded (the file was lintable), which is what this
            // flag reports to the quality checkers. It is not an AST verdict.
            parse_ok: true,
            parse_metadata: None,
        });
    }
    entries
}

fn run_analysis_with_fix_entries(
    code_analysis_linter: &Arc<dyn ICodeAnalysisAggregate>,
    filesystem: &Arc<dyn IFilesystemAggregate>,
    root: &FilePath,
    ignored: &[String],
) -> Vec<LintResult> {
    let entries = build_fix_entries(filesystem, root, ignored);
    code_analysis_linter
        .execute(CodeAnalysisRequest::RunAnalysis { files: entries })
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
    /// True when any per-item fix returned `Failed(reason)`. Per the exit-code
    /// contract this means the command is a runtime error (2), not a policy
    /// failure.
    pub has_failed: bool,
    /// Violations matching fixable rules — rendered in dry-run preview.
    pub fixable: Vec<LintResult>,
    /// Findings the run resolved, compared by identity rather than by count.
    pub resolved: Vec<LintResult>,
    /// Findings the run introduced. A non-empty list is a regression: the fix
    /// made the tree worse, which a count comparison could not see.
    pub introduced: Vec<LintResult>,
    /// Fixable findings still present after the run.
    pub remaining_fixable: usize,
}

impl FixReport {
    /// The one exit-code decision for every surface.
    ///
    /// The CLI and the MCP action each used to derive this by hand and they
    /// disagreed: the CLI answered 1 for a partial fix while MCP answered 0,
    /// so an agent gating on `exit_code` concluded the tree was clean.
    pub fn verdict(&self) -> ExitCode {
        if self.has_failed {
            // A per-item Failed(reason) is a runtime error and outranks policy.
            return ExitCode::RUNTIME_ERROR;
        }
        if self.dry_run {
            // A preview never changes the tree, so it cannot fail policy.
            return ExitCode::OK;
        }
        if !self.introduced.is_empty() || self.remaining_fixable > 0 {
            return ExitCode::POLICY_FAIL;
        }
        ExitCode::OK
    }
}

/// Assemble a report from the before/after analyses.
fn build_report(
    project_path: String,
    dry_run: bool,
    before: &[LintResult],
    after: Option<&[LintResult]>,
    output: String,
    error: Option<shared_common::ErrorMessage>,
) -> FixReport {
    let fixable: Vec<LintResult> = before
        .iter()
        .filter(|r| is_fixable(r.code.code()))
        .cloned()
        .collect();
    // `FixResult::error` is populated only for a runtime failure (a per-item
    // `Failed(reason)`), so it maps straight onto the exit-code contract's
    // exit 2. Policy skips never set it.
    let has_failed = error.is_some();

    let delta = match after {
        Some(after) => {
            let remaining = after.iter().filter(|r| is_fixable(r.code.code())).count();
            FixDelta::between(before, after).with_remaining_fixable(remaining)
        }
        None => FixDelta::preview(before, &fixable),
    };

    let after_count = delta.after_count;
    // A preview is successful when it ran, whatever it found: nothing was
    // written, so "remaining fixable" is a report line, not a failure.
    let success = if dry_run {
        !has_failed
    } else {
        delta.is_clean()
    };
    FixReport {
        project_path,
        dry_run,
        before_count: before.len(),
        after_count,
        fixed_count: delta.resolved.len(),
        output,
        success,
        has_failed,
        fixable,
        resolved: delta.resolved,
        introduced: delta.introduced,
        remaining_fixable: delta.remaining_fixable,
    }
}

/// Apply the fixes and measure the result.
fn run_fix(
    project_path: FilePath,
    dry_run: bool,
    code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    filesystem: Arc<dyn IFilesystemAggregate>,
    fix_orchestrator: Arc<dyn IFixAggregate>,
    ignored: &[String],
) -> FixReport {
    let before =
        run_analysis_with_fix_entries(&code_analysis_linter, &filesystem, &project_path, ignored);

    let fix_result = fix_orchestrator
        .execute(FixRequest::execute(&project_path, dry_run))
        .into_fix_result();

    // A dry run never touches disk, so there is no second analysis to run —
    // the pre-fix set *is* the post-fix set.
    let after = if dry_run {
        None
    } else {
        Some(run_analysis_with_fix_entries(
            &code_analysis_linter,
            &filesystem,
            &project_path,
            ignored,
        ))
    };

    build_report(
        project_path.value,
        dry_run,
        &before,
        after.as_deref(),
        fix_result.output.value,
        fix_result.error,
    )
}

pub fn collect_fix(
    path: Option<FilePath>,
    dry_run: bool,
    code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    filesystem: Arc<dyn IFilesystemAggregate>,
    fix_orchestrator_factory: Arc<dyn Fn(bool) -> Arc<dyn IFixAggregate> + Send + Sync>,
    ignored: &[String],
) -> Result<FixReport, String> {
    let project_path = match path {
        Some(p) => p,
        None => FilePath::new(".").unwrap_or_default(),
    };
    let fix_orch = (fix_orchestrator_factory)(dry_run);
    Ok(run_fix(
        project_path,
        dry_run,
        code_analysis_linter,
        filesystem,
        fix_orch,
        ignored,
    ))
}

/// Direct fix — takes the orchestrator directly instead of a factory closure.
/// For surfaces that hold a single pre-built orchestrator (e.g. TUI).
///
/// Shares [`run_fix`] with [`collect_fix`]; the two used to be line-for-line
/// duplicates, so a behaviour fix could land in one and not the other.
pub fn collect_fix_direct(
    path: Option<FilePath>,
    dry_run: bool,
    code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    filesystem: Arc<dyn IFilesystemAggregate>,
    fix_orchestrator: Arc<dyn IFixAggregate>,
    ignored: &[String],
) -> Result<FixReport, String> {
    let project_path = match path {
        Some(p) => p,
        None => FilePath::new(".").unwrap_or_default(),
    };
    Ok(run_fix(
        project_path,
        dry_run,
        code_analysis_linter,
        filesystem,
        fix_orchestrator,
        ignored,
    ))
}
