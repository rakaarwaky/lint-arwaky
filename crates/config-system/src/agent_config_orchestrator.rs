use dashmap::DashMap;
use shared_common::taxonomy_cache_key_vo::CacheKey;
use shared_common::taxonomy_common_vo::PatternList;
use shared_common::taxonomy_path_vo::FilePath;
use shared_config_system::contract_config_orchestrator_aggregate::IConfigOrchestratorAggregate;
use shared_config_system::contract_config_protocol::{
    IConfigMergeProtocol, IConfigReadProtocol, IWorkspaceMembersProtocol,
};
// Utility functions are free functions in shared crate, not traits:
// - ConfigLanguage::config_file_names()
// - parse_adapter_entries_from_yaml()
// - merge_default_ignored_paths()
// - ignored_paths_from_config()
// - default_config_for_language()
// These are imported via their modules below.
use shared_config_system::contract_config_protocol::WorkspaceType;
use shared_config_system::taxonomy_config_language_vo::ConfigLanguage;
use shared_config_system::taxonomy_config_system_error::ConfigError;
use shared_config_system::taxonomy_config_system_request::ConfigRequest;
use shared_config_system::taxonomy_config_system_response::ConfigResponse;
use shared_config_system::taxonomy_config_system_vo::ArchitectureConfig;
use shared_config_system::taxonomy_config_system_vo::ConfigResult;
use shared_config_system::taxonomy_config_system_vo::ConfigSource;
use shared_config_system::taxonomy_config_system_vo::WorkspaceInfo;
use shared_config_system::utility_config_parser::default_config_for_language;
use shared_filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use std::sync::Arc;

use tracing::warn;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct ConfigOrchestratorDeps {
    pub workspace_detector: Arc<dyn IWorkspaceMembersProtocol>,
    pub config_reader: Arc<dyn IConfigReadProtocol>,
    pub parser: Arc<dyn IConfigMergeProtocol>,
    pub filesystem: Arc<dyn IFilesystemAggregate>,
}

struct CachedConfig {
    content_hash: u64,
    config: Arc<ArchitectureConfig>,
}

pub struct ConfigOrchestrator {
    deps: ConfigOrchestratorDeps,
    config_cache: DashMap<CacheKey, CachedConfig>,
}

// ─── Block 2: Aggregate Trait Implementation ──────────────

impl IConfigOrchestratorAggregate for ConfigOrchestrator {
    fn execute(&self, request: ConfigRequest) -> ConfigResponse {
        match request {
            ConfigRequest::LoadProjectConfig { project_root } => {
                ConfigResponse::LoadProjectConfig {
                    result: self.load_project_config(&project_root),
                }
            }
            ConfigRequest::LoadForLanguage {
                project_root,
                language,
            } => ConfigResponse::LoadForLanguage {
                result: self.load_config_for_language(&project_root, language),
            },
            ConfigRequest::ReadConfig {
                project_root,
                language,
            } => ConfigResponse::ReadConfig {
                source: self.read_config(&project_root, language),
            },
            ConfigRequest::DiscoverWorkspaces { root } => ConfigResponse::DiscoverWorkspaces {
                workspaces: self.discover_workspaces(&root),
            },
            ConfigRequest::LoadSync { project_root } => ConfigResponse::LoadSync {
                config: self.load_config_sync(&project_root),
            },
            ConfigRequest::IgnoredPaths { project_root } => ConfigResponse::IgnoredPaths {
                patterns: self.ignored_paths(&project_root),
            },
            ConfigRequest::IgnoredPathsForLanguage {
                project_root,
                language,
            } => ConfigResponse::IgnoredPathsForLanguage {
                patterns: self.ignored_paths_for_language(&project_root, language),
            },
        }
    }
}

// ─── Block 3: Constructors, Std Traits, Protocol Delegations, Helpers ─────

impl IConfigReadProtocol for ConfigOrchestrator {
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

impl IWorkspaceMembersProtocol for ConfigOrchestrator {
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

impl ConfigOrchestrator {
    pub fn read_config(
        &self,
        project_root: &FilePath,
        language: ConfigLanguage,
    ) -> Option<ConfigSource> {
        match self.deps.config_reader.read_config(project_root, language) {
            Ok(source) => source,
            Err(e) => {
                warn!(root = %project_root.value, error = %e, "failed to read config; no config source");
                None
            }
        }
    }

    pub fn load_project_config(&self, project_root: &FilePath) -> ConfigResult {
        let ws_type = self.deps.workspace_detector.detect(project_root);
        let language = ConfigLanguage::from(ws_type);
        self.load_config_for_language(project_root, language)
    }

