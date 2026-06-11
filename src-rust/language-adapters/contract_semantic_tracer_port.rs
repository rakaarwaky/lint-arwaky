// PURPOSE: ISemanticTracerProvider — port trait for providing semantic trace operations

use crate::language_adapters::taxonomy_naming_list_vo::CallChainList;
use crate::language_adapters::taxonomy_naming_list_vo::SymbolNameList;
use crate::language_adapters::taxonomy_semantic_error::SemanticError;
use crate::pipeline_jobs::taxonomy_job_vo::ResponseData;
use crate::shared_common::taxonomy_common_vo::Count;
use crate::shared_common::taxonomy_common_vo::DataFlowList;
use crate::shared_common::taxonomy_common_vo::LineNumber;
use crate::shared_common::taxonomy_common_vo::ResponseDataList;
use crate::shared_common::taxonomy_lint_vo::ScopeRef;
use crate::shared_common::taxonomy_name_vo::SymbolName;
use crate::source_parsing::taxonomy_path_vo::DirectoryPath;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use async_trait::async_trait;

#[async_trait]
pub trait ISemanticTracerPort: Send + Sync {
    /// Return the name of the function or class enclosing the given line.
    async fn get_enclosing_scope(
        &self,
        file_path: &FilePath,
        line: LineNumber,
    ) -> Result<Option<ScopeRef>, SemanticError>;

    /// Find all call sites for the target name within the project.
    async fn trace_call_chain(
        &self,
        root_dir: &DirectoryPath,
        target_name: &SymbolName,
    ) -> Result<CallChainList, SemanticError>;

    /// Track the lifecycle of a variable within a file.
    async fn find_flow(
        &self,
        file_path: &FilePath,
        var_name: &SymbolName,
        start_line: LineNumber,
    ) -> DataFlowList;

    /// Return naming variants (camelCase, snake_case, etc.) for a name.
    async fn get_variant_dict(&self, name: &SymbolName) -> ResponseData;

    /// Rename a symbol across all files in the project.
    async fn project_wide_rename(
        &self,
        root_dir: &DirectoryPath,
        old_name: &SymbolName,
        new_name: &SymbolName,
    ) -> Count;

    /// Return the locations of a symbol within a file.
    async fn get_symbol_locations(
        &self,
        file_path: &FilePath,
        symbol: &SymbolName,
    ) -> ResponseDataList;

    /// Produce common naming variants for a given name.
    async fn build_variants(&self, name: &SymbolName) -> SymbolNameList;
}
