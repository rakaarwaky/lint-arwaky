// PURPOSE: TuiContainer — DI container for the TUI crate (root layer)
//
// Assembles the full TUI dependency graph:
//   FileSystemAdapter → ActionHandler → TuiOrchestrator → TuiCommandSurface
//   CodeAnalysisContainer → AutoFixContainer → OrphanContainer → MaintenanceContainer
//   → LintExecutor → ActionHandler
//
// The TUI follows the same 7-layer architecture as the CLI, with the main
// event loop in TuiCommandSurface driving ratatui rendering + crossterm input.
use crate::agent_tui_orchestrator::TuiOrchestrator;
use crate::capabilities_action_handler::ActionHandler;
use crate::capabilities_lint_executor::LintExecutor;
use crate::infrastructure_file_system_adapter::FileSystemAdapter;
use crate::surface_tui_command::TuiCommandSurface;
use shared::tui::contract_action_handler_protocol::IActionHandlerProtocol;
use shared::tui::contract_tui_aggregate::ITuiAggregate;
use std::sync::Arc;

pub struct TuiContainer;

impl TuiContainer {
    pub fn run() -> anyhow::Result<()> {
        let fs_adapter = Arc::new(FileSystemAdapter::new());

        let code_analysis_container =
            code_analysis::root_code_analysis_container::CodeAnalysisContainer::new();
        let code_analysis_aggregate = code_analysis_container.code_analysis_linter();

        let auto_fix_container = auto_fix::root_auto_fix_container::AutoFixContainer::new(
            code_analysis_aggregate.clone(),
        );
        let fix_orchestrator = auto_fix_container.orchestrator(false);

        let orphan_container =
            orphan_detector::root_orphan_detector_container::OrphanContainer::new();

        let maintenance_container =
            maintenance::root_maintenance_container::MaintenanceContainer::new();

        let lint_executor = Arc::new(
            LintExecutor::new_with_fix_and_orphan(
                code_analysis_aggregate,
                fix_orchestrator,
                orphan_container,
            )
            .with_maintenance(maintenance_container.orchestrator()),
        );

        let action_handler: Arc<dyn IActionHandlerProtocol> =
            Arc::new(ActionHandler::new(fs_adapter, lint_executor));

        let tui_aggregate: Arc<dyn ITuiAggregate> = Arc::new(TuiOrchestrator::new(action_handler));

        let surface = TuiCommandSurface::new(tui_aggregate);
        surface.run()?;

        Ok(())
    }
}
