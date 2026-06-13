// PURPOSE: FixCommandsSurface — CLI surface for auto-fix operations
use shared::auto_fix::contract_fix_aggregate::LintFixOrchestratorAggregate;
use std::path::PathBuf;
use std::process::ExitCode;
use std::sync::Arc;

use crate::surface_output_controller::{get_output_dir, tee_stdout, write_output};
use code_analysis::resolve_target;
use shared::source_parsing::taxonomy_path_vo::FilePath;

fn _use_contract_aggregates() {
    let _ = std::marker::PhantomData::<dyn LintFixOrchestratorAggregate>;
}

pub struct FixCommandsSurface {
    pub fix_orchestrator_factory: Arc<dyn Fn(bool) -> Arc<dyn LintFixOrchestratorAggregate> + Send + Sync>,
}

impl FixCommandsSurface {
    pub fn new(fix_orchestrator_factory: Arc<dyn Fn(bool) -> Arc<dyn LintFixOrchestratorAggregate> + Send + Sync>) -> Self {
        Self { fix_orchestrator_factory }
    }

    pub fn fix(&self, path: &str) {
        let project_path = FilePath {
            value: PathBuf::from(path)
                .canonicalize()
                .unwrap_or_else(|_| PathBuf::from(path))
                .to_string_lossy()
                .to_string(),
        };
        self.run_fix(project_path, false);
    }

    pub fn run_fix(&self, project_path: FilePath, dry_run: bool) {
        let output_dir = get_output_dir(None);

        let output = tee_stdout(None, || {
            if dry_run {
                println!("[DRY-RUN] Previewing fixes for {}...", project_path.value);
            } else {
                println!("Applying safe fixes to {}...", project_path.value);
            }

            let orchestrator =
                code_analysis::agent_codebase_scan_orchestrator::CodebaseScanOrchestrator::new();
            let results = orchestrator.run_self_lint(&project_path);
            println!("Found {} violations before fix", results.len());

            let fix_orch = (self.fix_orchestrator_factory)(dry_run);
            let fix_result = fix_orch.execute(&project_path);

            println!("{}", fix_result.output.value);

            if !dry_run {
                let after_results = orchestrator.run_self_lint(&project_path);
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
        });

        if let Some(_dir) = output_dir {
            write_output(None, &output, "fix", Some("txt"));
        }
    }
}

pub fn handle_fix(
    path: Option<String>,
    dry_run: bool,
    fix_orchestrator_factory: Arc<dyn Fn(bool) -> Arc<dyn LintFixOrchestratorAggregate> + Send + Sync>,
) -> ExitCode {
    let root = resolve_target(path);
    let fix_surface = FixCommandsSurface::new(fix_orchestrator_factory);
    fix_surface.run_fix(FilePath::new(root).unwrap_or_default(), dry_run);
    ExitCode::SUCCESS
}
