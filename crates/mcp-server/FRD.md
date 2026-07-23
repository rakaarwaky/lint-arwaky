# FRD — mcp-server

## System Overview

The mcp-server crate implements a Model Context Protocol (MCP) server that exposes the lint-arwaky pipeline as JSON-RPC tools accessible by AI agents and IDEs. It registers four MCP tools (`execute_command`, `list_commands`, `read_skill`, `health_check`) via the `rmcp` crate, delegates command execution to the `AnalysisPipelineOrchestrator`, and returns structured JSON responses. The crate follows the AES 7-layer architecture: `McpServerOrchestrator` (agent) implements `IMcpServerAggregate`, `LintArwakyMcpServer` (surface) registers tools and handles protocol, and `McpContainer` (root) wires all dependencies.

## Functional Requirements

### FR-001: Execute Command

- **Description**: Execute any lint-arwaky CLI command via the MCP interface. Supports actions: `check`, `scan`, `fix`, `ci`, `doctor`, `version`, `adapters`, `install-hook`, `uninstall-hook`, `init`, `install`, `mcp-config`, `config-show`, `orphan`, `security`, `duplicates`, `dependencies`.
- **Input**: `ExecuteCommandArgs` with `action: String` and optional `args` map (keys: `path`, `threshold`, `client`).
- **Output**: JSON string with `status`, `action`, and action-specific fields (e.g., `total_violations`, `results`, `error`).
- **Business Rules**:
  - `check` / `scan`: defaults path to `"."`, creates `ScanRequest` with `ScanMode::Scan`, delegates to `IAnalysisPipelineAggregate::run`.
  - `ci`: defaults path to `"."`, defaults threshold to 80, creates `ScanRequest` with `ScanMode::Ci { threshold }`, returns `pass` if violations == 0 else `fail`.
  - `fix`: returns placeholder success response.
  - `doctor`: checks availability of tools (`cargo`, `python3`, `ruff`, `mypy`, `bandit`, `node`, `git`) via `which`.
  - `adapters`: queries `IExternalLintAggregate::adapter_names()` and checks each via `which`.
  - `version`: returns `CARGO_PKG_VERSION`.
  - `install-hook` / `uninstall-hook`: returns success messages.
  - `init` / `install` / `mcp-config` / `config-show`: returns status JSON.
  - `orphan` / `security` / `duplicates` / `dependencies`: returns action + path.
  - Unknown action: returns `{"error": "Unknown action: <action>"}`.
- **Edge Cases**:
  - Missing `path` argument: defaults to `"."`.
  - Missing `threshold` argument: defaults to 80.
  - Missing `client` argument: defaults to `"all"`.
  - Pipeline execution error: returns `{"error": "pipeline failed: <message>"}`.
  - JSON serialization failure: returns empty string.
- **Error Handling**:
  - Pipeline errors are caught and returned as JSON error objects.
  - Unknown actions return descriptive error messages.

### FR-002: List Commands

- **Description**: List available CLI commands with descriptions and examples, optionally filtered by domain.
- **Input**: `ListCommandsArgs` with optional `domain: Option<String>`.
- **Output**: JSON string with `commands` array (each: `name`, `description`, `example`) and `total` count.
- **Business Rules**:
  - If `domain` is provided and non-empty, only commands whose name contains the domain string are returned.
  - If `domain` is `None` or empty, all commands are returned.
  - Commands are sourced from `COMMAND_CATALOG` in `taxonomy_command_catalog_vo`.
- **Edge Cases**:
  - No commands match the domain filter: returns empty `commands` array with `total: 0`.
  - Empty catalog: returns empty array.
- **Error Handling**:
  - JSON serialization failure: returns empty string.

### FR-003: Read Skill

- **Description**: Read SKILL.md documentation by section, searching multiple candidate locations.
- **Input**: `ReadSkillArgs` with optional `section: Option<String>`.
- **Output**: JSON string with `content` (full or section-specific) or `error` if not found.
- **Business Rules**:
  - Search order for SKILL.md: `../SKILL.md`, `./SKILL.md`, `./SKILL.md`, XDG config dir (`~/.config/lint-arwaky/SKILL.md`).
  - If `section` is provided, extracts content between `## <section>` headers.
  - Returns full content if no section specified.
- **Edge Cases**:
  - SKILL.md not found in any location: returns `{"error": "SKILL.md not found", "searched": [...]}`.
  - Section not found in SKILL.md: returns `{"error": "Section '<name>' not found"}`.
  - Section is the last section (no following `## ` header): returns content to end of file.
- **Error Handling**:
  - File read failure: treated as file not found.

### FR-004: Health Check

- **Description**: Check system health by verifying availability of linter adapters and returning system state.
- **Input**: None.
- **Output**: JSON string with `version`, `adapters_available`, `adapters_total`, and `adapters` array (each: `name`, `language`, `status`).
- **Business Rules**:
  - Checks: `ruff` (python), `mypy` (python), `bandit` (python), `clippy` (rust), `eslint` (javascript).
  - `clippy` is checked via `cargo clippy --version` instead of `which`.
  - Other adapters are checked via `which <name>`.
  - Status is `"available"` or `"not_installed"`.
- **Edge Cases**:
  - All adapters missing: `adapters_available: 0`.
  - `which` command fails (e.g., not installed on system): treated as not found.
- **Error Handling**:
  - Process spawn failure: treated as adapter not found.
  - JSON serialization failure: returns empty string.

### FR-005: MCP Protocol Registration

