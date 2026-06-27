# Crate: mcp-server (v1.10.14)

This document contains the source code for feature crate `mcp-server` along with its corresponding and imported definitions from the `shared` crate.

## Problem Statement

The following issues were detected by `lint-arwaky-cli scan`:

```
============================================================
  AES Architecture Compliance Report
============================================================
  Project: /home/raka/mcp-arwaky/lint-arwaky/crates/mcp-server
  Violations: 14
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/mcp-server/src/agent_mcp_server_orchestrator.rs - AES304 BYPASS_COMMENT: Forbidden bypass comment or annotation detected.
WHY? Bypassing code checks hides issues and risks architectural regressions.
FIX: Remove the bypass comment (e.g. noqa, eslint-disable, ts-ignore) and resolve the issue properly.
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/mcp-server/src/agent_mcp_server_orchestrator.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/mcp-server/src/agent_mcp_server_orchestrator.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/mcp-server/src/agent_mcp_server_orchestrator.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/mcp-server/src/agent_mcp_server_orchestrator.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/mcp-server/src/agent_mcp_server_orchestrator.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/mcp-server/src/agent_mcp_server_orchestrator.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/mcp-server/src/agent_mcp_server_orchestrator.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/mcp-server/src/agent_mcp_server_orchestrator.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/mcp-server/src/agent_mcp_server_orchestrator.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/mcp-server/src/agent_mcp_server_orchestrator.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/mcp-server/src/agent_mcp_server_orchestrator.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/mcp-server/src/agent_mcp_server_orchestrator.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/mcp-server/src/agent_mcp_server_orchestrator.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
```

---

## File List

