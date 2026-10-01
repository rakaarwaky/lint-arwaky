// Contract tests — MCP protocol surface (QA #641).
//
// Pins the externally-visible contract an out-of-repo MCP client (Claude
// Desktop, VS Code, Hermes) depends on: server identity, the exact 5-tool
// surface, and the tool-argument schema boundary for untrusted JSON input.
mod common;

use mcp_server_lint_arwaky::surface_mcp_tool_command::LintArwakyMcpServer;
use rmcp::ServerHandler;
use shared_mcp_server::{ExecuteCommandArgs, GetConfigArgs, ListCommandsArgs, ReadSkillArgs};
use std::sync::Arc;

fn server() -> LintArwakyMcpServer {
    LintArwakyMcpServer::new(Arc::new(common::make_action_surface()))
}

// ─── Protocol contract: server identity + capabilities ─────────────

#[test]
fn get_info_advertises_lint_arwaky_identity() {
    let info = server().get_info();
    assert_eq!(info.server_info.name, "lint-arwaky");
    assert!(!info.server_info.version.is_empty(), "server must advertise a version");
}

#[test]
fn get_info_advertises_tools_capability() {
    let info = server().get_info();
    assert!(
        info.capabilities.tools.is_some(),
        "tools capability must be advertised for tool calls to be possible"
    );
}

// ─── Protocol contract: exactly the documented 5 tools ─────────────

#[test]
fn tool_router_registers_exactly_the_documented_five_tools() {
    // TEST.md §5.3 promises exactly 5 tools: execute_command, list_commands,
    // read_skill, health_check, get_config.
    let mut names: Vec<String> = server()
        .router()
        .list_all()
        .iter()
        .map(|t| t.name.to_string())
        .collect();
    names.sort();
    assert_eq!(
        names,
        vec![
            "execute_command",
            "get_config",
            "health_check",
            "list_commands",
            "read_skill"
        ],
        "MCP tool surface drifted from the documented 5-tool contract"
    );
}

// ─── Argument schema boundary (untrusted client JSON) ──────────────

#[test]
fn execute_command_args_requires_action() {
    let missing_action: Result<ExecuteCommandArgs, _> =
        serde_json::from_value(serde_json::json!({}));
    assert!(
        missing_action.is_err(),
        "execute_command without `action` must be rejected at the schema boundary"
    );

    let wrong_type: Result<ExecuteCommandArgs, _> =
        serde_json::from_value(serde_json::json!({ "action": 42 }));
    assert!(wrong_type.is_err(), "non-string `action` must be rejected at the schema boundary");
}

#[test]
fn execute_command_args_accepts_minimal_and_full_shapes() {
    let minimal: ExecuteCommandArgs =
        serde_json::from_value(serde_json::json!({ "action": "scan" }))
            .expect("minimal args must deserialize");
    assert_eq!(minimal.action, "scan");
    assert!(minimal.args.is_none());

    let full: ExecuteCommandArgs = serde_json::from_value(
        serde_json::json!({ "action": "ci", "args": { "path": "/tmp/proj", "threshold": 90 } }),
    )
    .expect("full args must deserialize");
    assert_eq!(full.action, "ci");
    assert_eq!(full.args.as_ref().unwrap()["path"], "/tmp/proj");
}

#[test]
fn optional_arg_structs_default_every_field() {
    let list: ListCommandsArgs = serde_json::from_value(serde_json::json!({}))
        .expect("empty list_commands args must deserialize");
    assert!(list.domain.is_none());

    let read: ReadSkillArgs = serde_json::from_value(serde_json::json!({}))
        .expect("empty read_skill args must deserialize");
    assert!(read.section.is_none());

    let get: GetConfigArgs = serde_json::from_value(serde_json::json!({}))
        .expect("empty get_config args must deserialize");
    assert!(get.path.is_none());
    assert!(get.language.is_none());
}
