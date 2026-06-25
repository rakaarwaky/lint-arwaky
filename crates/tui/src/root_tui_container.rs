use crate::agent_tui_orchestrator::TuiOrchestrator;
use crate::capabilities_action_handler::ActionHandler;
use crate::capabilities_lint_executor::LintExecutor;
use shared::tui::contract_tui_aggregate::ITuiAggregate;
use crate::infrastructure_file_system_adapter::FileSystemAdapter;
use crate::surface_tui_command::TuiCommandSurface;
use shared::tui::contract_action_handler_protocol::IActionHandlerProtocol;
use std::sync::Arc;

pub struct TuiContainer;

impl TuiContainer {
    pub fn run() -> anyhow::Result<()> {
        let fs_adapter = Arc::new(FileSystemAdapter::new());

        let code_analysis_container =
            code_analysis::root_code_analysis_container::CodeAnalysisContainer::new();
        let code_analysis_aggregate = code_analysis_container.code_analysis_linter();

        let lint_executor = Arc::new(LintExecutor::new(code_analysis_aggregate));

        let action_handler: Arc<dyn IActionHandlerProtocol> =
            Arc::new(ActionHandler::new(fs_adapter, lint_executor));

        let tui_aggregate: Arc<dyn ITuiAggregate> = Arc::new(TuiOrchestrator::new(action_handler));

        let surface = TuiCommandSurface::new(tui_aggregate);
        surface.run()?;

        Ok(())
    }
}
