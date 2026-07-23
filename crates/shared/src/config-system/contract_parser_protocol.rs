// PURPOSE: IConfigParserProtocol — contract for config parser provider (YAML and TOML)
use crate::common::taxonomy_path_vo::FilePath;
use crate::config_system::taxonomy_config_error::ConfigError;
use crate::config_system::taxonomy_setting_vo::ProjectConfig;

/// Protocol for parsing project configuration files.
///
/// Implementations handle both YAML and TOML formats and return a
/// [`ProjectConfig`] on success or a [`ConfigError`] on failure.
pub trait IConfigParserProtocol: Send + Sync {
    fn parse_yaml_config(&self, path: &FilePath) -> Result<ProjectConfig, ConfigError>;
    fn parse_toml_config(&self, path: &FilePath) -> Result<Option<ProjectConfig>, ConfigError>;
}
