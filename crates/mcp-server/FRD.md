# FRD — mcp-server

## System Overview

The mcp-server crate implements a Model Context Protocol (MCP) server that exposes the lint-arwaky pipeline as JSON-RPC tools accessible by AI agents and IDEs. It registers four MCP tools (execute command, list commands, read skill, health check) via the MCP protocol library, delegates command execution to the analysis pipeline orchestrator, and returns structured JSON responses. The crate follows the AES 7-layer architecture: the MCP server orchestrator (agent) implements the MCP server aggregate, the lint-arwaky MCP server (surface) registers tools and handles protocol, and the MCP container (root) wires all dependencies.

## Functional Requirements

### FR-001: Execute Command

- **Description**: Execute any lint-arwaky CLI command via the MCP interface. Supports actions: `check`, `scan`, `fix`, `ci`, `doctor`, `version`, `adapters`, `install-hook`, `uninstall-hook`, `init`, `install`, `mcp-config`, `config-show`, `orphan`, `security`, `duplicates`, `dependencies`.
- **Input**: Execute command args with action string and optional argument map (keys: path, threshold, client).
- **Output**: JSON string with `status`, `action`, and action-specific fields (e.g., `total_violations`, `results`, `error`).
- **Business Rules**:
  - `check` / `scan`: defaults path to `"."`, creates a scan request with scan mode set to scan, delegates to the analysis pipeline aggregate's run method.
  - `ci`: defaults path to `"."`, defaults threshold to 80, creates a scan request with CI scan mode and threshold, returns `pass` if violations == 0 else `fail`.
  - `fix`: returns placeholder success response.
  - `doctor`: checks availability of tools (`cargo`, `python3`, `ruff`, `mypy`, `bandit`, `node`, `git`) via `which`.
  - `adapters`: queries the external lint aggregate's adapter names and checks each via `which`.
  - `version`: returns the crate version.
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
- **Input**: List commands args with optional domain filter string.
- **Output**: JSON string with `commands` array (each: `name`, `description`, `example`) and `total` count.
- **Business Rules**:
  - If `domain` is provided and non-empty, only commands whose name contains the domain string are returned.
  - If `domain` is `None` or empty, all commands are returned.
  - Commands are sourced from the command catalog in the taxonomy layer.
- **Edge Cases**:
  - No commands match the domain filter: returns empty `commands` array with `total: 0`.
  - Empty catalog: returns empty array.
- **Error Handling**:
  - JSON serialization failure: returns empty string.

### FR-003: Read Skill

- **Description**: Read SKILL.md documentation by section, searching multiple candidate locations.
- **Input**: Read skill args with optional section filter string.
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

- **Description**: Register MCP tools and server info with the MCP protocol framework.
- **Input**: None (configured at construction).
- **Output**: Server info with protocol version, server name ("lint-arwaky"), version, and tools capability.
- **Business Rules**:
  - Tools are registered via the tool router: execute command, list commands, read skill, health check.
  - Server name is "lint-arwaky", version from the crate version.
  - Protocol version uses the default protocol version.
- **Edge Cases**:
  - None (static configuration).
- **Error Handling**:
  - None (declarative registration).

## API Contract

| Function | Input | Output | Description |
|----------|-------|--------|-------------|
| The MCP server orchestrator's execute command method | execute command args | JSON string | Execute a CLI command via MCP |
| The MCP server orchestrator's list commands method | list commands args | JSON string | List available commands |
| The MCP server orchestrator's read skill method | read skill args | JSON string | Read SKILL.md by section |
| The MCP server surface's health check method | none | JSON string | Check adapter availability |
| The MCP server surface's get info method | none | server info | Return MCP server metadata |
| The MCP container's default factory | none | MCP container | Wire all dependencies |

## Integration Points

- **Internal**:
  - The CLI commands crate: the analysis pipeline aggregate for running lint pipelines, the command catalog for command metadata.
  - The external lint crate: the external lint aggregate for adapter discovery.
  - The analysis, import rules, naming rules, orphan detection, role rules, and config system crates: wired via the MCP container for full pipeline support.
  - The shared crate: VOs, contracts, config types.
- **External**:
  - `rmcp` crate: MCP protocol framework (JSON-RPC, tool registration, server handler).
  - `serde_json`: JSON serialization/deserialization.
  - `tokio`: async runtime for process spawning (`which` commands).
  - System: `which` command, `cargo clippy --version` for adapter detection.

## Non-functional Requirements (Detailed)

- **Performance**: Standard operations (execute_command, list_commands, read_skill) should complete under 5 seconds. Health check involves multiple `which` subprocess calls; concurrent execution via async join.
- **Memory**: Server holds shared references to all pipeline orchestrators. Memory usage scales with the number of registered aggregates.
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
- [ ] MCP get info returns correct server name and version.
- [ ] All 4 tools are discoverable via MCP tool listing.

## Assumptions & Constraints

- The `rmcp` crate provides a stable MCP JSON-RPC protocol implementation.
- The analysis pipeline orchestrator correctly handles all scan mode variants.
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
