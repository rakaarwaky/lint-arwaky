// Unit tests — MCP tool command: type accessibility.
// Note: Full tool count test (W6) requires constructing LintArwakyMcpServer
// which needs McpServerDependencies with all trait impls — deferred to integration test.

#[test]
fn mcp_tool_command_compiles() {
    let _ = std::any::type_name::<
        mcp_server_lint_arwaky::surface_mcp_tool_command::LintArwakyMcpServer,
    >();
}

#[test]
fn mcp_action_surface_compiles() {
    let _ = std::any::type_name::<
        mcp_server_lint_arwaky::surface_mcp_action_command::McpActionSurface,
    >();
}

#[test]
fn mcp_server_dependencies_struct_exists() {
    let _ = std::any::type_name::<
        mcp_server_lint_arwaky::surface_mcp_action_command::McpServerDependencies,
    >();
}
