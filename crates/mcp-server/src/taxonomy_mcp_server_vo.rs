// PURPOSE: McpToolArgs — typed argument structs for MCP tools with JsonSchema
use rmcp::schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
// ─── Block 1: Module Declaration ───────────────────────────
pub struct ExecuteCommandArgs {
    /// The command action to execute (e.g. "scan", "check", "security", "doctor").
    pub action: String,
    /// Supported options for the selected action. Unknown keys are rejected.
    pub args: Option<ExecuteCommandOptions>,
}

/// Typed options accepted by `execute_command`.
///
/// `path` is relative to the configured MCP workspace root; absolute paths are
/// accepted only when they remain inside that root. It defaults to the root.
#[derive(Debug, Default, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ExecuteCommandOptions {
    /// Workspace-relative or in-workspace absolute target path (default: workspace root).
    pub path: Option<String>,
    /// Minimum score for the `ci` action only (default: 80, range: 0..=100).
    pub threshold: Option<u64>,
    /// Preview fixes without writing files, for the `fix` action only (default: false).
    pub dry_run: Option<bool>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListCommandsArgs {
    /// Optional domain filter
    pub domain: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetConfigArgs {
    /// Workspace-relative or in-workspace absolute project path; defaults to workspace root.
    pub path: Option<String>,
    /// Optional language hint (rust, python, javascript)
    pub language: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
// ─── Docs scaffold args ────────────────────────────────────────────────
pub struct DocsScaffoldArgs {
    /// Workspace-relative or in-workspace absolute project path; defaults to workspace root.
    pub path: Option<String>,
    /// Scope to a specific member crate (e.g. `billing`). Omit to scaffold all.
    pub member: Option<String>,
    /// Preview what would be written without modifying the filesystem
    pub dry_run: Option<bool>,
}
