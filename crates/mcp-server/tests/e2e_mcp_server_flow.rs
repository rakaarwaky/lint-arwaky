// E2E tests — mcp server flow.
#[test]
fn e2e_mcp_server_compiles() {
    let _ = mcp_server_lint_arwaky::surface_mcp_tool_command::LintArwakyMcpServer;
}
