use super::*;

pub trait IJSScopeTracerPort: Send + Sync {
    fn show_enclosing_scope(&self, file_path: &FilePath, line: LineNumber) -> Option<ScopeRef>;
}
