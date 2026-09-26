// PURPOSE: ConfigRequest — request payload for the config aggregate

use crate::common::taxonomy_path_vo::FilePath;
use crate::config_system::taxonomy_config_language_vo::ConfigLanguage;

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
