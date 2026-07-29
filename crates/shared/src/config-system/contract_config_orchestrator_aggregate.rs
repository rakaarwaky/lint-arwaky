use crate::common::taxonomy_common_vo::PatternList;
use crate::common::taxonomy_path_vo::FilePath;
use crate::config_system::taxonomy_config_error::ConfigError;
use crate::config_system::taxonomy_config_language_vo::ConfigLanguage;
use crate::config_system::taxonomy_config_vo::ArchitectureConfig;
use crate::config_system::taxonomy_multi_project_workspace_info_vo::WorkspaceInfo;
use crate::config_system::taxonomy_source_vo::{ConfigResult, ConfigSource};
use crate::orphan_detector::IOrphanAggregate;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait IConfigOrchestratorAggregate: Send + Sync {
    async fn load_project_config(&self, project_root: &FilePath) -> ConfigResult;

    async fn load_config_for_language(
        &self,
        project_root: &FilePath,
        language: ConfigLanguage,
    ) -> ConfigResult;

    async fn discover_workspaces(&self, root: &FilePath) -> Vec<WorkspaceInfo>;

    /// Synchronous config loading for container initialization.
    /// Searches workspace root for config YAML, falls back to embedded defaults.
    fn load_config_sync(&self, project_root: &FilePath) -> ArchitectureConfig;

    /// Get ignored paths from config (hardcoded defaults + config values).
    fn ignored_paths(&self, project_root: &FilePath) -> PatternList;

    /// Get ignored paths for a specific language (hardcoded defaults + config values for that language).
    fn ignored_paths_for_language(
        &self,
        project_root: &FilePath,
        language: ConfigLanguage,
    ) -> PatternList;

    /// List config files found in the project.
    async fn list_config_files(
        &self,
        project_root: &FilePath,
    ) -> Result<Vec<(ConfigLanguage, FilePath)>, ConfigError>;

    /// Read config content for a specific language.
    async fn read_config(
        &self,
        project_root: &FilePath,
        language: ConfigLanguage,
    ) -> Result<Option<ConfigSource>, ConfigError>;

    /// Build an orphan analyzer from the config orchestrator for a given target path.
    ///
    /// This loads the real config (layers + enabled) for the target workspace,
    /// not the frozen CWD config. Surface layer should use this instead of
    /// importing OrphanContainer directly from root crate.
    fn create_orphan_analyzer(&self, project_root: &str) -> Arc<dyn IOrphanAggregate>;
}
