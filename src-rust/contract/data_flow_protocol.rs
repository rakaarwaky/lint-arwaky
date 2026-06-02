use super::*;

pub trait IDataFlowProtocol: Send + Sync {
    fn find_flow(&self, file_path: &FilePath, var_name: &SymbolName, start_line: LineNumber) -> Vec<String>;
}
