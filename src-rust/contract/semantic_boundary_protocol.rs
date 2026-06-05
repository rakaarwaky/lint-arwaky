use crate::taxonomy::{FilePath, LineNumber, ScopeRef};

pub trait IScopeBoundaryResolverProtocol: Send + Sync {
    fn resolve_enclosing_scope(&self, file_path: &FilePath, line: LineNumber) -> Option<ScopeRef>;
}
