use async_trait::async_trait;
use futures::stream::{self, StreamExt};
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::contract_config_orchestrator_aggregate::IConfigOrchestratorAggregate;
use shared::config_system::contract_reader_protocol::IConfigReaderProtocol;
use shared::config_system::contract_validator_protocol::IConfigValidatorProtocol;
use shared::config_system::contract_workspace_detector_protocol::IWorkspaceDetectorProtocol;
use shared::config_system::taxonomy_config_error::ConfigError;
use shared::config_system::taxonomy_config_language_vo::ConfigLanguage;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::config_system::taxonomy_multi_project_workspace_info_vo::WorkspaceInfo;
use shared::config_system::taxonomy_source_vo::ConfigResult;
use shared::config_system::taxonomy_source_vo::ConfigSource;
use shared::config_system::utility_config_defaults::default_config_for_language;
use shared::config_system::utility_config_parser::parse_config_yaml;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct ConfigOrchestrator {
    workspace_detector: Arc<dyn IWorkspaceDetectorProtocol>,
    config_reader: Arc<dyn IConfigReaderProtocol>,
    validator: Arc<dyn IConfigValidatorProtocol>,
    config_cache: Mutex<HashMap<String, Arc<ArchitectureConfig>>>,
}

// ─── Block 2: Aggregate Trait Implementation ──────────────

#[async_trait]
impl IConfigOrchestratorAggregate for ConfigOrchestrator {
    async fn load_project_config(&self, project_root: &FilePath) -> ConfigResult {
        let ws_type = self.workspace_detector.detect(project_root);
        let language = ConfigLanguage::from(ws_type);
        self.load_config_for_language(project_root, language).await
    }

    async fn load_config_for_language(
        &self,
        project_root: &FilePath,
        language: ConfigLanguage,
    ) -> ConfigResult {
        match self.config_reader.read_config(project_root, language).await {
            Ok(Some(source)) => {
                let cache_key = source.path.to_string();
                let mut parsed = {
                    let mut cache = self.config_cache.lock().unwrap_or_else(|e| e.into_inner());
                    cache
                        .entry(cache_key.clone())
                        .or_insert_with(|| Arc::new(parse_config_yaml(&source.raw_content)))
                        .as_ref()
                        .clone()
                };
                let mut warnings = Vec::new();
                if parsed.layers.is_empty() {
                    let defaults = default_config_for_language(language.as_str());
                    parsed.layers = defaults.layers;
                    warnings.push(
                        "Config file had no architecture layers, using built-in defaults for layers only."
                            .to_string(),
                    );
                }
                ConfigResult::new(parsed, source, warnings)
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
            let detector = self.workspace_detector.clone();
            let reader = self.config_reader.clone();
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

    fn load_config_sync(&self, project_root: &str) -> ArchitectureConfig {
        let root = std::path::Path::new(project_root);
        let ws_type = self
            .workspace_detector
            .detect(&FilePath::new(project_root.to_string()).unwrap_or_default());
        let language = ConfigLanguage::from(ws_type);

        // Search upward for config file (up to 3 levels)
        let mut current = root.to_path_buf();
        let mut depth = 0;
        let mut config = None;
        while !current.as_os_str().is_empty() && depth < 3 {
            for filename in language.config_file_names() {
                let candidate = current.join(filename);
                if let Ok(content) = std::fs::read_to_string(&candidate) {
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

    fn ignored_paths(&self, project_root: &str) -> Vec<String> {
        let config = self.load_config_sync(project_root);
        ignored_paths_from_config(&config)
    }

    fn ignored_paths_for_language(
        &self,
        project_root: &str,
        language: ConfigLanguage,
    ) -> Vec<String> {
        let path = FilePath::new(project_root.to_string()).unwrap_or_default();
        let runtime = tokio::runtime::Handle::try_current();
        let config = match runtime {
            Ok(rt) => match rt.runtime_flavor() {
                tokio::runtime::RuntimeFlavor::MultiThread => tokio::task::block_in_place(|| {
                    rt.block_on(async { self.load_config_for_language(&path, language).await })
                        .config
                }),
                _ => self.load_config_sync(project_root),
            },
            Err(_) => self.load_config_sync(project_root),
        };
        ignored_paths_from_config(&config)
    }

    async fn list_config_files(
        &self,
        project_root: &FilePath,
    ) -> Result<Vec<(ConfigLanguage, FilePath)>, ConfigError> {
        self.config_reader.list_config_files(project_root).await
    }

    async fn read_config(
        &self,
        project_root: &FilePath,
        language: ConfigLanguage,
    ) -> Result<Option<ConfigSource>, ConfigError> {
        self.config_reader.read_config(project_root, language).await
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl ConfigOrchestrator {
    pub fn new(
        workspace_detector: Arc<dyn IWorkspaceDetectorProtocol>,
        config_reader: Arc<dyn IConfigReaderProtocol>,
        validator: Arc<dyn IConfigValidatorProtocol>,
    ) -> Self {
        // Pre-allocate cache with capacity hint for multi-workspace projects
        Self {
            workspace_detector,
            config_reader,
            validator,
            config_cache: Mutex::new(HashMap::with_capacity(32)),
        }
    }

    pub fn validator(&self) -> &Arc<dyn IConfigValidatorProtocol> {
        &self.validator
    }
}

fn ignored_paths_from_config(config: &ArchitectureConfig) -> Vec<String> {
    // Use const array for default paths, HashSet for O(1) dedup
    const DEFAULT_IGNORED: [&str; 10] = [
        "target",
        ".mimocode",
        ".agents",
        "node_modules",
        "build.rs",
        ".git",
        "dist",
        "build",
        "coverage",
        ".venv",
    ];

    let mut seen: std::collections::HashSet<String> =
        std::collections::HashSet::from_iter(DEFAULT_IGNORED.iter().map(|s| s.to_string()));
    let mut ignored: Vec<String> = Vec::with_capacity(10 + config.ignored_paths.values.len());

    // Add default paths
    for &name in &DEFAULT_IGNORED {
        ignored.push(name.to_string());
    }

    // Add config paths with dedup
    for fp in config.ignored_paths.values.iter() {
        let v = fp.value.replace('/', std::path::MAIN_SEPARATOR_STR);
        if !v.is_empty() && seen.insert(v.clone()) {
            ignored.push(v);
        }
    }
    ignored
}
