// PURPOSE: IJavascriptScopePort, IJsTracerPort — ports for JavaScript scope detection and tracing
use async_trait::async_trait;
use crate::language_adapters::taxonomy_semantic_error::SemanticError;
use crate::taxonomy_lint_vo::ScopeRef;
use crate::taxonomy_name_vo::SymbolName;
use crate::taxonomy_layer_vo::LineContentVO;
use crate::taxonomy_common_vo::LineNumber;
use crate::source_parsing::taxonomy_path_vo::FilePath;

#[async_trait]
pub trait IJavascriptScopePort: Send + Sync {
    async fn detect_js_scope(
        &self,
        stripped_line: &LineContentVO,
    ) -> Result<Option<SymbolName>, SemanticError>;

    async fn find_scope_bounds(
        &self,
        stripped_line: &LineContentVO,
    ) -> Result<Option<(usize, usize)>, SemanticError>;
}

#[async_trait]
pub trait IJsTracerPort: Send + Sync {
    async fn show_enclosing_scope(
        &self,
        file_path: &FilePath,
        line: LineNumber,
    ) -> Result<Option<ScopeRef>, SemanticError>;
}