    pub fn load_config_for_language(
        &self,
        project_root: &FilePath,
        language: ConfigLanguage,
    ) -> ConfigResult {
        match self.deps.config_reader.read_config(project_root, language) {
            Ok(Some(source)) => {
                let (parsed_config, parse_warnings) = self
                    .deps
                    .parser
                    .parse_config_yaml_with_warnings(&source.raw_content);
                let had_layers = !parsed_config.layers.is_empty();
                let mut warnings = parse_warnings;
                if !had_layers {
                    warnings.push(
                        "Config file had no architecture layers, using built-in defaults for layers only."
                            .to_string(),
                    );
                }
                let config = self.merge_and_fill_defaults_cached(&source, language);
                ConfigResult::new(config, source, warnings)
            }
            Ok(None) => {
                let warnings = vec!["No config file found, using built-in defaults".to_string()];
                ConfigResult::new(
                    default_config_for_language(language.as_str()),
                    ConfigSource::new(language.as_str(), "embedded", ""),
                    warnings,
                )
            }
            Err(e) => {
                let warnings = vec![format!("Config error: {}; using defaults", e)];
                ConfigResult::new(
                    default_config_for_language(language.as_str()),
                    ConfigSource::new(language.as_str(), "embedded", ""),
                    warnings,
                )
            }
        }
    }

    pub fn discover_workspaces(&self, root: &FilePath) -> Vec<WorkspaceInfo> {
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
                match self.deps.config_reader.read_config(&ws, language) {
                    Ok(Some(source)) => {
                        let config = self.merge_and_fill_defaults_cached(&source, language);
                        WorkspaceInfo::new(ws, language.to_string(), config)
                    }
                    Ok(None) => {
                        warn!(
                            ws = %ws.value,
                            "workspace has no config file; using built-in defaults"
                        );
                        let config = default_config_for_language(language.as_str());
                        WorkspaceInfo::new(ws, language.to_string(), config)
                    }
                    Err(e) => {
                        warn!(
                            ws = %ws.value,
                            error = %e,
                            "workspace config read failed; using built-in defaults"
                        );
                        let config = default_config_for_language(language.as_str());
                        WorkspaceInfo::new(ws, language.to_string(), config)
                    }
                }
            })
            .collect()
    }

    pub fn load_config_sync(&self, project_root: &FilePath) -> ArchitectureConfig {
        let ws_type = self.deps.workspace_detector.detect(project_root);
        let language = ConfigLanguage::from(ws_type);

        match self.deps.config_reader.read_config(project_root, language) {
            Ok(Some(source)) => self.merge_and_fill_defaults_cached(&source, language),
            Ok(None) => {
                warn!(
                    root = %project_root.value,
                    "no config file found; using built-in defaults"
                );
                default_config_for_language(language.as_str())
            }
            Err(e) => {
                warn!(
                    root = %project_root.value,
                    error = %e,
                    "config read failed; using built-in defaults"
                );
                default_config_for_language(language.as_str())
            }
        }
    }

    pub fn ignored_paths(&self, project_root: &FilePath) -> PatternList {
        let ws_type = self.deps.workspace_detector.detect(project_root);
        let language = ConfigLanguage::from(ws_type);
        let result = self.load_config_for_language(project_root, language);
        PatternList::new(
            shared_config_system::utility_config_merger::merge_default_ignored_paths(
                shared_config_system::utility_config_merger::ignored_paths_from_config(
                    &result.config,
                ),
            ),
        )
    }

    pub fn ignored_paths_for_language(
        &self,
        project_root: &FilePath,
        language: ConfigLanguage,
    ) -> PatternList {
        let result = self.load_config_for_language(project_root, language);
        PatternList::new(
            shared_config_system::utility_config_merger::merge_default_ignored_paths(
                shared_config_system::utility_config_merger::ignored_paths_from_config(
                    &result.config,
                ),
            ),
        )
    }

    /// Create a new config orchestrator with all required protocol dependencies.
    pub fn new(deps: ConfigOrchestratorDeps) -> Self {
        Self {
            deps,
            // FR-007: DashMap with pre-allocated capacity 32
            config_cache: DashMap::with_capacity(32),
        }
    }

    pub fn parser(&self) -> &Arc<dyn IConfigMergeProtocol> {
        &self.deps.parser
    }

    /// Load from cache (single parse per key), apply merge, layer defaults.
    fn merge_and_fill_defaults_cached(
        &self,
        source: &ConfigSource,
        language: ConfigLanguage,
    ) -> ArchitectureConfig {
        let (parsed, _) =
            self.parse_cached(&CacheKey::new(source.path.value()), &source.raw_content);
        let (merged, _) = self
            .deps
            .parser
            .merge_config_with_defaults(&parsed, language);
        merged
    }

    /// Internal cache: parse once per key, serve later reads from the cache.
    fn parse_cached(
        &self,
        cache_key: &CacheKey,
        yaml_str: &str,
    ) -> (ArchitectureConfig, Vec<String>) {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        yaml_str.hash(&mut hasher);
        let content_hash = hasher.finish();
        if let Some(cached) = self.config_cache.get(cache_key)
            && cached.content_hash == content_hash
        {
            return (cached.config.as_ref().clone(), Vec::new());
        }
        let (parsed, warnings) = self.deps.parser.parse_config_yaml_with_warnings(yaml_str);
        self.config_cache.insert(
            cache_key.clone(),
            CachedConfig {
                content_hash,
                config: Arc::new(parsed.clone()),
            },
        );
        (parsed, warnings)
    }
}
