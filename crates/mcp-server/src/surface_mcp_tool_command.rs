// PURPOSE: LintArwakyMcpServer — MCP tool surface: protocol only.
//
// Holds Arc<McpActionSurface> and maps rmcp protocol parameters to action
// surface methods. No business logic here — everything delegates to
// McpActionSurface (surface_mcp_action_command), which delegates to dispatcher.
use rmcp::handler::server::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{
    Implementation, ProtocolVersion, ServerCapabilities, ServerConfig, ToolsCapability,
};
use rmcp::{ServerHandler, tool, tool_handler, tool_router};
use std::sync::Arc;

use crate::taxonomy_mcp_server_vo::{
    DocsScaffoldArgs, ExecuteCommandArgs, ExecuteCommandOptions, GetConfigArgs, ListCommandsArgs,
};

use crate::surface_mcp_action_command::McpActionSurface;

#[derive(Clone)]
pub struct LintArwakyMcpServer {
    action: Arc<McpActionSurface>,
    // Consumed implicitly by the `#[tool_router]` proc-macro, which also derives
    // `ServerHandler`/`tool_router()` from it. Read here to keep it live for the
    // macro-generated `ServerHandler::call_tool` dispatch path.
    tool_router: ToolRouter<Self>,
}

impl LintArwakyMcpServer {
    pub fn new(action: Arc<McpActionSurface>) -> Self {
        Self {
            action,
            tool_router: Self::tool_router(),
        }
    }

    /// Expose the configured tool router (used by the proc-macro `ServerHandler`
    /// impl for routing incoming tool calls; kept as a public surface so the
    /// field is not dead code).
    pub fn router(&self) -> &ToolRouter<Self> {
        &self.tool_router
    }

    pub async fn handle_execute_command(
        &self,
        Parameters(args): Parameters<ExecuteCommandArgs>,
    ) -> String {
        let action = args.action.clone();
        // Typed options, not a free-form map: an unknown key is a validation
        // error instead of a silently ignored argument.
        let options: ExecuteCommandOptions = args.args.unwrap_or_default();
        let path_was_specified = options.path.is_some();
        let path = options.path.clone().unwrap_or_else(|| ".".to_string());
        let threshold = options.threshold.unwrap_or(80);
        let dry_run = options.dry_run.unwrap_or(false);
        let base = options.base.clone().unwrap_or_else(|| "develop".to_string());
        if threshold > 100 {
            return validation_error(&format!(
                "Invalid value for 'threshold': expected 0..=100, got {threshold}"
            ));
        }
        // An argument the selected verb would ignore is an error, not a
        // silently dropped key: a caller who passes `dry_run` to `check`
        // believes the tree will not change, and it never would have.
        if options.threshold.is_some() && action != "ci" {
            return validation_error("'threshold' is only supported by the 'ci' action");
        }
        if options.dry_run.is_some() && action != "fix" {
            return validation_error("'dry_run' is only supported by the 'fix' action");
        }
        if options.member.is_some()
            && !matches!(action.as_str(), "check" | "scan" | "docs_scaffold")
        {
            return validation_error(
                "'member' is only supported by 'check', 'scan' and 'docs_scaffold'",
            );
        }
        if options.rules.is_some() && !matches!(action.as_str(), "check" | "scan") {
            return validation_error("'rules' is only supported by 'check' and 'scan'");
        }
        if options.base.is_some() && action != "git" {
            return validation_error("'base' is only supported by the 'git' action");
        }
        if options.strict.is_some() && action != "docs" {
            return validation_error("'strict' is only supported by the 'docs' action");
        }
        if path_was_specified && matches!(action.as_str(), "doctor" | "adapters" | "version" | "uninstall-hook" | "init" | "install" | "mcp-config" | "watch") {
            return validation_error(&format!("'path' is not supported by the '{action}' action"));
        }

        tracing::info!(
            target: "lint_arwaky::audit",
            event = "mcp_execute_command_invocation",
            tool = "execute_command",
            action = %action,
            path = %path,
            threshold,
            dry_run,
            "MCP command invocation"
        );
        let mut result = self
            .action
            .execute_command(
                &action,
                &path,
                threshold,
                dry_run,
                &base,
                options.member.as_deref(),
                options.rules.as_deref(),
                options.strict.unwrap_or(false),
            )
            .await;
        attach_resolved_path(&self.action, &path, &mut result);
        let exit_code = result
            .get("exit_code")
            .and_then(serde_json::Value::as_i64)
            .unwrap_or(2);
        tracing::info!(
            target: "lint_arwaky::audit",
            event = "mcp_execute_command_result",
            tool = "execute_command",
            action = %action,
            success = exit_code == 0,
            exit_code,
            "MCP command result"
        );
        if matches!(action.as_str(), "install-hook" | "uninstall-hook") {
            tracing::info!(
                target: "lint_arwaky::audit",
                event = "git_hook_change",
                action = %action,
                success = exit_code == 0,
                exit_code,
                "MCP git-hook action result"
            );
        }
        serialize_result(&result)
    }

