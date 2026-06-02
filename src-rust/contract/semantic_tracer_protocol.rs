use crate::taxonomy::{FilePath, DirectoryPath, SymbolName, SymbolNameList, LineNumber, ScopeRef, Count};

pub trait ISemanticTracerProtocol: Send + Sync {
    fn get_enclosing_scope(&self, file_path: &FilePath, line: LineNumber) -> Option<ScopeRef>;
    fn trace_call_chain(&self, root_dir: &DirectoryPath, target_name: &SymbolName) -> Vec<SymbolName>;
    fn find_flow(&self, file_path: &FilePath, var_name: &SymbolName, start_line: LineNumber) -> Vec<String>;
    fn get_variant_dict(&self, name: &SymbolName) -> serde_json::Value;
    fn project_wide_rename(&self, root_dir: &DirectoryPath, old_name: &SymbolName, new_name: &SymbolName) -> Count;
    fn get_symbol_locations(&self, file_path: &FilePath, symbol: &SymbolName) -> Vec<serde_json::Value>;
    fn build_variants(&self, name: &SymbolName) -> SymbolNameList;
}
