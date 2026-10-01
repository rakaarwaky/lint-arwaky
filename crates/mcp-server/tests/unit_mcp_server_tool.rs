// Unit tests — MCP tool surface: handler wiring with real deps (QA #641).
//
// Verifies `LintArwakyMcpServer` handlers map MCP `Parameters<Args>` into the
// action surface correctly, with assertion depth on every JSON field — no
// type-existence-only checks.
mod common;

use mcp_server_lint_arwaky::surface_mcp_tool_command::LintArwakyMcpServer;
use rmcp::handler::server::wrapper::Parameters;
use shared_mcp_server::{ExecuteCommandArgs, GetConfigArgs, ListCommandsArgs, ReadSkillArgs};
use std::sync::Arc;

fn server() -> LintArwakyMcpServer {
    LintArwakyMcpServer::new(Arc::new(common::make_action_surface()))
}

fn parse(json: &str) -> serde_json::Value {
    serde_json::from_str(json).expect("handler must return valid JSON")
}

// ─── execute_command handler wiring ────────────────────────────────

#[test]
fn handle_execute_command_wires_action_into_dispatch() {
    let args = ExecuteCommandArgs {
        action: "definitely-not-real".to_string(),
        args: None,
    };
    let result = parse(&server().handle_execute_command(Parameters(args)));
    assert_eq!(result["exit_code"], 2);
    assert!(
        result["error"]
            .as_str()
            .unwrap_or_default()
            .contains("Unknown action: definitely-not-real"),
        "{}",
        result
    );
}

#[test]
fn handle_execute_command_wires_path_arg_into_path_validation() {
    // An empty `path` argument must surface the `Invalid path` envelope,
    // proving args.args["path"] is actually read by the handler.
    let args = ExecuteCommandArgs {
        action: "fix".to_string(),
        args: Some(serde_json::json!({ "path": "" })),
    };
    let result = parse(&server().handle_execute_command(Parameters(args)));
    assert_eq!(result["exit_code"], 2);
    assert_eq!(result["error"], "Invalid path");
}

#[test]
fn handle_execute_command_version_has_stable_envelope() {
    let args = ExecuteCommandArgs {
        action: "version".to_string(),
        args: None,
    };
    let result = parse(&server().handle_execute_command(Parameters(args)));
    assert_eq!(result["exit_code"], 0);
    assert_eq!(result["name"], "lint-arwaky");
    assert!(
        result["version"].as_str().is_some_and(|v| !v.is_empty()),
        "{}",
        result
    );
}

// ─── list_commands handler wiring ──────────────────────────────────

#[test]
fn handle_list_commands_returns_full_catalog_without_filter() {
    let args = ListCommandsArgs { domain: None };
    let result = parse(&server().handle_list_commands(Parameters(args)));
    assert_eq!(result["exit_code"], 0);
    let commands = result["commands"].as_array().expect("commands array");
    assert_eq!(
        commands.len(),
        shared_cli_commands::taxonomy_cli_commands_vo::COMMAND_CATALOG.len()
    );
    assert!(
        commands.iter().all(|c| {
            c["name"].as_str().is_some_and(|s| !s.is_empty())
                && c["description"].as_str().is_some_and(|s| !s.is_empty())
                && c["example"].as_str().is_some()
        }),
        "every catalog entry must expose name/description/example: {}",
        result
    );
}

#[test]
fn handle_list_commands_applies_domain_filter() {
    let args = ListCommandsArgs {
        domain: Some("check".to_string()),
    };
    let result = parse(&server().handle_list_commands(Parameters(args)));
    assert_eq!(result["exit_code"], 0);
    let commands = result["commands"].as_array().expect("commands array");
    assert!(!commands.is_empty(), "'check' domain must not be empty");
    assert!(
        commands
            .iter()
            .all(|c| c["name"].as_str().unwrap_or_default().contains("check")),
        "filter must restrict results: {}",
        result
    );
}

// ─── get_config handler: real config reader against temp dirs ──────

#[test]
fn handle_get_config_reports_missing_config_loudly() {
    let tmp = tempfile::TempDir::new().unwrap();
    let args = GetConfigArgs {
        path: Some(tmp.path().to_string_lossy().to_string()),
        language: None,
    };
    let result = parse(&server().handle_get_config(Parameters(args)));
    assert_eq!(result["exit_code"], 0);
    let warnings = result["warnings"].as_array().expect("warnings array");
    assert!(
        warnings.iter().any(|w| w
            .as_str()
            .unwrap_or_default()
            .contains("No config files found")),
        "missing config must be visible, not silent: {}",
        result
    );
}

#[test]
fn handle_get_config_reads_score_threshold_from_project_config() {
    let tmp = tempfile::TempDir::new().unwrap();
    std::fs::write(
        tmp.path().join("lint_arwaky.config.yaml"),
        "thresholds:\n  score: 42.0\nadapters: []\n",
    )
    .unwrap();
    let args = GetConfigArgs {
        path: Some(tmp.path().to_string_lossy().to_string()),
        language: None,
    };
    let result = parse(&server().handle_get_config(Parameters(args)));
    assert_eq!(result["exit_code"], 0);
    assert_eq!(result["score_threshold"], 42.0);
    assert!(
        !result["config_files"]
            .as_array()
            .expect("config_files array")
            .is_empty(),
        "config file must be reported: {}",
        result
    );
}

#[test]
fn handle_get_config_rejects_invalid_path() {
    let args = GetConfigArgs {
        path: Some("".to_string()),
        language: None,
    };
    let result = parse(&server().handle_get_config(Parameters(args)));
    assert_eq!(result["exit_code"], 2);
    assert_eq!(result["error"], "Invalid path");
}

// ─── read_skill handler: missing documentation is a loud error ─────

#[test]
fn handle_read_skill_missing_documentation_fails_loudly() {
    // Tests run with cwd = crates/mcp-server, where no `.agents/skills/`
    // tree exists; the handler must report a searched-paths error, not
    // silently return empty content.
    let args = ReadSkillArgs { section: None };
    let result = parse(&server().handle_read_skill(Parameters(args)));
    if result["exit_code"] == 0 {
        // A developer machine may have the skill installed under XDG —
        // then content must be non-empty instead.
        assert!(
            result["content"]
                .as_str()
                .is_some_and(|c| !c.trim().is_empty()),
            "successful read_skill must return non-empty content"
        );
    } else {
        assert_eq!(result["exit_code"], 2);
        assert!(
            result["error"]
                .as_str()
                .unwrap_or_default()
                .contains("Skill documentation not found"),
            "{}",
            result
        );
        assert!(
            !result["searched"]
                .as_array()
                .expect("searched array")
                .is_empty(),
            "error must list searched candidate paths: {}",
            result
        );
    }
}
