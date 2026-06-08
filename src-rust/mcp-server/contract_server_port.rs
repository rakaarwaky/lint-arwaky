/// mcp_server_port — Port interface for MCP server lifecycle and tool management.
use crate::shared_common::taxonomy_suggestion_vo::DescriptionVO;
use crate::shared_common::taxonomy_common_error::ErrorMessage;
use crate::pipeline_jobs::taxonomy_job_vo::ResponseData;
use crate::naming_rules::taxonomy_symbol_vo::SymbolName;
use /* UNKNOWN: TransportError */ crate::cli_transport::taxonomy_client_error::TransportError;
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
