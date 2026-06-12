// PURPOSE: IJavascriptScopePort — port for JavaScript scope detection
use async_trait::async_trait;
use crate::language_adapters::taxonomy_semantic_error::SemanticError;
use crate::taxonomy_name_vo::SymbolName;
use crate::taxonomy_name_vo::LineContentVO;

#[async_trait]
pub trait IJavascriptScopePort: Send + Sync {
    async fn detect_js_scope(
        &self,
        stripped_line: &LineContentVO,
    ) -> Result<Option<SymbolName>, SemanticError>;
}
