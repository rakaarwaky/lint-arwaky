# MCP Server — DESIGN

## Kind

`mcp` — the Model Context Protocol surface. Exposes the lint toolset to an MCP
client over JSON-RPC 2.0 on stdin/stdout, so an agent can run scans, read
configuration, and pull skill documentation without shelling out.

## Entry Points

| File | Tool | Purpose |
| --- | --- | --- |
| `surface_mcp_tool_command.rs` | `execute_command` | Run any CLI command. The primary tool. |
| `surface_mcp_tool_command.rs` | `list_commands` | List available commands, optionally filtered by domain. |
| `surface_mcp_tool_command.rs` | `read_skill` | Read a skill's documentation by section. |
| `surface_mcp_tool_command.rs` | `health_check` | Report adapter availability and system state. |
| `surface_mcp_tool_command.rs` | `get_config` | Return the effective architecture config for a path and language. |
| `surface_mcp_action_command.rs` | — | The JSON-RPC request loop: parse, route, respond. |

## Request Shape

A request arrives as one JSON-RPC 2.0 message on stdin. The action command
parses it, matches the method against the tool router, and returns a response on
stdout. Tool arguments are `Parameters<T>` structs, so a malformed argument is
rejected before any lint work starts.

## States

| State | Condition | Response |
| --- | --- | --- |
| `uninitialized` | A request arrived before the MCP handshake | A protocol error; the server waits |
| `ready` | The handshake completed and the tool router is live | Normal tool results |
| `busy` | A tool call is executing | The client waits; the server is single-threaded |
| `error` | The request was malformed, or the tool failed | A JSON-RPC error object carrying the cause |

## Error States

| Condition | Behaviour |
| --- | --- |
| Malformed JSON on stdin | Return a JSON-RPC parse error and keep the process alive |
| Unknown method | Return a method-not-found error; do not exit |
| Tool argument fails to deserialize | Return an invalid-params error naming the field |
| The underlying CLI command fails | Return the command's output and exit code as a successful tool result, so the client can read why |

The server never terminates on a client error. A malformed request is answered
and the loop continues, because an MCP client may recover and retry.

## Invariants

- Tools reach lint behavior through the CLI command layer, never by importing a
  capabilities file directly.
- stdout carries protocol frames only. Diagnostics go to stderr, so a stray log
  line cannot corrupt a response.
- A tool returns text the client can read directly; it does not return raw
  protocol objects.

## Change Checklist

Adding a tool means adding one `#[tool]` method with its `Parameters<T>` args
struct, registering it in the tool router, and adding a row to the table above.
Changing the transport touches `surface_mcp_action_command.rs` only.
