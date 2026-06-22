# Plan: Proper MCP Server Implementation

## Current State (after cleanup)

MCP server sudah berjalan dengan 4 tools:
1. `execute_command` — wrap 18 CLI commands
2. `list_commands` — list from COMMAND_CATALOG
3. `read_skill` — read SKILL.md
4. `health_check` — check adapter availability

## CLI Commands (18 — wrapped by MCP)

| # | Action | Args | MCP Response Fields |
|---|---|---|---|
| 1 | `check` | `path?` | violations, report, score |
| 2 | `scan` | `path?` | violations, report, score |
| 3 | `fix` | `path?` | status, message |
| 4 | `ci` | `path?, threshold?` | status (pass/fail), score, threshold |
| 5 | `doctor` | — | checks[], adapters status |
| 6 | `orphan` | `path?` | status, path |
| 7 | `security` | `path?` | status, path |
| 8 | `duplicates` | `path?` | status, path |
| 9 | `dependencies` | `path?` | status, path |
| 10 | `version` | — | version, name |
| 11 | `adapters` | — | adapters[], enabled status |
| 12 | `install-hook` | — | status |
| 13 | `uninstall-hook` | — | status |
| 14 | `init` | `global?` | status |
| 15 | `install` | `sudo?` | status |
| 16 | `mcp-config` | `client?` | status |
| 17 | `config-show` | — | status |

## Architecture (AES Layers)

```
shared/mcp-server/
  taxonomy_mcp_request_vo.rs     ← MCP request/response VOs
  taxonomy_mcp_tool_vo.rs        ← tool definition VOs
  contract_server_port.rs        ← existing (IMcpServerPort)

mcp-server/
  infrastructure_server_wrapper.rs ← existing (McpServerWrapper)
  surface_command_controller.rs    ← existing (list_commands, read_skill)
  surface_health_controller.rs     ← existing (health_check)
  root_mcp_server_container.rs     ← wiring

root_mcp_main_entry.rs            ← MCP server entry (JSON-RPC 2.0 stdio)
```

## MCP Tools

| Tool | Input | Output |
|---|---|---|
| `execute_command` | `action: string`, `args: object` | JSON result |
| `list_commands` | `{}` | commands[] |
| `read_skill` | `section?: string` | content |
| `health_check` | `{}` | adapters, version |

## What's Done

- ✅ MCP server runs via JSON-RPC 2.0 over stdio
- ✅ 4 tools registered and functional
- ✅ 18 CLI commands wrapped via `execute_command`
- ✅ Proper AES architecture (agent entry → DI composition)
- ✅ Both CLI and MCP binaries compile clean

## What Could Be Improved (Future)

1. **AES compliance**: MCP entry imports from `code_analysis` and `import_rules` directly (root layer — OK)
2. **Error handling**: Currently uses `unwrap()` in several places — should use proper error propagation
3. **Streaming**: Could support `notifications/tools/list_changed` for dynamic tool registration
4. **Logging**: Could add structured logging via `tracing`
