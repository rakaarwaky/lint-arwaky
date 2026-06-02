/// config_parser_provider — Helpers for parsing config files (YAML, TOML, Jinja2).
use crate::contract::IConfigParserPort;
use crate::taxonomy::{ConfigError, ConfigKey, ErrorMessage, FilePath, ProjectConfig};
use std::path::Path;

pub struct ConfigParserProvider;

impl ConfigParserProvider {
    pub fn new() -> Self {
        Self
    }
}

impl IConfigParserPort for ConfigParserProvider {
    fn parse_yaml_config(&self, path: &Path) -> Result<ProjectConfig, ConfigError> {
        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(e) => {
                return Err(ConfigError {
                    key: ConfigKey::new("yaml.parse"),
                    message: ErrorMessage::new(format!("Failed to read config: {}", e)),
                    config_file: FilePath::new(path.to_string_lossy().to_string()),
                    ..Default::default()
                });
            }
        };
        let raw: serde_yaml::Value = match serde_yaml::from_str(&content) {
            Ok(v) => v,
            Err(e) => {
                return Err(ConfigError {
                    key: ConfigKey::new("yaml.parse"),
                    message: ErrorMessage::new(format!("Failed to parse YAML: {}", e)),
                    config_file: FilePath::new(path.to_string_lossy().to_string()),
                    ..Default::default()
                });
            }
        };
        let config: ProjectConfig = serde_json::from_value(serde_json::to_value(&raw).unwrap())
            .unwrap_or_else(|_| ProjectConfig::defaults());
        Ok(config)
    }

    fn parse_toml_config(&self, path: &Path) -> Result<Option<ProjectConfig>, ConfigError> {
        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(e) => {
                return Err(ConfigError {
                    key: ConfigKey::new("tool.auto_linter"),
                    message: ErrorMessage::new(format!("Failed to read TOML: {}", e)),
                    config_file: FilePath::new(path.to_string_lossy().to_string()),
                    ..Default::default()
                });
            }
        };
        let toml_value: toml::Value = match content.parse() {
            Ok(v) => v,
            Err(e) => {
                return Err(ConfigError {
                    key: ConfigKey::new("tool.auto_linter"),
                    message: ErrorMessage::new(format!("Failed to parse TOML: {}", e)),
                    config_file: FilePath::new(path.to_string_lossy().to_string()),
                    ..Default::default()
                });
            }
        };
        if let Some(tool_section) = toml_value.get("tool").and_then(|t| t.get("auto_linter")) {
            let config: ProjectConfig =
                serde_json::from_value(serde_json::to_value(tool_section).unwrap())
                    .unwrap_or_else(|_| ProjectConfig::defaults());
            Ok(Some(config))
        } else {
            Ok(None)
        }
    }
}