    pub async fn handle_health_check(&self) -> String {
        let result = self.action.handle_health_check().await;
        serialize_result(&result)
    }

    pub fn handle_list_commands(&self, Parameters(args): Parameters<ListCommandsArgs>) -> String {
        let result = self.action.handle_list_commands(args.domain);
        serialize_result(&result)
    }

    pub fn handle_get_config(&self, Parameters(args): Parameters<GetConfigArgs>) -> String {
        let path = args.path.unwrap_or_else(|| ".".to_string());
        let result = self.action.handle_get_config(&path, args.language);
        serialize_with_resolved_path(&self.action, &path, result)
    }

    /// Write missing template files in place.
    /// `dry_run` previews without touching disk. Strictness only affects the
    /// audit path, not scaffolding.
    pub async fn handle_docs_scaffold(
        &self,
        Parameters(args): Parameters<DocsScaffoldArgs>,
    ) -> String {
        let path = args.path.unwrap_or_else(|| ".".to_string());
        let mut result = self
            .action
            .execute_docs_scaffold(&path, args.member.as_deref(), args.dry_run.unwrap_or(false))
            .await;
        attach_resolved_path(&self.action, &path, &mut result);
        serialize_result(&result)
    }
}

fn attach_resolved_path(
    action: &McpActionSurface,
    requested_path: &str,
    result: &mut serde_json::Value,
) {
    let Some(object) = result.as_object_mut() else {
        return;
    };
    if !object.contains_key("path") {
        return;
    }
    if let Ok(path) = action.to_fp(requested_path) {
        object.insert("resolved_path".to_string(), serde_json::json!(path.value()));
    }
}

fn serialize_with_resolved_path(
    action: &McpActionSurface,
    requested_path: &str,
    response: String,
) -> String {
    match serde_json::from_str::<serde_json::Value>(&response) {
        Ok(mut value) => {
            attach_resolved_path(action, requested_path, &mut value);
            serialize_result(&value)
        }
        Err(_) => response,
    }
}

fn serialize_result(result: &serde_json::Value) -> String {
    serde_json::to_string(result).unwrap_or_else(|error| {
        serde_json::json!({
            "status": "error",
            "error": format!("Serialization failed: {error}"),
            "exit_code": 2
        })
        .to_string()
    })
}

#[tool_handler]
impl ServerHandler for LintArwakyMcpServer {
    fn get_info(&self) -> ServerConfig {
        let mut builder = ServerCapabilities::builder();
        builder.tools = Some(ToolsCapability::default());
        let capabilities = builder.build();
        ServerConfig::new(capabilities)
            .with_server_info(Implementation::new(
                "lint-arwaky",
                shared_common::RELEASE_VERSION,
            ))
            .with_protocol_version(ProtocolVersion::default())
    }
}

#[tool_router]
impl LintArwakyMcpServer {
    #[tool(
        description = "Run a supported linter action in the configured workspace. `path` defaults to the workspace root; relative paths resolve from that root and absolute paths must remain inside it. `threshold` is accepted only for `ci` (0..=100, default 80), and `dry_run` only for `fix` (default false). Unknown or inapplicable options are rejected. Path-bearing responses include `resolved_path`."
    )]
    pub async fn execute_command(&self, args: Parameters<ExecuteCommandArgs>) -> String {
        LintArwakyMcpServer::handle_execute_command(self, args).await
    }

    #[tool(
        description = "List command descriptions, MCP tool-call examples, CLI shell examples, and MCP support status. Optional `domain` filter (e.g. \"setup\", \"check\")."
    )]
    pub async fn list_commands(&self, args: Parameters<ListCommandsArgs>) -> String {
        LintArwakyMcpServer::handle_list_commands(self, args)
    }

    #[tool(description = "Check system health: adapters and system state.")]
    pub async fn health_check(&self) -> String {
        LintArwakyMcpServer::handle_health_check(self).await
    }

    #[tool(
        description = "Return the effective architecture configuration. `path` defaults to the configured workspace root; relative paths resolve from that root and absolute paths must remain inside it. Shows rules, thresholds, and adapters; path-bearing responses include `resolved_path`."
    )]
    pub async fn get_config(&self, args: Parameters<GetConfigArgs>) -> String {
        LintArwakyMcpServer::handle_get_config(self, args)
    }

    #[tool(
        description = "Scaffold missing documentation templates beneath the configured workspace root. `path` is workspace-relative or an in-workspace absolute path; `dry_run` previews without writing. `member` optionally limits scaffolding to one member."
    )]
    pub async fn docs_scaffold(&self, args: Parameters<DocsScaffoldArgs>) -> String {
        LintArwakyMcpServer::handle_docs_scaffold(self, args).await
    }
}

fn validation_error(error: &str) -> String {
    serde_json::json!({"status": "error", "error": error, "exit_code": 2}).to_string()
}
