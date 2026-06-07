use crate::taxonomy::{
    ContentString, FilePath, LineContentVO, LineNumber, LintResultList, ScopeBounds, SymbolName,
};

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
