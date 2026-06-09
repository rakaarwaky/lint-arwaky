//! Port trait for detecting programming languages.
//!
//! Provides the outbound interface for identifying the primary
//! programming language used in a project directory.

use crate::project_setup::taxonomy_language_vo::LanguageSource;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use async_trait::async_trait;

#[async_trait]
pub trait ILanguageDetectorPort: Send + Sync {
    async fn detect_language(&self, project_root: &FilePath) -> LanguageSource;
}
