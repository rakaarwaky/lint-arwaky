// PURPOSE: ConfigParserProvider — IConfigParserPort implementation for YAML and TOML config parsing
use config_system::contract_parser_port::IConfigParserPort;
use config_system::taxonomy_config_error::ConfigError;
use config_system::taxonomy_identifier_vo::ConfigKey;
use config_system::taxonomy_setting_vo::ProjectConfig;
use shared_common::taxonomy_common_error::ErrorMessage;
use source_parsing::taxonomy_path_vo::FilePath;
use std::path::Path;

pub struct ConfigParserProvider {}

impl Default for ConfigParserProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigParserProvider {
    pub fn new() -> Self {
        Self {}
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
                    config_file: path.clone(),
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
                    config_file: path.clone(),
                    ..Default::default()
                });
            }
        };
        let json_value = serde_json::to_value(&raw).map_err(|e| ConfigError {
            key: ConfigKey::new("yaml.convert"),
            message: ErrorMessage::new(format!("Failed to convert YAML to JSON: {}", e)),
            config_file: path.clone(),
            ..Default::default()
        })?;
        let config: ProjectConfig =
            serde_json::from_value(json_value).map_err(|e| ConfigError {
                key: ConfigKey::new("yaml.parse"),
                message: ErrorMessage::new(format!("Failed to deserialize config: {}", e)),
                config_file: path.clone(),
                ..Default::default()
            })?;
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
                    config_file: path.clone(),
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
                    config_file: path.clone(),
                    ..Default::default()
                }));
            }
        };
        if let Some(tool_section) = toml_value.get("tool").and_then(|t| t.get("lint_arwaky")) {
            let json_value = match serde_json::to_value(tool_section) {
                Ok(v) => v,
                Err(e) => {
                    return Some(Err(ConfigError {
                        key: ConfigKey::new("toml.convert"),
                        message: ErrorMessage::new(format!(
                            "Failed to convert TOML to JSON: {}",
                            e
                        )),
                        config_file: path.clone(),
                        ..Default::default()
                    }));
                }
            };
            let config: ProjectConfig =
                serde_json::from_value(json_value).unwrap_or_else(|_| ProjectConfig::defaults());
            Some(Ok(config))
        } else {
            None
        }
    }
}
