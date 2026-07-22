// PURPOSE: Verify that McpServerOrchestrator implements IMcpServerAggregate
//          and that LintArwakyMcpServer implements ServerHandler.

use mcp_server_lint_arwaky::agent_mcp_server_orchestrator::McpServerOrchestrator;
use mcp_server_lint_arwaky::surface_mcp_command::LintArwakyMcpServer;
use shared::mcp_server::contract_mcp_server_aggregate::IMcpServerAggregate;

// ─── Contract: McpServerOrchestrator implements IMcpServerAggregate ───

#[test]
fn mcp_server_orchestrator_implements_aggregate_trait() {
    fn assert_trait<T: IMcpServerAggregate>() {}
    assert_trait::<McpServerOrchestrator>();
}

#[test]
fn mcp_server_orchestrator_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<McpServerOrchestrator>();
}

// ─── Contract: LintArwakyMcpServer implements ServerHandler ───

#[test]
fn lint_arwaky_mcp_server_implements_server_handler() {
    fn assert_trait<T: rmcp::ServerHandler>() {}
    assert_trait::<LintArwakyMcpServer>();
}

#[test]
fn lint_arwaky_mcp_server_is_clone() {
    fn assert_clone<T: Clone>() {}
    assert_clone::<LintArwakyMcpServer>();
}

#[test]
fn lint_arwaky_mcp_server_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<LintArwakyMcpServer>();
}

// ─── Contract: McpContainer is constructible ───

#[test]
fn mcp_container_struct_is_public() {
    use mcp_server_lint_arwaky::root_mcp_container::McpContainer;
    fn assert_sized<T: Sized>() {}
    assert_sized::<McpContainer>();
}
