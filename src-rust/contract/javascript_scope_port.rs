// javascript_scope_port — Port for JS/TS scope detection.
use crate::taxonomy::{LineContentList, LineContentVO, LineNumber, ScopeBounds, SemanticError, SymbolName};
use async_trait::async_trait;

#[async_trait]
pub trait IJavascriptScopePort: Send + Sync {
    /// Detect if a line opens a named scope (class, function).
    async fn detect_js_scope(
        &self,
        stripped_line: &LineContentVO,
    ) -> Result<Option<SymbolName>, SemanticError>;

    /// Find start/end line numbers of enclosing function body.
    async fn find_scope_bounds(
        &self,
        lines: &LineContentList,
        scope_line: Option<LineNumber>,
    ) -> Result<Option<ScopeBounds>, SemanticError>;
}
