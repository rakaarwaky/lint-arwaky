// PURPOSE: FixCommandsSurface — CLI surface for auto-fix operations
use shared::auto_fix::contract_fix_aggregate::LintFixOrchestratorAggregate;
use shared::code_analysis::contract_lint_aggregate::IArchLintAggregate;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use std::path::PathBuf;
use std::process::ExitCode;
use std::sync::Arc;

pub struct FixCommandsSurface {
    pub arch_linter: Arc<dyn IArchLintAggregate>,
    pub fix_orchestrator_factory:
        Arc<dyn Fn(bool) -> Arc<dyn LintFixOrchestratorAggregate> + Send + Sync>,
}

impl FixCommandsSurface {
    pub fn new(
        arch_linter: Arc<dyn IArchLintAggregate>,
        fix_orchestrator_factory: Arc<
            dyn Fn(bool) -> Arc<dyn LintFixOrchestratorAggregate> + Send + Sync,
        >,
    ) -> Self {
        Self {
            arch_linter,
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

    pub fn run_fix(&self, project_path: FilePath, dry_run: bool) {
        if dry_run {
            println!("[DRY-RUN] Previewing fixes for {}...", project_path.value);
        } else {
            println!("Applying safe fixes to {}...", project_path.value);
        }

        let results = self.arch_linter.run_self_lint(&project_path.value);
        println!("Found {} violations before fix", results.len());

        let fix_orch = (self.fix_orchestrator_factory)(dry_run);
        let fix_result = fix_orch.execute(&project_path);

        println!("{}", fix_result.output.value);

        if !dry_run {
            let after_results = self.arch_linter.run_self_lint(&project_path.value);
            let fixed_count = results.len().saturating_sub(after_results.len());
            println!(
                "Fixed {} violations ({} remaining)",
                fixed_count,
                after_results.len()
            );
            println!("Fix complete.");
        } else {
            println!("Dry-run complete — no changes applied.");
        }
    }
}

pub fn handle_fix(
    path: Option<String>,
    dry_run: bool,
    arch_linter: Arc<dyn IArchLintAggregate>,
    fix_orchestrator_factory: Arc<
        dyn Fn(bool) -> Arc<dyn LintFixOrchestratorAggregate> + Send + Sync,
    >,
) -> ExitCode {
    let root = match path {
        Some(p) => p,
        None => ".".to_string(),
    };
    let fix_surface = FixCommandsSurface::new(arch_linter, fix_orchestrator_factory);
    let fp = FilePath::new(root).unwrap_or_default();
    fix_surface.run_fix(fp, dry_run);
    ExitCode::SUCCESS
}
