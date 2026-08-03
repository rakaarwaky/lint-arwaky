// Unit tests — MCP tool command.
#[test]
fn mcp_tool_command_compiles() {
    let _ = std::any::type_name::<
        mcp_server_lint_arwaky::surface_mcp_tool_command::LintArwakyMcpServer,
    >();
}
