use crate::surface_event_action::SurfaceActionHandler;
use crate::surface_lint_action::SurfaceLintExecutor;
use crate::surface_tui_command::TuiCommandSurface;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use std::sync::Arc;

// PURPOSE: Root-layer TUI container — composition root wiring surfaces directly.
// Receives pre-built domain aggregates via DI (no contract/aggregate/capabilities layers).

pub struct TuiContainer;

impl TuiContainer {
    /// Run the TUI application with injected dependencies.
    /// The caller (binary entry point) is responsible for creating all concrete
    /// containers and passing the aggregates here.
    pub fn run(
        lint_executor: Arc<SurfaceLintExecutor>,
        filesystem: Arc<dyn IFilesystemAggregate>,
    ) -> anyhow::Result<()> {
        crate::surface_logging_controller::init()?;
        tracing::info!(target = "tui", "TUI container starting");

        let action_handler = Arc::new(SurfaceActionHandler::new(lint_executor, filesystem));
        let surface = TuiCommandSurface::new(action_handler);
        surface.run()?;
        Ok(())
    }
}
