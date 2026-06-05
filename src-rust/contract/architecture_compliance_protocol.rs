use crate::taxonomy::{FilePath, LineNumber, LintResultList, ScopeBounds, SymbolName};
use super::*;

pub trait IArchComplianceProtocol: Send + Sync {
    fn execute(&self, path: &FilePath) -> LintResultList;
}

pub trait IScopeBoundaryProtocol: Send + Sync {
    fn detect_js_scope(&self, stripped_line: &str) -> Option<SymbolName>;
    fn find_scope_bounds(&self, content: &str, scope_line: Option<LineNumber>) -> ScopeBounds;
    fn get_enclosing_scope(&self, file_path: &FilePath, line: LineNumber) -> Option<SymbolName>;
}
