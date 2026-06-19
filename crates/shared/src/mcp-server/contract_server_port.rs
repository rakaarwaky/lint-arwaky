// PURPOSE: IMcpServerPort — port trait for MCP server lifecycle operations
use crate::cli_commands::taxonomy_transport_error::TransportError;
use crate::common::taxonomy_common_error::ErrorMessage;
use crate::common::taxonomy_name_vo::SymbolName;
/// mcp_server_port — Port interface for MCP server lifecycle and tool management.
use crate::common::taxonomy_suggestion_vo::DescriptionVO;
use crate::mcp_server::taxonomy_job_vo::ResponseData;
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
