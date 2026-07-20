// PURPOSE: main entry point for lint-arwaky-mcp — MCP server using rmcp official SDK
//
// The MCP server runs over stdio (JSON-RPC 2.0) and exposes 5 tools:
//   execute_command, list_commands, command_schema, read_skill, health_check
//
// AI agents (Claude, etc.) connect via the MCP protocol to lint codebases,
// scan projects, and read documentation. This allows autonomous code review
// and architecture enforcement without human intervention.
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

    // Initialize the MCP DI container — wires all linter dependencies
    // (import, naming, role, orphan, code-analysis, external-lint) into
    // a single McpServerDependencies struct.
    let container = McpContainer::new_default();

    // Create the orchestrator that dispatches incoming commands to the
    // correct linter pipeline, and wrap it in the MCP server handler.
    let agent = McpServerOrchestrator::new(McpServerDependencies {
        code_analysis_linter: container.code_analysis_linter,
        import_orchestrator: container.import_orchestrator,
        naming_orchestrator: container.naming_orchestrator,
        orphan_orchestrator: container.orphan_orchestrator,
        layer_detector: container.layer_detector,
        external_lint: container.external_lint,
        role_orchestrator: container.role_orchestrator,
    });

    // Start serving the MCP protocol over standard I/O.
    // The rmcp SDK handles JSON-RPC 2.0 framing over stdin/stdout.
    let server = LintArwakyMcpServer::new(Arc::new(agent));
    let service = server.serve(rmcp::transport::stdio()).await?;
    service.waiting().await?;

    Ok(())
}
