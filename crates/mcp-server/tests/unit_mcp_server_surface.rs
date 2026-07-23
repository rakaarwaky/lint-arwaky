// PURPOSE: Unit tests for LintArwakyMcpServer — tool registration, get_info, health_check

use mcp_server_lint_arwaky::surface_mcp_command::LintArwakyMcpServer;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::ServerHandler;
use shared::mcp_server::contract_mcp_server_aggregate::IMcpServerAggregate;
use shared::mcp_server::taxonomy_mcp_tool_args_vo::{
    ExecuteCommandArgs, ListCommandsArgs, ReadSkillArgs,
};
use std::sync::Arc;

// ─── Mock Aggregate ──────────────────────────────────────────────────

struct StubAggregate;

#[async_trait::async_trait]
impl IMcpServerAggregate for StubAggregate {
    async fn execute_command(&self, _args: Parameters<ExecuteCommandArgs>) -> String {
        r#"{"status":"stub"}"#.to_string()
    }
    async fn list_commands(&self, _args: Parameters<ListCommandsArgs>) -> String {
        r#"{"commands":[]}"#.to_string()
    }
    async fn read_skill(&self, _args: Parameters<ReadSkillArgs>) -> String {
        r#"{"content":"stub"}"#.to_string()
    }
}

fn build_surface() -> LintArwakyMcpServer {
    LintArwakyMcpServer::new(Arc::new(StubAggregate))
}

// ─── get_info ────────────────────────────────────────────────────────

#[test]
fn get_info_returns_correct_server_name() {
    let sut = build_surface();
    let info = sut.get_info();
    assert_eq!(info.server_info.name, "lint-arwaky");
}

#[test]
fn get_info_returns_version_string() {
    let sut = build_surface();
    let info = sut.get_info();
    assert!(!info.server_info.version.is_empty());
}

#[test]
fn get_info_declares_tools_capability() {
    let sut = build_surface();
    let info = sut.get_info();
    assert!(info.capabilities.tools.is_some());
}

// ─── Tool delegation ─────────────────────────────────────────────────

#[tokio::test]
async fn execute_command_delegates_to_agent() {
    let sut = build_surface();
    let args = Parameters(ExecuteCommandArgs {
        action: "version".to_string(),
        args: None,
    });
    let result = sut.execute_command(args).await;
    assert!(result.contains("stub"));
}

#[tokio::test]
async fn list_commands_delegates_to_agent() {
    let sut = build_surface();
    let args = Parameters(ListCommandsArgs { domain: None });
    let result = sut.list_commands(args).await;
    assert!(result.contains("commands"));
}

#[tokio::test]
async fn read_skill_delegates_to_agent() {
    let sut = build_surface();
    let args = Parameters(ReadSkillArgs { section: None });
    let result = sut.read_skill(args).await;
    assert!(result.contains("content"));
}

// ─── health_check ────────────────────────────────────────────────────

#[tokio::test]
async fn health_check_returns_valid_json() {
    let sut = build_surface();
    let result = sut.health_check().await;
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert!(parsed["version"].is_string());
    assert!(parsed["adapters"].is_array());
    assert!(parsed["adapters_available"].is_number());
    assert!(parsed["adapters_total"].is_number());
}

#[tokio::test]
async fn health_check_lists_expected_adapters() {
    let sut = build_surface();
    let result = sut.health_check().await;
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    let adapters = parsed["adapters"].as_array().unwrap();
    let names: Vec<&str> = adapters
        .iter()
        .map(|a| a["name"].as_str().unwrap())
        .collect();
    assert!(names.contains(&"ruff"));
    assert!(names.contains(&"mypy"));
    assert!(names.contains(&"clippy"));
    assert!(names.contains(&"eslint"));
    assert!(names.contains(&"bandit"));
}

#[tokio::test]
async fn health_check_adapters_total_is_five() {
    let sut = build_surface();
    let result = sut.health_check().await;
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    assert_eq!(parsed["adapters_total"], 5);
}

// ─── Clone ───────────────────────────────────────────────────────────

#[test]
fn surface_is_cloneable() {
    let sut = build_surface();
    let _cloned = sut.clone();
}
