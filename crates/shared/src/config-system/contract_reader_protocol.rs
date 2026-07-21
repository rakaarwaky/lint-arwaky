// PURPOSE: IConfigReaderProtocol — protocol trait for reading configuration from external sources

use crate::common::taxonomy_path_vo::FilePath;
use crate::config_system::taxonomy_config_error::ConfigError;
use crate::config_system::taxonomy_config_language_vo::ConfigLanguage;
use crate::config_system::taxonomy_source_vo::ConfigSource;
use async_trait::async_trait;

#[async_trait]
pub trait IConfigReaderProtocol: Send + Sync {
    async fn read_config(
        &self,
        project_root: &FilePath,
        language: ConfigLanguage,
    ) -> Result<Option<ConfigSource>, ConfigError>;

    async fn list_config_files(
        &self,
        project_root: &FilePath,
    ) -> Result<Vec<(ConfigLanguage, FilePath)>, ConfigError>;
}
