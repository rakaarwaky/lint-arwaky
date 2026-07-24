// PURPOSE: Acceptance test — FRD Requirement: execute_command tool
// "execute_command — execute any CLI command via the MCP interface."

use mcp_server_lint_arwaky::agent_mcp_server_orchestrator::{
    McpServerDependencies, McpServerOrchestrator,
};
use mcp_server_lint_arwaky::root_mcp_container::McpContainer;
use mcp_server_lint_arwaky::surface_mcp_command::LintArwakyMcpServer;
use rmcp::handler::server::wrapper::Parameters;
use shared::mcp_server::taxonomy_mcp_tool_args_vo::ExecuteCommandArgs;
use std::sync::Arc;

fn build_surface() -> LintArwakyMcpServer {
    let container = McpContainer::new_default();
    let deps = McpServerDependencies {
        external_lint: container.external_lint.clone(),
    };
    LintArwakyMcpServer::new(Arc::new(McpServerOrchestrator::new(deps)))
}

/// FRD-MCP-001: execute_command accepts any valid CLI action and returns JSON
#[tokio::test]
async fn frd_mcp_001_execute_command_scan() {
    let surface = build_surface();
    let args = Parameters(ExecuteCommandArgs {
        action: "scan".to_string(),
        args: Some(serde_json::json!({"path": "."})),
    });
    let result = surface.execute_command(args).await;
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed["status"], "success");
    assert_eq!(parsed["action"], "scan");
}

/// FRD-MCP-001: execute_command handles check alias
#[tokio::test]
async fn frd_mcp_001_execute_command_check_alias() {
    let surface = build_surface();
    let args = Parameters(ExecuteCommandArgs {
        action: "check".to_string(),
        args: Some(serde_json::json!({"path": "."})),
    });
    let result = surface.execute_command(args).await;
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed["status"], "success");
}

/// FRD-MCP-001: execute_command rejects unknown actions gracefully
#[tokio::test]
async fn frd_mcp_001_execute_command_unknown_action_error() {
    let surface = build_surface();
    let args = Parameters(ExecuteCommandArgs {
        action: "invalid_xyz".to_string(),
        args: None,
    });
    let result = surface.execute_command(args).await;
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert!(parsed["error"].is_string());
}

/// FRD-MCP-001: execute_command supports all documented actions
#[tokio::test]
async fn frd_mcp_001_all_documented_actions_accepted() {
    let surface = build_surface();
    let actions = vec![
        "check",
        "scan",
        "fix",
        "ci",
        "doctor",
        "orphan",
        "security",
        "dependencies",
        "version",
        "adapters",
        "install-hook",
        "uninstall-hook",
        "init",
        "install",
        "mcp-config",
        "config-show",
    ];
    for action in actions {
        let args = Parameters(ExecuteCommandArgs {
            action: action.to_string(),
            args: Some(serde_json::json!({"path": "."})),
        });
        let result = surface.execute_command(args).await;
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert!(
            !parsed
                .get("error")
                .is_some_and(|e| { e.as_str().is_some_and(|s| s.contains("Unknown action")) }),
            "Action '{}' should be recognized",
            action
        );
    }
}
