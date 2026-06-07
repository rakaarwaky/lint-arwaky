use crate::taxonomy::{
    Count, DataFlowList, DirectoryPath, FilePath, LineNumber, ResponseData, ResponseDataList,
    ScopeRef, SymbolName, SymbolNameList,
};

pub trait ISemanticTracerProtocol: Send + Sync {
    fn get_enclosing_scope(&self, file_path: &FilePath, line: LineNumber) -> Option<ScopeRef>;
    fn trace_call_chain(
        &self,
        root_dir: &DirectoryPath,
        target_name: &SymbolName,
    ) -> SymbolNameList;
    fn find_flow(
        &self,
        file_path: &FilePath,
        var_name: &SymbolName,
        start_line: LineNumber,
    ) -> DataFlowList;
    fn get_variant_dict(&self, name: &SymbolName) -> ResponseData;
    fn project_wide_rename(
        &self,
        root_dir: &DirectoryPath,
        old_name: &SymbolName,
        new_name: &SymbolName,
    ) -> Count;
    fn get_symbol_locations(&self, file_path: &FilePath, symbol: &SymbolName) -> ResponseDataList;
    fn build_variants(&self, name: &SymbolName) -> SymbolNameList;
}
