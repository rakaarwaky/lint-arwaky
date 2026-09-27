// PURPOSE: config-domain capability contracts (AES102 `_protocol`).
//
// One file for the config feature. One trait per functional requirement in
// `crates/config-system/FRD.md`, each carrying only the methods that FR
// specifies, so a capability implements its trait outright and never carries
// unimplemented stubs. `IConfigOrchestratorAggregate` is the entry point and
// does not count toward the per-FR protocols.

use crate::common::taxonomy_adapter_name_vo::AdapterName;
use crate::common::taxonomy_cache_key_vo::CacheKey;
use crate::common::taxonomy_common_vo::PatternList;
use crate::common::taxonomy_path_vo::FilePath;
use crate::config_system::taxonomy_config_error::ConfigError;
use crate::config_system::taxonomy_config_language_vo::ConfigLanguage;
use crate::config_system::taxonomy_config_vo::AdapterEntry;
use crate::config_system::taxonomy_config_vo::ArchitectureConfig;
use crate::config_system::taxonomy_config_vo::ConfigSource;
use crate::config_system::taxonomy_config_vo::ProjectConfig;
use crate::config_system::taxonomy_config_vo::ValidationResult;
pub use crate::config_system::taxonomy_config_vo::WorkspaceType;

/// FR-ConfigSystem-001: locate and load the first matching YAML config for a
/// project root and language, following the 5-level priority chain.
pub trait IConfigReadProtocol: Send + Sync {
    /// Read the first config matching the priority chain for `language`;
    /// `None` means the caller falls back to embedded defaults.
    fn read_config(
        &self,
        project_root: &FilePath,
        language: ConfigLanguage,
    ) -> Result<Option<ConfigSource>, ConfigError>;
}

/// FR-ConfigSystem-002: map a language to the config filenames to search for.
pub trait IConfigLanguageProtocol: Send + Sync {
    /// Config filenames to search, in priority order, for `language`.
    fn config_file_names(&self, language: ConfigLanguage) -> Vec<String>;
}

/// FR-ConfigSystem-003: detect workspace type from marker files and parent
/// directory conventions.
pub trait IWorkspaceDetectProtocol: Send + Sync {
    /// Detect workspace type by checking marker files and folder structure.
    fn detect(&self, path: &FilePath) -> WorkspaceType;

    /// Check if a path is a workspace root (contains crates/, packages/, or modules/).
    fn is_workspace(&self, path: &FilePath) -> bool;
}

/// FR-ConfigSystem-004: discover all workspace member directories under
/// crates/, packages/, and modules/. Discovery reads the same workspace layout
/// that `FR-003` detection classifies, so it extends `IWorkspaceDetectProtocol`.
pub trait IWorkspaceMembersProtocol: IWorkspaceDetectProtocol {
    /// Discover workspace member directories under the given root.
    fn discover_workspace_members(&self, root: &FilePath) -> Vec<FilePath>;
}

/// FR-ConfigSystem-005: parse config content and merge it with embedded
/// defaults using rule-based layer merging with scoped sub-layer creation.
/// Config parsing covers every supported format, so it extends the TOML seam
/// of FR-009.
pub trait IConfigParseProtocol: IConfigTomlProtocol {
    /// Parse a YAML config file into a project config.
    fn parse_yaml_config(&self, path: &FilePath) -> Result<ProjectConfig, ConfigError>;

    /// Parse YAML config content into ArchitectureConfig plus warnings.
    fn parse_config_yaml_with_warnings(&self, yaml_str: &str) -> (ArchitectureConfig, Vec<String>);

    /// Parse adapter entries from a YAML content string.
    fn parse_adapter_entries_from_yaml(&self, yaml_str: &str) -> Vec<AdapterEntry>;

    /// Merge a parsed config into the embedded defaults, returning the merged
    /// config and the warnings produced by the merge.
    fn merge_config_with_defaults(
        &self,
        config: &ArchitectureConfig,
        language: ConfigLanguage,
    ) -> (ArchitectureConfig, Vec<String>);
}

/// FR-ConfigSystem-006: validate loaded project config thresholds and adapter
/// settings against schema constraints.
pub trait IConfigValidateProtocol: Send + Sync {
    /// Determines if a specific adapter should run based on configuration rules.
    fn is_adapter_enabled(&self, config: &ProjectConfig, adapter_name: &AdapterName) -> bool;

    /// Validates that scoring thresholds are sane.
    fn validate_thresholds(&self, config: &ProjectConfig) -> ValidationResult;
}

/// FR-ConfigSystem-007: cache parsed config by file path to avoid repeated YAML
/// parsing.
pub trait IConfigCacheProtocol: Send + Sync {
    /// Parse `yaml_str` and memoize it under `cache_key`; a hit returns the
    /// previously cached value without re-parsing.
    fn parse_cached(
        &self,
        cache_key: &CacheKey,
        yaml_str: &str,
    ) -> (ArchitectureConfig, Vec<String>);

    /// Whether `cache_key` currently holds a parsed config.
    fn is_cached(&self, cache_key: &CacheKey) -> bool;
}

/// FR-ConfigSystem-008: build the complete ignored-path list from config plus
/// the hardcoded universal defaults.
pub trait IConfigIgnoredPathsProtocol: Send + Sync {
    /// Ignored path patterns: universal defaults plus config, deduplicated.
    fn build_ignored_paths(&self, config: &ArchitectureConfig) -> PatternList;
}

/// FR-ConfigSystem-009: parse TOML config files into a project config.
pub trait IConfigTomlProtocol: Send + Sync {
    /// Parse the `[tool.lint-arwaky]` section of a TOML file.
    fn parse_toml_config(&self, path: &FilePath) -> Result<Option<ProjectConfig>, ConfigError>;
}

/// FR-ConfigSystem-010: list every config file present at the project root for
/// all supported languages. Listing resolves config paths per language, so it
/// also carries the language seam of FR-002.
pub trait IConfigListProtocol: IConfigReadProtocol + IConfigLanguageProtocol {
    /// Config file paths present at the project root, deduplicated by path and
    /// broken after the first config found per language.
    fn list_config_files(
        &self,
        project_root: &FilePath,
    ) -> Result<Vec<(ConfigLanguage, FilePath)>, ConfigError>;
}
