// PURPOSE: IJavascriptFlowPort — port for JavaScript data flow tracing
use async_trait::async_trait;
use crate::language_adapters::taxonomy_semantic_error::SemanticError;
use crate::taxonomy_common_vo::{DataFlowList, LineNumber};
use crate::taxonomy_name_vo::SymbolName;
use crate::source_parsing::taxonomy_path_vo::FilePath;

#[async_trait]
pub trait IJavascriptFlowPort: Send + Sync {
    async fn find_flow(
        &self,
        file_path: &FilePath,
        var_name: &SymbolName,
        start_line: Option<LineNumber>,
    ) -> Result<DataFlowList, SemanticError>;
}
