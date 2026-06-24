// PURPOSE: main entry point for lint-arwaky-mcp — MCP server using rmcp official SDK
use rmcp::ServiceExt;
use std::sync::Arc;

use mcp_server::agent_mcp_server_orchestrator::McpServerOrchestrator;
use mcp_server::root_mcp_container::McpContainer;
use mcp_server::surface_mcp_command::LintArwakyMcpServer;

pub struct McpMainEntry {}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    eprintln!("Lint Arwaky MCP Server v{}", env!("CARGO_PKG_VERSION"));
    eprintln!("Listening on stdin/stdout (JSON-RPC 2.0 via rmcp)");

    let container = McpContainer::new_default();

    let agent = McpServerOrchestrator::from_container(container);

    let server = LintArwakyMcpServer::new(Arc::new(agent));
    let service = server.serve(rmcp::transport::stdio()).await?;
    service.waiting().await?;

    Ok(())
}
