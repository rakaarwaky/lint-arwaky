/home/raka/mcp-arwaky/lint-arwaky/crates/auto-fix
/home/raka/mcp-arwaky/lint-arwaky/crates/auto-fix/src
/home/raka/mcp-arwaky/lint-arwaky/crates/auto-fix/auto_fix_container.rs// PURPOSE: main entry point for lint-arwaky-mcp — initializes MCP server with stdio transport
use serde_json::{json, Value};
use std::sync::{Arc, Mutex};

/// MCP binary entry point for lint-arwaky-mcp.
pub struct McpMainEntry {}

use lint_arwaky::composition_root::CompositionRoot;
use lint_arwaky::di_containers::contract_service_aggregate::ServiceContainerAggregate;
use lint_arwaky::mcp_server::surface_tools_command;

struct ServerState {
    container: Arc<dyn ServiceContainerAggregate>,
}

async fn handle_request(request: Value, state: &Arc<Mutex<ServerState>>) -> Value {
    let method = request.get("method").and_then(|m| m.as_str()).unwrap_or("");
    let params = request.get("params").cloned().unwrap_or_else(|| json!({}));
    let id = request
        .get("id")
        .cloned()
        .unwrap_or(serde_json::Value::Null);

    match method {
        "initialize" => {
            json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": {
                    "protocolVersion": "2024-11-05",
                    "capabilities": {
                        "tools": {
                            "listChanged": false
                        }
                    },
                    "serverInfo": {
                        "name": "lint-arwaky",
                        "version": env!("CARGO_PKG_VERSION")
                    }
                }
            })
        }

        "tools/list" => {
            json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": {
                    "tools": [
                        {
                            "name": "execute_command",
                            "description": "Execute any CLI command. This is the primary tool.",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "action": {
                                        "type": "string",
                                        "description": "The command action to execute"
                                    },
                                    "args": {
                                        "type": "object",
                                        "description": "Additional arguments"
                                    }
                                },
                                "required": ["action"]
                            }
                        },
                        {
                            "name": "list_commands",
                            "description": "Lists all available CLI commands along with examples.",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "domain": {
                                        "type": "string",
                                        "description": "Command domain to list"
                                    }
                                }
                            }
                        },
                        {
                            "name": "commands_schema",
                            "description": "Retrieve the JSON schemas for the registered MCP tools.",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "tool_name": {
                                        "type": "string",
                                        "description": "Tool name to get schema for"
                                    }
                                }
                            }
                        },
                        {
                            "name": "read_skill_context",
                            "description": "Read this SKILL.md documentation by section or in its entirety.",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "section": {
                                        "type": "string",
                                        "description": "Section to read"
                                    }
                                }
                            }
                        },
                        {
                            "name": "health_check",
                            "description": "Check system health: adapters and system state.",
                            "inputSchema": {
                                "type": "object",
                                "properties": {}
                            }
                        }
                    ]
                }
            })
        }

        "tools/call" => {
            let tool_name = params.get("name").and_then(|n| n.as_str()).unwrap_or("");
            let arguments = params
                .get("arguments")
                .cloned()
                .unwrap_or_else(|| json!({}));

            let result = match tool_name {
                "execute_command" => {
                    let action = arguments
                        .get("action")
                        .and_then(|a| a.as_str())
                        .unwrap_or("")
                        .to_string();
                    let args = arguments.get("args").cloned();
                    let container = state.lock().unwrap().container.clone();
                    surface_tools_command::execute_command_tool(container, action, args).await
                }

                "list_commands" => {
                    let domain = arguments.get("domain").and_then(|d| d.as_str());
                    surface_tools_command::list_commands_tool(domain)
                }

                "commands_schema" => {
                    let tool_name = arguments.get("tool_name").and_then(|t| t.as_str());
                    surface_tools_command::commands_schema_tool(tool_name)
                }

                "read_skill_context" => {
                    let section = arguments.get("section").and_then(|s| s.as_str());
                    let project_root = std::env::current_dir()
                        .map(|p| p.to_string_lossy().to_string())
                        .unwrap_or_else(|_| ".".to_string());
                    surface_tools_command::read_skill_context_tool(section, &project_root)
                }

                "health_check" => surface_tools_command::health_check_tool(),

                _ => {
                    json!({"error": format!("Unknown tool: {}", tool_name)})
                }
            };

            json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": {
                    "content": [{
                        "type": "text",
                        "text": serde_json::to_string_pretty(&result).unwrap_or_default()
                    }]
                }
            })
        }

        _ => {
            json!({
                "jsonrpc": "2.0",
                "id": id,
                "error": {
                    "code": -32601,
                    "message": format!("Method not found: {}", method)
                }
            })
        }
    }
}

pub async fn run_server() {
    eprintln!("Lint Arwaky MCP Server v{}", env!("CARGO_PKG_VERSION"));
    eprintln!("Listening on stdin/stdout (JSON-RPC 2.0)");
    eprintln!("Press Ctrl+C to stop");

    let container: Arc<dyn ServiceContainerAggregate> = Arc::new(
        CompositionRoot::new(),
    );
    let state = Arc::new(Mutex::new(ServerState { container }));

    use tokio::io::{stdin, AsyncBufReadExt, BufReader};
    let stdin = stdin();
    let mut reader = BufReader::new(stdin);
    let mut line = String::new();

    loop {
        line.clear();
        match reader.read_line(&mut line).await {
            Ok(0) => break,
            Ok(_) => {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    continue;
                }
                match serde_json::from_str::<Value>(trimmed) {
                    Ok(request) => {
                        let response = handle_request(request, &state).await;
                        println!("{}", serde_json::to_string(&response).unwrap_or_default());
                    }
                    Err(e) => {
                        eprintln!("Failed to parse request: {}", e);
                        let error_response = json!({
                            "jsonrpc": "2.0",
                            "id": null,
                            "error": {
                                "code": -32700,
                                "message": format!("Parse error: {}", e)
                            }
                        });
                        println!(
                            "{}",
                            serde_json::to_string(&error_response).unwrap_or_default()
                        );
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading stdin: {}", e);
                break;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    run_server().await;
}
