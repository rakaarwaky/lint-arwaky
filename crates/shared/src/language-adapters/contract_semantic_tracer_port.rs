// PURPOSE: ISemanticTracerPort — port for semantic scope tracing across languages
use async_trait::async_trait;
use crate::language_adapters::taxonomy_semantic_error::SemanticError;
use crate::language_adapters::taxonomy_naming_list_vo::{CallChainList, SymbolNameList};
use crate::taxonomy_name_vo::{ScopeRef, SymbolName};
use crate::taxonomy_common_vo::LineNumber;
use crate::source_parsing::taxonomy_path_vo::{DirectoryPath, FilePath};

#[async_trait]
pub trait ISemanticTracerPort: Send + Sync {
    async fn get_enclosing_scope(
        &self,
        file_path: &FilePath,
        line: LineNumber,
    ) -> Result<Option<ScopeRef>, SemanticError>;

    async fn trace_call_chain(
        &self,
        root_dir: &DirectoryPath,
        target_name: &SymbolName,
    ) -> Result<CallChainList, SemanticError>;
}
