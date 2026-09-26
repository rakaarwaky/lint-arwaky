// PURPOSE: ConfigRequest/ConfigResponse — request/response VOs for the config aggregate
use crate::common::taxonomy_common_vo::PatternList;
use crate::common::taxonomy_path_vo::FilePath;
use crate::config_system::taxonomy_config_language_vo::ConfigLanguage;
use crate::config_system::taxonomy_config_vo::ArchitectureConfig;
use crate::config_system::taxonomy_multi_project_workspace_info_vo::WorkspaceInfo;
use crate::config_system::taxonomy_source_vo::ConfigResult;
use crate::config_system::taxonomy_source_vo::ConfigSource;

/// Consumer verb carried by the config aggregate's single entry point.
pub enum ConfigRequest {
    /// Load config for the detected language of a project.
    LoadProjectConfig { project_root: FilePath },
    /// Load config for a specific language.
    LoadForLanguage {
        project_root: FilePath,
        language: ConfigLanguage,
    },
    /// Read config directly without parsing/merging — return the raw source.
    ReadConfig {
        project_root: FilePath,
        language: ConfigLanguage,
    },
    /// Discover workspace members and their configs.
    DiscoverWorkspaces { root: FilePath },
    /// Synchronous config loading for container initialization.
    LoadSync { project_root: FilePath },
    /// Get ignored paths from config.
    IgnoredPaths { project_root: FilePath },
    /// Get ignored paths for a specific language.
    IgnoredPathsForLanguage {
        project_root: FilePath,
        language: ConfigLanguage,
    },
}

impl ConfigRequest {
    pub fn load_project_config(project_root: &FilePath) -> Self {
        Self::LoadProjectConfig {
            project_root: project_root.clone(),
        }
    }
    pub fn load_for_language(project_root: &FilePath, language: ConfigLanguage) -> Self {
        Self::LoadForLanguage {
            project_root: project_root.clone(),
            language,
        }
    }
    pub fn read_config(project_root: &FilePath, language: ConfigLanguage) -> Self {
        Self::ReadConfig {
            project_root: project_root.clone(),
            language,
        }
    }
    pub fn discover_workspaces(root: &FilePath) -> Self {
        Self::DiscoverWorkspaces { root: root.clone() }
    }
    pub fn load_sync(project_root: &FilePath) -> Self {
        Self::LoadSync {
            project_root: project_root.clone(),
        }
    }
    pub fn ignored_paths(project_root: &FilePath) -> Self {
        Self::IgnoredPaths {
            project_root: project_root.clone(),
        }
    }
    pub fn ignored_paths_for_language(project_root: &FilePath, language: ConfigLanguage) -> Self {
        Self::IgnoredPathsForLanguage {
            project_root: project_root.clone(),
            language,
        }
    }
}

/// Result of a config aggregate request.
pub enum ConfigResponse {
    LoadProjectConfig { result: ConfigResult },
    LoadForLanguage { result: ConfigResult },
    ReadConfig { source: Option<ConfigSource> },
    DiscoverWorkspaces { workspaces: Vec<WorkspaceInfo> },
    LoadSync { config: ArchitectureConfig },
    IgnoredPaths { patterns: PatternList },
    IgnoredPathsForLanguage { patterns: PatternList },
}

impl ConfigResponse {
    pub fn into_config_result(self) -> ConfigResult {
        match self {
            Self::LoadProjectConfig { result } | Self::LoadForLanguage { result } => result,
            _ => ConfigResult::default(),
        }
    }
    pub fn into_read_source(self) -> Option<ConfigSource> {
        match self {
            Self::ReadConfig { source } => source,
            _ => None,
        }
    }
    pub fn into_workspaces(self) -> Vec<WorkspaceInfo> {
        match self {
            Self::DiscoverWorkspaces { workspaces } => workspaces,
            _ => Vec::new(),
        }
    }
    pub fn into_sync_config(self) -> ArchitectureConfig {
        match self {
            Self::LoadSync { config } => config,
            _ => ArchitectureConfig::default(),
        }
    }
    pub fn into_patterns(self) -> PatternList {
        match self {
            Self::IgnoredPaths { patterns } | Self::IgnoredPathsForLanguage { patterns } => {
                patterns
            }
            _ => PatternList::new(Vec::<String>::new()),
        }
    }
}
