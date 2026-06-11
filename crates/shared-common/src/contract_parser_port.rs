// PURPOSE: IConfigParserPort + ITomlConfigParserPort — port traits for parsing YAML and TOML config formats

use config_system::taxonomy_config_error::ConfigError;
/* UNKNOWN: ProjectConfig */
use config_system::taxonomy_setting_vo::ProjectConfig;
use source_parsing::taxonomy_path_vo::FilePath;

pub trait IConfigParserPort: Send + Sync {
    fn parse_yaml_config(&self, path: &FilePath) -> Result<ProjectConfig, ConfigError>;
    fn parse_toml_config(&self, path: &FilePath) -> Option<Result<ProjectConfig, ConfigError>>;
}
