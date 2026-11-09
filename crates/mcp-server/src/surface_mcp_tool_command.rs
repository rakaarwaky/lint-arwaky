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
    DocsScaffoldArgs, ExecuteCommandArgs, GetConfigArgs, ListCommandsArgs,
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
        let options = args.args.clone().unwrap_or_default();
        let path = options.path.clone().unwrap_or_else(|| ".".to_string());
        let threshold = match options.threshold {
            Some(value) if value <= 100 => value,
            Some(value) => {
                return validation_error(&format!(
                    "Invalid value for 'threshold': expected 0..=100, got {value}"
                ));
            }
            None => 80,
        };
        let dry_run = options.dry_run.unwrap_or(false);
        let base = options.base.clone().unwrap_or_else(|| "develop".to_string());

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
        let result = self
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
        serde_json::to_string(&result).unwrap_or_default()
    }

    pub async fn handle_health_check(&self) -> String {
        let result = self.action.handle_health_check().await;
        serde_json::to_string(&result).unwrap_or_else(|e| {
            serde_json::json!({"error": format!("Serialization failed: {e}"), "exit_code": 2})
                .to_string()
        })
    }

    pub fn handle_list_commands(&self, Parameters(args): Parameters<ListCommandsArgs>) -> String {
        let result = self.action.handle_list_commands(args.domain);
        serde_json::to_string(&result).unwrap_or_default()
    }

    pub fn handle_get_config(&self, Parameters(args): Parameters<GetConfigArgs>) -> String {
        let path = args.path.unwrap_or_else(|| ".".to_string());
        self.action.handle_get_config(&path, args.language)
    }

    /// Write missing template files in place.
    /// `dry_run` previews without touching disk. Strictness only affects the
    /// audit path, not scaffolding.
    pub async fn handle_docs_scaffold(
        &self,
        Parameters(args): Parameters<DocsScaffoldArgs>,
    ) -> String {
        let path = args.path.unwrap_or_else(|| ".".to_string());
        let result = self
            .action
            .execute_docs_scaffold(&path, args.member.as_deref(), args.dry_run.unwrap_or(false))
            .await;
        serde_json::to_string(&result).unwrap_or_else(|e| {
            serde_json::json!({"error": format!("Serialization failed: {e}"), "exit_code": 2})
                .to_string()
        })
    }
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
    #[tool(description = "Execute any CLI command. This is the primary tool.")]
    pub async fn execute_command(&self, args: Parameters<ExecuteCommandArgs>) -> String {
        LintArwakyMcpServer::handle_execute_command(self, args).await
    }

    #[tool(
        description = "List all available CLI commands with descriptions and examples. Optional `domain` filter (e.g. \"setup\", \"check\")."
    )]
    pub async fn list_commands(&self, args: Parameters<ListCommandsArgs>) -> String {
        LintArwakyMcpServer::handle_list_commands(self, args)
    }

    #[tool(description = "Check system health: adapters and system state.")]
    pub async fn health_check(&self) -> String {
        LintArwakyMcpServer::handle_health_check(self).await
    }

    #[tool(
        description = "Return the effective architecture configuration for a target path/language. Shows rules, thresholds, adapters."
    )]
    pub async fn get_config(&self, args: Parameters<GetConfigArgs>) -> String {
        LintArwakyMcpServer::handle_get_config(self, args)
    }

    #[tool(
        description = "Scaffold missing documentation templates from the embedded template set. Use `dry_run` to preview without writing. `strict` only affects the audit path."
    )]
    pub async fn docs_scaffold(&self, args: Parameters<DocsScaffoldArgs>) -> String {
        LintArwakyMcpServer::handle_docs_scaffold(self, args).await
    }
}

// The `extract_*` helpers are retained as the documented shape of the old
// untyped path and are exercised by the unit tests below; production reads the
// typed `ExecuteCommandOptions` instead.
#[allow(dead_code)]
fn extract_string(
    args: Option<&serde_json::Value>,
    key: &str,
    default: &str,
) -> Result<String, String> {
    let values = extract_args_object(args)?;
    match values.and_then(|values| values.get(key)) {
        None => Ok(default.to_string()),
        Some(value) => value
            .as_str()
            .map(String::from)
            .ok_or_else(|| format!("Invalid type for '{key}': expected a string")),
    }
}

#[allow(dead_code)]
fn extract_u64(args: Option<&serde_json::Value>, key: &str, default: u64) -> Result<u64, String> {
    let values = extract_args_object(args)?;
    match values.and_then(|values| values.get(key)) {
        None => Ok(default),
        Some(value) => value
            .as_u64()
            .ok_or_else(|| format!("Invalid type for '{key}': expected a non-negative integer")),
    }
}

#[allow(dead_code)]
fn extract_bool(
    args: Option<&serde_json::Value>,
    key: &str,
    default: bool,
) -> Result<bool, String> {
    let values = extract_args_object(args)?;
    match values.and_then(|values| values.get(key)) {
        None => Ok(default),
        Some(value) => value
            .as_bool()
            .ok_or_else(|| format!("Invalid type for '{key}': expected a boolean")),
    }
}

#[allow(dead_code)]
fn extract_args_object(
    args: Option<&serde_json::Value>,
) -> Result<Option<&serde_json::Map<String, serde_json::Value>>, String> {
    match args {
        None => Ok(None),
        Some(serde_json::Value::Object(values)) => Ok(Some(values)),
        Some(_) => Err("Invalid type for 'args': expected an object".to_string()),
    }
}

fn validation_error(error: &str) -> String {
    serde_json::json!({"status": "error", "error": error, "exit_code": 2}).to_string()
}

#[cfg(test)]
mod tests {
    use super::{extract_bool, extract_string, extract_u64};

    #[test]
    fn malformed_present_values_are_not_defaulted() {
        let values = serde_json::json!({
            "path": 42,
            "threshold": "95",
            "dry_run": "true"
        });
        assert!(extract_string(Some(&values), "path", ".").is_err());
        assert!(extract_u64(Some(&values), "threshold", 80).is_err());
        assert!(extract_bool(Some(&values), "dry_run", false).is_err());
    }

    #[test]
    fn absent_values_use_documented_defaults() {
        assert_eq!(extract_string(None, "path", ".").unwrap(), ".");
        assert_eq!(extract_u64(None, "threshold", 80).unwrap(), 80);
        assert!(!extract_bool(None, "dry_run", false).unwrap());
    }
}
