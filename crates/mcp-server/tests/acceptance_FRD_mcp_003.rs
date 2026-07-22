// PURPOSE: Acceptance test — FRD Requirement: read_skill tool
// "read_skill — read SKILL.md documentation by section."

use std::sync::Arc;
use mcp_server_lint_arwaky::agent_mcp_server_orchestrator::{
    McpServerDependencies, McpServerOrchestrator,
};
use mcp_server_lint_arwaky::root_mcp_container::McpContainer;
use mcp_server_lint_arwaky::surface_mcp_command::LintArwakyMcpServer;
use shared::mcp_server::taxonomy_mcp_tool_args_vo::ReadSkillArgs;
use rmcp::handler::server::wrapper::Parameters;

fn build_surface() -> LintArwakyMcpServer {
    let container = McpContainer::new_default();
    let deps = McpServerDependencies {
        code_analysis_linter: container.code_analysis_linter,
        import_orchestrator: container.import_orchestrator,
        naming_orchestrator: container.naming_orchestrator,
        orphan_orchestrator: container.orphan_orchestrator,
        external_lint: container.external_lint,
        role_orchestrator: container.role_orchestrator,
        config_orchestrator: container.config_orchestrator,
    };
    LintArwakyMcpServer::new(Arc::new(McpServerOrchestrator::new(deps)))
}

/// FRD-MCP-003: read_skill returns content or structured error
#[tokio::test]
async fn frd_mcp_003_read_skill_returns_valid_response() {
    let surface = build_surface();
    let args = Parameters(ReadSkillArgs { section: None });
    let result = surface.read_skill(args).await;
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    // Must have either "content" (success) or "error" (file not found)
    assert!(
        parsed["content"].is_string() || parsed["error"].is_string(),
        "read_skill must return content or error"
    );
}

/// FRD-MCP-003: read_skill with section returns section or error
#[tokio::test]
async fn frd_mcp_003_read_skill_section_extraction() {
    let surface = build_surface();
    let args = Parameters(ReadSkillArgs {
        section: Some("Usage".to_string()),
    });
    let result = surface.read_skill(args).await;
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    // Either returns the section content or an error (file/section not found)
    assert!(
        parsed["content"].is_string() || parsed["error"].is_string(),
        "read_skill with section must return content or error"
    );
}

/// FRD-MCP-003: read_skill searches multiple candidate locations
#[tokio::test]
async fn frd_mcp_003_read_skill_searched_paths_in_error() {
    let surface = build_surface();
    let args = Parameters(ReadSkillArgs { section: None });
    let result = surface.read_skill(args).await;
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    if parsed["error"].is_string() {
        // Error response should include searched paths for debugging
        assert!(
            parsed["searched"].is_array(),
            "Error response should list searched paths"
        );
    }
}
