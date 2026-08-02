use crate::agent_tui_orchestrator::TuiOrchestrator;
use crate::capabilities_action_handler::ActionHandler;
use crate::surface_tui_command::TuiCommandSurface;
use shared::tui::{IActionHandlerProtocol, ILintExecutorProtocol, ITuiAggregate};

use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use std::sync::Arc;

// PURPOSE: Root-layer TUI container — composition root that wires all dependencies.
// Receives pre-built aggregates via DI (no direct crate imports for concrete types).

pub struct TuiContainer;

impl TuiContainer {
    /// Run the TUI application with injected dependencies.
    /// The caller (binary entry point) is responsible for creating all concrete
    /// containers and passing the aggregates here.
    pub fn run(
        lint_executor: Arc<dyn ILintExecutorProtocol>,
        filesystem: Arc<dyn IFilesystemAggregate>,
    ) -> anyhow::Result<()> {
        crate::surface_logging_controller::init()?;
        tracing::info!(target = "tui", "TUI container starting");

        let action_handler: Arc<dyn IActionHandlerProtocol> =
            Arc::new(ActionHandler::new(lint_executor, filesystem));
        let tui_aggregate: Arc<dyn ITuiAggregate> = Arc::new(TuiOrchestrator::new(action_handler));
        let surface = TuiCommandSurface::new(tui_aggregate);
        surface.run()?;
        Ok(())
    }
}
