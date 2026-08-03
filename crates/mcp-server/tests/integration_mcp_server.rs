// Integration tests — mcp-server with real dependencies.
#[test]
fn mcp_server_container_creates() {
    let _ = mcp_server_lint_arwaky::surface_mcp_action_command::McpActionSurface;
}
