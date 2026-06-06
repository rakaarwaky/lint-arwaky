use crate::taxonomy::ConfigError;
use crate::taxonomy::FilePath;
use crate::taxonomy::ProjectConfig;


pub trait IConfigParserPort: Send + Sync {
    fn parse_yaml_config(&self, path: &FilePath) -> Result<ProjectConfig, ConfigError>;
    fn parse_toml_config(&self, path: &FilePath) -> Option<Result<ProjectConfig, ConfigError>>;
}
