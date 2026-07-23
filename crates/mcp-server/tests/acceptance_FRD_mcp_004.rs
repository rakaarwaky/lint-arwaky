// PURPOSE: Acceptance test — FRD Requirement: health_check tool + JSON-RPC conformance
// "health_check — check system health: adapters and system state."
// "JSON-RPC conformance; tool discovery by AI clients."

use mcp_server_lint_arwaky::agent_mcp_server_orchestrator::{
    McpServerDependencies, McpServerOrchestrator,
};
use mcp_server_lint_arwaky::root_mcp_container::McpContainer;
use mcp_server_lint_arwaky::surface_mcp_command::LintArwakyMcpServer;
use rmcp::ServerHandler;
use std::sync::Arc;

fn build_surface() -> LintArwakyMcpServer {
    let container = McpContainer::new_default();
    let deps = McpServerDependencies {
        analysis_pipeline: container.analysis_pipeline.clone(),
        external_lint: container.external_lint.clone(),
    };
    LintArwakyMcpServer::new(Arc::new(McpServerOrchestrator::new(deps)))
}

/// FRD-MCP-004: health_check returns adapter availability
#[tokio::test]
async fn frd_mcp_004_health_check_adapter_status() {
    let surface = build_surface();
    let result = surface.health_check().await;
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert!(parsed["version"].is_string());
    assert!(parsed["adapters_available"].is_number());
    assert!(parsed["adapters_total"].is_number());

    let total = parsed["adapters_total"].as_u64().unwrap();
    let available = parsed["adapters_available"].as_u64().unwrap();
    assert!(available <= total, "available cannot exceed total");
}

/// FRD-MCP-004: Tool discovery — ServerInfo declares tools capability
#[test]
fn frd_mcp_004_tool_discovery_capabilities() {
    let surface = build_surface();
    let info = surface.get_info();

    // MCP clients discover tools via capabilities.tools
    assert!(
        info.capabilities.tools.is_some(),
        "Server must declare tools capability for AI client discovery"
    );
}

/// FRD-MCP-004: Server identity is correct for JSON-RPC handshake
#[test]
fn frd_mcp_004_server_identity_for_jsonrpc() {
    let surface = build_surface();
    let info = surface.get_info();

    assert_eq!(info.server_info.name, "lint-arwaky");
    assert!(!info.server_info.version.is_empty());
    // Protocol version must be set for JSON-RPC conformance
    assert!(!format!("{:?}", info.protocol_version).is_empty());
}

/// FRD-MCP-004: Response time under 5 seconds for standard operations
#[tokio::test]
async fn frd_mcp_004_response_time_under_5_seconds() {
    let surface = build_surface();
    let start = std::time::Instant::now();

    let _ = surface.health_check().await;

    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "health_check took {:?}, exceeds 5s SLA",
        elapsed
    );
}
