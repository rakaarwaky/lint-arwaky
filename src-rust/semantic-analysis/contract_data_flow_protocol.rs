// PURPOSE: Protocol: Contract trait for Data Flow
use crate::naming_rules::taxonomy_name_vo::SymbolName;
use crate::shared_common::taxonomy_common_vo::DataFlowList;
use crate::shared_common::taxonomy_common_vo::LineNumber;
use crate::source_parsing::taxonomy_path_vo::FilePath;

pub trait IDataFlowProtocol: Send + Sync {
    fn find_flow(
        &self,
        file_path: &FilePath,
        var_name: &SymbolName,
        start_line: LineNumber,
    ) -> DataFlowList;
}
