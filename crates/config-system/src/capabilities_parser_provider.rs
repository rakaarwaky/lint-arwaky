// PURPOSE: ConfigParserProvider — implements IConfigMergeProtocol (FR-003: Config Merger)
use shared_common::taxonomy_common_vo::ErrorMessage;
use shared_common::taxonomy_path_vo::FilePath;
use shared_config_system::contract_config_protocol::IConfigMergeProtocol;
use shared_config_system::taxonomy_config_language_vo::ConfigLanguage;
use shared_config_system::taxonomy_config_system_error::ConfigError;
use shared_config_system::taxonomy_config_system_vo::ArchitectureConfig;
use shared_config_system::taxonomy_config_system_vo::ConfigKey;
use shared_config_system::taxonomy_config_system_vo::ProjectConfig;
use shared_config_system::utility_config_parser::default_config_for_language;
use shared_filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
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
        shared_config_system::utility_config_parser::parse_config_yaml_with_warnings(yaml_str)
    }

    /// FR-003: lock the `architecture:` section to the embedded defaults.
    ///
    /// The AES business rules (layer definitions, per-rule scope/allowed/
    /// forbidden/mandatory/severity, naming) are fixed in the tool. A user
    /// config may only carry tool-policy fields: `thresholds`, `adapters`,
    /// `ignored_rules` (read on a separate path), and `ignored_paths`.
    ///
    /// The two rule-level fields a user *may* set — `enabled` and
    /// `exceptions` — are folded into the embedded defaults so a project
    /// can scope a rule without changing its body.
    fn merge_config_with_defaults(
        &self,
        config: &ArchitectureConfig,
        language: ConfigLanguage,
    ) -> (ArchitectureConfig, Vec<String>) {
        let embedded = default_config_for_language(language.as_str());
        let mut merged = embedded;
        fold_user_rule_toggles(&mut merged, config);
        let (merged_layers, _) =
            shared_config_system::utility_config_merger::merge_config(&merged);
        merged.layers = merged_layers;
        // User policy fields carry through; architecture business fields stay
        // from the embedded defaults.
        merged.ignored_paths = config.ignored_paths.clone();
        let mut warnings = Vec::new();
        if !config.layers.is_empty()
            || config.naming.word_count.value != 0
            || !config.enabled.value
        {
            warnings.push(
                "architecture section in user config is ignored: AES rules are fixed by the tool. \
                 Only `ignored_rules` and `ignored_paths` apply from the user config file."
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

/// Fold the user's rule-level toggles and exceptions into the embedded
/// defaults, keyed by rule code.
///
/// `enabled` and `exceptions` are the two fields a user config may set on a
/// rule; every other field (scope, allowed, forbidden, mandatory, severity,
/// …) stays at its embedded value. An unknown rule code is appended so the
/// toggle still reaches the checkers.
fn fold_user_rule_toggles(merged: &mut ArchitectureConfig, user: &ArchitectureConfig) {
    for user_rule in &user.rules {
        if let Some(embedded) = merged
            .rules
            .iter_mut()
            .find(|r| r.rule_type == user_rule.rule_type)
        {
            embedded.enabled = user_rule.enabled.clone();
            for val in &user_rule.exceptions.values {
                if !embedded.exceptions.values.contains(val) {
                    embedded.exceptions.values.push(val.clone());
                }
            }
        } else {
            merged.rules.push(user_rule.clone());
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
