// PURPOSE: Integration tests — real DI container wiring, orchestrator + surface composition

use mcp_server_lint_arwaky::agent_mcp_server_orchestrator::{
    McpServerDependencies, McpServerOrchestrator,
};
use mcp_server_lint_arwaky::root_mcp_container::McpContainer;
use mcp_server_lint_arwaky::surface_mcp_command::LintArwakyMcpServer;
use rmcp::handler::server::wrapper::Parameters;
use shared::mcp_server::taxonomy_mcp_tool_args_vo::{ExecuteCommandArgs, ListCommandsArgs};
use std::sync::Arc;

// ─── DI Container Wiring ─────────────────────────────────────────────

#[test]
fn container_new_default_produces_all_dependencies() {
    // This test requires the real filesystem and config system.
    // It validates that McpContainer::new_default() wires all 7 dependencies.
    let container = McpContainer::new_default();

    // All Arc<dyn Trait> fields must be non-null (they always are with Arc,
    // but we verify the container doesn't panic during construction).
    assert!(Arc::strong_count(&container.code_analysis_linter) >= 1);
    assert!(Arc::strong_count(&container.import_orchestrator) >= 1);
    assert!(Arc::strong_count(&container.naming_orchestrator) >= 1);
    assert!(Arc::strong_count(&container.orphan_orchestrator) >= 1);
    assert!(Arc::strong_count(&container.external_lint) >= 1);
    assert!(Arc::strong_count(&container.role_orchestrator) >= 1);
    assert!(Arc::strong_count(&container.config_orchestrator) >= 1);
}

// ─── Orchestrator from Container ─────────────────────────────────────

#[test]
fn orchestrator_constructed_from_container_deps() {
    let container = McpContainer::new_default();
    let deps = McpServerDependencies {
        analysis_pipeline: container.analysis_pipeline.clone(),
        external_lint: container.external_lint.clone(),
    };
    let orchestrator = McpServerOrchestrator::new(deps);
    let _surface = LintArwakyMcpServer::new(Arc::new(orchestrator));
}

// ─── Full Pipeline: Surface → Agent → Mock Capabilities ─────────────

#[tokio::test]
async fn surface_execute_command_flows_through_orchestrator() {
    let container = McpContainer::new_default();
    let deps = McpServerDependencies {
        analysis_pipeline: container.analysis_pipeline.clone(),
        external_lint: container.external_lint.clone(),
    };
    let orchestrator = McpServerOrchestrator::new(deps);
    let surface = LintArwakyMcpServer::new(Arc::new(orchestrator));

    let args = Parameters(ExecuteCommandArgs {
        action: "version".to_string(),
        args: None,
    });
    let result = surface.execute_command(args).await;
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed["name"], "lint-arwaky");
}

#[tokio::test]
async fn surface_list_commands_returns_catalog() {
    let container = McpContainer::new_default();
    let deps = McpServerDependencies {
        analysis_pipeline: container.analysis_pipeline.clone(),
        external_lint: container.external_lint.clone(),
    };
    let orchestrator = McpServerOrchestrator::new(deps);
    let surface = LintArwakyMcpServer::new(Arc::new(orchestrator));

    let args = Parameters(ListCommandsArgs { domain: None });
    let result = surface.list_commands(args).await;
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert!(parsed["total"].as_u64().unwrap() >= 10);
}

// ─── Server Info via Surface ─────────────────────────────────────────

#[test]
fn surface_get_info_reports_tools_capability() {
    let container = McpContainer::new_default();
    let deps = McpServerDependencies {
        analysis_pipeline: container.analysis_pipeline.clone(),
        external_lint: container.external_lint.clone(),
    };
    let orchestrator = McpServerOrchestrator::new(deps);
    let surface = LintArwakyMcpServer::new(Arc::new(orchestrator));
    let info = surface.get_info();
    assert!(info.capabilities.tools.is_some());
    assert_eq!(info.server_info.name, "lint-arwaky");
}
