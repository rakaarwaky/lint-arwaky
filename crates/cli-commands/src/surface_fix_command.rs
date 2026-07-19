// PURPOSE: FixCommandsSurface — CLI surface for auto-fix operations
//
// Runs lint → apply auto-fixes → re-lint to measure improvement.
// Supports dry-run mode (preview only) via the fix_orchestrator_factory closure.
//
// The factory pattern allows the DI container to control whether fixes are
// actually applied (real mode) or just simulated (dry-run).
//
// Fixable violations: AES101 (naming), AES203 (unused imports), AES304 (bypass)
use shared::auto_fix::contract_fix_aggregate::LintFixOrchestratorAggregate;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::common::taxonomy_path_vo::FilePath;
use std::path::PathBuf;
use std::process::ExitCode;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct FixCommandsSurface {
    pub code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    pub fix_orchestrator_factory:
        Arc<dyn Fn(bool) -> Arc<dyn LintFixOrchestratorAggregate> + Send + Sync>,
}

// ─── Block 3: Constructors & Helpers ──────────────────────
// ─── Block 2: Public Contract ─────────────────────────────
impl FixCommandsSurface {
    pub fn new(
        code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
        fix_orchestrator_factory: Arc<
            dyn Fn(bool) -> Arc<dyn LintFixOrchestratorAggregate> + Send + Sync,
        >,
    ) -> Self {
        Self {
            code_analysis_linter,
            fix_orchestrator_factory,
        }
    }

    pub fn fix(&self, path: &str) {
        let canonical = match PathBuf::from(path).canonicalize() {
            Ok(p) => p,
            Err(_) => PathBuf::from(path),
        };
        let project_path = FilePath {
            value: canonical.to_string_lossy().to_string(),
        };
        self.run_fix(project_path, false);
    }

    pub fn run_fix(&self, project_path: FilePath, dry_run: bool) -> ExitCode {
        if dry_run {
            println!("[DRY-RUN] Previewing fixes for {}...", project_path.value);
        } else {
            println!("Applying safe fixes to {}...", project_path.value);
        }

        let results = self
            .code_analysis_linter
            .run_code_analysis(&project_path);
        println!("Found {} violations before fix (AES301-305 only; other rules not included in count — #107 P1 #15)", results.len());

        let fix_orch = (self.fix_orchestrator_factory)(dry_run);
        let fix_result = fix_orch.execute(&project_path);

        println!("{}", fix_result.output.value);

        if !dry_run {
            let after_results = self
                .code_analysis_linter
                .run_code_analysis(&project_path);
            let fixed_count = results.len().saturating_sub(after_results.len());
            println!(
                "Fixed {} violations ({} remaining, AES301-305 only — #107 P1 #15)",
                fixed_count,
                after_results.len()
            );
            if after_results.is_empty() {
                println!("Fix complete — all violations resolved.");
                ExitCode::SUCCESS
            } else {
                println!("Fix complete — {} violations remain.", after_results.len());
                ExitCode::from(1)
            }
        } else {
            println!("Dry-run complete — no changes applied.");
            ExitCode::SUCCESS
        }
    }
}

pub fn handle_fix(
    path: Option<String>,
    dry_run: bool,
    code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    fix_orchestrator_factory: Arc<
        dyn Fn(bool) -> Arc<dyn LintFixOrchestratorAggregate> + Send + Sync,
    >,
) -> ExitCode {
    let root = match path {
        Some(p) => p,
        None => ".".to_string(),
    };
    let fix_surface = FixCommandsSurface::new(code_analysis_linter, fix_orchestrator_factory);
    let fp = FilePath::new(root).unwrap_or_default();
    fix_surface.run_fix(fp, dry_run)
}
