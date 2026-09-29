// PURPOSE: Surface-layer TUI orchestrator — coordinates event handling, lint execution,
// and view rendering for the terminal UI. Injects two protocol seams:
//   1. IFileSystemIOProtocol — reads directories and file contents for the file list
//   2. SurfaceLintExecutor — runs all lint actions (check, scan, fix, ci, orphan, etc.)
// The orchestrator delegates state-machine logic to SurfaceActionHandler and terminal
// rendering to TuiCommandSurface. It is the single entry point that wires both seams
// into a running TUI session.
use crate::surface_event_action::SurfaceActionHandler;
use crate::surface_lint_action::SurfaceLintExecutor;
use crate::surface_tui_command::TuiCommandSurface;
use shared::filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use std::sync::Arc;

/// Constructs and runs a TUI session by wiring the event handler, lint executor,
/// and event-loop surface together. This is the agent-layer orchestrator for
/// the TUI crate; it injects two protocol seams to satisfy AES405.
pub struct TuiOrchestrator {
    action_handler: Arc<SurfaceActionHandler>,
}

impl TuiOrchestrator {
    /// Build the orchestrator from two protocol seams:
    /// `io` — filesystem read access for directory listings and file previews.
    /// `lint_executor` — lint action execution bridge (check/scan/fix/ci/orphan/…).
    pub fn new(
        lint_executor: Arc<SurfaceLintExecutor>,
        io: Arc<dyn IFileSystemIOProtocol>,
    ) -> Self {
        let action_handler = Arc::new(SurfaceActionHandler::new(lint_executor, io));
        Self { action_handler }
    }

    /// Run the TUI event loop. Blocks until the user quits.
    pub fn run(&self) -> anyhow::Result<()> {
        let surface = TuiCommandSurface::new(self.action_handler.clone());
        crate::surface_logging_controller::init()?;
        tracing::info!(target = "tui", "TUI orchestrator starting");
        surface.run()?;
        Ok(())
    }
}
