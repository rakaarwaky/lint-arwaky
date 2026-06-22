# Plan: Migrate MCP to Official SDK (`rmcp`)

## Why

- Official SDK dari modelcontextprotocol org (3.5k stars)
- `#[tool]` macro — auto-generate JSON Schema dari struct
- Built-in stdio transport — tidak perlu hand-roll JSON-RPC
- `ServerHandler` trait — proper lifecycle management
- Lebih sedikit boilerplate, lebih compliant dengan MCP spec

## Current → Target

| Before | After |
|---|---|
| Hand-rolled JSON-RPC parsing | `rmcp::transport::stdio` |
| Manual tool dispatch `match tool_name` | `#[tool]` macro auto-dispatch |
| Manual `tools/list` response | Auto-generated from `#[tool]` |
| Manual `initialize` response | `ServerHandler::get_info()` |
| `mcp-sdk-rs = "0.3.4"` (unused) | `rmcp = { version = "0.16", features = ["server"] }` |

## Step 1: Update Cargo.toml

### Workspace `Cargo.toml`
```toml
rmcp = { version = "0.16", features = ["server"] }
```

Remove `mcp-sdk-rs = "0.3.4"`.

### `mcp-server/Cargo.toml`
```toml
rmcp.workspace = true
```

## Step 2: Define Tool Structs

### `taxonomy_mcp_tool_vo.rs` (shared)
```rust
use rmcp::schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ExecuteCommandArgs {
    pub action: String,
    pub args: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListCommandsArgs {
    pub domain: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ReadSkillArgs {
    pub section: Option<String>,
}
```

## Step 3: Implement Server

### `agent_mcp_server.rs` (mcp-server)
```rust
use rmcp::{tool, tool_router, ServerHandler, ServiceExt, transport::stdio};
use rmcp::handler::server::wrapper::Parameters;

#[derive(Clone)]
pub struct LintArwakyServer {
    arch_linter: Arc<dyn IArchLintProtocol>,
}

#[tool_router(server_handler)]
impl LintArwakyServer {
    #[tool(description = "Execute any CLI command")]
    async fn execute_command(
        &self,
        Parameters(args): Parameters<ExecuteCommandArgs>,
    ) -> String {
        // route to CLI handler
    }

    #[tool(description = "List all available CLI commands")]
    async fn list_commands(
        &self,
        Parameters(args): Parameters<ListCommandsArgs>,
    ) -> String {
        // return command catalog
    }

    #[tool(description = "Read SKILL.md documentation")]
    async fn read_skill(
        &self,
        Parameters(args): Parameters<ReadSkillArgs>,
    ) -> String {
        // read SKILL.md
    }

    #[tool(description = "Check system health")]
    async fn health_check(&self) -> String {
        // check adapters
    }
}
```

## Step 4: Update Entry Point

### `root_mcp_main_entry.rs`
```rust
use rmcp::ServiceExt;
use rmcp::transport::stdio;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server = LintArwakyServer::new(arch_linter);
    let service = server.serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}
```

## Step 5: Remove Old Code

- Delete hand-rolled JSON-RPC handler
- Delete manual `tools/list` definition
- Delete manual `initialize` handler
- Remove `serde_json` from mcp-server deps (use rmcp's)

## File Changes

| Action | File |
|---|---|
| NEW | `shared/src/mcp-server/taxonomy_mcp_tool_vo.rs` |
| NEW | `mcp-server/src/agent_mcp_server.rs` |
| UPDATE | `mcp-server/Cargo.toml` |
| UPDATE | `root_mcp_main_entry.rs` |
| DELETE | Old JSON-RPC handler code |

## Example (from rmcp docs)

```rust
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct AddParams { a: i32, b: i32 }

#[derive(Clone)]
struct Calculator;

#[tool_router(server_handler)]
impl Calculator {
    #[tool(description = "Add two numbers")]
    fn add(&self, Parameters(AddParams { a, b }): Parameters<AddParams>) -> String {
        (a + b).to_string()
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let service = Calculator.serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}
```
