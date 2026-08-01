use async_trait::async_trait;
use dashmap::DashMap;
use futures::stream::{self, StreamExt};
use shared::common::{FilePath, PatternList};
use shared::config_system::{
    ArchitectureConfig, ConfigError, ConfigLanguage, ConfigResult, ConfigSource,
    IConfigOrchestratorAggregate, IConfigReaderProtocol, IConfigValidatorProtocol,
    IWorkspaceDetectorProtocol, WorkspaceInfo,
};

use shared::config_system::utility_config_defaults::default_config_for_language;
use shared::config_system::utility_config_parser::parse_config_yaml;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;

pub struct ConfigOrchestratorDeps {
    pub workspace_detector: Arc<dyn IWorkspaceDetectorProtocol>,
    pub config_reader: Arc<dyn IConfigReaderProtocol>,
    pub validator: Arc<dyn IConfigValidatorProtocol>,
    pub filesystem: Arc<dyn IFilesystemAggregate>,
}

pub struct ConfigOrchestrator {
    deps: ConfigOrchestratorDeps,
    config_cache: DashMap<String, Arc<ArchitectureConfig>>,
}

// ─── Block 2: Aggregate Trait Implementation ──────────────

#[async_trait]
impl IConfigOrchestratorAggregate for ConfigOrchestrator {
    async fn load_project_config(&self, project_root: &FilePath) -> ConfigResult {
        let ws_type = self.deps.workspace_detector.detect(project_root);
        let language = ConfigLanguage::from(ws_type);
        self.load_config_for_language(project_root, language).await
    }

    async fn load_config_for_language(
        &self,
        project_root: &FilePath,
        language: ConfigLanguage,
    ) -> ConfigResult {
        match self
            .deps
            .config_reader
            .read_config(project_root, language)
            .await
        {
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

    async fn discover_workspaces(&self, root: &FilePath) -> Vec<WorkspaceInfo> {
        let workspaces = self
            .deps
            .workspace_detector
            .discover_workspace_members(root)
            .await;

        if workspaces.is_empty() {
            eprintln!(
                "Warning: No AES-compliant workspace members (crates/, packages/, or modules/) found in '{}'. \
                This system mandates a multi-module structure. Please refactor your project.",
                root.value
            );
            return Vec::new();
        }

        let futures = workspaces.into_iter().map(|ws| {
            let detector = self.deps.workspace_detector.clone();
            let reader = self.deps.config_reader.clone();
            async move {
                let ws_type = detector.detect(&ws);
                let language = ConfigLanguage::from(ws_type);
                let config = match reader.read_config(&ws, language).await {
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
            }
        });

        stream::iter(futures).buffered(8).collect().await
    }

    fn load_config_sync(&self, project_root: &FilePath) -> ArchitectureConfig {
        let root = std::path::Path::new(project_root.value());
        let ws_type = self.deps.workspace_detector.detect(project_root);
        let language = ConfigLanguage::from(ws_type);

        // FR-001: Search upward for config file (up to 5 levels)
        let mut current = root.to_path_buf();
        let mut depth = 0;
        let mut config = None;
        while !current.as_os_str().is_empty() && depth < 5 {
            for filename in language.config_file_names() {
                let candidate = current.join(filename);
                // FR-001: Reject symlinks pointing outside project root
                if let Ok(meta) = self.deps.filesystem.symlink_metadata(&candidate) {
                    if meta.file_type().is_symlink() {
                        if let Ok(canonical) = self.deps.filesystem.canonicalize(&candidate) {
                            let root_canonical =
                                self.deps.filesystem.canonicalize(root).unwrap_or_else(|_| root.to_path_buf());
                            if !canonical.starts_with(&root_canonical) {
                                eprintln!(
                                    "Warning: Symlink '{}' points outside project root, rejected.",
                                    candidate.display()
                                );
                                continue;
                            }
                        }
                    }
                }
                if let Ok(content) = self.deps.filesystem.read_to_string(&candidate) {
                    config = Some(parse_config_yaml(&content));
                    break;
                }
            }
            if config.is_some() {
                break;
            }
            if let Some(parent) = current.parent() {
                current = parent.to_path_buf();
                depth += 1;
            } else {
                break;
            }
        }

        let mut config = config.unwrap_or_else(|| default_config_for_language(language.as_str()));

        // Merge layers into config (same as make_layer_map in entry points)
        let (merged_layers, _) =
            shared::config_system::utility_config_merger::merge_config(&config);
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
        let runtime = tokio::runtime::Handle::try_current();
        let config = match runtime {
            Ok(rt) => match rt.runtime_flavor() {
                tokio::runtime::RuntimeFlavor::MultiThread => tokio::task::block_in_place(|| {
                    rt.block_on(async {
                        self.load_config_for_language(project_root, language).await
                    })
                    .config
                }),
                _ => self.load_config_sync(project_root),
            },
            Err(_) => self.load_config_sync(project_root),
        };
        PatternList::new(ignored_paths_from_config(&config))
    }

    async fn list_config_files(
        &self,
        project_root: &FilePath,
    ) -> Result<Vec<(ConfigLanguage, FilePath)>, ConfigError> {
        self.deps
            .config_reader
            .list_config_files(project_root)
            .await
    }

    async fn read_config(
        &self,
        project_root: &FilePath,
        language: ConfigLanguage,
    ) -> Result<Option<ConfigSource>, ConfigError> {
        self.deps
            .config_reader
            .read_config(project_root, language)
            .await
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

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

    let mut seen: std::collections::HashSet<String> =
        std::collections::HashSet::from_iter(DEFAULT_IGNORED.iter().map(|s| s.to_string()));
    // Pre-allocated capacity: 8 defaults + config count
    let mut ignored: Vec<String> = Vec::with_capacity(8 + config.ignored_paths.values.len());

    // Add default paths
    for &name in &DEFAULT_IGNORED {
        ignored.push(name.to_string());
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
