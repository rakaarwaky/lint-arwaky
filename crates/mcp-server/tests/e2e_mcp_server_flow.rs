// PURPOSE: E2E tests — full request lifecycle through all layers (Surface → Agent → Capabilities)

use mcp_server_lint_arwaky::agent_mcp_server_orchestrator::{
    McpServerDependencies, McpServerOrchestrator,
};
use mcp_server_lint_arwaky::root_mcp_container::McpContainer;
use mcp_server_lint_arwaky::surface_mcp_command::LintArwakyMcpServer;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::ServerHandler;
use shared::mcp_server::taxonomy_mcp_tool_args_vo::{
    ExecuteCommandArgs, ListCommandsArgs, ReadSkillArgs,
};
use std::sync::Arc;

fn build_full_stack() -> LintArwakyMcpServer {
    let container = McpContainer::new_default();
    let deps = McpServerDependencies {
        analysis_pipeline: container.analysis_pipeline.clone(),
        external_lint: container.external_lint.clone(),
    };
    let orchestrator = McpServerOrchestrator::new(deps);
    LintArwakyMcpServer::new(Arc::new(orchestrator))
}

// ─── E2E: Full scan lifecycle ────────────────────────────────────────

#[tokio::test]
async fn e2e_scan_current_directory_returns_compliance_report() {
    let surface = build_full_stack();

    let args = Parameters(ExecuteCommandArgs {
        action: "scan".to_string(),
        args: Some(serde_json::json!({"path": "."})),
    });
    let result = surface.execute_command(args).await;
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["status"], "success");
    assert_eq!(parsed["action"], "scan");
    assert!(parsed["total_violations"].is_number());
    assert!(parsed["results"].is_array());
}

// ─── E2E: CI gate lifecycle ──────────────────────────────────────────

#[tokio::test]
async fn e2e_ci_with_threshold_returns_pass_or_fail() {
    let surface = build_full_stack();

    let args = Parameters(ExecuteCommandArgs {
        action: "ci".to_string(),
        args: Some(serde_json::json!({"path": ".", "threshold": 0})),
    });
    let result = surface.execute_command(args).await;
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert!(parsed["status"] == "pass" || parsed["status"] == "fail");
    assert_eq!(parsed["threshold"], 0);
}

// ─── E2E: Doctor lifecycle ───────────────────────────────────────────

#[tokio::test]
async fn e2e_doctor_checks_all_expected_tools() {
    let surface = build_full_stack();

    let args = Parameters(ExecuteCommandArgs {
        action: "doctor".to_string(),
        args: None,
    });
    let result = surface.execute_command(args).await;
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert_eq!(parsed["status"], "success");
    let checks = parsed["checks"].as_array().unwrap();
    let tool_names: Vec<&str> = checks.iter().map(|c| c["tool"].as_str().unwrap()).collect();
    assert!(tool_names.contains(&"cargo"));
    assert!(tool_names.contains(&"git"));
    assert!(tool_names.contains(&"node"));
}

// ─── E2E: List commands → execute one ────────────────────────────────

#[tokio::test]
async fn e2e_discover_then_execute_command() {
    let surface = build_full_stack();

    // Step 1: Discover commands
    let list_args = Parameters(ListCommandsArgs { domain: None });
    let list_result = surface.list_commands(list_args).await;
    let list_parsed: serde_json::Value = serde_json::from_str(&list_result).unwrap();
    let commands = list_parsed["commands"].as_array().unwrap();
    assert!(!commands.is_empty());

    // Step 2: Pick "version" and execute it
    let version_cmd = commands
        .iter()
        .find(|c| c["name"] == "version")
        .expect("version command must exist in catalog");
    assert!(version_cmd["description"].is_string());
    assert!(version_cmd["example"].is_string());

    let exec_args = Parameters(ExecuteCommandArgs {
        action: "version".to_string(),
        args: None,
    });
    let exec_result = surface.execute_command(exec_args).await;
    let exec_parsed: serde_json::Value = serde_json::from_str(&exec_result).unwrap();
    assert_eq!(exec_parsed["name"], "lint-arwaky");
}

// ─── E2E: Health check full lifecycle ────────────────────────────────

#[tokio::test]
async fn e2e_health_check_reports_adapter_status() {
    let surface = build_full_stack();
    let result = surface.health_check().await;
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert!(parsed["version"].is_string());
    assert_eq!(parsed["adapters_total"], 5);

    let adapters = parsed["adapters"].as_array().unwrap();
    for adapter in adapters {
        assert!(adapter["name"].is_string());
        assert!(adapter["language"].is_string());
        let status = adapter["status"].as_str().unwrap();
        assert!(
            status == "available" || status == "not_installed",
            "Unexpected adapter status: {}",
            status
        );
    }
}

// ─── E2E: Adapters listing ───────────────────────────────────────────

#[tokio::test]
async fn e2e_adapters_command_returns_enabled_status() {
    let surface = build_full_stack();

    let args = Parameters(ExecuteCommandArgs {
        action: "adapters".to_string(),
        args: None,
    });
    let result = surface.execute_command(args).await;
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert!(parsed["adapters"].is_array());
    let adapters = parsed["adapters"].as_array().unwrap();
    for adapter in adapters {
        assert!(adapter["name"].is_string());
        assert!(adapter["enabled"].is_boolean());
    }
}
