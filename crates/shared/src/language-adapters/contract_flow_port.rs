// PURPOSE: IFlowProviderPort — port trait for data flow analysis across languages
use crate::common::taxonomy_common_vo::DataFlowList;
use crate::common::taxonomy_common_vo::LineNumber;
use crate::common::taxonomy_name_vo::SymbolName;
use crate::language_adapters::taxonomy_semantic_error::SemanticError;
use crate::source_parsing::taxonomy_path_vo::FilePath;
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
