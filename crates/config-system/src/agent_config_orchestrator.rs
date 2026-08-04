use dashmap::DashMap;
use shared::common::{AdapterName, FilePath, PatternList};
use shared::config_system::{
    AdapterEntry, ArchitectureConfig, ConfigError, ConfigLanguage, ConfigResult, ConfigSource,
    IConfigOrchestratorAggregate, IConfigParserProtocol, IConfigReaderProtocol,
    IConfigValidatorProtocol, IWorkspaceDetectorProtocol, ProjectConfig, ValidationResult,
    WorkspaceInfo, WorkspaceType,
};
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use crate::utility_config_defaults::default_config_for_language;
use crate::utility_config_parser::parse_config_yaml;
use std::sync::Arc;

use tracing::warn;

// ─── Block 1: Struct Definition ───────────────────────────



pub struct ConfigOrchestratorDeps {
    pub workspace_detector: Arc<dyn IWorkspaceDetectorProtocol>,
    pub config_reader: Arc<dyn IConfigReaderProtocol>,
    pub parser: Arc<dyn IConfigParserProtocol>,
    pub validator: Arc<dyn IConfigValidatorProtocol>,
    pub filesystem: Arc<dyn IFilesystemAggregate>,
}

pub struct ConfigOrchestrator {
    deps: ConfigOrchestratorDeps,
    config_cache: DashMap<String, Arc<ArchitectureConfig>>,
}

// ─── Block 2: Protocol Trait Delegations ──────────────────

impl IConfigReaderProtocol for ConfigOrchestrator {
    fn read_config(
        &self,
        project_root: &FilePath,
        language: ConfigLanguage,
    ) -> Result<Option<ConfigSource>, ConfigError> {
        self.deps.config_reader.read_config(project_root, language)
    }

    fn list_config_files(
        &self,
        project_root: &FilePath,
    ) -> Result<Vec<(ConfigLanguage, FilePath)>, ConfigError> {
        self.deps.config_reader.list_config_files(project_root)
    }
}

impl IConfigParserProtocol for ConfigOrchestrator {
    fn parse_yaml_config(&self, path: &FilePath) -> Result<ProjectConfig, ConfigError> {
        self.deps.parser.parse_yaml_config(path)
    }

    fn parse_toml_config(&self, path: &FilePath) -> Result<Option<ProjectConfig>, ConfigError> {
        self.deps.parser.parse_toml_config(path)
    }

    fn parse_config_yaml_with_warnings(&self, yaml_str: &str) -> (ArchitectureConfig, Vec<String>) {
        self.deps.parser.parse_config_yaml_with_warnings(yaml_str)
    }

    fn parse_adapter_entries_from_yaml(&self, yaml_str: &str) -> Vec<AdapterEntry> {
        self.deps.parser.parse_adapter_entries_from_yaml(yaml_str)
    }
}

impl IConfigValidatorProtocol for ConfigOrchestrator {
    fn is_adapter_enabled(&self, config: &ProjectConfig, adapter_name: &AdapterName) -> bool {
        self.deps.validator.is_adapter_enabled(config, adapter_name)
    }

    fn validate_thresholds(&self, config: &ProjectConfig) -> ValidationResult {
        self.deps.validator.validate_thresholds(config)
    }
}

impl IWorkspaceDetectorProtocol for ConfigOrchestrator {
    fn detect(&self, path: &FilePath) -> WorkspaceType {
        self.deps.workspace_detector.detect(path)
    }

    fn is_workspace(&self, path: &FilePath) -> bool {
        self.deps.workspace_detector.is_workspace(path)
    }

    fn discover_workspace_members(&self, root: &FilePath) -> Vec<FilePath> {
        self.deps
            .workspace_detector
            .discover_workspace_members(root)
    }
}

// ─── Block 3: Aggregate Trait Implementation ──────────────

impl IConfigOrchestratorAggregate for ConfigOrchestrator {
    fn load_project_config(&self, project_root: &FilePath) -> ConfigResult {
        let ws_type = self.deps.workspace_detector.detect(project_root);
        let language = ConfigLanguage::from(ws_type);
        self.load_config_for_language(project_root, language)
    }

    fn load_config_for_language(
        &self,
        project_root: &FilePath,
        language: ConfigLanguage,
    ) -> ConfigResult {
        match self.deps.config_reader.read_config(project_root, language) {
            Ok(Some(source)) => {
                let cache_key = source.path.to_string();
                // FR-007: DashMap — single parse per key, no lock poisoning
                let parsed = self
                    .config_cache
                    .entry(cache_key)
                    .or_insert_with(|| Arc::new(parse_config_yaml(&source.raw_content)))
                    .value()
                    .as_ref()
                    .clone();
                let mut warnings = Vec::new();
                let mut config = parsed;
                if config.layers.is_empty() {
                    let defaults = default_config_for_language(language.as_str());
                    config.layers = defaults.layers;
                    warnings.push(
                        "Config file had no architecture layers, using built-in defaults for layers only."
                            .to_string(),
                    );
                }
                ConfigResult::new(config, source, warnings)
            }
            Ok(None) => {
                let warnings = vec!["No config file found, using built-in defaults".to_string()];
                let config = default_config_for_language(language.as_str());
                let source = ConfigSource::new(language.as_str(), "embedded", "");
                ConfigResult::new(config, source, warnings)
            }
            Err(e) => {
                let warnings = vec![format!("Config error: {}; using defaults", e)];
                let config = default_config_for_language(language.as_str());
                let source = ConfigSource::new(language.as_str(), "embedded", "");
                ConfigResult::new(config, source, warnings)
            }
        }
    }

