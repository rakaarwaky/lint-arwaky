// Unit tests — MCP action surface: dispatch + envelope contract (QA #641).
//
// Exercises the real `McpActionSurface` dispatch logic with assertion depth:
// every assertion pins the exact JSON envelope shape an MCP client sees.
mod common;

use mcp_server_lint_arwaky::surface_mcp_action_command::McpActionSurface;

fn surface() -> McpActionSurface {
    common::make_action_surface()
}

// ─── Dispatch: unknown actions fail loudly, never silently ─────────

#[test]
fn execute_command_unknown_action_fails_loudly() {
    let result = surface().execute_command("not-a-real-action", ".", 80, false);
    assert_eq!(result["exit_code"], 2);
    assert!(
        result["error"]
            .as_str()
            .unwrap_or_default()
            .contains("Unknown action: not-a-real-action"),
        "unknown action must be named in the error: {}",
        result
    );
}

// ─── Envelope: unsupported transports/actions ──────────────────────

#[test]
fn execute_command_watch_is_explicitly_unsupported_via_mcp() {
    let result = surface().execute_command("watch", ".", 80, false);
    assert_eq!(result["exit_code"], 2);
    assert!(
        result["error"]
            .as_str()
            .unwrap_or_default()
            .contains("watch is not supported via MCP"),
        "{}",
        result
    );
}

#[test]
fn execute_command_mcp_config_redirects_to_cli() {
    let result = surface().execute_command("mcp-config", ".", 80, false);
    assert_eq!(result["exit_code"], 2);
    assert!(
        result["error"]
            .as_str()
            .unwrap_or_default()
            .contains("mcp-config requires transport configuration"),
        "{}",
        result
    );
}

#[test]
fn execute_command_version_reports_configured_version() {
    let result = surface().execute_command("version", ".", 80, false);
    assert_eq!(result["exit_code"], 0);
    assert_eq!(result["name"], "lint-arwaky");
    assert_eq!(result["version"], "0.0.0-qa-test");
}

// ─── Path-validation boundary (QA #641 / BE path-validation finding) ──

#[test]
fn execute_command_fix_rejects_empty_path_with_invalid_path_envelope() {
    let result = surface().execute_command("fix", "", 80, false);
    assert_eq!(result["exit_code"], 2);
    assert_eq!(result["error"], "Invalid path");
}

#[test]
fn execute_command_check_rejects_whitespace_only_path() {
    let result = surface().execute_command("check", "   ", 80, false);
    assert_eq!(result["exit_code"], 2);
    assert_eq!(result["error"], "Invalid path");
}

#[test]
fn execute_command_ci_rejects_empty_path() {
    let result = surface().execute_command("ci", "", 80, false);
    assert_eq!(result["exit_code"], 2);
    assert_eq!(result["error"], "Invalid path");
}

#[test]
fn execute_command_install_hook_rejects_empty_path() {
    let result = surface().execute_command("install-hook", "", 80, false);
    assert_eq!(result["exit_code"], 2);
    assert_eq!(result["error"], "Invalid path");
}

// Documents the CURRENT boundary semantics at the MCP input layer:
// the underlying `FilePath` VO normalizes traversal sequences instead of
// rejecting them, so `../` paths currently reach the dispatcher (this is the
// confirmed BE path-validation CRITICAL, tracked by SE/BE issues #654–#657).
// When that fix lands, FLIP these assertions to require an `Invalid path`
// rejection for such paths — this test is the tripwire for that change.
#[test]
fn execute_command_path_traversal_documents_current_normalization_gap() {
    use shared_common::taxonomy_path_vo::FilePath;

    let normalized = FilePath::new("..\\..\\etc\\passwd".to_string())
        .expect("documented gap: traversal paths are accepted today, not rejected");
    assert_eq!(normalized.value, "../../etc/passwd");
}
