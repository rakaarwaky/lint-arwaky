// PURPOSE: TuiContainer — DI container for the TUI crate (root layer)
use crate::agent_tui_orchestrator::TuiOrchestrator;
use crate::capabilities_action_handler::ActionHandler;
use crate::capabilities_lint_executor::LintExecutor;
use crate::infrastructure_file_system_adapter::FileSystemAdapter;
use crate::surface_tui_command::TuiCommandSurface;
use shared::common::taxonomy_path_vo::FilePath;
use shared::git_hooks::contract_manager_port::IHookManagerPort;
use shared::tui::contract_action_handler_protocol::IActionHandlerProtocol;
use shared::tui::contract_tui_aggregate::ITuiAggregate;
use std::sync::Arc;

pub struct TuiContainer;

impl TuiContainer {
    pub fn run() -> anyhow::Result<()> {
        let fs_adapter = Arc::new(FileSystemAdapter::new());
        let code_analysis_container = code_analysis::root_code_analysis_container::CodeAnalysisContainer::new();
        let code_analysis_aggregate = code_analysis_container.code_analysis_linter();
        let auto_fix_container = auto_fix::root_auto_fix_container::AutoFixContainer::new(code_analysis_aggregate.clone());
        let fix_orchestrator = auto_fix_container.orchestrator(false);
        let orphan_container = orphan_detector::root_orphan_detector_container::OrphanContainer::new();
        let maintenance_container = maintenance::root_maintenance_container::MaintenanceContainer::new();
        let setup_container = project_setup::root_project_setup_container::SetupContainer::new();
        let hook_adapter: Arc<dyn IHookManagerPort> = Arc::new(git_hooks::infrastructure_hook_adapter::GitHookAdapter::new(FilePath::new(".".to_string()).unwrap_or_default()));

        let lint_executor = Arc::new(
            LintExecutor::new_with_fix_and_orphan(code_analysis_aggregate, fix_orchestrator, orphan_container)
                .with_maintenance(maintenance_container.orchestrator())
                .with_hook_port(hook_adapter)
                .with_setup(setup_container.aggregate()),
        );

        let action_handler: Arc<dyn IActionHandlerProtocol> = Arc::new(ActionHandler::new(fs_adapter, lint_executor));
        let tui_aggregate: Arc<dyn ITuiAggregate> = Arc::new(TuiOrchestrator::new(action_handler));
        let surface = TuiCommandSurface::new(tui_aggregate);
        surface.run()?;
        Ok(())
    }
}
