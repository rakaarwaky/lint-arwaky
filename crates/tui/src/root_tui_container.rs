use crate::agent_tui_orchestrator::TuiOrchestrator;
use crate::capabilities_action_handler::ActionHandler;
use crate::capabilities_lint_executor::LintExecutor;
use crate::surface_tui_command::TuiCommandSurface;
use maintenance::root_maintenance_container::MaintenanceContainer;
use shared::tui::contract_action_handler_protocol::IActionHandlerProtocol;
use shared::tui::contract_tui_aggregate::ITuiAggregate;
use std::sync::Arc;

pub struct TuiContainer;

impl TuiContainer {
    pub fn run() -> anyhow::Result<()> {
        crate::root_logging_entry::init()?;
        tracing::info!(target = "tui", "TUI container starting");

        let import_container =
            import_rules::root_import_rules_container::ImportContainer::new_default();
        let (config, layer_map) = {
            let aes_config = shared::config_system::utility_config_defaults::default_aes_config();
            let (merged, _) =
                shared::config_system::utility_config_merger::merge_config(&aes_config);
            let mut c = aes_config;
            c.layers = merged;
            let lm = shared::taxonomy_definition_vo::LayerMapVO::new(c.layers.clone());
            (c, lm)
        };

        let code_analysis_container =
            code_analysis::root_code_analysis_container::CodeAnalysisContainer::new_with_config(
                config, layer_map,
            );
        let code_analysis_aggregate = code_analysis_container.code_analysis_linter();
        let auto_fix_container = auto_fix::root_auto_fix_container::AutoFixContainer::new(
            code_analysis_aggregate.clone(),
        );
        let fix_orchestrator = auto_fix_container.orchestrator(false);
        let setup_container = project_setup::root_project_setup_container::SetupContainer::new();
        let setup_aggregate = setup_container.aggregate();
        let hook_adapter: Arc<
            dyn shared::git_hooks::contract_manager_protocol::IHookManagerProtocol,
        > = Arc::new(git_hooks::capabilities_hook_adapter::GitHookAdapter::new(
            shared::common::taxonomy_path_vo::FilePath::new(".".to_string()).unwrap_or_default(),
        ));
        let config_container = config_system::root_config_system_container::ConfigContainer::new();
        let maintenance_container = MaintenanceContainer::new();
        let orphan_container =
            orphan_detector::root_orphan_detector_container::OrphanContainer::new();
        let external_lint_container =
            external_lint::root_external_lint_container::ExternalLintContainer::new_default();
        let naming_container =
            naming_rules::root_naming_rules_container::NamingContainer::default();
        let role_container = role_rules::root_role_rules_container::RoleContainer::new();
        let lint_executor = Arc::new(
            LintExecutor::new(code_analysis_aggregate)
                .with_fix(fix_orchestrator)
                .with_setup(setup_aggregate)
                .with_hook_port(hook_adapter)
                .with_config(config_container.orchestrator())
                .with_maintenance(maintenance_container.orchestrator())
                .with_orphan(orphan_container.analyzer())
                .with_external_lint(external_lint_container.aggregate())
                .with_import_orchestrator(import_container.orchestrator())
                .with_naming_orchestrator(naming_container.orchestrator())
                .with_role_orchestrator(role_container.orchestrator())
                .with_multi_project_orchestrator(config_container.orchestrator()),
        );
        let action_handler: Arc<dyn IActionHandlerProtocol> =
            Arc::new(ActionHandler::new(lint_executor));
        let tui_aggregate: Arc<dyn ITuiAggregate> = Arc::new(TuiOrchestrator::new(action_handler));
        let surface = TuiCommandSurface::new(tui_aggregate);
        surface.run()?;
        Ok(())
    }
}
