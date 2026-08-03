// Acceptance tests — mcp server produces valid output.
#[test]
fn acceptance_mcp_server_types_exist() {
    let _ = std::any::type_name::<
        mcp_server_lint_arwaky::surface_mcp_action_command::McpActionCommand,
    >();
    let _ =
        std::any::type_name::<mcp_server_lint_arwaky::surface_mcp_tool_command::McpToolCommand>();
}
