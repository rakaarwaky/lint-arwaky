// PURPOSE: ServerSchemas — JSON-RPC method schemas for all MCP endpoints
/* UNKNOWN: MAX_STRING_LENGTH */
use mcp_server::taxonomy_server_constant::MAX_STRING_LENGTH;

use serde_json::Value;

#[derive(Debug, Clone)]
pub struct ToolSchema {
    pub name: String,
    pub description: String,
    pub input_schema: Value,
    pub output_schema: Option<Value>,
}

impl ToolSchema {
    pub fn new(
        name: &str,
        description: &str,
        input_schema: Value,
        output_schema: Option<Value>,
    ) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            input_schema,
            output_schema,
        }
    }
}

pub fn build_tool_schemas() -> Vec<ToolSchema> {
    vec![
        ToolSchema::new(
            "lint_arwaky_exec",
            "Execute a lint-arwaky command.",
            serde_json::json!({
                "type": "object",
                "properties": {
                    "action": { "type": "string", "description": "Command to execute", "minLength": 1, "maxLength": MAX_STRING_LENGTH },
                    "args": { "type": "object", "description": "Command arguments", "properties": {
                        "path": { "type": "string", "description": "Target path" },
                        "format": { "type": "string", "enum": ["json", "text", "html", "sarif", "markdown"] }
                    }}
                },
                "required": ["action"]
            }),
            None,
        ),
        ToolSchema::new(
            "lint_arwaky_list_commands",
            "List all available lint-arwaky commands.",
            serde_json::json!({ "type": "object", "properties": { "domain": { "type": "string" } }, "additionalProperties": false }),
            None,
        ),
        ToolSchema::new(
            "lint_arwaky_check_status",
            "Check status of running lint jobs.",
            serde_json::json!({ "type": "object", "properties": { "job_id": { "type": "string" } }, "additionalProperties": false }),
            None,
        ),
        ToolSchema::new(
            "lint_arwaky_cancel_job",
            "Cancel a running lint job.",
            serde_json::json!({ "type": "object", "properties": { "job_id": { "type": "string", "minLength": 1 } }, "required": ["job_id"], "additionalProperties": false }),
            None,
        ),
        ToolSchema::new(
            "lint_arwaky_health_check",
            "Check overall system health.",
            serde_json::json!({ "type": "object", "properties": {}, "additionalProperties": false }),
            None,
        ),
        ToolSchema::new(
            "lint_arwaky_read_docs",
            "Read SKILL.md documentation sections.",
            serde_json::json!({ "type": "object", "properties": { "section": { "type": "string" } }, "additionalProperties": false }),
            None,
        ),
    ]
}
