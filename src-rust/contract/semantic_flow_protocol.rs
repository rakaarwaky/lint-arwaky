use crate::taxonomy::{DataFlowList, FilePath, LineNumber, SymbolName};

pub trait IDataFlowProtocol: Send + Sync {
    fn find_flow(
        &self,
        file_path: &FilePath,
        var_name: &SymbolName,
        start_line: LineNumber,
    ) -> DataFlowList;
}
