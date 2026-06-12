// PURPOSE: IConfigParserPort — contract for config parser provider (YAML and TOML)
use crate::config_system::taxonomy_config_error::ConfigError;
use crate::config_system::taxonomy_setting_vo::ProjectConfig;
use crate::source_parsing::taxonomy_path_vo::FilePath;

pub trait IConfigParserPort: Send + Sync {
    fn parse_yaml_config(&self, path: &FilePath) -> Result<ProjectConfig, ConfigError>;
    fn parse_toml_config(&self, path: &FilePath) -> Option<Result<ProjectConfig, ConfigError>>;
}
