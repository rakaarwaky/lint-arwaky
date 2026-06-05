use crate::taxonomy::ConfigError;
use crate::taxonomy::DirectoryPath;
use crate::taxonomy::FilePath;
use super::*;

pub trait IConfigDiscoveryPort: Send + Sync {
    fn find_env_file(&self, start: Option<&DirectoryPath>) -> Option<Result<FilePath, ConfigError>>;
    fn find_yaml_config(&self, start: Option<&DirectoryPath>) -> Option<Result<FilePath, ConfigError>>;
    fn find_toml_config(&self, start: Option<&DirectoryPath>) -> Option<Result<FilePath, ConfigError>>;
}