- **Description**: Register MCP tools and server info with the `rmcp` protocol framework.
- **Input**: None (configured at construction).
- **Output**: `ServerInfo` with protocol version, server name (`"lint-arwaky"`), version, and `ToolsCapability`.
- **Business Rules**:
  - Tools are registered via `#[tool_router]` macro: `execute_command`, `list_commands`, `read_skill`, `health_check`.
  - Server name is `"lint-arwaky"`, version from `CARGO_PKG_VERSION`.
  - Protocol version uses `ProtocolVersion::default()`.
- **Edge Cases**:
  - None (static configuration).
- **Error Handling**:
  - None (declarative registration).

## Data Model / Entity Relationship

```
ExecuteCommandArgs (input VO)
  ├── action: String
  └── args: Option<Map<String, Value>>
        ├── "path": String
        ├── "threshold": u64
        └── "client": String

ListCommandsArgs (input VO)
  └── domain: Option<String>

ReadSkillArgs (input VO)
  └── section: Option<String>

ScanRequest (to pipeline)
  ├── target: ScanTarget (path)
  └── mode: ScanMode (Scan | Ci { threshold })

ServerInfo (MCP protocol)
  ├── protocol_version: ProtocolVersion
  ├── server_info: Implementation { name, version }
  └── capabilities: ServerCapabilities { tools }
```

## API Contract

| Function | Input | Output | Description |
|----------|-------|--------|-------------|
| `McpServerOrchestrator::execute_command(args)` | `Parameters<ExecuteCommandArgs>` | `String` (JSON) | Execute a CLI command via MCP |
| `McpServerOrchestrator::list_commands(args)` | `Parameters<ListCommandsArgs>` | `String` (JSON) | List available commands |
| `McpServerOrchestrator::read_skill(args)` | `Parameters<ReadSkillArgs>` | `String` (JSON) | Read SKILL.md by section |
| `LintArwakyMcpServer::health_check()` | None | `String` (JSON) | Check adapter availability |
| `LintArwakyMcpServer::get_info()` | None | `ServerInfo` | Return MCP server metadata |
| `McpContainer::new_default()` | None | `McpContainer` | Wire all dependencies |

## Integration Points

- **Internal**:
  - `cli-commands` crate: `IAnalysisPipelineAggregate` for running lint pipelines, `COMMAND_CATALOG` for command metadata.
  - `external-lint` crate: `IExternalLintAggregate` for adapter discovery.
  - `code-analysis`, `import-rules`, `naming-rules`, `orphan-detector`, `role-rules`, `config-system` crates: wired via `McpContainer` for full pipeline support.
  - `shared` crate: VOs, contracts, config types.
- **External**:
  - `rmcp` crate: MCP protocol framework (JSON-RPC, tool registration, server handler).
  - `serde_json`: JSON serialization/deserialization.
  - `tokio`: async runtime for process spawning (`which` commands).
  - System: `which` command, `cargo clippy --version` for adapter detection.

## Non-functional Requirements (Detailed)

- **Performance**: Standard operations (execute_command, list_commands, read_skill) should complete under 5 seconds. Health check involves multiple `which` subprocess calls; concurrent execution via `futures::future::join_all`.
- **Memory**: Server holds references to all pipeline orchestrators (`Arc<dyn ...>`). Memory usage scales with the number of registered aggregates.
- **Accuracy**: JSON responses must conform to MCP JSON-RPC standards. Tool discovery must be reliable for AI clients.
- **Concurrency**: MCP server is `Clone`-able; tool handlers are `async` and can handle concurrent requests.
- **Security**: The `execute_command` tool accepts arbitrary action strings; unknown actions return errors but do not execute arbitrary system commands.

## Test Scenarios / QA Checklist

- [ ] `execute_command` with `action: "check"` and valid path returns violation count and results.
- [ ] `execute_command` with `action: "check"` and no path defaults to `"."`.
- [ ] `execute_command` with `action: "ci"` and threshold returns `pass` or `fail`.
- [ ] `execute_command` with unknown action returns `{"error": "Unknown action: ..."}`.
- [ ] `execute_command` with pipeline error returns `{"error": "pipeline failed: ..."}`.
- [ ] `list_commands` with no domain filter returns all commands.
- [ ] `list_commands` with domain filter returns matching subset.
- [ ] `read_skill` with no section returns full SKILL.md content.
- [ ] `read_skill` with valid section returns section content.
- [ ] `read_skill` with nonexistent section returns error.
- [ ] `read_skill` when SKILL.md not found returns error with searched paths.
- [ ] `health_check` returns adapter availability for all 5 adapters.
- [ ] `health_check` with clippy installed returns `"available"`.
- [ ] `health_check` with missing adapter returns `"not_installed"`.
- [ ] MCP `get_info` returns correct server name and version.
- [ ] All 4 tools are discoverable via MCP tool listing.

## Assumptions & Constraints

- The `rmcp` crate provides a stable MCP JSON-RPC protocol implementation.
- The `AnalysisPipelineOrchestrator` correctly handles all `ScanMode` variants.
- System has `which` command available for adapter detection.
- SKILL.md exists in at least one of the searched candidate locations for `read_skill` to succeed.
- The MCP server runs within a Tokio async runtime.

## Glossary

- **MCP**: Model Context Protocol — a JSON-RPC standard for AI agent tool integration.
- **Tool**: An MCP-exposed function that AI clients can discover and invoke.
- **Adapter**: An external linter binary (ruff, mypy, clippy, eslint, etc.) used by the pipeline.
- **Pipeline**: The analysis pipeline that runs all lint rules against a target path.
- **Surface layer**: The outermost AES layer that handles protocol/framework concerns (MCP tool registration).

## Reference

- PRD: [PRD.md](../../PRD.md)
