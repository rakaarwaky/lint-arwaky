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
use shared_common::taxonomy_language_vo::ConfigLanguage;
use shared_config_system::taxonomy_config_system_request::ConfigRequest;
use shared_config_system::taxonomy_config_system_response::ConfigResponse;
use shared_config_system::taxonomy_config_system_vo::ArchitectureConfig;
use shared_config_system::taxonomy_config_system_vo::ConfigResult;
use shared_config_system::taxonomy_config_system_vo::ConfigSource;
use shared_config_system::taxonomy_config_system_vo::ProjectConfig;
use shared_config_system::taxonomy_config_system_vo::WorkspaceInfo;
use shared_config_system::utility_config_parser::default_config_for_language;
use shared_config_system::utility_config_parser::parse_project_config;
use shared_config_system::utility_config_parser::validate_project_config;
use shared_filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use std::sync::Arc;

use tracing::warn;

// ─── Block 1: Module Declaration ───────────────────────────

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

/// Hard bound on the parse cache.
///
/// `DashMap::with_capacity(32)` only pre-allocates — it never evicts — so a
/// long-lived process (the MCP server, which holds one orchestrator for hours
/// and can be pointed at many roots) grew this map without limit. One entry
/// per distinct config path, bounded at 64, is ample for any real workspace.
const CONFIG_CACHE_CAPACITY: usize = 64;

pub struct ConfigOrchestrator {
    deps: ConfigOrchestratorDeps,
    config_cache: DashMap<CacheKey, CachedConfig>,
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

                // The file was found and read — so the values in it must hold.
                // This used to be a warning at most, which meant a `threshold:`
                // typo left the effective gate at the embedded default of 80
                // and every command still exited 0.
                let parsed_project = parse_project_block(&source.raw_content);
                let mut errors = Vec::new();
                if let Some(project) = parsed_project.as_ref() {
                    let validation = validate_project_config(project);
                    if !validation.is_valid {
                        errors.push(format!(
                            "Invalid configuration in {}: {}",
                            source.path.value,
                            validation.reason.unwrap_or_default()
                        ));
                    }
                } else if let Err(reason) = &parse_strictness_error(&source.raw_content) {
                    // Unknown keys are rejected by the typed structs; a
                    // malformed file is reported the same way so the caller
                    // cannot mistake it for "no config".
                    errors.push(format!(
                        "Invalid configuration in {}: {reason}",
                        source.path.value
                    ));
                }
                ConfigResult::new(config, source, warnings).with_errors(errors)
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
                // A config that exists but cannot be read is a prerequisite
                // failure, not a quiet fall back to defaults: scoring a tree
                // against settings the operator never wrote is how a broken
                // config looks like a passing build.
                let warnings = vec![format!("Config error: {}; using defaults", e)];
                ConfigResult::new(
                    default_config_for_language(language.as_str()),
                    ConfigSource::new(language.as_str(), "embedded", ""),
                    warnings,
                )
                .with_errors(vec![format!("Cannot read configuration: {e}")])
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
            // FR-007: pre-allocated, and bounded by `evict_if_full`.
            config_cache: DashMap::with_capacity(CONFIG_CACHE_CAPACITY),
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

    /// Keep the parse cache bounded.
    ///
    /// Evicts one arbitrary entry when at capacity. Which entry goes is not
    /// important: a miss costs one YAML parse, and the alternative — evicting
    /// nothing — costs unbounded memory.
    fn evict_if_full(&self) {
        if self.config_cache.len() >= CONFIG_CACHE_CAPACITY {
            let victim = self.config_cache.iter().next().map(|e| e.key().clone());
            if let Some(victim) = victim {
                self.config_cache.remove(&victim);
            }
        }
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
        self.evict_if_full();
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
// ─── Block 2: Module Contract ──────────────

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

// ─── Config validation helpers ─────

/// Read the top-level project keys out of a config body.
///
/// The YAML root is the project block itself (`adapters:`, `ignored_rules:`,
/// …) with the rule tree nested under `architecture:`. Deserialising the whole
/// document into `ProjectConfig` would therefore always fail, so the
/// `architecture:` subtree is stripped first — it is validated by the parser.
fn parse_project_block(raw: &str) -> Option<ProjectConfig> {
    let stripped = project_block_yaml(raw);
    parse_project_config(&stripped).ok()
}

/// The project-level YAML with `architecture:` removed and empty keys dropped.
///
/// `ignored_paths:` with no value is written in the shipped config and means
/// "no ignore list". `#[serde(default)]` covers an *absent* key, not an
/// explicit null, so the nulls are pruned here — otherwise every load of the
/// default config would report a schema error that the operator cannot fix
/// without inventing an empty list.
fn project_block_yaml(raw: &str) -> String {
    let mut value: serde_yaml_ng::Value = match serde_yaml_ng::from_str(raw) {
        Ok(value) => value,
        Err(_) => return String::new(),
    };
    if let Some(map) = value.as_mapping_mut() {
        map.remove(serde_yaml_ng::Value::String("architecture".to_string()));
        map.retain(|_, v| !v.is_null());
    }
    serde_yaml_ng::to_string(&value).unwrap_or_default()
}

/// Why a config body could not be read as a `ProjectConfig` — surfaced so an
/// unknown key or bad value names the offending path instead of vanishing.
fn parse_strictness_error(raw: &str) -> Result<(), String> {
    if let Err(e) = serde_yaml_ng::from_str::<serde_yaml_ng::Value>(raw) {
        return Err(format!("YAML is not well formed: {e}"));
    }
    parse_project_config(&project_block_yaml(raw)).map(|_| ())
}

// ─── Block 3: Module Helpers ─────

// FR-ConfigSystem-001/002 are satisfied by the capabilities themselves
// (`ConfigYamlReader` implements `IConfigReadProtocol`, `WorkspaceDetector`
// implements `IWorkspaceMembersProtocol`). The orchestrator does not
// re-implement those seams: an agent must not implement a contract protocol
// (AES405), and the container wires the capabilities directly for callers
// that need them. The orchestrator holds the same capabilities as deps and
// uses them to compose its own behaviour.
