use crate::contract::config_parser_port::IConfigParserPort;
use crate::taxonomy::{ConfigError, ConfigKey, ErrorMessage, FilePath, ProjectConfig};
use std::path::Path;

pub struct ConfigParserProvider;

impl Default for ConfigParserProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigParserProvider {
    pub fn new() -> Self {
        Self
    }
}

impl IConfigParserPort for ConfigParserProvider {
    fn parse_yaml_config(&self, path: &FilePath) -> Result<ProjectConfig, ConfigError> {
        let p = Path::new(&path.value);
        let content = match std::fs::read_to_string(p) {
            Ok(c) => c,
            Err(e) => {
                return Err(ConfigError {
                    key: ConfigKey::new("yaml.parse"),
                    message: ErrorMessage::new(format!("Failed to read config: {}", e)),
                    config_file: Some(path.clone()),
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
                    config_file: Some(path.clone()),
                    ..Default::default()
                });
            }
        };
        let config: ProjectConfig = serde_json::from_value(serde_json::to_value(&raw).unwrap_or_default())
            .unwrap_or_else(|_| ProjectConfig::defaults());
        Ok(config)
    }

    fn parse_toml_config(&self, path: &FilePath) -> Option<Result<ProjectConfig, ConfigError>> {
        let p = Path::new(&path.value);
        let content = match std::fs::read_to_string(p) {
            Ok(c) => c,
            Err(e) => {
                return Some(Err(ConfigError {
                    key: ConfigKey::new("tool.lint_arwaky"),
                    message: ErrorMessage::new(format!("Failed to read TOML: {}", e)),
                    config_file: Some(path.clone()),
                    ..Default::default()
                }));
            }
        };
        let toml_value: toml::Value = match content.parse() {
            Ok(v) => v,
            Err(e) => {
                return Some(Err(ConfigError {
                    key: ConfigKey::new("tool.lint_arwaky"),
                    message: ErrorMessage::new(format!("Failed to parse TOML: {}", e)),
                    config_file: Some(path.clone()),
                    ..Default::default()
                }));
            }
        };
        if let Some(tool_section) = toml_value.get("tool").and_then(|t| t.get("lint_arwaky")) {
            let config: ProjectConfig =
                serde_json::from_value(serde_json::to_value(tool_section).unwrap_or_default())
                    .unwrap_or_else(|_| ProjectConfig::defaults());
            Some(Ok(config))
        } else {
            None
        }
    }
}
