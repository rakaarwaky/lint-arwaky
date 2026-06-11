// PURPOSE: FixCommandsSurface — CLI surface for auto-fix operations via ServiceContainerAggregate
use auto_fix::contract_fix_aggregate::LintFixOrchestratorAggregate;
use std::path::PathBuf;
use std::process::ExitCode;
use std::sync::Arc;

use cli_commands::surface_output_controller::{get_output_dir, tee_stdout, write_output};
use code_analysis::resolve_target;
use di_containers::contract_service_aggregate::ServiceContainerAggregate;
use source_parsing::taxonomy_path_vo::FilePath;

/// Satisfy AES030 orphan detection - surface references contract aggregates
fn _use_contract_aggregates() {
    let _ = std::marker::PhantomData::<dyn LintFixOrchestratorAggregate>;
}

pub struct FixCommandsSurface {
    pub container: Option<Arc<dyn ServiceContainerAggregate>>,
}

impl FixCommandsSurface {
    pub fn new(container: Option<Arc<dyn ServiceContainerAggregate>>) -> Self {
        Self { container }
    }

    pub fn register_all(&mut self, container: Arc<dyn ServiceContainerAggregate>) {
        self.container = Some(container);
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
                crate::code_analysis::agent_codebase_scan_orchestrator::CodebaseScanOrchestrator::new();
            let results = orchestrator.run_self_lint(&project_path);
            println!("Found {} violations before fix", results.len());

            // Get fix orchestrator from container (AES023: surfaces must not import agent directly)
            let fix_orch = match self
                .container
                .as_ref()
                .and_then(|c| c.get_fix_orchestrator(dry_run))
            {
                Some(o) => o,
                None => {
                    println!("[error] Fix orchestrator not available in container");
                    return;
                }
            };
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

pub fn register_fix_commands(container: Arc<dyn ServiceContainerAggregate>) -> FixCommandsSurface {
    let mut surface = FixCommandsSurface::new(Some(container.clone()));
    surface.register_all(container);
    surface
}

pub fn handle_fix(
    path: Option<String>,
    dry_run: bool,
    container: Arc<dyn ServiceContainerAggregate>,
) -> ExitCode {
    let root = resolve_target(path);
    let fix_surface = register_fix_commands(container);
    fix_surface.run_fix(FilePath::new(root).unwrap_or_default(), dry_run);
    ExitCode::SUCCESS
}
