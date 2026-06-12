// PURPOSE: ISemanticTracerPort — port for semantic scope tracing across languages
use async_trait::async_trait;
use crate::language_adapters::taxonomy_semantic_error::SemanticError;
use crate::taxonomy_name_vo::{ScopeRef, SymbolName};
use crate::taxonomy_common_vo::{LineNumber, DataFlowList};
use crate::taxonomy_name_vo::NameVariants;
use crate::pipeline_jobs::taxonomy_job_vo::ResponseData;
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
    ) -> Result<Vec<SymbolName>, SemanticError>;

    async fn find_flow(
        &self,
        file_path: &FilePath,
        var_name: &SymbolName,
        start_line: Option<LineNumber>,
    ) -> Result<DataFlowList, SemanticError>;

    async fn get_variant_dict(&self, name: &SymbolName) -> ResponseData;

    async fn project_wide_rename(
        &self,
        root_dir: &DirectoryPath,
        old_name: &SymbolName,
        new_name: &SymbolName,
    ) -> Result<Vec<FilePath>, SemanticError>;

    async fn get_symbol_locations(
        &self,
        root_dir: &DirectoryPath,
        target_name: &SymbolName,
    ) -> Result<Vec<FilePath>, SemanticError>;

    async fn build_variants(&self, name: &SymbolName) -> Vec<SymbolName>;
}
