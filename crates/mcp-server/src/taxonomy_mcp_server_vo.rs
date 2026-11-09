// PURPOSE: McpToolArgs — typed argument structs for MCP tools with JsonSchema
use rmcp::schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
// ─── Block 1: Module Declaration ───────────────────────────
pub struct ExecuteCommandArgs {
    /// The command action to execute (e.g. "scan", "check", "security", "doctor")
    pub action: String,
    /// Action arguments. `path` resolves against the confined workspace root;
    /// `member`, `rules`, `base`, `strict`, `dry_run` and `threshold` are
    /// forwarded to the verb that owns them.
    ///
    /// Unknown keys are rejected rather than ignored: a silently dropped
    /// `member` widened a member-scoped scan to the whole workspace, and the
    /// caller could not tell the two results apart.
    pub args: Option<ExecuteCommandOptions>,
}

/// Every argument any CLI verb accepts through `execute_command`.
///
/// One flat struct rather than per-action enums because the action name is
/// already the discriminator; the cost is that an argument is accepted for a
/// verb that ignores it, which is reported rather than silently dropped.
#[derive(Debug, Default, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ExecuteCommandOptions {
    /// Workspace-confined path. Relative paths resolve against the configured
    /// workspace root, not the MCP server cwd.
    #[serde(default)]
    pub path: Option<String>,
    /// CI score threshold (0–100).
    #[serde(default)]
    pub threshold: Option<u64>,
    /// Preview without writing (fix, docs_scaffold).
    #[serde(default)]
    pub dry_run: Option<bool>,
    /// Restrict a scan to one workspace member by name.
    #[serde(default)]
    pub member: Option<String>,
    /// Filter reported violations by AES rule code.
    #[serde(default)]
    pub rules: Option<String>,
    // NOTE: no `layer`, `format` or `group` keys. The MCP surface already
    // discriminates those through the action name — `taxonomy`/`contract`/…
    // select a layer, `quality`/`import`/… select a group — and its output is
    // always JSON. Accepting them here would recreate the bug this struct
    // exists to fix: a documented key that is decoded and then ignored.
    /// Git base ref for the `git` verb.
    #[serde(default)]
    pub base: Option<String>,
    /// Force strict doc auditing for `docs`.
    #[serde(default)]
    pub strict: Option<bool>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListCommandsArgs {
    /// Optional domain filter
    pub domain: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetConfigArgs {
    /// Optional project path to inspect config for
    pub path: Option<String>,
    /// Optional language hint (rust, python, javascript)
    pub language: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
// ─── Docs scaffold args ────────────────────────────────────────────────
pub struct DocsScaffoldArgs {
    /// Project path to scaffold docs into
    pub path: Option<String>,
    /// Scope to a specific member crate (e.g. `billing`). Omit to scaffold all.
    pub member: Option<String>,
    /// Preview what would be written without modifying the filesystem
    pub dry_run: Option<bool>,
}
