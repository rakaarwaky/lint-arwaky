use crate::common::taxonomy_common_vo::PatternList;
use crate::common::taxonomy_path_vo::FilePath;
use crate::config_system::taxonomy_config_language_vo::ConfigLanguage;
use crate::config_system::taxonomy_config_vo::ArchitectureConfig;
use crate::config_system::taxonomy_multi_project_workspace_info_vo::WorkspaceInfo;
use crate::config_system::taxonomy_source_vo::ConfigResult;

use crate::config_system::contract_parser_protocol::IConfigParserProtocol;
use crate::config_system::contract_reader_protocol::IConfigReaderProtocol;
use crate::config_system::contract_validator_protocol::IConfigValidatorProtocol;
use crate::config_system::contract_workspace_detector_protocol::IWorkspaceDetectorProtocol;

/// Aggregate trait — composes all 4 focused config protocol traits.
/// Consumer crates depending on config data import this aggregate;
/// others import the specific protocol trait they need.
pub trait IConfigOrchestratorAggregate:
    IConfigReaderProtocol
    + IConfigParserProtocol
    + IConfigValidatorProtocol
    + IWorkspaceDetectorProtocol
{
    /// Load config for the detected language of a project.
    fn load_project_config(&self, project_root: &FilePath) -> ConfigResult;

    /// Load config for a specific language.
    fn load_config_for_language(
        &self,
        project_root: &FilePath,
        language: ConfigLanguage,
    ) -> ConfigResult;

    /// Discover workspace members and their configs.
    fn discover_workspaces(&self, root: &FilePath) -> Vec<WorkspaceInfo>;

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
}
