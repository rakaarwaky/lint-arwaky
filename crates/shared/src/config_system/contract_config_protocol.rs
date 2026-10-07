// PURPOSE: config-domain capability contracts (AES102 `_protocol`).
//
// Only 3 business-capability protocols mapped to 3 FRs:
//   FR-001 Config Discovery    → IConfigReadProtocol
//   FR-002 Multi-Workspace     → IWorkspaceMembersProtocol
//   FR-003 Config Merger       → IConfigMergeProtocol
//
// All other methods are utility functions (no traits) in the shared crate.
// `IConfigOrchestratorAggregate` is the entry point and does not count toward per-FR protocols.

use crate::taxonomy_config_system_error::ConfigError;
use crate::taxonomy_config_system_vo::{ArchitectureConfig, ConfigSource, ProjectConfig};
use crate::taxonomy_language_vo::ConfigLanguage;
use shared_common::taxonomy_path_vo::FilePath;
// Re-exported so consumers keep one import path for the workspace enum the
// FR-002 seam returns.
pub use crate::taxonomy_config_system_vo::WorkspaceType;

/// FR-001: Locate and load the first matching YAML config for a project root
/// and language, following the 5-level priority chain.
pub trait IConfigReadProtocol: Send + Sync {
    /// Read the first config matching the priority chain for `language`;
    /// `None` means the caller falls back to embedded defaults.
    fn read_config(
        &self,
        project_root: &FilePath,
        language: ConfigLanguage,
    ) -> Result<Option<ConfigSource>, ConfigError>;

    /// List every config file present at the project root, one per language,
    /// deduplicated by path. Discovery resolves config paths per language, so
    /// it lives on this same FR-001 seam.
    fn list_config_files(
        &self,
        project_root: &FilePath,
    ) -> Result<Vec<(ConfigLanguage, FilePath)>, ConfigError>;
}

/// FR-002: Discover all workspace member directories under crates/packages/modules.
/// Workspace type detection and workspace-directory probing are part of this
/// same multi-workspace concern, so they live on this trait.
pub trait IWorkspaceMembersProtocol: Send + Sync {
    /// Detect workspace type by checking marker files and folder structure.
    fn detect(&self, path: &FilePath) -> WorkspaceType;

    /// Check if a path is a workspace root (contains crates/, packages/, or modules/).
    fn is_workspace(&self, path: &FilePath) -> bool;

    /// Discover workspace member directories under the given root.
    fn discover_workspace_members(&self, root: &FilePath) -> Vec<FilePath>;
}

/// FR-003: Merge loaded configuration with embedded defaults using rule-based
/// layer merging, including scoped sub-layer creation.
pub trait IConfigMergeProtocol: Send + Sync {
    /// Parse a YAML config file into a ProjectConfig (which wraps ArchitectureConfig).
    fn parse_yaml_config(&self, path: &FilePath) -> Result<ProjectConfig, ConfigError>;

    /// Parse YAML config content into ArchitectureConfig plus warnings.
    fn parse_config_yaml_with_warnings(&self, yaml_str: &str) -> (ArchitectureConfig, Vec<String>);

    /// Merge a parsed config into the embedded defaults, returning the merged
    /// config and the warnings produced by the merge.
    fn merge_config_with_defaults(
        &self,
        config: &ArchitectureConfig,
        language: ConfigLanguage,
    ) -> (ArchitectureConfig, Vec<String>);

    /// Parse the `[tool.lint-arwaky]` section of a TOML file into a ProjectConfig.
    fn parse_toml_config(&self, path: &FilePath) -> Result<Option<ProjectConfig>, ConfigError>;
}
