// PURPOSE: IConfigReaderPort — port trait for reading configuration from external sources

use config_system::taxonomy_source_vo::ConfigSource;
use source_parsing::taxonomy_path_vo::FilePath;
use async_trait::async_trait;

#[async_trait]
pub trait IConfigReaderPort: Send + Sync {
    async fn read_config(&self, project_root: &FilePath, language: &str) -> Option<ConfigSource>;
    async fn list_config_files(&self, project_root: &FilePath) -> Vec<(String, String)>;
}
