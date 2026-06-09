//! Port trait for reading configuration from external sources.
//!
//! Defines the outbound interface for reading configuration files
//! and listing available configuration sources for a project.

use crate::config_system::taxonomy_source_vo::ConfigSource;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use async_trait::async_trait;

#[async_trait]
pub trait IConfigReaderPort: Send + Sync {
    async fn read_config(&self, project_root: &FilePath, language: &str) -> Option<ConfigSource>;
    async fn list_config_files(&self, project_root: &FilePath) -> Vec<(String, String)>;
}
