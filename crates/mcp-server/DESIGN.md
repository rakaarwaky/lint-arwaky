# MCP Server — DESIGN

## Kind

`mcp` — serve the lint toolset over the Model Context Protocol on stdin/stdout.

## Entry points

| Function | Role |
|----------|------|
| `surface_mcp_action_command` | Handle one JSON-RPC 2.0 request and return its response. |
| `lint-arwaky-mcp` | Process entry that reads requests from stdin. |

## States

| State | Condition |
|-------|-----------|
| `initialized` | The client completed the MCP handshake. |
| `tool_invoked` | A tool call was dispatched to the rule groups. |
| `error` | The request was malformed or the tool failed. |
