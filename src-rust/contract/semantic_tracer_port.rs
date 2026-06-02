// semantic_tracer_port — Protocol interface for semantic analysis capabilities.
// Infrastructure implements this. Capabilities consume it via DI.
use crate::taxonomy::{
    CallChainList, Count, DataFlowList, DirectoryPath, FilePath, LineNumber, ResponseData,
    ResponseDataList, ScopeRef, SemanticError, SymbolName, SymbolNameList,
};
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
    ) -> u32;

    /// Return the locations of a symbol within a file.
    async fn get_symbol_locations(
        &self,
        file_path: &FilePath,
        symbol: &SymbolName,
    ) -> Vec<ResponseData>;

    /// Produce common naming variants for a given name.
    async fn build_variants(&self, name: &SymbolName) -> Vec<SymbolName>;
}
