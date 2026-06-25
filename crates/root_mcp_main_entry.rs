// PURPOSE: main entry point for lint-arwaky-mcp — MCP server using rmcp official SDK
use rmcp::ServiceExt;
use std::sync::Arc;

use mcp_server::agent_mcp_server_orchestrator::{McpServerDependencies, McpServerOrchestrator};
use mcp_server::root_mcp_container::McpContainer;
use mcp_server::surface_mcp_command::LintArwakyMcpServer;

pub struct McpMainEntry {}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    eprintln!("Lint Arwaky MCP Server v{}", env!("CARGO_PKG_VERSION"));
    eprintln!("Listening on stdin/stdout (JSON-RPC 2.0 via rmcp)");

    let container = McpContainer::new_default();

    let agent = McpServerOrchestrator::new(McpServerDependencies {
        code_analysis_linter: container.code_analysis_linter,
        import_orchestrator: container.import_orchestrator,
        naming_orchestrator: container.naming_orchestrator,
        orphan_orchestrator: container.orphan_orchestrator,
        layer_detector: container.layer_detector,
        scanner_provider: container.scanner_provider,
        external_lint: container.external_lint,
        role_orchestrator: container.role_orchestrator,
    });

    let server = LintArwakyMcpServer::new(Arc::new(agent));
    let service = server.serve(rmcp::transport::stdio()).await?;
    service.waiting().await?;

    Ok(())
}
