// PURPOSE: ISemanticTracerPort — port for semantic scope tracing across languages
use crate::language_adapters::taxonomy_naming_list_vo::CallChainList;
use crate::language_adapters::taxonomy_naming_list_vo::SymbolNameList;
use crate::language_adapters::taxonomy_semantic_error::SemanticError;
use crate::pipeline_jobs::taxonomy_job_vo::ResponseData;
use crate::source_parsing::taxonomy_path_vo::DirectoryPath;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use crate::taxonomy_common_vo::Count;
use crate::taxonomy_common_vo::DataFlowList;
use crate::taxonomy_common_vo::LineNumber;
use crate::taxonomy_common_vo::ResponseDataList;
use crate::taxonomy_lint_vo::ScopeRef;
use crate::taxonomy_name_vo::SymbolName;
use async_trait::async_trait;

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

    async fn find_flow(
        &self,
        file_path: &FilePath,
        var_name: &SymbolName,
        start_line: LineNumber,
    ) -> DataFlowList;

    async fn get_variant_dict(&self, name: &SymbolName) -> ResponseData;

    async fn project_wide_rename(
        &self,
        root_dir: &DirectoryPath,
        old_name: &SymbolName,
        new_name: &SymbolName,
    ) -> Count;

    async fn get_symbol_locations(
        &self,
        file_path: &FilePath,
        symbol: &SymbolName,
    ) -> ResponseDataList;

    async fn build_variants(&self, name: &SymbolName) -> SymbolNameList;
}
