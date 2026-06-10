// PURPOSE: Port: Interface for Discovery
use crate::config_system::taxonomy_config_error::ConfigError;
use crate::source_parsing::taxonomy_path_vo::DirectoryPath;
use crate::source_parsing::taxonomy_path_vo::FilePath;

pub trait IConfigDiscoveryPort: Send + Sync {
    fn find_env_file(&self, start: Option<&DirectoryPath>)
        -> Option<Result<FilePath, ConfigError>>;
    fn find_yaml_config(
        &self,
        start: Option<&DirectoryPath>,
    ) -> Option<Result<FilePath, ConfigError>>;
    fn find_toml_config(
        &self,
        start: Option<&DirectoryPath>,
    ) -> Option<Result<FilePath, ConfigError>>;
}
