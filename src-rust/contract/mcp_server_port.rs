/// mcp_server_port — Port interface for MCP server lifecycle and tool management.
use crate::taxonomy::{DescriptionVO, ErrorMessage, ResponseData, SymbolName, TransportError};
use std::future::Future;
use std::pin::Pin;

pub type ToolHandler = Box<
    dyn Fn(ResponseData) -> Pin<Box<dyn Future<Output = Result<ResponseData, ErrorMessage>> + Send>>
        + Send
        + Sync,
>;

#[async_trait::async_trait]
pub trait IMcpServerPort: Send + Sync {
    /// Register a tool with the MCP server.
    fn register_tool(&self, name: SymbolName, description: DescriptionVO, handler: ToolHandler);

    /// Start the MCP server.
    async fn run_server(&self) -> Result<(), TransportError>;

    /// Stop the MCP server.
    fn stop_server(&self);
}
