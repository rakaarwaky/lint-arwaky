// PURPOSE: Port: Interface for Server
use crate::cli_transport::taxonomy_transport_error::TransportError;
use crate::naming_rules::taxonomy_name_vo::SymbolName;
use crate::pipeline_jobs::taxonomy_job_vo::ResponseData;
use crate::shared_common::taxonomy_common_error::ErrorMessage;
/// mcp_server_port — Port interface for MCP server lifecycle and tool management.
use crate::shared_common::taxonomy_suggestion_vo::DescriptionVO;
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
