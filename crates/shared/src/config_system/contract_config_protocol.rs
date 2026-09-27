// PURPOSE: config-domain capability contracts (AES102 `_protocol`).
//
// One file for the config feature. Each trait below is one capability
// seam: a trait carries every method that capability implements, with one
// concrete return type each, so a capability implements its trait outright
// and never carries unimplemented stubs.

use crate::common::taxonomy_adapter_name_vo::AdapterName;
use crate::common::taxonomy_path_vo::FilePath;
use crate::config_system::taxonomy_config_error::ConfigError;
use crate::config_system::taxonomy_config_language_vo::ConfigLanguage;
use crate::config_system::taxonomy_config_vo::ConfigSource;
use crate::config_system::taxonomy_config_vo::ProjectConfig;
use crate::config_system::taxonomy_config_vo::ValidationResult;
pub use crate::config_system::taxonomy_config_vo::WorkspaceType;

pub trait IConfigParserProtocol: Send + Sync {
    fn parse_yaml_config(&self, path: &FilePath) -> Result<ProjectConfig, ConfigError>;
    fn parse_toml_config(&self, path: &FilePath) -> Result<Option<ProjectConfig>, ConfigError>;

    /// Parse YAML config content string into ArchitectureConfig + warnings.
    fn parse_config_yaml_with_warnings(
        &self,
        yaml_str: &str,
    ) -> (
        crate::config_system::taxonomy_config_vo::ArchitectureConfig,
        Vec<String>,
    );

    /// Parse adapter entries from YAML content string.
    fn parse_adapter_entries_from_yaml(
        &self,
        yaml_str: &str,
    ) -> Vec<crate::config_system::taxonomy_config_vo::AdapterEntry>;
}

pub trait IConfigReaderProtocol: Send + Sync {
    fn read_config(
        &self,
        project_root: &FilePath,
        language: ConfigLanguage,
    ) -> Result<Option<ConfigSource>, ConfigError>;

    fn list_config_files(
        &self,
        project_root: &FilePath,
    ) -> Result<Vec<(ConfigLanguage, FilePath)>, ConfigError>;
}

pub trait IConfigValidatorProtocol: Send + Sync {
    /// Determines if a specific adapter should run based on configuration rules.
    fn is_adapter_enabled(&self, config: &ProjectConfig, adapter_name: &AdapterName) -> bool;

    /// Validates that scoring thresholds are sane.
    fn validate_thresholds(&self, config: &ProjectConfig) -> ValidationResult;
}

pub trait IWorkspaceDetectorProtocol: Send + Sync {
    /// Detect workspace type by checking folder structure and config files.
    fn detect(&self, path: &FilePath) -> WorkspaceType;

    /// Check if a path is a workspace root (contains crates/, packages/, or modules/).
    fn is_workspace(&self, path: &FilePath) -> bool;

    /// Discover workspace member directories under the given root.
    fn discover_workspace_members(&self, root: &FilePath) -> Vec<FilePath>;
}
