// PURPOSE: ISemanticTracerProtocol — protocol trait for semantic tracing across call chains
use crate::shared_common::taxonomy_name_vo::SymbolName;
use crate::shared_common::taxonomy_naming_list_vo::SymbolNameList;
use crate::pipeline_jobs::taxonomy_job_vo::ResponseData;
use crate::shared_common::taxonomy_common_vo::Count;
use crate::shared_common::taxonomy_common_vo::DataFlowList;
use crate::shared_common::taxonomy_common_vo::LineNumber;
use crate::shared_common::taxonomy_common_vo::ResponseDataList;
use crate::shared_common::taxonomy_lint_vo::ScopeRef;
use crate::source_parsing::taxonomy_path_vo::DirectoryPath;
use crate::source_parsing::taxonomy_path_vo::FilePath;

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
