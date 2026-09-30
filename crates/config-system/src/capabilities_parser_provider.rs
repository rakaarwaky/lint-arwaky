// PURPOSE: ConfigParserProvider — implements IConfigMergeProtocol (FR-003: Config Merger)
use shared::common::taxonomy_common_vo::ErrorMessage;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::contract_config_protocol::IConfigMergeProtocol;
use shared::config_system::taxonomy_config_language_vo::ConfigLanguage;
use shared::config_system::taxonomy_config_system_error::ConfigError;
use shared::config_system::taxonomy_config_system_vo::ArchitectureConfig;
use shared::config_system::taxonomy_config_system_vo::ConfigKey;
use shared::config_system::taxonomy_config_system_vo::ProjectConfig;
use shared::config_system::utility_config_parser::default_config_for_language;
use shared::filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct ConfigParserProvider {
    io: Arc<dyn IFileSystemIOProtocol>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IConfigMergeProtocol for ConfigParserProvider {
    /// FR-003: deserialize a YAML config file into a ProjectConfig.
    fn parse_yaml_config(&self, path: &FilePath) -> Result<ProjectConfig, ConfigError> {
        let p = &path.value;
        let err_path = path.clone();
        let content = match self.io.read_to_string(std::path::Path::new(p)) {
            Ok(c) => c,
            Err(e) => {
                let msg = if e.kind() == std::io::ErrorKind::NotFound {
                    "Failed to read config: file not found".to_string()
                } else {
                    format!("Failed to read config: {}", e)
                };
                return Err(ConfigError {
                    key: ConfigKey::new("yaml.parse"),
                    message: ErrorMessage::new(msg),
                    config_file: err_path,
                    ..Default::default()
                });
            }
        };
        let config: ProjectConfig =
            serde_yaml_ng::from_str(content.value()).map_err(|e| ConfigError {
                key: ConfigKey::new("yaml.parse"),
                message: ErrorMessage::new(format!("Failed to deserialize YAML config: {}", e)),
                config_file: err_path,
                ..Default::default()
            })?;
        Ok(config)
    }

    fn parse_config_yaml_with_warnings(&self, yaml_str: &str) -> (ArchitectureConfig, Vec<String>) {
        shared::config_system::utility_config_parser::parse_config_yaml_with_warnings(yaml_str)
    }

    /// FR-003: merge rules into layer definitions, injecting the embedded
    /// defaults when the config declares no layers.
    fn merge_config_with_defaults(
        &self,
        config: &ArchitectureConfig,
        language: ConfigLanguage,
    ) -> (ArchitectureConfig, Vec<String>) {
        let (merged_layers, _) = shared::config_system::utility_config_merger::merge_config(config);
        let mut merged = config.clone();
        merged.layers = merged_layers;
        let mut warnings = Vec::new();
        if merged.layers.is_empty() {
            merged.layers = default_config_for_language(language.as_str()).layers;
            warnings.push(
                "Config file had no architecture layers, using built-in defaults for layers only."
                    .to_string(),
            );
        }
        (merged, warnings)
    }

    /// FR-003: Parse the [tool.lint-arwaky] section of a TOML file.
    fn parse_toml_config(&self, path: &FilePath) -> Result<Option<ProjectConfig>, ConfigError> {
        let p = &path.value;
        let err_path = path.clone();
        let content = match self.io.read_to_string(std::path::Path::new(p)) {
            Ok(c) => c,
            Err(e) => {
                let msg = if e.kind() == std::io::ErrorKind::NotFound {
                    "Failed to read TOML: file not found".to_string()
                } else {
                    format!("Failed to read TOML: {}", e)
                };
                return Err(ConfigError {
                    key: ConfigKey::new("tool.lint-arwaky"),
                    message: ErrorMessage::new(msg),
                    config_file: err_path,
                    ..Default::default()
                });
            }
        };
        let toml_value: toml::Value = match toml::from_str(content.value()) {
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

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl ConfigParserProvider {
    /// Create a new config parser with filesystem IO dependency.
    pub fn new(io: Arc<dyn IFileSystemIOProtocol>) -> Self {
        Self { io }
    }
}
