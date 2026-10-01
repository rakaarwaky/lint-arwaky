// PURPOSE: ConfigInit — FR-004 protocol implementation (capabilities layer)
//
// Owns the config-side requirements: default config initialization and
// ignore-rule management. Both are config lifecycle operations grouped
// into one seam per FR-004.

use shared_common::taxonomy_suggestion_vo::DescriptionVO;
use shared_filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use shared_git_hooks::contract_git_hooks_protocol::IConfigInitProtocol;
use shared_git_hooks::taxonomy_git_hooks_vo::HookIgnoreUpdateVO;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct ConfigInit {
    io: Arc<dyn IFileSystemIOProtocol>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IConfigInitProtocol for ConfigInit {
    fn initialize_config(&self, path: &str) -> DescriptionVO {
        let config_file = format!("{}/lint_arwaky.config.yaml", path);
        let config_path = std::path::Path::new(&config_file);
        if self.io.path_exists(config_path) {
            return DescriptionVO::new(format!("ALREADY_EXISTS:{}", config_file));
        }
        let default_config = "# Lint Arwaky Configuration\nignored_paths: []\n";
        match self.io.write_string(config_path, default_config) {
            Ok(()) => DescriptionVO::new(format!("Initialized {}", config_file)),
            Err(e) => DescriptionVO::new(format!("Failed to initialize config: {}", e)),
        }
    }

    fn update_ignore_rule(&self, request: HookIgnoreUpdateVO) -> DescriptionVO {
        let config_path = std::path::Path::new(&request.config_path);
        if !config_path.exists() {
            return DescriptionVO::new(format!(
                "Config file not found: {}. Run lint-arwaky-cli init first.",
                request.config_path
            ));
        }

        let content = match self.io.read_to_string(config_path) {
            Ok(c) => c,
            Err(e) => {
                return DescriptionVO::new(format!("Failed to read config: {}", e));
            }
        };

        let mut doc: serde_yaml_ng::Value = match serde_yaml_ng::from_str(&content.value) {
            Ok(v) => v,
            Err(e) => {
                return DescriptionVO::new(format!("Failed to parse config YAML: {}", e));
            }
        };

        let ignored_paths = doc
            .as_mapping_mut()
            .and_then(|m| m.get_mut(serde_yaml_ng::Value::String("ignored_paths".to_string())))
            .and_then(|v| v.as_sequence_mut());

        let ignored_paths = match ignored_paths {
            Some(p) => p,
            None => {
                return DescriptionVO::new("Config missing 'ignored_paths' key".to_string());
            }
        };

        let rule_value = serde_yaml_ng::Value::String(request.rule.clone());

        if request.remove {
            let before_len = ignored_paths.len();
            ignored_paths.retain(|v| v != &rule_value);
            if ignored_paths.len() == before_len {
                return DescriptionVO::new(format!("'{}' not found in ignore list", request.rule));
            }
        } else {
            if ignored_paths.contains(&rule_value) {
                return DescriptionVO::new(format!(
                    "'{}' already present in ignore list",
                    request.rule
                ));
            }
            ignored_paths.push(rule_value);
        }

        match serde_yaml_ng::to_string(&doc) {
            Ok(yaml_str) => {
                if let Err(e) = self.io.write_string(config_path, &yaml_str) {
                    return DescriptionVO::new(format!("Failed to write config: {}", e));
                }
                let verb = if request.remove { "Removed" } else { "Added" };
                DescriptionVO::new(format!("{} '{}' from ignore list", verb, request.rule))
            }
            Err(e) => DescriptionVO::new(format!("Failed to serialize config: {}", e)),
        }
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl ConfigInit {
    pub fn new(io: Arc<dyn IFileSystemIOProtocol>) -> Self {
        Self { io }
    }
}
