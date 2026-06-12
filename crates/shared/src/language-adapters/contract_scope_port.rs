// PURPOSE: ILanguageScopePort — port trait for scope analysis
use crate::common::taxonomy_common_vo::LineContentList;
use crate::common::taxonomy_common_vo::LineNumber;
use crate::common::taxonomy_layer_vo::LineContentVO;
use crate::common::taxonomy_lint_vo::ScopeBounds;
use crate::common::taxonomy_name_vo::SymbolName;
use crate::language_adapters::taxonomy_semantic_error::SemanticError;
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

#[async_trait]
pub trait IJsTracerPort: Send + Sync {
    async fn show_enclosing_scope(
        &self,
        file_path: &crate::source_parsing::taxonomy_path_vo::FilePath,
        line: LineNumber,
    ) -> Result<Option<crate::common::taxonomy_lint_vo::ScopeRef>, SemanticError>;
}
