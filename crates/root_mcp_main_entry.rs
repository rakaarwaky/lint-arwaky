// PURPOSE: MCP binary entry point — wiring MCP-specific deps + rmcp stdio serve.
use lint_arwaky::root_entry_container::CommonDeps;
use mcp_server::surface_mcp_action_command::{McpActionSurface, McpServerDependencies};
use mcp_server::surface_mcp_tool_command::LintArwakyMcpServer;
use rmcp::ServiceExt;
use rmcp::transport::stdio;
use shared::config_system::utility_config_parser::parse_config_yaml;
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
    let deps = CommonDeps::build();

    let mcp_deps = McpServerDependencies {
        code_analysis_linter: deps.code_analysis_linter,
        fix_orchestrator_factory: deps.fix_orchestrator_factory,
        orphan_orchestrator: deps.orphan_orchestrator,
        maintenance_orchestrator: deps.maintenance_orchestrator,
        git_hooks_aggregate: deps.git_hooks_aggregate,
        setup_orchestrator: deps.setup_orchestrator,
        config_orchestrator: deps.config_orchestrator,
        external_lint: deps.external_lint,
        import_orchestrator: deps.import_orchestrator,
        naming_orchestrator: deps.naming_orchestrator,
        role_orchestrator: deps.role_orchestrator,
        filesystem: deps.filesystem,
        fs_factory: deps.fs_factory,
        orphan_factory: deps.orphan_factory,
        parse_config_yaml,
        parse_adapter_names: config_system::utility_config_parser::parse_adapter_names_from_yaml,
        parse_score_threshold: config_system::utility_config_parser::parse_score_threshold,
    };

    let action_surface = Arc::new(McpActionSurface::new(mcp_deps));
    let server = LintArwakyMcpServer::new(action_surface);

    let (stdin, stdout) = stdio();
    let running = server.serve((stdin, stdout)).await?;
    running.waiting().await?;
    Ok(())
}
