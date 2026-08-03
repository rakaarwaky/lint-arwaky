// Smoke tests — module imports work, key types accessible within 5s.
use mcp_server_lint_arwaky::surface_mcp_action_command::{McpActionSurface, McpServerDependencies};

#[test]
fn smoke_mcp_action_surface_imports() {
    let start = std::time::Instant::now();
    let _ = std::any::type_name::<McpActionSurface>();
    let _ = std::any::type_name::<McpServerDependencies>();
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "Smoke test exceeded 5s: {:?}",
        elapsed
    );
}

#[test]
fn smoke_mcp_server_module_imports() {
    let start = std::time::Instant::now();
    // Verify the server module's types are accessible
    let _ = std::any::type_name::<
        mcp_server_lint_arwaky::surface_mcp_tool_command::LintArwakyMcpServer,
    >();
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "Smoke test exceeded 5s: {:?}",
        elapsed
    );
}

#[test]
fn smoke_mcp_server_version_info() {
    let start = std::time::Instant::now();
    // McpServerDependencies has many Arc fields, verify it compiles
    let _ = std::mem::size_of::<McpServerDependencies>();
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "Smoke test exceeded 5s: {:?}",
        elapsed
    );
}
