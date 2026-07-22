// PURPOSE: Smoke test — MCP server boots and responds within 5 seconds

use std::sync::Arc;
use std::time::Instant;
use mcp_server_lint_arwaky::agent_mcp_server_orchestrator::{
    McpServerDependencies, McpServerOrchestrator,
};
use mcp_server_lint_arwaky::root_mcp_container::McpContainer;
use mcp_server_lint_arwaky::surface_mcp_command::LintArwakyMcpServer;
use shared::mcp_server::taxonomy_mcp_tool_args_vo::ExecuteCommandArgs;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::ServerHandler;

#[tokio::test]
async fn mcp_server_boots_and_responds_under_5_seconds() {
    let start = Instant::now();

    // Boot: construct container → orchestrator → surface
    let container = McpContainer::new_default();
    let deps = McpServerDependencies {
        code_analysis_linter: container.code_analysis_linter,
        import_orchestrator: container.import_orchestrator,
        naming_orchestrator: container.naming_orchestrator,
        orphan_orchestrator: container.orphan_orchestrator,
        external_lint: container.external_lint,
        role_orchestrator: container.role_orchestrator,
        config_orchestrator: container.config_orchestrator,
    };
    let orchestrator = McpServerOrchestrator::new(deps);
    let surface = LintArwakyMcpServer::new(Arc::new(orchestrator));

    // Respond: call version (lightest operation)
    let args = Parameters(ExecuteCommandArgs {
        action: "version".to_string(),
        args: None,
    });
    let result = surface.execute_command(args).await;

    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "Smoke test exceeded 5s: took {:?}",
        elapsed
    );

    // Verify response is valid
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed["name"], "lint-arwaky");
}

#[tokio::test]
async fn mcp_server_health_check_responds_under_5_seconds() {
    let start = Instant::now();

    let container = McpContainer::new_default();
    let deps = McpServerDependencies {
        code_analysis_linter: container.code_analysis_linter,
        import_orchestrator: container.import_orchestrator,
        naming_orchestrator: container.naming_orchestrator,
        orphan_orchestrator: container.orphan_orchestrator,
        external_lint: container.external_lint,
        role_orchestrator: container.role_orchestrator,
        config_orchestrator: container.config_orchestrator,
    };
    let orchestrator = McpServerOrchestrator::new(deps);
    let surface = LintArwakyMcpServer::new(Arc::new(orchestrator));

    let result = surface.health_check().await;

    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "Health check exceeded 5s: took {:?}",
        elapsed
    );

    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert!(parsed["adapters_total"].as_u64().unwrap() > 0);
}
