// PURPOSE: ConfigParserProvider — IConfigParserPort implementation for YAML and TOML config parsing
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::contract_parser_port::IConfigParserPort;
use shared::config_system::taxonomy_config_error::ConfigError;
use shared::config_system::taxonomy_identifier_vo::ConfigKey;
use shared::config_system::taxonomy_setting_vo::ProjectConfig;
use shared::taxonomy_common_error::ErrorMessage;
use std::path::Path;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct ConfigParserProvider {}

// ─── Block 2: Public Contract ─────────────────────────────
impl IConfigParserPort for ConfigParserProvider {
    fn parse_yaml_config(&self, path: &FilePath) -> Result<ProjectConfig, ConfigError> {
        let p = Path::new(&path.value);
        let err_path = path.clone();
        let content = match std::fs::read_to_string(p) {
            Ok(c) => c,
            Err(e) => {
                return Err(ConfigError {
                    key: ConfigKey::new("yaml.parse"),
                    message: ErrorMessage::new(format!("Failed to read config: {}", e)),
                    config_file: err_path,
                    ..Default::default()
                });
            }
        };
        let config: ProjectConfig = serde_yaml_ng::from_str(&content).map_err(|e| ConfigError {
            key: ConfigKey::new("yaml.parse"),
            message: ErrorMessage::new(format!("Failed to deserialize YAML config: {}", e)),
            config_file: err_path,
            ..Default::default()
        })?;
        Ok(config)
    }

    fn parse_toml_config(&self, path: &FilePath) -> Result<Option<ProjectConfig>, ConfigError> {
        let p = Path::new(&path.value);
        let err_path = path.clone();
        let content = match std::fs::read_to_string(p) {
            Ok(c) => c,
            Err(e) => {
                return Err(ConfigError {
                    key: ConfigKey::new("tool.lint-arwaky"),
                    message: ErrorMessage::new(format!("Failed to read TOML: {}", e)),
                    config_file: err_path,
                    ..Default::default()
                });
            }
        };
        let toml_value: toml::Value = match toml::from_str(&content) {
            Ok(v) => v,
            Err(e) => {
                return Err(ConfigError {
                    key: ConfigKey::new("tool.lint-arwaky"),
                    message: ErrorMessage::new(format!("Failed to parse TOML: {}", e)),
                    config_file: err_path,
                    ..Default::default()
                });
            }
        };
        let tool_section = toml_value
            .get("tool")
            .and_then(|t| t.get("lint-arwaky").or_else(|| t.get("lint_arwaky")));
        if let Some(tool_section) = tool_section {
            let json_value = serde_json::to_value(tool_section).map_err(|e| ConfigError {
                key: ConfigKey::new("toml.convert"),
                message: ErrorMessage::new(format!("Failed to convert TOML to JSON: {}", e)),
                config_file: err_path.clone(),
                ..Default::default()
            })?;
            let config: ProjectConfig =
                serde_json::from_value(json_value).map_err(|e| ConfigError {
                    key: ConfigKey::new("toml.parse"),
                    message: ErrorMessage::new(format!("Failed to deserialize TOML config: {}", e)),
                    config_file: err_path,
                    ..Default::default()
                })?;
            Ok(Some(config))
        } else {
            Ok(None)
        }
    }
}

// ─── Block 3: Constructors & Helpers ──────────────────────
impl ConfigParserProvider {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for ConfigParserProvider {
    fn default() -> Self {
        Self::new()
    }
}
