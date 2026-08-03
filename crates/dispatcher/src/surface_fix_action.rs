// PURPOSE: FixCommandsSurface — auto-fix business logic, no formatting.
// Runs lint → apply auto-fixes → re-lint to measure improvement.
// Supports dry-run mode (preview only) via the fix_orchestrator_factory closure.
// Adapted: sync (no async_trait, no tokio).
use shared::auto_fix::LintFixOrchestratorAggregate;
use shared::cli_commands::LintResult;
use shared::common::FilePath;
use shared::quality_rules::ICodeAnalysisAggregate;

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
    /// Violations matching fixable rules (AES101/203/304) — rendered in dry-run preview.
    pub fixable: Vec<LintResult>,
}

pub fn collect_fix(
    path: Option<FilePath>,
    dry_run: bool,
    code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    fix_orchestrator_factory: Arc<
        dyn Fn(bool) -> Arc<dyn LintFixOrchestratorAggregate> + Send + Sync,
    >,
) -> Result<FixReport, String> {
    let project_path = match path {
        Some(p) => p,
        None => FilePath::new(".").unwrap_or_default(),
    };

    let results = code_analysis_linter.run_code_analysis(&project_path);

    let fixable: Vec<LintResult> = results
        .iter()
        .filter(|r| {
            let code_str = r.code.code();
            code_str == "AES101" || code_str == "AES203" || code_str == "AES304"
        })
        .cloned()
        .collect();

    let fix_orch = (fix_orchestrator_factory)(dry_run);
    let fix_result = fix_orch.execute(&project_path);

    let (after_count, fixed_count, success) = if dry_run {
        (results.len(), 0usize, true)
    } else {
        let after_results = code_analysis_linter.run_code_analysis(&project_path);
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
        fixable,
    })
}
