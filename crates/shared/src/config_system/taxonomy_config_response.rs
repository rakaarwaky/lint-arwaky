// PURPOSE: ConfigResponse — response payload for the config aggregate

use crate::common::taxonomy_common_vo::PatternList;
use crate::config_system::taxonomy_config_vo::ArchitectureConfig;
use crate::config_system::taxonomy_multi_project_workspace_info_vo::WorkspaceInfo;
use crate::config_system::taxonomy_source_vo::ConfigResult;
use crate::config_system::taxonomy_source_vo::ConfigSource;

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
