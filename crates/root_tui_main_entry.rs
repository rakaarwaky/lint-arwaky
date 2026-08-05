// PURPOSE: TUI binary entry point — composition root wiring domain aggregates
// directly into TUI surfaces (surface-only: no contract/aggregate/capabilities).
use lint_arwaky::root_entry_container::CommonDeps;
use std::sync::Arc;

fn main() -> anyhow::Result<()> {
    let deps = CommonDeps::build();

    // TUI needs a direct fix orchestrator (not a factory).
    let fix_orchestrator = (deps.fix_orchestrator_factory)(false);

    // Build TUI surfaces via dispatcher — SurfaceLintExecutor delegates to dispatcher functions.
    let lint_executor = Arc::new(
        tui::surface_lint_executor::SurfaceLintExecutor::new(
            deps.code_analysis_linter,
            deps.filesystem.clone(),
            deps.fs_factory,
            deps.orphan_factory,
        )
        .with_fix(fix_orchestrator)
        .with_setup(deps.setup_orchestrator)
        .with_maintenance(deps.maintenance_orchestrator)
        .with_hook_port(deps.git_hooks_aggregate)
        .with_config(deps.config_orchestrator)
        .with_external_lint(deps.external_lint)
        .with_orphan(deps.orphan_orchestrator)
        .with_import_orchestrator(deps.import_orchestrator)
        .with_naming_orchestrator(deps.naming_orchestrator)
        .with_role_orchestrator(deps.role_orchestrator),
    );

    tui::root_tui_container::TuiContainer::run(lint_executor, deps.filesystem)
}
