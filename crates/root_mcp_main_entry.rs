// PURPOSE: MCP binary entry point — wiring all dependencies + rmcp stdio serve.
use mcp_server::surface_mcp_action_command::{McpActionSurface, McpServerDependencies};
use mcp_server::surface_mcp_tool_command::LintArwakyMcpServer;
use rmcp::ServiceExt;
use rmcp::transport::stdio;
use std::sync::Arc;
use tracing_subscriber::prelude::*;

fn init_tracing() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "warn".into()),
        )
        .finish()
        .with(tracing_error::ErrorLayer::default())
        .init();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_tracing();
    let filesystem: Arc<
        dyn shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate,
    > = filesystem::root_filesystem_container::FilesystemContainer::new().orchestrator();

    let config_container =
        config_system::root_config_system_container::ConfigContainer::new(filesystem.clone());
    let config_orchestrator = config_container.orchestrator();

    let code_analysis_linter =
        quality_rules::root_quality_rules_container::CodeAnalysisContainer::from_orchestrator(
            &config_orchestrator,
            ".",
        )
        .code_analysis_linter();

    let import_container =
        import_rules::root_import_rules_container::ImportContainer::from_orchestrator(
            &config_orchestrator,
            ".",
            filesystem.clone(),
        );
    let import_orchestrator = import_container.orchestrator();

    let naming_container = naming_rules::root_naming_rules_container::NamingContainer::new(
        Arc::new(config_orchestrator.load_config_sync(
            &shared::common::taxonomy_path_vo::FilePath::new(".").unwrap_or_default(),
        )),
        Arc::new(shared::common::LayerMapVO::new(
            config_orchestrator
                .load_config_sync(
                    &shared::common::taxonomy_path_vo::FilePath::new(".").unwrap_or_default(),
                )
                .layers
                .clone(),
        )),
    );
    let naming_orchestrator = naming_container.orchestrator();

    let orphan_container =
        orphan_rules::root_orphan_detector_container::OrphanContainer::from_orchestrator(
            &config_orchestrator,
            ".",
            filesystem.clone(),
        );
    let orphan_orchestrator = orphan_container.analyzer();

    let ext_container = external_lint::root_external_lint_container::ExternalLintContainer::new(
        filesystem.clone(),
    );
    let external_lint = ext_container.aggregate();

    let role_container = role_rules::root_role_rules_container::RoleContainer::new_with_config(
        config_orchestrator.load_config_sync(
            &shared::common::taxonomy_path_vo::FilePath::new(".").unwrap_or_default(),
        ),
    );
    let role_orchestrator = role_container.orchestrator();

    let auto_fix_container =
        auto_fix::root_auto_fix_container::AutoFixContainer::new(code_analysis_linter.clone());
    // BF-1: dry_run is now per-request via execute(path, dry_run), not baked into orchestrator.
    // Factory ignores the bool parameter for backwards compatibility; callers pass dry_run to execute().
    let fix_orchestrator_factory: Arc<
        dyn Fn(bool) -> Arc<dyn shared::auto_fix::LintFixOrchestratorAggregate> + Send + Sync,
    > = {
        let container = auto_fix_container;
        let fs_for_factory = filesystem.clone();
        Arc::new(move |_dry| container.orchestrator_with_filesystem(fs_for_factory.clone()))
    };

    let git_container = git_hooks::root_git_hooks_container::GitContainer::new(
        shared::common::taxonomy_path_vo::FilePath::new(".").unwrap_or_default(),
        filesystem.clone(),
    );
    let git_hooks_aggregate = git_container.aggregate();

    let maintenance_container =
        maintenance::root_maintenance_container::MaintenanceContainer::new(filesystem.clone());
    let maintenance_orchestrator = maintenance_container.orchestrator();

    let setup_container =
        project_setup::root_project_setup_container::SetupContainer::new(filesystem.clone());
    let setup_orchestrator = setup_container.aggregate();

    // DI: inject config_system parsing functions
    let deps = McpServerDependencies {
        code_analysis_linter,
        fix_orchestrator_factory,
        orphan_orchestrator,
        maintenance_orchestrator,
        git_hooks_aggregate,
        setup_orchestrator,
        config_orchestrator: config_orchestrator.clone(),
        external_lint,
        import_orchestrator,
        naming_orchestrator,
        role_orchestrator,
        filesystem,
        parse_config_yaml: config_system::utility_config_parser::parse_config_yaml,
        parse_adapter_names: config_system::utility_config_parser::parse_adapter_names_from_yaml,
        parse_score_threshold: config_system::utility_config_parser::parse_score_threshold,
    };

    let action_surface = Arc::new(McpActionSurface::new(deps));
    let server = LintArwakyMcpServer::new(action_surface);

    let (stdin, stdout) = stdio();
    let running = server.serve((stdin, stdout)).await?;
    running.waiting().await?;
    Ok(())
}
