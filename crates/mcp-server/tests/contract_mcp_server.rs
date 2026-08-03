// Contract tests — verify mcp-server modules compile and are accessible.
#[test]
fn mcp_action_command_module_exists() {
    let _ = std::any::type_name::<
        mcp_server_lint_arwaky::surface_mcp_action_command::McpActionSurface,
    >();
}

#[test]
fn mcp_tool_command_module_exists() {
    let _ =
        std::any::type_name::<mcp_server_lint_arwaky::surface_mcp_tool_command::LintArwakyMcpServer>();
}