    fn discover_workspaces(&self, root: &FilePath) -> Vec<WorkspaceInfo> {
        let workspaces = self
            .deps
            .workspace_detector
            .discover_workspace_members(root);

        if workspaces.is_empty() {
            warn!(root = %root.value, "no AES-compliant workspace members found, please refactor to multi-module structure");
            return Vec::new();
        }

        workspaces
            .into_iter()
            .map(|ws| {
                let ws_type = self.deps.workspace_detector.detect(&ws);
                let language = ConfigLanguage::from(ws_type);
                let config = match self.deps.config_reader.read_config(&ws, language) {
                    Ok(Some(source)) => {
                        let mut parsed = parse_config_yaml(&source.raw_content);
                        if parsed.layers.is_empty() {
                            parsed.layers = default_config_for_language(language.as_str()).layers;
                        }
                        parsed
                    }
                    _ => default_config_for_language(language.as_str()),
                };
                WorkspaceInfo::new(ws, language.to_string(), config)
            })
            .collect()
    }

    fn load_config_sync(&self, project_root: &FilePath) -> ArchitectureConfig {
        let root = std::path::Path::new(project_root.value());
        let ws_type = self.deps.workspace_detector.detect(project_root);
        let language = ConfigLanguage::from(ws_type);

        // FR-001: Search upward for config file (up to depth 5 for deeply nested files)
        let mut current = root.to_path_buf();
        let mut depth = 0;
        let mut source_content: Option<(String, String)> = None; // (path_str, raw_content)
        while !current.as_os_str().is_empty() && depth < 5 {
            for filename in language.config_file_names() {
                let candidate = current.join(filename);
                // FR-001: Reject symlinks pointing outside project root
                if let Ok(meta) = self.deps.filesystem.symlink_metadata(&candidate)
                    && meta.file_type().is_symlink()
                    && let Ok(canonical) = self.deps.filesystem.canonicalize(&candidate)
                {
                    let root_canonical = self
                        .deps
                        .filesystem
                        .canonicalize(root)
                        .unwrap_or_else(|_| root.to_path_buf());
                    if !canonical.starts_with(&root_canonical) {
                        warn!(path = %candidate.display(), "symlink points outside project root, rejected");
                        continue;
                    }
                }
                if let Ok(content) = self.deps.filesystem.read_to_string(&candidate) {
                    source_content = Some((candidate.to_string_lossy().to_string(), content));
                    break;
                }
            }
            if source_content.is_some() {
                break;
            }
            if let Some(parent) = current.parent() {
                current = parent.to_path_buf();
                depth += 1;
            } else {
                break;
            }
        }

        let (path_str, raw_content) = match source_content {
            Some(sc) => sc,
            None => {
                let mut config = default_config_for_language(language.as_str());
                let (merged_layers, _) = crate::utility_config_merger::merge_config(&config);
                config.layers = merged_layers;
                return config;
            }
        };

        // FR-007: Use DashMap cache — parse only once per file path
        let parsed = self
            .config_cache
            .entry(path_str)
            .or_insert_with(|| Arc::new(parse_config_yaml(&raw_content)))
            .value()
            .as_ref()
            .clone();

        // FR-005: Apply full merge protocol (layers merged, rules deduped)
        let (merged_layers, _) = crate::utility_config_merger::merge_config(&parsed);
        let mut config = parsed;
        config.layers = merged_layers;
        config
    }

    fn ignored_paths(&self, project_root: &FilePath) -> PatternList {
        let config = self.load_config_sync(project_root);
        PatternList::new(ignored_paths_from_config(&config))
    }

    fn ignored_paths_for_language(
        &self,
        project_root: &FilePath,
        language: ConfigLanguage,
    ) -> PatternList {
        let result = self.load_config_for_language(project_root, language);
        PatternList::new(ignored_paths_from_config(&result.config))
    }
}

// ─── Block 4: Constructors, Helpers, Private Methods ──────

impl ConfigOrchestrator {
    pub fn new(deps: ConfigOrchestratorDeps) -> Self {
        Self {
            deps,
            // FR-007: DashMap with pre-allocated capacity 32
            config_cache: DashMap::with_capacity(32),
        }
    }

    pub fn validator(&self) -> &Arc<dyn IConfigValidatorProtocol> {
        &self.deps.validator
    }
}

/// FR-008: Build complete ignored paths from hardcoded defaults + config-specified paths.
fn ignored_paths_from_config(config: &ArchitectureConfig) -> Vec<String> {
    // FR-008: Default ignored paths (hardcoded, universal)
    const DEFAULT_IGNORED: [&str; 8] = [
        ".git",
        "node_modules",
        "target",
        "dist",
        "build",
        "coverage",
        ".venv",
        "__pycache__",
    ];

    // FR-008: Normalize default paths to platform separators for consistent dedup
    let normalized_defaults: Vec<String> = DEFAULT_IGNORED
        .iter()
        .map(|s| s.replace('/', std::path::MAIN_SEPARATOR_STR))
        .collect();

    let mut seen: std::collections::HashSet<String> =
        std::collections::HashSet::from_iter(normalized_defaults.iter().cloned());
    // Pre-allocated capacity: 8 defaults + config count
    let mut ignored: Vec<String> = Vec::with_capacity(8 + config.ignored_paths.values.len());

    // Add default paths
    for name in &normalized_defaults {
        ignored.push(name.clone());
    }

    // FR-008: Config-specified paths appended with dedup, empty strings filtered
    for fp in config.ignored_paths.values.iter() {
        let v = fp.value.replace('/', std::path::MAIN_SEPARATOR_STR);
        if !v.is_empty() && seen.insert(v.clone()) {
            ignored.push(v);
        }
    }
    ignored
}
