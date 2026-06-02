// javascript_flow_port — Port for JS variable flow tracking.
use crate::taxonomy::{DataFlowList, FilePath, LineNumber, SemanticError, SymbolName};
use async_trait::async_trait;

#[async_trait]
pub trait IJavascriptFlowPort: Send + Sync {
    /// Track lifecycle of a variable.
    async fn find_flow(
        &self,
        file_path: &FilePath,
        var_name: &SymbolName,
        start_line: Option<LineNumber>,
    ) -> Result<DataFlowList, SemanticError>;

    /// Trace data flow in a Javascript file.
    async fn trace_flow(&self, path: &FilePath) -> Result<DataFlowList, SemanticError>;
}
