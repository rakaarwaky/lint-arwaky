// AES025 — mcp-tool-schema-violation
// This infrastructure file defines an MCP tool with missing docstrings,
// untyped parameters, and invalid JSON Schema — violating all MCP tool requirements.

use serde_json::{json, Value};

/// MCP tool definitions — intentionally malformed to trigger AES025
pub fn build_invalid_mcp_tools() -> Vec<Value> {
    vec![
        // Missing docstring/description entirely
        json!({
            "name": "bad_tool_no_desc",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "path": { "type": "string" }
                }
            }
        }),
        // Untyped parameters — missing "type" on action field
        json!({
            "name": "bad_tool_untyped",
            "description": "A tool with untyped parameters",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "action": { "description": "No type defined!" },
                    "count": { "description": "Also no type!" }
                },
                "required": ["action", "count"]
            }
        }),
        // Invalid JSON Schema — "enum" with empty array, "minLength" on wrong type
        json!({
            "name": "bad_tool_schema",
            "description": "A tool with invalid schema keywords",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "format": { "type": "string", "enum": [] },
                    "count": { "type": "integer", "minLength": 1 }
                }
            }
        }),
    ]
}

pub struct InvalidMcpToolAdapter;
