// PURPOSE: IArchComplianceProtocol + IScopeBoundaryProtocol — port traits for compliance checking and JS scope detection
use crate::naming_rules::taxonomy_name_vo::SymbolName;
use crate::output_report::taxonomy_result_vo::LintResultList;
use crate::shared_common::taxonomy_common_vo::LineNumber;
use crate::shared_common::taxonomy_layer_vo::LineContentVO;
use crate::shared_common::taxonomy_lint_vo::ScopeBounds;
use crate::shared_common::taxonomy_source_vo::ContentString;
use crate::source_parsing::taxonomy_path_vo::FilePath;

pub trait IArchComplianceProtocol: Send + Sync {
    fn execute(&self, path: &FilePath) -> LintResultList;
}

pub trait IScopeBoundaryProtocol: Send + Sync {
    fn detect_js_scope(&self, stripped_line: &LineContentVO) -> Option<SymbolName>;
    fn find_scope_bounds(
        &self,
        content: &ContentString,
        scope_line: Option<LineNumber>,
    ) -> ScopeBounds;
    fn get_enclosing_scope(&self, file_path: &FilePath, line: LineNumber) -> Option<SymbolName>;
}
