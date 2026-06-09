//! Port trait for parsing configuration files.
//!
//! Provides outbound interfaces for parsing YAML and TOML
//! configuration formats into structured project configuration.

use crate::config_system::taxonomy_provider_error::ConfigError;
/* UNKNOWN: ProjectConfig */
use crate::config_system::taxonomy_setting_vo::ProjectConfig;
use crate::source_parsing::taxonomy_path_vo::FilePath;

pub trait IConfigParserPort: Send + Sync {
    fn parse_yaml_config(&self, path: &FilePath) -> Result<ProjectConfig, ConfigError>;
    fn parse_toml_config(&self, path: &FilePath) -> Option<Result<ProjectConfig, ConfigError>>;
}
