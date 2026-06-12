// PURPOSE: IJsTracerPort — port for JavaScript scope tracing
use async_trait::async_trait;
use crate::language_adapters::taxonomy_semantic_error::SemanticError;
use crate::taxonomy_lint_vo::ScopeRef;
use crate::taxonomy_common_vo::LineNumber;
use crate::source_parsing::taxonomy_path_vo::FilePath;

#[async_trait]
pub trait IJsTracerPort: Send + Sync {
    async fn show_enclosing_scope(
        &self,
        file_path: &FilePath,
        line: LineNumber,
    ) -> Result<Option<ScopeRef>, SemanticError>;
}
