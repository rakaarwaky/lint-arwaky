// PURPOSE: FixCommandsSurface — CLI surface for auto-fix operations
//
// Runs lint → apply auto-fixes → re-lint to measure improvement.
// Supports dry-run mode (preview only) via the fix_orchestrator_factory closure.
//
// The factory pattern allows the DI container to control whether fixes are
// actually applied (real mode) or just simulated (dry-run).
//
// Fixable violations: AES101 (naming), AES203 (unused imports), AES304 (bypass)
use shared::auto_fix::LintFixOrchestratorAggregate;
use shared::code_analysis::ICodeAnalysisAggregate;
use shared::common::{
    ExitCode,
    FilePath,
};

use std::path::PathBuf;
use std::sync::Arc;

pub struct FixCommandsSurface {
    pub code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    pub fix_orchestrator_factory:
        Arc<dyn Fn(bool) -> Arc<dyn LintFixOrchestratorAggregate> + Send + Sync>,
}

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
        let results = self.code_analysis_linter.run_code_analysis(&project_path);

        if dry_run {
            println!("[DRY-RUN] Previewing fixes for {}...", project_path.value);
            for r in results.iter() {
                let code_str = r.code.code();
                if code_str == "AES101" || code_str == "AES203" || code_str == "AES304" {
                    let loc = match (r.line.value(), r.column.value()) {
                        (l, c) if l > 0 && c > 0 => format!("{}:{}:{}", r.file.value(), l, c),
                        (l, _) if l > 0 => format!("{}:{}", r.file.value(), l),
                        _ => r.file.value().to_string(),
                    };
                    println!("  [fixable] {} [{}] {}", loc, code_str, r.message.value());
                }
            }
        } else {
            println!("Applying safe fixes to {}...", project_path.value);
        }

        println!("Found {} violations before fix (AES301-305 only; other rules not included in count — #107 P1 #15)", results.len());

        let fix_orch = (self.fix_orchestrator_factory)(dry_run);
        let fix_result = fix_orch.execute(&project_path);

        println!("{}", fix_result.output.value);

        if dry_run {
            // Skip second scan in dry-run mode — no changes applied, no need to re-lint
            println!("Dry-run complete — no changes applied.");
            ExitCode::OK
        } else {
            let after_results = self.code_analysis_linter.run_code_analysis(&project_path);
            let fixed_count = results.len().saturating_sub(after_results.len());
            println!(
                "Fixed {} violations ({} remaining, AES301-305 only — #107 P1 #15)",
                fixed_count,
                after_results.len()
            );
            if after_results.is_empty() {
                println!("Fix complete — all violations resolved.");
                ExitCode::OK
            } else {
                eprintln!("Fix complete — {} violations remain.", after_results.len());
                ExitCode::POLICY_FAIL
            }
        }
    }
}

pub fn handle_fix(
    path: Option<FilePath>,
    dry_run: bool,
    code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    fix_orchestrator_factory: Arc<
        dyn Fn(bool) -> Arc<dyn LintFixOrchestratorAggregate> + Send + Sync,
    >,
) -> ExitCode {
    let root = match path {
        Some(p) => p,
        None => FilePath::new(".").unwrap_or_default(),
    };
    let fix_surface = FixCommandsSurface::new(code_analysis_linter, fix_orchestrator_factory);
    fix_surface.run_fix(root, dry_run)
}
