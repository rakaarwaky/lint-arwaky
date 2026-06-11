// PURPOSE: IDiscoveryPort — port trait for discovering config files in project directory
use config_system::taxonomy_config_error::ConfigError;
use source_parsing::taxonomy_path_vo::DirectoryPath;
use source_parsing::taxonomy_path_vo::FilePath;

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