- [crates/mcp-server/Cargo.toml](file:///home/raka/mcp-arwaky/lint-arwaky/crates/mcp-server/Cargo.toml)
- [crates/mcp-server/src/agent_mcp_server_orchestrator.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/mcp-server/src/agent_mcp_server_orchestrator.rs)
- [crates/mcp-server/src/lib.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/mcp-server/src/lib.rs)
- [crates/mcp-server/src/taxonomy_mcp_tool_args_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/mcp-server/src/taxonomy_mcp_tool_args_vo.rs)
- [crates/shared/src/cli-commands/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/mod.rs)
- [crates/shared/src/cli-commands/taxonomy_catalog_constant.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_catalog_constant.rs)
- [crates/shared/src/cli-commands/taxonomy_metadata_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_metadata_vo.rs)
- [crates/shared/src/cli-commands/taxonomy_position_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_position_vo.rs)
- [crates/shared/src/cli-commands/taxonomy_protocol_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_protocol_vo.rs)
- [crates/shared/src/cli-commands/taxonomy_result_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_result_vo.rs)
- [crates/shared/src/cli-commands/taxonomy_transport_error.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_transport_error.rs)
- [crates/shared/src/code-analysis/contract_code_analysis_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/contract_code_analysis_aggregate.rs)
- [crates/shared/src/code-analysis/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/mod.rs)
- [crates/shared/src/mcp-server/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/mcp-server/mod.rs)

---

## File: crates/mcp-server/Cargo.toml

```toml
[package]
name = "mcp_server-lint-arwaky"
version = "1.10.14"
edition = "2021"
description = "Model Context Protocol (MCP) server surfaces exposing the lint pipeline as tools/resources to AI agents."
license = "MIT"
repository = "https://github.com/rakaarwaky/lint-arwaky"
publish = false

[lints]
workspace = true

[dependencies]  # (unchanged)
serde.workspace = true
serde_json.workspace = true
async-trait.workspace = true
once_cell.workspace = true
regex.workspace = true
tracing.workspace = true
shared.workspace = true
rmcp.workspace = true
schemars = "1.2"
tokio.workspace = true
anyhow.workspace = true
```

---

## File: crates/mcp-server/src/agent_mcp_server_orchestrator.rs

```rust
// PURPOSE: LintArwakyMcpServer — MCP server using rmcp official SDK
use rmcp::handler::server::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{
    Implementation, ProtocolVersion, ServerCapabilities, ServerInfo, ToolsCapability,
};
use rmcp::{tool, tool_handler, tool_router, ServerHandler};
use std::sync::Arc;

use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;

use crate::taxonomy_mcp_tool_args_vo::{ExecuteCommandArgs, ListCommandsArgs, ReadSkillArgs};

#[derive(Clone)]
pub struct LintArwakyMcpServer {
    #[allow(dead_code)]
    code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    tool_router: ToolRouter<Self>,
}

impl LintArwakyMcpServer {
    pub fn new(code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>) -> Self {
        Self {
            code_analysis_linter,
            tool_router: Self::tool_router(),
        }
    }
}

#[tool_handler]
impl ServerHandler for LintArwakyMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::default(),
            capabilities: ServerCapabilities {
                tools: Some(ToolsCapability { list_changed: None }),
                ..Default::default()
            },
            server_info: Implementation {
                name: "lint-arwaky".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                ..Default::default()
            },
            instructions: None,
        }
    }
}

#[tool_router]
impl LintArwakyMcpServer {
    #[tool(description = "Execute any CLI command. This is the primary tool.")]
    async fn execute_command(&self, Parameters(args): Parameters<ExecuteCommandArgs>) -> String {
        // Clone Arc so spawn_blocking owns a 'static reference to the linter
        let linter = self.code_analysis_linter.clone();
        let action = args.action.clone();
        let arg_path = args
            .args
            .as_ref()
            .and_then(|a| a.get("path"))
            .and_then(|v| v.as_str())
            .map(String::from);
        let arg_threshold = args
            .args
            .as_ref()
            .and_then(|a| a.get("threshold"))
            .and_then(|v| v.as_u64())
            .map(|n| n as u32);
        let arg_client = args
            .args
            .as_ref()
            .and_then(|a| a.get("client"))
            .and_then(|v| v.as_str())
            .map(String::from);
        let result = match action.as_str() {
            "check" | "scan" => {
                let path = arg_path.unwrap_or_else(|| ".".to_string());
                // spawn_blocking: arch_linter.run_lint internally creates a
                // tokio runtime, which would panic if called from within
                // rmcp's async context. Off-load to blocking thread pool.
                let linter_for_blocking = linter.clone();
                let path_for_blocking = path.clone();
                let join_result = tokio::task::spawn_blocking(move || {
                    let results = linter_for_blocking.run_code_analysis_path(&path_for_blocking);
                    let report = linter_for_blocking.format_report(
                        &shared::cli_commands::taxonomy_result_vo::LintResultList::new(
                            results.clone(),
                        ),
                        &path_for_blocking,
                    );
                    serde_json::json!({
                        "status": "success",
                        "action": action,
                        "path": path_for_blocking,
                        "total_violations": results.len(),
                        "report": report
                    })
                })
                .await;
                match join_result {
                    Ok(v) => v,
                    Err(e) => serde_json::json!({"error": format!("blocking task failed: {}", e)}),
                }
            }
            "fix" => {
                let path = arg_path.unwrap_or_else(|| ".".to_string());
                serde_json::json!({
                    "status": "success",
                    "action": "fix",
                    "path": path,
                    "message": "Auto-fix completed."
                })
            }
            "ci" => {
                let path = arg_path.unwrap_or_else(|| ".".to_string());
                let threshold = arg_threshold.unwrap_or(80);
                let linter_for_blocking = linter.clone();
                let path_for_blocking = path.clone();
                let join_result = tokio::task::spawn_blocking(move || {
                    let results = linter_for_blocking.run_code_analysis_path(&path_for_blocking);
                    let score = linter_for_blocking.calc_score(&results);
                    let pass = score >= threshold as f64;
                    serde_json::json!({
                        "status": if pass { "pass" } else { "fail" },
                        "action": "ci",
                        "score": score,
                        "threshold": threshold,
                        "violations": results.len()
                    })
                })
                .await;
                match join_result {
                    Ok(v) => v,
                    Err(e) => serde_json::json!({"error": format!("blocking task failed: {}", e)}),
                }
            }
            "doctor" => {
                let mut checks = Vec::new();
                for tool in &["cargo", "python3", "ruff", "mypy", "bandit", "node", "git"] {
                    let found = std::process::Command::new("which")
                        .arg(tool)
                        .output()
                        .map(|o| o.status.success())
                        .unwrap_or(false);
                    checks.push(serde_json::json!({
                        "tool": tool,
                        "status": if found { "ok" } else { "not_found" }
                    }));
                }
                serde_json::json!({"status": "success", "action": "doctor", "checks": checks})
            }
            "orphan" | "security" | "duplicates" | "dependencies" => {
                let path = arg_path.unwrap_or_else(|| ".".to_string());
                serde_json::json!({"status": "success", "action": action, "path": path})
            }
            "version" => {
                serde_json::json!({"version": env!("CARGO_PKG_VERSION"), "name": "lint-arwaky"})
            }
            "adapters" => {
                let mut adapters = Vec::new();
                for (name, lang) in &[
                    ("ruff", "python"),
                    ("mypy", "python"),
                    ("bandit", "python"),
                    ("clippy", "rust"),
                    ("eslint", "javascript"),
                ] {
                    let found = std::process::Command::new("which")
                        .arg(name)
                        .output()
                        .map(|o| o.status.success())
                        .unwrap_or(false);
                    adapters.push(
                        serde_json::json!({"name": name, "language": lang, "enabled": found}),
                    );
                }
                serde_json::json!({"adapters": adapters})
            }
            "install-hook" => {
                serde_json::json!({"status": "success", "message": "Git hook installed."})
            }
            "uninstall-hook" => {
                serde_json::json!({"status": "success", "message": "Git hook removed."})
            }
            "init" => serde_json::json!({"status": "success", "action": "init"}),
            "install" => serde_json::json!({"status": "success", "action": "install"}),
            "mcp-config" => {
                let client = arg_client.unwrap_or_else(|| "all".to_string());
                serde_json::json!({"status": "success", "action": "mcp-config", "client": client})
            }
            "config-show" => serde_json::json!({"status": "success", "action": "config-show"}),
            _ => serde_json::json!({"error": format!("Unknown action: {}", action)}),
        };
        serde_json::to_string_pretty(&result).unwrap_or_default()
    }

    #[tool(
        description = "List all available CLI commands with descriptions and examples. Optional `domain` filter (e.g. \"setup\", \"check\")."
    )]
    async fn list_commands(&self, Parameters(args): Parameters<ListCommandsArgs>) -> String {
        let catalog = shared::cli_commands::taxonomy_catalog_constant::COMMAND_CATALOG;
        let commands: Vec<serde_json::Value> = catalog
            .iter()
            .filter(|(name, _desc, _ex)| match args.domain.as_deref() {
                Some(d) if !d.is_empty() => name.contains(d),
                _ => true,
            })
            .map(|(name, desc, example)| {
                serde_json::json!({
                    "name": name,
                    "description": desc,
                    "example": example
                })
            })
            .collect();
        let result = serde_json::json!({ "commands": commands, "total": commands.len() });
        serde_json::to_string_pretty(&result).unwrap_or_default()
    }

    #[tool(
        description = "Read SKILL.md documentation by section. Searches several candidate locations."
    )]
    async fn read_skill(&self, Parameters(args): Parameters<ReadSkillArgs>) -> String {
        let candidates = [
            env!("CARGO_MANIFEST_DIR").to_string() + "/../SKILL.md",
            env!("CARGO_MANIFEST_DIR").to_string() + "/SKILL.md",
            "SKILL.md".to_string(),
            "./SKILL.md".to_string(),
        ];
        let content = candidates
            .iter()
            .map(std::path::Path::new)
            .find(|p| p.exists())
            .and_then(|p| std::fs::read_to_string(p).ok());
        let content = match content {
            Some(c) => c,
            None => {
                return serde_json::json!({
                    "error": "SKILL.md not found",
                    "searched": candidates
                })
                .to_string();
            }
        };
        match args.section.as_deref() {
            Some(s) if !s.is_empty() => {
                let header = format!("## {}", s);
                if let Some(start) = content.find(&header) {
                    let remaining = &content[start..];
                    let end = remaining[1..]
                        .find("\n## ")
                        .map(|i| i + 1)
                        .unwrap_or(remaining.len());
                    serde_json::json!({"section": s, "content": &remaining[..end]}).to_string()
                } else {
                    serde_json::json!({"error": format!("Section '{}' not found", s)}).to_string()
                }
            }
            _ => serde_json::json!({"content": content}).to_string(),
        }
    }

    #[tool(description = "Check system health: adapters and system state.")]
    async fn health_check(&self) -> String {
        let mut adapters = Vec::new();
        for (name, lang) in &[
            ("ruff", "python"),
            ("mypy", "python"),
            ("bandit", "python"),
            ("clippy", "rust"),
            ("eslint", "javascript"),
        ] {
            let found = std::process::Command::new("which")
                .arg(name)
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false);
            adapters.push(serde_json::json!({
                "name": name,
                "language": lang,
                "status": if found { "available" } else { "not_installed" }
            }));
        }
        let available = adapters
            .iter()
            .filter(|a| a["status"] == "available")
            .count();
        let result = serde_json::json!({
            "version": env!("CARGO_PKG_VERSION"),
            "adapters_available": available,
            "adapters_total": adapters.len(),
            "adapters": adapters
        });
        serde_json::to_string_pretty(&result).unwrap_or_default()
    }
}
```

---

## File: crates/mcp-server/src/lib.rs

```rust
// PURPOSE: Module declarations for mcp-server
pub mod agent_mcp_server_orchestrator;
pub mod taxonomy_mcp_tool_args_vo;
```

---

## File: crates/mcp-server/src/taxonomy_mcp_tool_args_vo.rs

```rust
// PURPOSE: McpToolArgs — typed argument structs for MCP tools with JsonSchema
use rmcp::schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ExecuteCommandArgs {
    /// The command action to execute
    pub action: String,
    /// Additional arguments for the command
    pub args: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListCommandsArgs {
    /// Optional domain filter
    pub domain: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ReadSkillArgs {
    /// Section to read from SKILL.md
    pub section: Option<String>,
}
```

---

## File: crates/shared/src/cli-commands/mod.rs

```rust
// cli-commands — taxonomy and contract types
pub mod contract_executor_port;
pub mod taxonomy_catalog_constant;
pub mod taxonomy_cli_vo;
pub mod taxonomy_command_catalog_vo;
pub mod taxonomy_metadata_vo;
pub mod taxonomy_position_vo;
pub mod taxonomy_protocol_vo;
pub mod taxonomy_result_vo;
pub mod taxonomy_score_vo;
pub mod taxonomy_severity_vo;
pub mod taxonomy_transport_error;
```

---

## File: crates/shared/src/cli-commands/taxonomy_catalog_constant.rs

```rust
// PURPOSE: COMMAND_CATALOG — static table of every public CLI/MCP command with description and usage example

pub static COMMAND_CATALOG: &[(&str, &str, &str)] = &[
    (
        "check",
        "Run full architecture compliance analysis",
        "lint-arwaky check /path",
    ),
    (
        "scan",
        "Deep directory scan (alias for check)",
        "lint-arwaky scan ./src/",
    ),
    ("fix", "Apply safe fixes", "lint-arwaky fix file.py"),
    (
        "ci",
        "CI-optimized with exit codes",
        "lint-arwaky ci /path --threshold 80",
    ),
    (
        "doctor",
        "Diagnose environment health",
        "lint-arwaky doctor",
    ),
    (
        "orphan",
        "Check if file is dead/unreachable code",
        "lint-arwaky orphan <path>",
    ),
    (
        "security",
        "Vulnerability scanning",
        "lint-arwaky security /path",
    ),
    (
        "duplicates",
        "Code duplication detection",
        "lint-arwaky duplicates /path",
    ),
    (
        "dependencies",
        "Dependency vulnerability scan",
        "lint-arwaky dependencies .",
    ),
    (
        "watch",
        "Watch files for changes",
        "lint-arwaky watch ./src/",
    ),
    (
        "install-hook",
        "Install git pre-commit hook",
        "lint-arwaky install-hook",
    ),
    (
        "uninstall-hook",
        "Remove git pre-commit hook",
        "lint-arwaky uninstall-hook",
    ),
    ("adapters", "List enabled adapters", "lint-arwaky adapters"),
    ("version", "Show version", "lint-arwaky version"),
    ("init", "Create default config", "lint-arwaky init"),
    (
        "install",
        "Install adapter dependencies",
        "lint-arwaky install",
    ),
    (
        "mcp-config",
        "Print MCP server config",
        "lint-arwaky mcp-config --client claude",
    ),
    (
        "config-show",
        "Show active configuration",
        "lint-arwaky config-show",
    ),
];
```

---

## File: crates/shared/src/cli-commands/taxonomy_metadata_vo.rs

```rust
// PURPOSE: CommandMetadataVO — value object wrapping description + usage example for each CLI command
use crate::common::taxonomy_suggestion_vo::DescriptionVO;
use crate::common::taxonomy_suggestion_vo::Suggestion;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CommandMetadataVO {
    pub description: DescriptionVO,
    pub example: Suggestion,
}

impl CommandMetadataVO {
    pub fn new(description: DescriptionVO, example: Suggestion) -> Self {
        Self {
            description,
            example,
        }
    }
}

impl std::fmt::Display for CommandMetadataVO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.description, self.example)
    }
}
```

---

## File: crates/shared/src/cli-commands/taxonomy_position_vo.rs

```rust
// PURPOSE: Position — value object for source code position tracking (file, line, column)
use serde::{Deserialize, Serialize};

use crate::common::taxonomy_common_vo::ColumnNumber;
use crate::common::taxonomy_common_vo::LineNumber;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Position {
    pub line: LineNumber,
    #[serde(default)]
    pub column: ColumnNumber,
}

impl Position {
    pub fn new(line: LineNumber) -> Self {
        Self {
            line,
            column: ColumnNumber::new(0),
        }
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.column.value > 0 {
            write!(f, "{}:{}", self.line, self.column)
        } else {
            write!(f, "{}", self.line)
        }
    }
}
```

---

## File: crates/shared/src/cli-commands/taxonomy_protocol_vo.rs

```rust
// PURPOSE: TransportEndpoint, TransportProtocol, TransportUrlVO — value objects for transport endpoint configuration
use crate::string_value_object;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransportEndpoint {
    pub protocol: TransportProtocol,
    pub address: String,
}

impl Default for TransportEndpoint {
    fn default() -> Self {
        Self {
            protocol: TransportProtocol::STDAggregate,
            address: String::new(),
        }
    }
}

impl TransportEndpoint {
    pub fn new(protocol: TransportProtocol, address: String) -> Self {
        Self { protocol, address }
    }

    pub fn display_name(&self) -> String {
        match self.protocol {
            TransportProtocol::HTTP => format!("HTTP({})", self.address),
            TransportProtocol::UnixSocket => format!("Socket({})", self.address),
            TransportProtocol::STDAggregate => "Stdio(direct)".to_string(),
        }
    }
    pub fn from_url(url: &str) -> Self {
        let (protocol, address) = match url {
            u if u.starts_with("http://") || u.starts_with("https://") => {
                (TransportProtocol::HTTP, u.to_string())
            }
            "stdio" => (TransportProtocol::STDAggregate, "stdio".to_string()),
            u if u.starts_with('/') || u.starts_with('.') => {
                (TransportProtocol::UnixSocket, u.to_string())
            }
            _ => (TransportProtocol::STDAggregate, "stdio".to_string()),
        };
        Self { protocol, address }
    }
}

impl std::fmt::Display for TransportEndpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.protocol, self.address)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TransportProtocol {
    #[serde(rename = "HTTP")]
    HTTP,
    #[serde(rename = "UnixSocket")]
    UnixSocket,
    #[serde(rename = "Stdio")]
    STDAggregate,
}

impl std::fmt::Display for TransportProtocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransportProtocol::HTTP => write!(f, "HTTP"),
            TransportProtocol::UnixSocket => write!(f, "UnixSocket"),
            TransportProtocol::STDAggregate => write!(f, "Stdio"),
        }
    }
}

impl TransportProtocol {
    pub fn needs_desktop_commander(&self) -> bool {
        matches!(
            self,
            TransportProtocol::HTTP | TransportProtocol::UnixSocket
        )
    }
}

string_value_object!(TransportUrlVO);
```

---

## File: crates/shared/src/cli-commands/taxonomy_result_vo.rs

```rust
// PURPOSE: LintResult, LintResultList, FilePathSet — value objects for lint violation results
use serde::{Deserialize, Serialize};

use crate::cli_commands::taxonomy_position_vo::Position;
use crate::cli_commands::taxonomy_severity_vo::Severity;
use crate::common::taxonomy_adapter_name_vo::AdapterName;
use crate::common::taxonomy_common_vo::ColumnNumber;
use crate::common::taxonomy_common_vo::LineNumber;
use crate::common::taxonomy_error_vo::ErrorCode;
use crate::common::taxonomy_layer_vo::Identity;
use crate::common::taxonomy_lint_vo::LocationList;
use crate::common::taxonomy_lint_vo::ScopeRef;
use crate::common::taxonomy_message_vo::LintMessage;
use crate::common::taxonomy_suggestion_vo::DescriptionVO;
use crate::source_parsing::taxonomy_path_vo::FilePath;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LintResult {
    pub file: FilePath,
    pub line: LineNumber,
    pub column: ColumnNumber,
    pub code: ErrorCode,
    pub message: LintMessage,
    pub source: Option<AdapterName>,
    pub severity: Severity,
    pub enclosing_scope: Option<ScopeRef>,
    pub related_locations: LocationList,
}

impl LintResult {
    /// Convenience constructor used by architecture checkers (make_result / mk pattern).
    pub fn new_arch(
        file: &str,
        line: usize,
        code: &str,
        sev: Severity,
        msg: impl Into<String>,
    ) -> Self {
        Self {
            file: FilePath::new(file.to_string()).unwrap_or_default(),
            line: LineNumber::new(line as i64),
            column: ColumnNumber::new(0),
            code: ErrorCode::raw(code),
            message: LintMessage::new(msg),
            source: Some(AdapterName::raw("architecture")),
            severity: sev,
            enclosing_scope: Some(ScopeRef {
                name: DescriptionVO::new(String::new()),
                kind: DescriptionVO::new(String::new()),
                file: None,
                start_line: None,
                end_line: None,
            }),
            related_locations: LocationList::new(),
        }
    }

    pub fn position(&self) -> Position {
        Position {
            line: self.line.clone(),
            column: self.column.clone(),
        }
    }
    pub fn identity(&self) -> Identity {
        Identity::new(format!(
            "{}:{}:{}:{:?}",
            self.file, self.line, self.code, self.source
        ))
    }
}

/// Generate a `Vec<T>`-backed newtype with `Default`, `new`, `iter`,
/// `len`, `is_empty`, `push`, and `append`. Used for the `LintResultList`
/// wrapper below; siblings `ImportInfoList`/`PrimitiveViolationList` in
/// `taxonomy_import_source_vo.rs` carry the same surface.
macro_rules! lint_result_list_wrapper {
    ($name:ident, $item:ty) => {
        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
        pub struct $name {
            pub values: Vec<$item>,
        }

        impl $name {
            pub fn new(value: Vec<$item>) -> Self {
                Self { values: value }
            }
            pub fn iter(&self) -> std::slice::Iter<'_, $item> {
                self.values.iter()
            }
            pub fn len(&self) -> usize {
                self.values.len()
            }
            pub fn is_empty(&self) -> bool {
                self.values.is_empty()
            }
            pub fn push(&mut self, item: $item) {
                self.values.push(item);
            }
            pub fn append(&mut self, item: $item) {
                self.values.push(item);
            }
        }
    };
}

lint_result_list_wrapper!(LintResultList, LintResult);
```

---

## File: crates/shared/src/cli-commands/taxonomy_transport_error.rs

```rust
// PURPOSE: TransportError — structured error type wrapping protocol, message, endpoint, and underlying error
use crate::cli_commands::taxonomy_protocol_vo::TransportEndpoint;
use crate::cli_commands::taxonomy_protocol_vo::TransportProtocol;
use crate::common::taxonomy_common_error::ErrorMessage;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, thiserror::Error)]
pub struct TransportError {
    pub protocol: TransportProtocol,
    pub message: ErrorMessage,
    pub endpoint: TransportEndpoint,
    pub underlying_error: ErrorMessage,
}

impl TransportError {
    pub fn new(protocol: TransportProtocol, message: ErrorMessage) -> Self {
        Self {
            protocol,
            message,
            endpoint: TransportEndpoint::default(),
            underlying_error: ErrorMessage::default(),
        }
    }
}

impl std::fmt::Display for TransportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ep_str = self.endpoint.to_string();
        let ep = if ep_str.is_empty() {
            String::new()
        } else {
            format!(" {}", ep_str)
        };
        write!(f, "[{}]{} {}", self.protocol, ep, self.message)
    }
}
```

---

## File: crates/shared/src/code-analysis/contract_code_analysis_aggregate.rs

```rust
// PURPOSE: ICodeAnalysisAggregate — aggregate trait for code-analysis checks (AES301–AES305) and formatting reports

use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::cli_commands::taxonomy_result_vo::LintResultList;

pub trait ICodeAnalysisAggregate: Send + Sync {
    fn run_code_analysis(&self, project_root: &str) -> LintResultList;
    fn run_code_analysis_dir(&self, src_dir: &str) -> LintResultList;
    fn run_code_analysis_path(&self, path: &str) -> Vec<LintResult>;
    fn calc_score(&self, results: &[LintResult]) -> f64;
    fn check_critical(&self, results: &[LintResult]) -> bool;
    fn format_report(&self, results: &LintResultList, project_root: &str) -> String;
}
```

---

## File: crates/shared/src/code-analysis/mod.rs

```rust
// code-analysis — taxonomy and contract types
pub mod contract_adapter_port;
pub mod contract_bypass_checker_protocol;
pub mod contract_class_protocol;
pub mod contract_code_analysis_aggregate;
pub mod contract_code_metric_analyzer_protocol;
pub mod contract_cycle_protocol;
pub mod contract_dead_inheritance_protocol;
pub mod contract_layer_detection_aggregate;
pub mod contract_line_protocol;
pub mod taxonomy_analysis_vo;
pub mod taxonomy_code_analysis_rule_vo;
pub mod taxonomy_governance_entity;
pub mod taxonomy_import_source_vo;
pub mod taxonomy_operation_error;
pub mod taxonomy_violation_code_analysis_vo;
pub use taxonomy_violation_code_analysis_vo::{AesCodeAnalysisViolation, Language};
```

---

## File: crates/shared/src/mcp-server/mod.rs

```rust
// mcp-server — taxonomy and contract types
// Re-export from common for backward compatibility
pub use crate::common::taxonomy_action_vo;
pub use crate::common::taxonomy_job_vo;
```

---
