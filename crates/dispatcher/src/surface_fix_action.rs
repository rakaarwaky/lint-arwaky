// PURPOSE: FixCommandsSurface — auto-fix business logic, no formatting.
// Runs lint → apply auto-fixes → re-lint to measure improvement.
// Supports dry-run mode (preview only) via the fix_orchestrator_factory closure.
// Adapted: sync (no async_trait, no tokio).
use shared_auto_fix::{FixRequest, IFixAggregate};
use shared_cli_commands::LintResult;
use shared_common::FilePath;
use shared_quality_rules::ICodeAnalysisAggregate;

use shared_quality_rules::CodeAnalysisRequest;
use std::sync::Arc;

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

    let results = code_analysis_linter
        .execute(CodeAnalysisRequest::run_analysis(&[]))
        .into_violations();

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
        let after_results = code_analysis_linter
            .execute(CodeAnalysisRequest::run_analysis(&[]))
            .into_violations();
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

    let results = code_analysis_linter
        .execute(CodeAnalysisRequest::run_analysis(&[]))
        .into_violations();

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
        let after_results = code_analysis_linter
            .execute(CodeAnalysisRequest::run_analysis(&[]))
            .into_violations();
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
