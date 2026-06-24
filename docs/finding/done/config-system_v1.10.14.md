# Crate: config-system (v1.10.14)

This document contains the source code for feature crate `config-system` along with its corresponding and imported definitions from the `shared` crate.

## Problem Statement

The following issues were detected by `lint-arwaky-cli scan`:

```
============================================================
  AES Architecture Compliance Report
============================================================
  Project: /home/raka/mcp-arwaky/lint-arwaky/crates/config-system
  Violations: 2
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/config-system/src/infrastructure_parser_provider.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/config-system/src/infrastructure_yaml_reader.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
```

---

## File List

- [crates/config-system/Cargo.toml](file:///home/raka/mcp-arwaky/lint-arwaky/crates/config-system/Cargo.toml)
- [crates/config-system/src/agent_config_loading_orchestrator.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/config-system/src/agent_config_loading_orchestrator.rs)
- [crates/config-system/src/agent_multi_project_orchestrator.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/config-system/src/agent_multi_project_orchestrator.rs)
- [crates/config-system/src/capabilities_rules_validator.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/config-system/src/capabilities_rules_validator.rs)
- [crates/config-system/src/infrastructure_parser_provider.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/config-system/src/infrastructure_parser_provider.rs)
- [crates/config-system/src/infrastructure_workspace_detector_provider.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/config-system/src/infrastructure_workspace_detector_provider.rs)
- [crates/config-system/src/infrastructure_yaml_reader.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/config-system/src/infrastructure_yaml_reader.rs)
- [crates/config-system/src/lib.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/config-system/src/lib.rs)
- [crates/config-system/src/root_config_system_container.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/config-system/src/root_config_system_container.rs)
- [crates/shared/src/common/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/mod.rs)
- [crates/shared/src/common/taxonomy_adapter_name_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_adapter_name_vo.rs)
- [crates/shared/src/common/taxonomy_common_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_common_vo.rs)
- [crates/shared/src/config-system/contract_multi_project_orchestrator_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/contract_multi_project_orchestrator_aggregate.rs)
- [crates/shared/src/config-system/contract_orchestration_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/contract_orchestration_aggregate.rs)
- [crates/shared/src/config-system/contract_parser_port.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/contract_parser_port.rs)
- [crates/shared/src/config-system/contract_reader_port.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/contract_reader_port.rs)
- [crates/shared/src/config-system/contract_validator_protocol.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/contract_validator_protocol.rs)
- [crates/shared/src/config-system/contract_workspace_detector_port.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/contract_workspace_detector_port.rs)
- [crates/shared/src/config-system/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/mod.rs)
- [crates/shared/src/config-system/taxonomy_adapter_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/taxonomy_adapter_vo.rs)
- [crates/shared/src/config-system/taxonomy_app_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/taxonomy_app_vo.rs)
- [crates/shared/src/config-system/taxonomy_config_error.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/taxonomy_config_error.rs)
- [crates/shared/src/config-system/taxonomy_config_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/taxonomy_config_vo.rs)
- [crates/shared/src/config-system/taxonomy_identifier_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/taxonomy_identifier_vo.rs)
- [crates/shared/src/config-system/taxonomy_multi_project_summary_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/taxonomy_multi_project_summary_vo.rs)
- [crates/shared/src/config-system/taxonomy_multi_project_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/taxonomy_multi_project_vo.rs)
- [crates/shared/src/config-system/taxonomy_multi_project_workspace_info_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/taxonomy_multi_project_workspace_info_vo.rs)
- [crates/shared/src/config-system/taxonomy_setting_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/taxonomy_setting_vo.rs)
- [crates/shared/src/config-system/taxonomy_source_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/taxonomy_source_vo.rs)
- [crates/shared/src/config-system/taxonomy_validation_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/taxonomy_validation_vo.rs)
- [crates/shared/src/source-parsing/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/mod.rs)
- [crates/shared/src/source-parsing/taxonomy_path_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/taxonomy_path_vo.rs)

---

## File: crates/config-system/Cargo.toml

```toml
[package]
name = "config_system-lint-arwaky"
version = "1.10.14"
edition = "2021"
description = "Configuration loading, parsing, validation, and workspace detection. Resolves `lint_arwaky.config.*.yaml` and merges it with project-level overrides."
license = "MIT"
repository = "https://github.com/rakaarwaky/lint-arwaky"
publish = false

[lints]
workspace = true

[dependencies]  # (unchanged)
serde.workspace = true
serde_json.workspace = true
async-trait.workspace = true
serde_yaml.workspace = true
toml.workspace = true
shared.workspace = true
dirs.workspace = true
```

---

## File: crates/config-system/src/agent_config_loading_orchestrator.rs

```rust
// PURPOSE: ConfigLoadingOrchestrator — orchestrates config discovery, loading, parsing across languages
use async_trait::async_trait;
use shared::config_system::contract_orchestration_aggregate::IConfigOrchestrationAggregate;
use shared::config_system::contract_reader_port::IConfigReaderPort;
use shared::config_system::contract_workspace_detector_port::IWorkspaceDetectorPort;
use shared::config_system::taxonomy_config_vo::default_config_for_language;
use shared::config_system::taxonomy_config_vo::parse_config_yaml;
use shared::config_system::taxonomy_source_vo::ConfigResult;
use shared::config_system::taxonomy_source_vo::ConfigSource;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use std::sync::Arc;

pub struct ConfigLoadingOrchestrator {
    workspace_detector: Arc<dyn IWorkspaceDetectorPort>,
    config_reader: Arc<dyn IConfigReaderPort>,
}

impl ConfigLoadingOrchestrator {
    pub fn new(
        workspace_detector: Arc<dyn IWorkspaceDetectorPort>,
        config_reader: Arc<dyn IConfigReaderPort>,
    ) -> Self {
        Self {
            workspace_detector,
            config_reader,
        }
    }
}

#[async_trait]
impl IConfigOrchestrationAggregate for ConfigLoadingOrchestrator {
    fn workspace_detector(&self) -> Arc<dyn IWorkspaceDetectorPort> {
        self.workspace_detector.clone()
    }

    fn config_reader(&self) -> Arc<dyn IConfigReaderPort> {
        self.config_reader.clone()
    }

    async fn load_project_config(&self, project_root: &FilePath) -> ConfigResult {
        let ws_type = self.workspace_detector.detect(project_root);
        let language = ws_type.as_str().to_string();
        self.load_config_for_language(project_root, &language).await
    }

    async fn load_config_for_language(
        &self,
        project_root: &FilePath,
        language: &str,
    ) -> ConfigResult {
        match self.config_reader.read_config(project_root, language).await {
            Some(source) => {
                let parsed = parse_config_yaml(&source.raw_content);
                if !parsed.layers.is_empty() {
                    ConfigResult::new(parsed, source, Vec::new())
                } else {
                    let warnings = vec![
                        "Config file had no architecture layers, using built-in defaults"
                            .to_string(),
                    ];
                    let config = default_config_for_language(language);
                    ConfigResult::new(config, source, warnings)
                }
            }
            None => {
                let warnings = vec!["No config file found, using built-in defaults".to_string()];
                let config = default_config_for_language(language);
                let source = ConfigSource::new(language, "embedded", "");
                ConfigResult::new(config, source, warnings)
            }
        }
    }
}
```

---

## File: crates/config-system/src/agent_multi_project_orchestrator.rs

```rust
use async_trait::async_trait;
use shared::config_system::contract_multi_project_orchestrator_aggregate::MultiProjectOrchestratorAggregate;
use shared::config_system::contract_reader_port::IConfigReaderPort;
use shared::config_system::contract_workspace_detector_port::IWorkspaceDetectorPort;
use shared::config_system::taxonomy_config_vo::default_config_for_language;
use shared::config_system::taxonomy_config_vo::parse_config_yaml;
use shared::config_system::taxonomy_multi_project_workspace_info_vo::WorkspaceInfo;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use std::sync::Arc;

pub struct MultiProjectOrchestrator {
    workspace_detector: Arc<dyn IWorkspaceDetectorPort>,
    config_reader: Arc<dyn IConfigReaderPort>,
}

impl MultiProjectOrchestrator {
    pub fn new(
        workspace_detector: Arc<dyn IWorkspaceDetectorPort>,
        config_reader: Arc<dyn IConfigReaderPort>,
    ) -> Self {
        Self {
            workspace_detector,
            config_reader,
        }
    }

    fn collect_subdirs(dir: &std::path::Path) -> Vec<FilePath> {
        let mut results = Vec::new();
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let sub = entry.path();
                if sub.is_dir() {
                    if let Ok(fp) = FilePath::new(sub.to_string_lossy().to_string()) {
                        results.push(fp);
                    }
                }
            }
        }
        results
    }

    fn scan_workspace_dirs(root: &std::path::Path) -> Vec<FilePath> {
        let workspace_dirs = ["crates", "packages", "modules"];

        let is_root_workspace_dir = match root.file_name() {
            Some(name) => {
                let name_str = name.to_string_lossy();
                workspace_dirs.contains(&name_str.as_ref())
            }
            None => false,
        };

        if is_root_workspace_dir {
            return Self::collect_subdirs(root);
        }

        let mut results = Vec::new();
        for dir in &workspace_dirs {
            let dir_path = root.join(dir);
            if dir_path.is_dir() {
                results.extend(Self::collect_subdirs(&dir_path));
            }
        }
        results
    }
}

#[async_trait]
impl MultiProjectOrchestratorAggregate for MultiProjectOrchestrator {
    async fn discover_workspaces(&self, root: &FilePath) -> Vec<WorkspaceInfo> {
        let root_path = std::path::Path::new(&root.value);
        let workspaces = Self::scan_workspace_dirs(root_path);

        let mut results = Vec::new();
        for ws in &workspaces {
            let ws_type = self.workspace_detector.detect(ws);
            let language = ws_type.as_str();

            let config = match self.config_reader.read_config(ws, language).await {
                Some(source) => {
                    let parsed = parse_config_yaml(&source.raw_content);
                    if !parsed.layers.is_empty() {
                        parsed
                    } else {
                        default_config_for_language(language)
                    }
                }
                None => default_config_for_language(language),
            };

            results.push(WorkspaceInfo::new(ws.clone(), language.to_string(), config));
        }

        results
    }
}
```

---

## File: crates/config-system/src/capabilities_rules_validator.rs

```rust
use shared::config_system::contract_validator_protocol::IConfigValidatorProtocol;
use shared::config_system::taxonomy_setting_vo::AdapterStatus;
use shared::config_system::taxonomy_setting_vo::ProjectConfig;
use shared::config_system::taxonomy_validation_vo::ValidationResult;
use shared::taxonomy_adapter_name_vo::AdapterName;

/// Business logic for interpreting and validating project configuration.
pub struct ConfigRulesValidator {
    config: ProjectConfig,
}

impl ConfigRulesValidator {
    pub fn new(config: ProjectConfig) -> Self {
        Self { config }
    }
}

impl IConfigValidatorProtocol for ConfigRulesValidator {
    /// Determines if a specific adapter should run based on configuration rules.
    fn is_adapter_enabled(&self, adapter_name: &AdapterName) -> bool {
        for adapter in &self.config.adapters {
            if adapter.name == *adapter_name {
                return adapter.status == AdapterStatus::Enabled;
            }
        }
        // Default policy: enabled if not explicitly mentioned
        true
    }

    /// Validates that scoring thresholds are sane.
    fn validate_thresholds(&self) -> ValidationResult {
        let t = &self.config.thresholds;

        // Score must be 0-100
        if !(0.0..=100.0).contains(&t.score.value) {
            return ValidationResult::fail("Score threshold must be between 0 and 100.");
        }

        // Complexity and line limits must be positive
        if t.complexity.value <= 0 {
            return ValidationResult::fail("Complexity threshold must be positive.");
        }

        if t.max_file_lines.value <= 0 {
            return ValidationResult::fail("max_file_lines threshold must be positive.");
        }

        ValidationResult::ok()
    }
}
```

---

## File: crates/config-system/src/infrastructure_parser_provider.rs

```rust
// PURPOSE: ConfigParserProvider — IConfigParserPort implementation for YAML and TOML config parsing
use shared::config_system::contract_parser_port::IConfigParserPort;
use shared::config_system::taxonomy_config_error::ConfigError;
use shared::config_system::taxonomy_identifier_vo::ConfigKey;
use shared::config_system::taxonomy_setting_vo::ProjectConfig;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use shared::taxonomy_common_error::ErrorMessage;
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
```

---

## File: crates/config-system/src/infrastructure_workspace_detector_provider.rs

```rust
// PURPOSE: WorkspaceDetector — IWorkspaceDetectorPort implementation for workspace type detection
use shared::config_system::contract_workspace_detector_port::IWorkspaceDetectorPort;
use shared::config_system::contract_workspace_detector_port::WorkspaceType;
use shared::source_parsing::taxonomy_path_vo::FilePath;

pub struct WorkspaceDetector;

impl WorkspaceDetector {
    pub fn new() -> Self {
        Self
    }
}

impl Default for WorkspaceDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl IWorkspaceDetectorPort for WorkspaceDetector {
    fn detect(&self, path: &FilePath) -> WorkspaceType {
        let path_buf = std::path::Path::new(&path.value).to_path_buf();

        // 1. Check for explicit language markers in the workspace directory itself
        if path_buf.join("Cargo.toml").exists() {
            return WorkspaceType::Rust;
        }
        if path_buf.join("package.json").exists() {
            return WorkspaceType::TypeScript;
        }
        if path_buf.join("pyproject.toml").exists()
            || path_buf.join("setup.py").exists()
            || path_buf.join("requirements.txt").exists()
        {
            return WorkspaceType::Python;
        }

        // 2. Check parent workspace folder context (crates/ → Rust, packages/ → TS, modules/ → Python)
        // This handles multi-language root dirs (e.g. test-workspaces/ which has all three).
        if let Some(parent) = path_buf.parent() {
            match parent.file_name().and_then(|n| n.to_str()) {
                Some("modules") => return WorkspaceType::Python,
                Some("packages") => return WorkspaceType::TypeScript,
                Some("crates") => return WorkspaceType::Rust,
                _ => {}
            }
        }

        // 3. Walk up parent chain looking for config files (fallback)
        let mut current = path_buf;
        while !current.as_os_str().is_empty() {
            if current.join("Cargo.toml").exists() {
                return WorkspaceType::Rust;
            }
            if current.join("package.json").exists() {
                return WorkspaceType::TypeScript;
            }
            if current.join("pyproject.toml").exists()
                || current.join("setup.py").exists()
                || current.join("requirements.txt").exists()
            {
                return WorkspaceType::Python;
            }
            if let Some(parent) = current.parent() {
                current = parent.to_path_buf();
            } else {
                break;
            }
        }

        WorkspaceType::Unknown
    }

    fn is_workspace(&self, path: &FilePath) -> bool {
        let root = std::path::Path::new(&path.value);
        ["crates", "packages", "modules"]
            .iter()
            .any(|dir| root.join(dir).is_dir())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workspace_detection_concept() {
        assert_eq!(WorkspaceType::Rust.as_str(), "rust");
        assert_eq!(WorkspaceType::TypeScript.as_str(), "typescript");
        assert_eq!(WorkspaceType::Python.as_str(), "python");
        assert_eq!(WorkspaceType::Unknown.as_str(), "unknown");
    }
}
```

---

## File: crates/config-system/src/infrastructure_yaml_reader.rs

```rust
// PURPOSE: ConfigYamlReader — reads and parses lint-arwaky YAML config files from disk
// XDG Base Directory Specification compliant config lookup
use async_trait::async_trait;
use shared::config_system::contract_reader_port::IConfigReaderPort;
use shared::config_system::taxonomy_source_vo::ConfigSource;
use shared::source_parsing::taxonomy_path_vo::FilePath;

pub struct ConfigYamlReader;

impl ConfigYamlReader {
    pub fn new() -> Self {
        Self
    }

    fn config_filename(language: &str) -> String {
        format!("lint_arwaky.config.{}.yaml", language)
    }

    /// Get XDG data directory for lint-arwaky ($XDG_DATA_HOME/lint-arwaky/)
    pub fn data_dir() -> Option<std::path::PathBuf> {
        dirs::data_dir().map(|d| d.join("lint-arwaky"))
    }

    /// Get XDG cache directory for lint-arwaky ($XDG_CACHE_HOME/lint-arwaky/)
    pub fn cache_dir() -> Option<std::path::PathBuf> {
        dirs::cache_dir().map(|d| d.join("lint-arwaky"))
    }

    /// Get XDG state directory for lint-arwaky ($XDG_STATE_HOME/lint-arwaky/)
    pub fn state_dir() -> Option<std::path::PathBuf> {
        dirs::state_dir().map(|d| d.join("lint-arwaky"))
    }

    /// Read config from XDG-compliant directories in priority order.
    /// Called after project-local lookup fails. Returns `None` to fall back to
    /// compiled-in defaults.
    ///
    /// Priority order (XDG Base Directory Specification):
    /// 1) `$XDG_CONFIG_HOME/lint-arwaky/`  (default `~/.config/lint-arwaky/`)
    /// 2) Each dir in `$XDG_CONFIG_DIRS`    (default `/etc/xdg/lint-arwaky/`)
    fn read_any(language: &str) -> Option<ConfigSource> {
        let filename = Self::config_filename(language);
        let mut candidates: Vec<std::path::PathBuf> = Vec::new();

        // Priority 1: XDG user config dir — $XDG_CONFIG_HOME (default ~/.config)
        if let Some(user_config) = dirs::config_dir() {
            candidates.push(user_config.join("lint-arwaky").join(&filename));
        }

        // Priority 2: XDG system config dirs — $XDG_CONFIG_DIRS (default /etc/xdg)
        // dirs crate doesn't expose config_dirs(), so parse env var manually
        let system_dirs =
            std::env::var("XDG_CONFIG_DIRS").unwrap_or_else(|_| "/etc/xdg".to_string());
        for dir in system_dirs.split(':').filter(|s| !s.is_empty()) {
            candidates.push(
                std::path::PathBuf::from(dir)
                    .join("lint-arwaky")
                    .join(&filename),
            );
        }

        for path in &candidates {
            if path.exists() {
                if let Ok(content) = std::fs::read_to_string(path) {
                    return Some(ConfigSource::new(
                        language,
                        path.to_string_lossy().to_string(),
                        content,
                    ));
                }
            }
        }
        None
    }
}

impl Default for ConfigYamlReader {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl IConfigReaderPort for ConfigYamlReader {
    /// Read config with priority order:
    /// 1) Project root directory
    /// 2) Parent of project root
    /// 3) XDG config dirs ($XDG_CONFIG_HOME, $XDG_CONFIG_DIRS)
    async fn read_config(&self, project_root: &FilePath, language: &str) -> Option<ConfigSource> {
        let filename = Self::config_filename(language);
        let mut current = std::path::PathBuf::from(&project_root.value);

        while !current.as_os_str().is_empty() {
            let candidate = current.join(&filename);
            if let Ok(content) = std::fs::read_to_string(&candidate) {
                return Some(ConfigSource::new(
                    language,
                    candidate.to_string_lossy().to_string(),
                    content,
                ));
            }

            if let Some(parent) = current.parent() {
                current = parent.to_path_buf();
            } else {
                break;
            }
        }

        // Fallback: XDG config dirs
        Self::read_any(language)
    }

    async fn list_config_files(&self, project_root: &FilePath) -> Vec<(String, String)> {
        let mut found = Vec::new();
        for lang in &["rust", "python", "javascript", "typescript"] {
            if let Some(config) = self.read_config(project_root, lang).await {
                found.push((lang.to_string(), config.path.to_string()));
            }
        }
        found
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xdg_config_dir() {
        let config_dir = dirs::config_dir();
        assert!(config_dir.is_some());
    }

    #[test]
    fn test_xdg_directories() {
        let data_dir = ConfigYamlReader::data_dir();
        assert!(data_dir.is_some());
        let data_path = data_dir.unwrap();
        assert!(data_path.ends_with("lint-arwaky"));

        let cache_dir = ConfigYamlReader::cache_dir();
        assert!(cache_dir.is_some());
        let cache_path = cache_dir.unwrap();
        assert!(cache_path.ends_with("lint-arwaky"));

        let state_dir = ConfigYamlReader::state_dir();
        assert!(state_dir.is_some());
        let state_path = state_dir.unwrap();
        assert!(state_path.ends_with("lint-arwaky"));
    }

    #[test]
    fn test_config_filename() {
        assert_eq!(
            ConfigYamlReader::config_filename("rust"),
            "lint_arwaky.config.rust.yaml"
        );
        assert_eq!(
            ConfigYamlReader::config_filename("python"),
            "lint_arwaky.config.python.yaml"
        );
    }
}
```

---

## File: crates/config-system/src/lib.rs

```rust
// PURPOSE: Module declarations for config-system (orchestrator, validators, providers)
pub mod agent_config_loading_orchestrator;
pub use agent_config_loading_orchestrator::ConfigLoadingOrchestrator;
pub mod capabilities_rules_validator;
pub use capabilities_rules_validator::ConfigRulesValidator;
pub mod infrastructure_workspace_detector_provider;
pub use infrastructure_workspace_detector_provider::WorkspaceDetector;
pub mod infrastructure_parser_provider;
pub use infrastructure_parser_provider::ConfigParserProvider;
pub mod infrastructure_yaml_reader;
pub use infrastructure_yaml_reader::ConfigYamlReader;
pub mod agent_multi_project_orchestrator;
pub use agent_multi_project_orchestrator::MultiProjectOrchestrator;
pub mod root_config_system_container;
```

---

## File: crates/config-system/src/root_config_system_container.rs

```rust
use shared::config_system::contract_multi_project_orchestrator_aggregate::MultiProjectOrchestratorAggregate;
use shared::config_system::contract_orchestration_aggregate::IConfigOrchestrationAggregate;
use shared::config_system::contract_parser_port::IConfigParserPort;
use shared::config_system::contract_validator_protocol::IConfigValidatorProtocol;
use std::sync::Arc;

pub struct ConfigContainer {
    orchestrator: Arc<dyn IConfigOrchestrationAggregate>,
    parser: Arc<dyn IConfigParserPort>,
    validator: Arc<dyn IConfigValidatorProtocol>,
    multi_project_orchestrator: Arc<dyn MultiProjectOrchestratorAggregate>,
}

impl Default for ConfigContainer {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigContainer {
    pub fn new() -> Self {
        let workspace_detector =
            Arc::new(crate::infrastructure_workspace_detector_provider::WorkspaceDetector::new());
        let yaml_reader = Arc::new(crate::infrastructure_yaml_reader::ConfigYamlReader::new());

        Self {
            orchestrator: Arc::new(
                crate::agent_config_loading_orchestrator::ConfigLoadingOrchestrator::new(
                    workspace_detector.clone(),
                    yaml_reader.clone(),
                ),
            ),
            parser: Arc::new(crate::infrastructure_parser_provider::ConfigParserProvider::new()),
            validator: Arc::new(
                crate::capabilities_rules_validator::ConfigRulesValidator::new(
                    shared::config_system::taxonomy_setting_vo::ProjectConfig::defaults(),
                ),
            ),
            multi_project_orchestrator: Arc::new(
                crate::agent_multi_project_orchestrator::MultiProjectOrchestrator::new(
                    workspace_detector,
                    yaml_reader,
                ),
            ),
        }
    }

    pub fn orchestrator(&self) -> Arc<dyn IConfigOrchestrationAggregate> {
        self.orchestrator.clone()
    }

    pub fn parser(&self) -> Arc<dyn IConfigParserPort> {
        self.parser.clone()
    }

    pub fn validator(&self) -> Arc<dyn IConfigValidatorProtocol> {
        self.validator.clone()
    }

    pub fn multi_project_orchestrator(&self) -> Arc<dyn MultiProjectOrchestratorAggregate> {
        self.multi_project_orchestrator.clone()
    }
}
```

---

## File: crates/shared/src/common/mod.rs

```rust
// common — truly shared types used by multiple features
pub mod taxonomy_action_vo;
pub mod taxonomy_adapter_name_vo;
pub mod taxonomy_common_error;
pub mod taxonomy_common_vo;
pub mod taxonomy_definition_vo;
pub mod taxonomy_duration_vo;
pub mod taxonomy_error_vo;
pub mod taxonomy_job_id_vo;
pub mod taxonomy_job_vo;
pub mod taxonomy_layer_vo;
pub mod taxonomy_lint_vo;
pub mod taxonomy_message_vo;
pub mod taxonomy_name_vo;
pub mod taxonomy_response_data_vo;
pub mod taxonomy_severity_vo;
pub mod taxonomy_source_vo;
pub mod taxonomy_suggestion_vo;
pub mod taxonomy_value_object_utility;
```

---

## File: crates/shared/src/common/taxonomy_adapter_name_vo.rs

```rust
// PURPOSE: AdapterName — validated newtype for adapter/linter name strings
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

/// adapter_name_vo — Adapter and tool identifier value objects.
///
/// Adapter/tool identifier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AdapterName {
    pub value: String,
}

impl AdapterName {
    pub fn value(&self) -> &str {
        &self.value
    }
    /// Create a new AdapterName from a string.
    ///
    /// # Errors
    /// Returns an error if the adapter name is empty or only whitespace.
    pub fn new<S: Into<String>>(value: S) -> Result<Self, String> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err("Adapter name cannot be empty".to_string());
        }
        Ok(AdapterName {
            value: value.trim().to_string(),
        })
    }

    /// Create a raw AdapterName without error validation (for static compile-time safe inputs).
    pub fn raw<S: Into<String>>(value: S) -> Self {
        AdapterName {
            value: value.into(),
        }
    }
}

impl std::ops::Deref for AdapterName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::fmt::Display for AdapterName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Hash for AdapterName {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::AdapterName;

    #[test]
    fn test_adapter_name_new() {
        let name = AdapterName::new("ruff").unwrap_or_default();
        assert_eq!(name.value, "ruff");

        // Test trimming
        let name = AdapterName::new("  ruff  ").unwrap_or_default();
        assert_eq!(name.value, "ruff");

        // Test that internal spaces are preserved
        let name = AdapterName::new("my adapter").unwrap_or_default();
        assert_eq!(name.value, "my adapter");
    }

    #[test]
    fn test_adapter_name_invalid() {
        assert!(AdapterName::new("").is_err());
        assert!(AdapterName::new("   ").is_err());
        assert!(AdapterName::new("\t\n  ").is_err());
    }
}
```

---

## File: crates/shared/src/common/taxonomy_common_vo.rs

```rust
// PURPOSE: BooleanVO, ColumnNumber, Count, DataFlowList, LineContentList, LineNumber, PatternList, Score, Timestamp — common VOs
use serde::{Deserialize, Serialize};

use crate::common::taxonomy_job_id_vo::JobId;
use crate::common::taxonomy_layer_vo::LineContentVO;
use crate::common::taxonomy_response_data_vo::ResponseData;
use crate::common::taxonomy_severity_vo::Severity;

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(transparent)]
#[derive(Default)]
pub struct BooleanVO {
    pub value: bool,
}

impl BooleanVO {
    pub fn new(value: bool) -> Self {
        Self { value }
    }
    pub fn value(&self) -> bool {
        self.value
    }
}

impl std::fmt::Display for BooleanVO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<bool> for BooleanVO {
    fn from(v: bool) -> Self {
        Self { value: v }
    }
}

impl<'de> serde::Deserialize<'de> for BooleanVO {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct BooleanVOVisitor {}
        impl<'de> serde::de::Visitor<'de> for BooleanVOVisitor {
            type Value = BooleanVO;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(BooleanVO { value: v })
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(k) = map.next_key::<String>()? {
                    if k == "value" {
                        value = Some(map.next_value::<bool>()?);
                    } else {
                        let _: serde::de::IgnoredAny = map.next_value()?;
                    }
                }
                let val = value.ok_or_else(|| serde::de::Error::missing_field("value"))?;
                Ok(BooleanVO { value: val })
            }
        }
        deserializer.deserialize_any(BooleanVOVisitor {})
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(transparent)]
#[derive(Default)]
pub struct ColumnNumber {
    pub value: i64,
}

impl ColumnNumber {
    pub fn new(value: i64) -> Self {
        Self { value }
    }
    pub fn value(&self) -> i64 {
        self.value
    }
}

impl std::fmt::Display for ColumnNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<i64> for ColumnNumber {
    fn from(v: i64) -> Self {
        Self { value: v }
    }
}

impl<'de> serde::Deserialize<'de> for ColumnNumber {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ColumnNumberVisitor {}
        impl<'de> serde::de::Visitor<'de> for ColumnNumberVisitor {
            type Value = ColumnNumber;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(ColumnNumber { value: v })
            }
            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(ColumnNumber { value: v as i64 })
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(k) = map.next_key::<String>()? {
                    if k == "value" {
                        value = Some(map.next_value::<i64>()?);
                    } else {
                        let _: serde::de::IgnoredAny = map.next_value()?;
                    }
                }
                let val = value.ok_or_else(|| serde::de::Error::missing_field("value"))?;
                Ok(ColumnNumber { value: val })
            }
        }
        deserializer.deserialize_any(ColumnNumberVisitor {})
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(transparent)]
#[derive(Default)]
pub struct Count {
    pub value: i64,
}

impl Count {
    pub fn new(value: i64) -> Self {
        Self { value }
    }
    pub fn value(&self) -> i64 {
        self.value
    }
}

impl std::fmt::Display for Count {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<i64> for Count {
    fn from(v: i64) -> Self {
        Self { value: v }
    }
}

impl<'de> serde::Deserialize<'de> for Count {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct CountVisitor {}
        impl<'de> serde::de::Visitor<'de> for CountVisitor {
            type Value = Count;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Count { value: v })
            }
            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Count { value: v as i64 })
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(k) = map.next_key::<String>()? {
                    if k == "value" {
                        value = Some(map.next_value::<i64>()?);
                    } else {
                        let _: serde::de::IgnoredAny = map.next_value()?;
                    }
                }
                let val = value.ok_or_else(|| serde::de::Error::missing_field("value"))?;
                Ok(Count { value: val })
            }
        }
        deserializer.deserialize_any(CountVisitor {})
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DataFlowList {
    pub values: Vec<ErrorMessage>,
}

impl DataFlowList {
    pub fn new(value: Vec<ErrorMessage>) -> Self {
        Self { values: value }
    }
    pub fn values(&self) -> &[ErrorMessage] {
        &self.values
    }
    pub fn iter(&self) -> std::slice::Iter<'_, ErrorMessage> {
        self.values.iter()
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    pub fn push(&mut self, item: ErrorMessage) {
        self.values.push(item);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JobIdList {
    pub values: Vec<JobId>,
}

impl JobIdList {
    pub fn new(value: Vec<JobId>) -> Self {
        Self { values: value }
    }
    pub fn values(&self) -> &[JobId] {
        &self.values
    }
    pub fn iter(&self) -> std::slice::Iter<'_, JobId> {
        self.values.iter()
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    pub fn push(&mut self, item: JobId) {
        self.values.push(item);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LineContentList {
    pub values: Vec<LineContentVO>,
}

impl LineContentList {
    pub fn new(value: Vec<LineContentVO>) -> Self {
        Self { values: value }
    }
    pub fn values(&self) -> &[LineContentVO] {
        &self.values
    }
    pub fn iter(&self) -> std::slice::Iter<'_, LineContentVO> {
        self.values.iter()
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    pub fn push(&mut self, item: LineContentVO) {
        self.values.push(item);
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
#[serde(transparent)]
#[derive(Default)]
pub struct LineNumber {
    pub value: i64,
}

impl LineNumber {
    pub fn new(value: i64) -> Self {
        Self { value }
    }
    pub fn value(&self) -> i64 {
        self.value
    }
}

impl std::fmt::Display for LineNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<i64> for LineNumber {
    fn from(v: i64) -> Self {
        Self { value: v }
    }
}

impl<'de> serde::Deserialize<'de> for LineNumber {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct LineNumberVisitor {}
        impl<'de> serde::de::Visitor<'de> for LineNumberVisitor {
            type Value = LineNumber;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(LineNumber { value: v })
            }
            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(LineNumber { value: v as i64 })
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(k) = map.next_key::<String>()? {
                    if k == "value" {
                        value = Some(map.next_value::<i64>()?);
                    } else {
                        let _: serde::de::IgnoredAny = map.next_value()?;
                    }
                }
                let val = value.ok_or_else(|| serde::de::Error::missing_field("value"))?;
                Ok(LineNumber { value: val })
            }
        }
        deserializer.deserialize_any(LineNumberVisitor {})
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(transparent)]
#[derive(Default)]
pub struct PatternList {
    pub values: Vec<String>,
}

impl PatternList {
    pub fn new(value: impl IntoPatternListValues) -> Self {
        Self {
            values: value.into_pattern_list_values(),
        }
    }
    pub fn values(&self) -> &[String] {
        &self.values
    }
}

impl PatternList {
    pub fn iter(&self) -> std::slice::Iter<'_, String> {
        self.values.iter()
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    pub fn push(&mut self, item: String) {
        self.values.push(item);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseDataList {
    pub values: Vec<ResponseData>,
}

impl ResponseDataList {
    pub fn new(value: Vec<ResponseData>) -> Self {
        Self { values: value }
    }
    pub fn values(&self) -> &[ResponseData] {
        &self.values
    }
    pub fn iter(&self) -> std::slice::Iter<'_, ResponseData> {
        self.values.iter()
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    pub fn push(&mut self, item: ResponseData) {
        self.values.push(item);
    }
}

#[derive(Debug, Clone, Default, Serialize, PartialEq)]
#[serde(transparent)]
pub struct Score {
    pub value: f64,
}

impl Score {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
    pub fn value(&self) -> f64 {
        self.value
    }
    pub fn is_perfect(&self) -> bool {
        self.value >= 100.0
    }
    pub fn is_passing(&self, threshold: &Score) -> bool {
        self.value >= threshold.value
    }
    pub fn deduct(&self, severity: &Severity) -> Score {
        Score {
            value: self.value - severity.score_impact(),
        }
    }
}

impl std::fmt::Display for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.1}", self.value)
    }
}

impl From<f64> for Score {
    fn from(v: f64) -> Self {
        Self { value: v }
    }
}

impl<'de> serde::Deserialize<'de> for Score {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ScoreVisitor {}
        impl<'de> serde::de::Visitor<'de> for ScoreVisitor {
            type Value = Score;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Score { value: v })
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(k) = map.next_key::<String>()? {
                    if k == "value" {
                        value = Some(map.next_value::<f64>()?);
                    } else {
                        let _: serde::de::IgnoredAny = map.next_value()?;
                    }
                }
                let val = value.ok_or_else(|| serde::de::Error::missing_field("value"))?;
                Ok(Score { value: val })
            }
        }
        deserializer.deserialize_any(ScoreVisitor {})
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(transparent)]
#[derive(Default)]
pub struct Timestamp {
    pub value: String,
}

impl Timestamp {
    pub fn value(&self) -> &str {
        &self.value
    }
    pub fn now() -> Self {
        Self {
            value: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl std::fmt::Display for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<&str> for Timestamp {
    fn from(s: &str) -> Self {
        Self {
            value: s.to_string(),
        }
    }
}

impl From<String> for Timestamp {
    fn from(s: String) -> Self {
        Self { value: s }
    }
}

impl<'de> serde::Deserialize<'de> for Timestamp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct TimestampVisitor {}
        impl<'de> serde::de::Visitor<'de> for TimestampVisitor {
            type Value = Timestamp;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Timestamp {
                    value: v.to_string(),
                })
            }
            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Timestamp { value: v })
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(k) = map.next_key::<String>()? {
                    if k == "value" {
                        value = Some(map.next_value::<String>()?);
                    } else {
                        let _: serde::de::IgnoredAny = map.next_value()?;
                    }
                }
                let val = value.ok_or_else(|| serde::de::Error::missing_field("value"))?;
                Ok(Timestamp { value: val })
            }
        }
        deserializer.deserialize_any(TimestampVisitor {})
    }
}

// Custom Coercion Traits for PatternList

pub trait IntoPatternListValues {
    fn into_pattern_list_values(self) -> Vec<String>;
}

impl IntoPatternListValues for &str {
    fn into_pattern_list_values(self) -> Vec<String> {
        vec![self.to_string()]
    }
}

impl IntoPatternListValues for String {
    fn into_pattern_list_values(self) -> Vec<String> {
        vec![self]
    }
}

impl IntoPatternListValues for Vec<String> {
    fn into_pattern_list_values(self) -> Vec<String> {
        self
    }
}

impl IntoPatternListValues for Vec<&str> {
    fn into_pattern_list_values(self) -> Vec<String> {
        self.into_iter().map(|s| s.to_string()).collect()
    }
}

impl IntoPatternListValues for &Vec<String> {
    fn into_pattern_list_values(self) -> Vec<String> {
        self.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(transparent)]
#[derive(Default)]
pub struct ErrorMessage {
    pub value: String,
}

impl ErrorMessage {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl std::fmt::Display for ErrorMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<&str> for ErrorMessage {
    fn from(s: &str) -> Self {
        Self {
            value: s.to_string(),
        }
    }
}

impl From<String> for ErrorMessage {
    fn from(s: String) -> Self {
        Self { value: s }
    }
}
```

---

## File: crates/shared/src/config-system/contract_multi_project_orchestrator_aggregate.rs

```rust
use crate::config_system::taxonomy_multi_project_workspace_info_vo::WorkspaceInfo;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use async_trait::async_trait;

#[async_trait]
pub trait MultiProjectOrchestratorAggregate: Send + Sync {
    async fn discover_workspaces(&self, root: &FilePath) -> Vec<WorkspaceInfo>;
}
```

---

## File: crates/shared/src/config-system/contract_orchestration_aggregate.rs

```rust
// PURPOSE: IConfigOrchestrationAggregate — aggregate contract for orchestrating configuration loading across languages

use crate::config_system::contract_reader_port::IConfigReaderPort;
use crate::config_system::contract_workspace_detector_port::IWorkspaceDetectorPort;
use crate::config_system::taxonomy_source_vo::ConfigResult;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait IConfigOrchestrationAggregate: Send + Sync {
    fn workspace_detector(&self) -> Arc<dyn IWorkspaceDetectorPort>;
    fn config_reader(&self) -> Arc<dyn IConfigReaderPort>;

    async fn load_project_config(&self, project_root: &FilePath) -> ConfigResult;
    async fn load_config_for_language(
        &self,
        project_root: &FilePath,
        language: &str,
    ) -> ConfigResult;
}
```

---

## File: crates/shared/src/config-system/contract_parser_port.rs

```rust
// PURPOSE: IConfigParserPort — contract for config parser provider (YAML and TOML)
use crate::config_system::taxonomy_config_error::ConfigError;
use crate::config_system::taxonomy_setting_vo::ProjectConfig;
use crate::source_parsing::taxonomy_path_vo::FilePath;

pub trait IConfigParserPort: Send + Sync {
    fn parse_yaml_config(&self, path: &FilePath) -> Result<ProjectConfig, ConfigError>;
    fn parse_toml_config(&self, path: &FilePath) -> Option<Result<ProjectConfig, ConfigError>>;
}
```

---

## File: crates/shared/src/config-system/contract_reader_port.rs

```rust
// PURPOSE: IConfigReaderPort — port trait for reading configuration from external sources

use crate::config_system::taxonomy_source_vo::ConfigSource;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use async_trait::async_trait;

#[async_trait]
pub trait IConfigReaderPort: Send + Sync {
    async fn read_config(&self, project_root: &FilePath, language: &str) -> Option<ConfigSource>;
    async fn list_config_files(&self, project_root: &FilePath) -> Vec<(String, String)>;
}
```

---

## File: crates/shared/src/config-system/contract_validator_protocol.rs

```rust
// PURPOSE: IConfigValidatorProtocol — protocol for project config and scoring threshold validation

use crate::common::taxonomy_adapter_name_vo::AdapterName;
use crate::config_system::taxonomy_validation_vo::ValidationResult;

pub trait IConfigValidatorProtocol: Send + Sync {
    /// Determines if a specific adapter should run based on configuration rules.
    fn is_adapter_enabled(&self, adapter_name: &AdapterName) -> bool;

    /// Validates that scoring thresholds are sane.
    fn validate_thresholds(&self) -> ValidationResult;
}
```

---

## File: crates/shared/src/config-system/contract_workspace_detector_port.rs

```rust
// PURPOSE: IWorkspaceDetectorPort — port trait for detecting workspace type from directory structure
use crate::source_parsing::taxonomy_path_vo::FilePath;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkspaceType {
    Rust,
    TypeScript,
    Python,
    Unknown,
}

impl WorkspaceType {
    pub fn as_str(&self) -> &'static str {
        match self {
            WorkspaceType::Rust => "rust",
            WorkspaceType::TypeScript => "typescript",
            WorkspaceType::Python => "python",
            WorkspaceType::Unknown => "unknown",
        }
    }
}

impl std::fmt::Display for WorkspaceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

pub trait IWorkspaceDetectorPort: Send + Sync {
    /// Detect workspace type by checking folder structure and config files.
    fn detect(&self, path: &FilePath) -> WorkspaceType;

    /// Check if a path is a workspace root (contains crates/, packages/, or modules/).
    fn is_workspace(&self, path: &FilePath) -> bool;
}
```

---

## File: crates/shared/src/config-system/mod.rs

```rust
// config-system — taxonomy and contract types
pub mod contract_multi_project_orchestrator_aggregate;
pub mod contract_orchestration_aggregate;
pub mod contract_parser_port;
pub mod contract_reader_port;
pub mod contract_validator_protocol;
pub mod contract_workspace_detector_port;
pub mod taxonomy_adapter_vo;
pub mod taxonomy_app_vo;
pub mod taxonomy_config_error;
pub mod taxonomy_config_vo;
pub mod taxonomy_identifier_vo;
pub mod taxonomy_multi_project_summary_vo;
pub mod taxonomy_multi_project_vo;
pub mod taxonomy_multi_project_workspace_info_vo;
pub mod taxonomy_setting_vo;
pub mod taxonomy_source_vo;
pub mod taxonomy_validation_vo;
```

---

## File: crates/shared/src/config-system/taxonomy_adapter_vo.rs

```rust
// PURPOSE: AdapterClassMap, AdapterMetadataList, AdapterNameList — VOs for adapter registration metadata
use crate::common::taxonomy_adapter_name_vo::AdapterName;
use crate::mcp_server::taxonomy_job_vo::AdapterMetadata;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AdapterMetadataList {
    #[serde(default)]
    pub values: Vec<AdapterMetadata>,
}

impl Default for AdapterMetadataList {
    fn default() -> Self {
        Self::new()
    }
}

impl AdapterMetadataList {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }
    pub fn push(&mut self, item: AdapterMetadata) {
        self.values.push(item);
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

impl std::ops::Deref for AdapterMetadataList {
    type Target = Vec<AdapterMetadata>;
    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AdapterNameList {
    #[serde(default)]
    pub values: Vec<AdapterName>,
}

impl Default for AdapterNameList {
    fn default() -> Self {
        Self::new()
    }
}

impl AdapterNameList {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }
    pub fn push(&mut self, item: AdapterName) {
        self.values.push(item);
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

impl std::ops::Deref for AdapterNameList {
    type Target = Vec<AdapterName>;
    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AdapterClassMap {
    #[serde(default)]
    pub values: std::collections::HashMap<String, String>,
}

impl Default for AdapterClassMap {
    fn default() -> Self {
        Self::new()
    }
}

impl AdapterClassMap {
    pub fn new() -> Self {
        Self {
            values: std::collections::HashMap::new(),
        }
    }
    pub fn get(&self, key: &str) -> Option<&String> {
        self.values.get(key)
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}
```

---

## File: crates/shared/src/config-system/taxonomy_app_vo.rs

```rust
// PURPOSE: AppConfigVO, AppName, AppVersion — value objects for application configuration metadata
use std::env;

use crate::common::taxonomy_common_vo::BooleanVO;
use crate::config_system::taxonomy_adapter_vo::AdapterNameList;
use crate::config_system::taxonomy_setting_vo::{AdapterStatus, ProjectConfig, Thresholds};
use crate::source_parsing::taxonomy_path_vo::DirectoryPath;

/// app_config_vo — Unified configuration for the application.
///
/// Unified configuration — transport, paths, and project settings.
#[derive(Debug, Clone)]
pub struct AppConfig {
    phantom_root: DirectoryPath,
    project: ProjectConfig,
}

impl AppConfig {
    /// Create a new AppConfig.
    ///
    /// # Arguments
    /// * `phantom_root` - Optional phantom root directory. Defaults to environment variable `PHANTOM_ROOT` or home directory.
    /// * `project_root` - Optional project root directory. Defaults to environment variable `PROJECT_ROOT` or current directory.
    /// * `project` - Optional project configuration. Defaults to `crate::config_system::taxonomy_setting_vo::ProjectConfig::default()`.
    pub fn create(
        phantom_root: Option<String>,
        project_root: Option<String>,
        project: Option<ProjectConfig>,
    ) -> Self {
        let p_root = match phantom_root.or_else(|| env::var("PHANTOM_ROOT").ok()) {
            Some(r) => r,
            None => match env::var("HOME") {
                Ok(h) => h,
                Err(_) => ".".to_string(),
            },
        };
        let _proj_root = match project_root.or_else(|| env::var("PROJECT_ROOT").ok()) {
            Some(r) => r,
            None => match env::current_dir() {
                Ok(d) => d.to_string_lossy().to_string(),
                Err(_) => ".".to_string(),
            },
        };
        let proj = project.unwrap_or_default();

        Self {
            phantom_root: DirectoryPath::new(p_root).unwrap_or_default(),
            project: proj,
        }
    }

    /// Get the thresholds from the project configuration.
    pub fn thresholds(&self) -> &Thresholds {
        &self.project.thresholds
    }

    /// Get status for a named adapter.
    pub fn adapter_status(&self, name: &str) -> AdapterStatus {
        for entry in &self.project.adapters {
            if entry.name.value == name {
                return entry.status;
            }
        }
        AdapterStatus::NotInstalled
    }

    /// Check if an adapter is enabled.
    pub fn is_adapter_enabled(&self, name: &str) -> BooleanVO {
        let status = self.adapter_status(name);
        BooleanVO::new(status == AdapterStatus::Enabled)
    }

    /// Names of enabled adapters.
    pub fn active_adapters(&self) -> AdapterNameList {
        let mut values = Vec::new();
        for entry in &self.project.adapters {
            if entry.is_active() {
                values.push(entry.name.clone());
            }
        }
        AdapterNameList { values }
    }
}

impl std::fmt::Display for AppConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "AppConfig(phantom={}, adapters={:?})",
            self.phantom_root,
            self.active_adapters()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::AppConfig;
    use super::ProjectConfig;
    use std::env;

    #[test]
    fn test_app_config_create() {
        let config = AppConfig::create(
            Some("/phantom".to_string()),
            Some("/project".to_string()),
            Some(ProjectConfig::default()),
        );
        assert_eq!(config.phantom_root.to_string(), "/phantom");
    }

    #[test]
    fn test_app_config_defaults() {
        // Set environment variables for deterministic test
        env::set_var("PHANTOM_ROOT", "/test/phantom");
        env::set_var("PROJECT_ROOT", "/test/project");
        let config = AppConfig::create(None, None, None);
        assert_eq!(config.phantom_root.to_string(), "/test/phantom");
        // Clean up
        env::remove_var("PHANTOM_ROOT");
        env::remove_var("PROJECT_ROOT");
    }
}
```

---

## File: crates/shared/src/config-system/taxonomy_config_error.rs

```rust
// PURPOSE: ConfigError, ConfigErrorKind — structured error types for configuration loading failures
use crate::common::taxonomy_common_error::ErrorMessage;
use crate::config_system::taxonomy_identifier_vo::ConfigKey;
use crate::config_system::taxonomy_setting_vo::ActualValue;
use crate::config_system::taxonomy_setting_vo::ExpectedValue;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default, thiserror::Error)]
pub struct ConfigError {
    pub key: ConfigKey,
    pub message: ErrorMessage,
    pub expected: ExpectedValue,
    pub actual: ActualValue,
    pub config_file: FilePath,
}

impl ConfigError {
    pub fn new(key: ConfigKey, message: ErrorMessage) -> Self {
        Self {
            key,
            message,
            expected: ExpectedValue::default(),
            actual: ActualValue::default(),
            config_file: FilePath::default(),
        }
    }
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let file_str = self.config_file.to_string();
        let file_info = if file_str.is_empty() {
            String::new()
        } else {
            format!(" in {}", file_str)
        };
        write!(
            f,
            "Config error on '{}'{}: {}",
            self.key, file_info, self.message
        )
    }
}
```

---

## File: crates/shared/src/config-system/taxonomy_config_vo.rs

```rust
// PURPOSE: ArchitectureConfig, LayerDefinition, ConfigRule — configuration value objects for AES rules definition
use serde::{Deserialize, Serialize};

use crate::common::taxonomy_common_vo::BooleanVO;
use crate::common::taxonomy_common_vo::Count;
use crate::common::taxonomy_common_vo::PatternList;
use crate::common::taxonomy_definition_vo::LayerDefinition;
use crate::common::taxonomy_definition_vo::NamingConfig;
use crate::common::taxonomy_error_vo::ErrorCode;
use crate::common::taxonomy_layer_vo::LayerNameVO;
use crate::common::taxonomy_suggestion_vo::DescriptionVO;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use crate::source_parsing::taxonomy_paths_vo::FilePathList;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(default)]
pub struct ArchitectureRule {
    pub name: DescriptionVO,
    pub description: DescriptionVO,
    pub rule_type: ErrorCode,
    pub scope: LayerNameVO,
    pub exceptions: PatternList,
    #[serde(default)]
    pub allowed: PatternList,
    #[serde(default)]
    pub forbidden: PatternList,
    #[serde(default)]
    pub mandatory: PatternList,

    #[serde(flatten)]
    pub naming: crate::naming_rules::taxonomy_naming_rule_vo::NamingRuleVO,
    #[serde(flatten)]
    pub code_analysis: crate::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO,
    #[serde(flatten)]
    pub role: crate::role_rules::taxonomy_role_rule_vo::RoleRuleVO,
    #[serde(flatten)]
    pub orphan: crate::orphan_detector::taxonomy_orphan_rule_vo::OrphanRuleVO,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct ArchitectureConfig {
    pub enabled: BooleanVO,
    pub layers: std::collections::HashMap<LayerNameVO, LayerDefinition>,
    pub rules: Vec<ArchitectureRule>,
    pub naming: NamingConfig,
    pub ignored_paths: FilePathList,
    pub mandatory_class_definition: BooleanVO,
}

impl ArchitectureConfig {
    pub fn new(
        enabled: BooleanVO,
        layers: std::collections::HashMap<LayerNameVO, LayerDefinition>,
        rules: Vec<ArchitectureRule>,
        naming: NamingConfig,
        ignored_paths: FilePathList,
        mandatory_class_definition: BooleanVO,
    ) -> Self {
        Self {
            enabled,
            layers,
            rules,
            naming,
            ignored_paths,
            mandatory_class_definition,
        }
    }
}

impl Default for ArchitectureConfig {
    fn default() -> Self {
        Self {
            enabled: BooleanVO::new(true),
            layers: HashMap::new(),
            rules: Vec::new(),
            naming: NamingConfig::new(Count::new(2)),
            ignored_paths: FilePathList { values: vec![] },
            mandatory_class_definition: BooleanVO::new(false),
        }
    }
}

pub fn parse_config_yaml(yaml_str: &str) -> ArchitectureConfig {
    let raw: serde_yaml::Value = serde_yaml::from_str(yaml_str).unwrap_or_default();
    if let Some(arch_val) = raw.get("architecture") {
        let mut arch_json = serde_json::to_value(arch_val).unwrap_or_default();
        // Extract layers from rules.AES102.layers if not at top-level layers
        if arch_json
            .get("rules")
            .and_then(|r| r.get("AES102"))
            .and_then(|a| a.get("layers"))
            .is_some()
            && arch_json.get("layers").is_none()
        {
            if let Some(rules_obj) = arch_json.get_mut("rules").and_then(|r| r.as_object_mut()) {
                if let Some(aes102) = rules_obj.get_mut("AES102").and_then(|a| a.as_object_mut()) {
                    if let Some(layers) = aes102.remove("layers") {
                        arch_json["layers"] = layers;
                    }
                }
            }
        }
        let mut json = arch_json;
        fn remove_nulls(val: &mut serde_json::Value) {
            match val {
                serde_json::Value::Object(m) => {
                    m.retain(|_, v| !v.is_null());
                    for v in m.values_mut() {
                        remove_nulls(v);
                    }
                }
                serde_json::Value::Array(arr) => {
                    for v in arr.iter_mut() {
                        remove_nulls(v);
                    }
                }
                _ => {}
            }
        }
        remove_nulls(&mut json);
        // Convert ignored_paths from array to {values: [...]} format because the Rust struct expects an object with a "values" field.
        if let Some(arr) = json.get("ignored_paths").and_then(|v| v.as_array()) {
            json["ignored_paths"] = serde_json::json!({"values": arr});
        }
        if let Some(layers_obj) = json.get_mut("layers") {
            if let Some(obj) = layers_obj.as_object_mut() {
                let mut suffix_updates: Vec<(
                    String,
                    Option<String>,
                    serde_json::Value,
                    serde_json::Value,
                )> = Vec::new();
                for (layer_name, layer) in obj.iter() {
                    if let Some(suffix_val) = layer.get("suffix") {
                        if let Some(arr) = suffix_val.as_array() {
                            let mut policy: Option<String> = None;
                            let mut allowed = serde_json::Value::Array(Vec::new());
                            let mut forbidden = serde_json::Value::Array(Vec::new());
                            for entry in arr {
                                if let Some(entry_obj) = entry.as_object() {
                                    for (pkey, plist) in entry_obj {
                                        match pkey.as_str() {
                                            "strict" | "flexible" => {
                                                policy = Some(pkey.clone());
                                                if let Some(list) = plist.as_array() {
                                                    allowed = serde_json::json!(list);
                                                }
                                            }
                                            "forbidden" => {
                                                if let Some(list) = plist.as_array() {
                                                    forbidden = serde_json::json!(list);
                                                }
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                            }
                            suffix_updates.push((layer_name.clone(), policy, allowed, forbidden));
                        }
                    }
                }
                for (name, policy, allowed, forbidden) in suffix_updates {
                    if let Some(layer) = obj.get_mut(&name) {
                        if let Some(layer_obj) = layer.as_object_mut() {
                            if let Some(ref p) = policy {
                                layer_obj.insert("suffix_policy".to_string(), serde_json::json!(p));
                            }
                            layer_obj.insert("allowed_suffix".to_string(), allowed);
                            if let Some(arr) = forbidden.as_array() {
                                if !arr.is_empty() {
                                    layer_obj.insert("forbidden_suffix".to_string(), forbidden);
                                }
                            }
                            layer_obj.remove("suffix");
                        }
                    }
                }
            }
        }
        if let Some(rules_obj) = json.get_mut("rules") {
            if let Some(obj) = rules_obj.as_object_mut() {
                let mut flat = serde_json::Value::Array(Vec::new());
                for (code, rule_val) in obj.iter() {
                    if let Some(rule_obj) = rule_val.as_object() {
                        let mut base = rule_obj.clone();
                        base.insert("name".to_string(), serde_json::json!(code));
                        // Expand scope array into multiple entries — one per scope element
                        // Only applies to rules WITHOUT conditions (conditions have their own scopes)
                        if let Some(scope_arr) = base.get("scope").and_then(|s| s.as_array()) {
                            if !base.contains_key("conditions") && scope_arr.len() > 1 {
                                for scope_val in scope_arr {
                                    if let Some(s) = scope_val.as_str() {
                                        let mut entry = base.clone();
                                        entry.insert("scope".to_string(), serde_json::json!(s));
                                        if let Some(arr) = flat.as_array_mut() {
                                            arr.push(serde_json::Value::Object(entry));
                                        }
                                    }
                                }
                                continue; // Already pushed per-scope entries, skip single push below
                            } else if let Some(first) = scope_arr.first().and_then(|v| v.as_str()) {
                                base.insert("scope".to_string(), serde_json::json!(first));
                            }
                        }
                        if let Some(conditions) = base.remove("conditions") {
                            if let Some(conds) = conditions.as_array() {
                                if !conds.is_empty() {
                                    for cond in conds {
                                        if let Some(cond_obj) = cond.as_object() {
                                            let mut entry = base.clone();
                                            for (k, v) in cond_obj {
                                                entry.insert(k.clone(), v.clone());
                                            }
                                            // Remove top-level scope array leftovers if condition has its own scope
                                            if let Some(arr) = flat.as_array_mut() {
                                                arr.push(serde_json::Value::Object(entry));
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            if let Some(arr) = flat.as_array_mut() {
                                arr.push(serde_json::Value::Object(base));
                            }
                        }
                    }
                }
                *rules_obj = flat;
            }
        }
        let mut config = match serde_json::from_value::<ArchitectureConfig>(json) {
            Ok(c) => c,
            Err(e) => {
                println!("[debug] serde_json from_value error: {:?}", e);
                ArchitectureConfig::default()
            }
        };
        // Top-level ignored_paths (outside architecture section) — merge into config
        if config.ignored_paths.values.is_empty() {
            if let Some(arr) = raw.get("ignored_paths").and_then(|v| v.as_sequence()) {
                let paths: Vec<_> = arr
                    .iter()
                    .filter_map(|v| v.as_str())
                    .map(|s| FilePath::new(s.to_string()).unwrap_or_default())
                    .collect();
                if !paths.is_empty() {
                    config.ignored_paths = FilePathList::new(paths);
                }
            }
        }
        config
    } else {
        let mut config = ArchitectureConfig::default();
        if let Some(arr) = raw.get("ignored_paths").and_then(|v| v.as_sequence()) {
            let paths: Vec<_> = arr
                .iter()
                .filter_map(|v| v.as_str())
                .map(|s| FilePath::new(s.to_string()).unwrap_or_default())
                .collect();
            if !paths.is_empty() {
                config.ignored_paths = FilePathList::new(paths);
            }
        }
        config
    }
}

/// All 3 config YAMLs are baked into the binary at compile time via `include_str!`.
/// Runtime project-level config files override these defaults.
pub fn default_aes_config() -> ArchitectureConfig {
    parse_config_yaml(include_str!("../../../../lint_arwaky.config.rust.yaml"))
}

pub fn default_config_for_language(language: &str) -> ArchitectureConfig {
    match language {
        "python" => parse_config_yaml(include_str!("../../../../lint_arwaky.config.python.yaml")),
        "javascript" | "typescript" => parse_config_yaml(include_str!(
            "../../../../lint_arwaky.config.javascript.yaml"
        )),
        _ => default_aes_config(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_parsing() {
        let config = default_config_for_language("typescript");
        println!("typescript layers: {:?}", config.layers.keys());
        assert!(!config.layers.is_empty());
    }
}
```

---

## File: crates/shared/src/config-system/taxonomy_identifier_vo.rs

```rust
// PURPOSE: ConfigIdentifier — value object for named configuration identifiers
use crate::string_value_object;

string_value_object!(ConfigKey);

impl ConfigKey {
    /// Returns each dot-separated segment of the key.
    pub fn parts(&self) -> Vec<String> {
        self.value.split('.').map(|s| s.to_string()).collect()
    }

    /// Returns the parent key, dropping the last segment. Empty when the
    /// key has no parent (single segment).
    pub fn parent(&self) -> String {
        let parts = self.parts();
        if parts.len() > 1 {
            parts[..parts.len() - 1].join(".")
        } else {
            String::new()
        }
    }

    /// Returns the last segment of the key, or the full value when the
    /// key has no `.` separators.
    pub fn leaf(&self) -> String {
        match self.parts().last() {
            Some(part) => part.clone(),
            None => self.value.clone(),
        }
    }
}
```

---

## File: crates/shared/src/config-system/taxonomy_multi_project_summary_vo.rs

```rust
use serde::{Deserialize, Serialize};

use crate::common::taxonomy_common_error::ErrorMessage;
use crate::common::taxonomy_common_vo::Count;
use crate::common::taxonomy_common_vo::PatternList;
use crate::common::taxonomy_common_vo::Score;
use crate::common::taxonomy_message_vo::ComplianceStatus;
use crate::source_parsing::taxonomy_path_vo::FilePath;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AggregatedResults {
    pub projects: Vec<ProjectResult>,
    pub total_projects: Count,
    pub passing_projects: Count,
    pub failing_projects: Count,
    pub average_score: Score,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectResult {
    pub path: FilePath,
    pub score: Score,
    pub is_passing: ComplianceStatus,
    pub issues: Vec<std::collections::HashMap<String, serde_json::Value>>,
    pub adapters: PatternList,
    pub error: ErrorMessage,
}

impl AggregatedResults {
    pub fn new(
        projects: Vec<ProjectResult>,
        total_projects: Count,
        passing_projects: Count,
        failing_projects: Count,
        average_score: Score,
    ) -> Self {
        Self {
            projects,
            total_projects,
            passing_projects,
            failing_projects,
            average_score,
        }
    }
}

impl ProjectResult {
    pub fn new(
        path: FilePath,
        score: Score,
        is_passing: ComplianceStatus,
        issues: Vec<std::collections::HashMap<String, serde_json::Value>>,
        adapters: PatternList,
        error: ErrorMessage,
    ) -> Self {
        Self {
            path,
            score,
            is_passing,
            issues,
            adapters,
            error,
        }
    }
}
```

---

## File: crates/shared/src/config-system/taxonomy_multi_project_vo.rs

```rust
use crate::common::taxonomy_common_vo::BooleanVO;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use crate::source_parsing::taxonomy_paths_vo::FilePathList;

#[derive(Debug, Clone, Default)]
pub struct MultiProjectVO {
    pub paths: Option<FilePathList>,
    pub use_retry: Option<BooleanVO>,
    pub config_path: Option<FilePath>,
}
```

---

## File: crates/shared/src/config-system/taxonomy_multi_project_workspace_info_vo.rs

```rust
use crate::config_system::taxonomy_config_vo::ArchitectureConfig;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceInfo {
    pub path: FilePath,
    pub workspace_type: String,
    pub config: ArchitectureConfig,
}

impl WorkspaceInfo {
    pub fn new(path: FilePath, workspace_type: String, config: ArchitectureConfig) -> Self {
        Self {
            path,
            workspace_type,
            config,
        }
    }
}
```

---

## File: crates/shared/src/config-system/taxonomy_setting_vo.rs

```rust
// PURPOSE: SettingsConfigVO — value object for application-wide settings configuration

use crate::string_value_object;
use serde::{Deserialize, Serialize};

use crate::common::taxonomy_adapter_name_vo::AdapterName;
use crate::common::taxonomy_common_vo::Count;
use crate::common::taxonomy_common_vo::PatternList;
use crate::common::taxonomy_common_vo::Score;
use crate::common::taxonomy_suggestion_vo::DescriptionVO;
use crate::config_system::taxonomy_config_vo::ArchitectureConfig;
use crate::source_parsing::taxonomy_path_vo::DirectoryPath;
use crate::source_parsing::taxonomy_paths_vo::FilePathList;

string_value_object!(ActualValue);
string_value_object!(ExpectedValue);

/// Scoring thresholds.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Thresholds {
    pub score: Score,
    pub complexity: Count,
    pub max_file_lines: Count,
}

impl Thresholds {
    pub fn new(score: Score, complexity: Count, max_file_lines: Count) -> Self {
        Self {
            score,
            complexity,
            max_file_lines,
        }
    }
}

impl Default for Thresholds {
    fn default() -> Self {
        Self {
            score: Score::new(80.0),
            complexity: Count::new(10),
            max_file_lines: Count::new(500),
        }
    }
}

/// Adapter status enum.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum AdapterStatus {
    #[default]
    Enabled,
    Disabled,
    NotInstalled,
}

impl AdapterStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            AdapterStatus::Enabled => "enabled",
            AdapterStatus::Disabled => "disabled",
            AdapterStatus::NotInstalled => "not_installed",
        }
    }
}

impl std::fmt::Display for AdapterStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Single adapter configuration.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AdapterEntry {
    pub name: AdapterName,
    #[serde(default)]
    pub status: AdapterStatus,
    #[serde(default = "default_weight")]
    pub weight: f64,
}

fn default_weight() -> f64 {
    1.0
}

impl AdapterEntry {
    pub fn new(name: AdapterName, status: AdapterStatus, weight: f64) -> Self {
        Self {
            name,
            status,
            weight,
        }
    }

    pub fn enabled(name: AdapterName) -> Self {
        Self::new(name, AdapterStatus::Enabled, 1.0)
    }

    pub fn is_active(&self) -> bool {
        matches!(self.status, AdapterStatus::Enabled)
    }
}

/// Project configuration.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ProjectConfig {
    #[serde(default = "default_project_name")]
    pub project_name: DescriptionVO,
    #[serde(default)]
    pub thresholds: Thresholds,
    #[serde(default)]
    pub adapters: Vec<AdapterEntry>,
    #[serde(default)]
    pub ignored_paths: FilePathList,
    #[serde(default)]
    pub ignored_rules: PatternList,
    #[serde(default)]
    pub layer_map: std::collections::HashMap<String, String>,
    #[serde(default)]
    pub output_dir: Option<DirectoryPath>,
    #[serde(default)]
    pub architecture: ArchitectureConfig,
}

fn default_project_name() -> DescriptionVO {
    DescriptionVO::new("lint-arwaky")
}

impl ProjectConfig {
    /// Returns a ProjectConfig with default linter adapters enabled.
    pub fn defaults() -> Self {
        Self {
            project_name: default_project_name(),
            thresholds: Thresholds::default(),
            adapters: vec![
                AdapterEntry::enabled(AdapterName::raw("ruff")),
                AdapterEntry::enabled(AdapterName::raw("mypy")),
                AdapterEntry::enabled(AdapterName::raw("bandit")),
                AdapterEntry::enabled(AdapterName::raw("radon")),
            ],
            ignored_paths: FilePathList::default(),
            ignored_rules: PatternList::default(),
            layer_map: std::collections::HashMap::new(),
            output_dir: None,
            architecture: ArchitectureConfig::default(),
        }
    }
}
```

---

## File: crates/shared/src/config-system/taxonomy_source_vo.rs

```rust
// PURPOSE: ConfigResult, ConfigSource for config-system
pub use crate::common::taxonomy_source_vo::ContentString;
pub use crate::common::taxonomy_source_vo::SourceContentVO;

use crate::config_system::taxonomy_config_vo::ArchitectureConfig;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use serde::{Deserialize, Serialize};

/// Represents a configuration source with its language, path, and raw content.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConfigSource {
    pub language: String,
    pub path: FilePath,
    pub raw_content: String,
}

impl ConfigSource {
    pub fn new(
        language: impl Into<String>,
        path: impl Into<String>,
        raw_content: impl Into<String>,
    ) -> Self {
        Self {
            language: language.into(),
            path: FilePath::new(path.into()).unwrap_or_default(),
            raw_content: raw_content.into(),
        }
    }
}

/// Result type for config loading operations containing the parsed config, source info, and warnings.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConfigResult {
    pub config: ArchitectureConfig,
    pub source: ConfigSource,
    pub warnings: Vec<String>,
}

impl ConfigResult {
    pub fn new(config: ArchitectureConfig, source: ConfigSource, warnings: Vec<String>) -> Self {
        Self {
            config,
            source,
            warnings,
        }
    }
}
```

---

## File: crates/shared/src/config-system/taxonomy_validation_vo.rs

```rust
// PURPOSE: ValidationResult — value object for config system validation results

/// Result of a validation operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub reason: Option<String>,
}

impl ValidationResult {
    pub fn ok() -> Self {
        Self {
            is_valid: true,
            reason: None,
        }
    }
    pub fn fail(reason: &str) -> Self {
        Self {
            is_valid: false,
            reason: Some(reason.to_string()),
        }
    }
}
```

---

## File: crates/shared/src/source-parsing/mod.rs

```rust
// source-parsing — taxonomy and contract types
pub mod contract_language_detector_port;
pub mod contract_parser_port;
pub mod contract_path_normalization_port;
pub mod contract_scanner_provider_port;
pub mod infrastructure_file_collector_provider;
pub mod taxonomy_adapter_error;
pub mod taxonomy_barrel_provider_vo;
pub mod taxonomy_file_collector_helper;
pub mod taxonomy_language_detector_helper;
pub mod taxonomy_naming_error;
pub mod taxonomy_naming_list_vo;
pub mod taxonomy_parser_error;
pub mod taxonomy_path_vo;
pub mod taxonomy_paths_vo;
pub mod taxonomy_semantic_error;
pub use infrastructure_file_collector_provider::{
    collect_all_source_files, count_loc, walk_rs_files, FileCollectorProvider,
};
```

---

## File: crates/shared/src/source-parsing/taxonomy_path_vo.rs

```rust
// PURPOSE: FilePath, DirectoryPath — value objects for validated file and directory paths
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

/// file_path_vo — File and directory path value objects.
///
/// File path identifier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct FilePath {
    pub value: String,
}

impl FilePath {
    pub fn value(&self) -> &str {
        &self.value
    }
    /// Create a new FilePath from a string.
    ///
    /// # Errors
    /// Returns an error if the path is invalid (empty or only whitespace).
    pub fn new<S: Into<String>>(value: S) -> Result<Self, String> {
        let mut value = value.into();
        if value.trim().is_empty() {
            return Err("File path cannot be empty".to_string());
        }
        // Normalize: replace backslashes with forward slashes, and collapse multiple slashes.
        value = value.replace('\\', "/");
        // Remove all trailing slashes
        while value.ends_with('/') && value.len() > 1 {
            value.pop();
        }
        // If after normalization it's empty, then it was all slashes -> treat as root
        if value.is_empty() {
            return Ok(FilePath {
                value: "/".to_string(),
            });
        }
        Ok(FilePath { value })
    }

    /// File extension without dot.
    pub fn extension(&self) -> String {
        let special_files = [
            "Makefile",
            "Dockerfile",
            "Dockerfile.dev",
            "Dockerfile.prod",
            ".bashrc",
            ".profile",
            ".zshrc",
            ".gitignore",
            ".dockerignore",
        ];
        // Operate on the basename, not the full path — `./foo.rs` must still yield
        // `rs` as its extension, and `.bashrc` (which is fully a basename) must NOT
        // be confused with a hidden file mid-path.
        let basename = match self.value.rsplit('/').next() {
            Some(b) => b,
            None => return String::new(),
        };
        if special_files.contains(&basename) || basename.starts_with('.') {
            return String::new();
        }
        match basename.rsplit('.').next() {
            Some(ext) => ext.to_string(),
            None => String::new(),
        }
    }

    /// Check if path has given extension (without dot).
    pub fn has_extension(&self, ext: &str) -> bool {
        self.extension().eq_ignore_ascii_case(ext)
    }

    /// Extract filename/basename of the path.
    pub fn basename(&self) -> String {
        match self.value.rsplit('/').next() {
            Some(f) => f.to_string(),
            None => self.value.clone(),
        }
    }

    /// Check if the path is a barrel file.
    pub fn is_barrel_file(&self) -> bool {
        let f = self.basename();
        matches!(
            f.as_ref(),
            "__init__.py" | "mod.rs" | "index.ts" | "index.js"
        )
    }

    /// Check if the path is a module/layer entry point file.
    pub fn is_entry_point(&self) -> bool {
        let f = self.basename();
        matches!(
            f.as_ref(),
            "__init__.py" | "main.py" | "py.typed" | "app.py" | "lib.rs"
        )
    }
}

impl std::ops::Deref for FilePath {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::fmt::Display for FilePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Hash for FilePath {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

/// Directory path identifier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Default)]
pub struct DirectoryPath {
    pub value: String,
}

impl DirectoryPath {
    pub fn value(&self) -> &str {
        &self.value
    }
    /// Create a new DirectoryPath from a string.
    ///
    /// # Errors
    /// Returns an error if the path is invalid (empty or only whitespace).
    pub fn new<S: Into<String>>(value: S) -> Result<Self, String> {
        let mut value = value.into();
        if value.trim().is_empty() {
            return Err("Directory path cannot be empty".to_string());
        }
        // Normalize: replace backslashes with forward slashes, and remove trailing slash.
        value = value.replace('\\', "/");
        // Remove trailing slash unless it's just "/"
        if value.ends_with('/') && value.len() > 1 {
            value.pop();
        }
        Ok(DirectoryPath { value })
    }
}

impl std::ops::Deref for DirectoryPath {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::fmt::Display for DirectoryPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<'de> serde::Deserialize<'de> for DirectoryPath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        DirectoryPath::new(s).map_err(serde::de::Error::custom)
    }
}

impl Hash for DirectoryPath {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::{DirectoryPath, FilePath};

    #[test]
    fn test_file_path_new() {
        let fp = FilePath::new("test.txt").unwrap_or_default();
        assert_eq!(fp.value, "test.txt");
        assert_eq!(fp.extension(), "txt");
        assert!(fp.has_extension("txt"));
        assert!(!fp.has_extension("md"));

        // Test normalization
        let fp = FilePath::new("path\\to\\file.txt").unwrap_or_default();
        assert_eq!(fp.value, "path/to/file.txt");

        let fp = FilePath::new("path/to/file/").unwrap_or_default();
        assert_eq!(fp.value, "path/to/file");

        let fp = FilePath::new("/").unwrap_or_default();
        assert_eq!(fp.value, "/");

        let fp = FilePath::new("///").unwrap_or_default();
        assert_eq!(fp.value, "/");
    }

    #[test]
    fn test_file_path_invalid() {
        assert!(FilePath::new("").is_err());
        assert!(FilePath::new("   ").is_err());
    }

    #[test]
    fn test_directory_path_new() {
        let dp = DirectoryPath::new("test/dir").unwrap_or_default();
        assert_eq!(dp.value, "test/dir");

        let dp = DirectoryPath::new("test/dir/").unwrap_or_default();
        assert_eq!(dp.value, "test/dir");

        let dp = DirectoryPath::new("/").unwrap_or_default();
        assert_eq!(dp.value, "/");
    }

    #[test]
    fn test_directory_path_invalid() {
        assert!(DirectoryPath::new("").is_err());
        assert!(DirectoryPath::new("   ").is_err());
    }

    /// Regression: `./foo.rs` must report `rs` as its extension, not empty string.
    /// The old implementation treated any path starting with `.` as having no
    /// extension, which caused `LanguageDetector::is_lintable` to skip relative
    /// paths emitted by `std::fs::read_dir` in `collect_source_files`. Result: zero
    /// files collected when the user runs `lint-arwaky check .` on a directory
    /// tree with non-`.git`-anchored paths.
    #[test]
    fn test_extension_with_dot_slash_prefix() {
        let fp = FilePath::new("./foo.rs").unwrap_or_default();
        assert_eq!(fp.extension(), "rs");
        let fp = FilePath::new("./nested/foo.py").unwrap_or_default();
        assert_eq!(fp.extension(), "py");
        let fp = FilePath::new(".//foo.ts").unwrap_or_default();
        assert_eq!(fp.extension(), "ts");
    }

    /// Regression: a hidden-file basename (e.g. `.bashrc`) must still report no
    /// extension, since the basename itself starts with a dot.
    #[test]
    fn test_extension_hidden_basename() {
        let fp = FilePath::new(".bashrc").unwrap_or_default();
        assert_eq!(fp.extension(), "");
        let fp = FilePath::new("/home/user/.gitignore").unwrap_or_default();
        assert_eq!(fp.extension(), "");
    }

    /// Regression: full paths must still resolve the extension on the basename.
    #[test]
    fn test_extension_full_path() {
        let fp =
            FilePath::new("/tmp/bypass_test/capabilities_unwrap_checker.rs").unwrap_or_default();
        assert_eq!(fp.extension(), "rs");
        let fp = FilePath::new("crates/code-analysis/src/foo.rs").unwrap_or_default();
        assert_eq!(fp.extension(), "rs");
    }

    /// Makefile / Dockerfile — special filenames, no extension.
    #[test]
    fn test_extension_special_filenames() {
        let fp = FilePath::new("Makefile").unwrap_or_default();
        assert_eq!(fp.extension(), "");
        let fp = FilePath::new("Dockerfile").unwrap_or_default();
        assert_eq!(fp.extension(), "");
    }
}
```

---

