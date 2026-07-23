// PURPOSE: Acceptance test — FRD Requirement: list_commands tool
// "list_commands — list available CLI commands with descriptions and examples."

use mcp_server_lint_arwaky::agent_mcp_server_orchestrator::{
    McpServerDependencies, McpServerOrchestrator,
};
use mcp_server_lint_arwaky::root_mcp_container::McpContainer;
use mcp_server_lint_arwaky::surface_mcp_command::LintArwakyMcpServer;
use rmcp::handler::server::wrapper::Parameters;
use shared::mcp_server::taxonomy_mcp_tool_args_vo::ListCommandsArgs;
use std::sync::Arc;

fn build_surface() -> LintArwakyMcpServer {
    let container = McpContainer::new_default();
    let deps = McpServerDependencies {
        analysis_pipeline: container.analysis_pipeline.clone(),
        external_lint: container.external_lint.clone(),
    };
    LintArwakyMcpServer::new(Arc::new(McpServerOrchestrator::new(deps)))
}

/// FRD-MCP-002: list_commands returns all commands with name, description, example
#[tokio::test]
async fn frd_mcp_002_list_commands_complete_entries() {
    let surface = build_surface();
    let args = Parameters(ListCommandsArgs { domain: None });
    let result = surface.list_commands(args).await;
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    let commands = parsed["commands"].as_array().unwrap();
    assert!(
        commands.len() >= 10,
        "Expected at least 10 commands in catalog"
    );

    for cmd in commands {
        assert!(cmd["name"].is_string(), "Each command must have a name");
        assert!(
            cmd["description"].is_string(),
            "Each command must have a description"
        );
        assert!(
            cmd["example"].is_string(),
            "Each command must have an example"
        );
        assert!(!cmd["name"].as_str().unwrap().is_empty());
        assert!(!cmd["description"].as_str().unwrap().is_empty());
    }
}

/// FRD-MCP-002: list_commands supports domain filtering
#[tokio::test]
async fn frd_mcp_002_list_commands_domain_filter() {
    let surface = build_surface();
    let args = Parameters(ListCommandsArgs {
        domain: Some("hook".to_string()),
    });
    let result = surface.list_commands(args).await;
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    let commands = parsed["commands"].as_array().unwrap();
    assert!(
        !commands.is_empty(),
        "Domain 'hook' should match install-hook/uninstall-hook"
    );
    for cmd in commands {
        assert!(cmd["name"].as_str().unwrap().contains("hook"));
    }
}

/// FRD-MCP-002: total count matches commands array length
#[tokio::test]
async fn frd_mcp_002_total_matches_array_length() {
    let surface = build_surface();
    let args = Parameters(ListCommandsArgs { domain: None });
    let result = surface.list_commands(args).await;
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    let total = parsed["total"].as_u64().unwrap();
    let commands = parsed["commands"].as_array().unwrap();
    assert_eq!(total as usize, commands.len());
}
