// PURPOSE: Config value objects — architecture config, rule groups, project settings,
//          config source/result, workspace info, config keys, and validation results.
//
// Merged from the former taxonomy_config_system_vo, taxonomy_identifier_vo,
// taxonomy_multi_project_workspace_info_vo, taxonomy_setting_vo,
// taxonomy_source_vo, and taxonomy_validation_vo modules.
use crate::common::taxonomy_adapter_name_vo::AdapterName;
use crate::common::taxonomy_code_analysis_vo::CodeAnalysisRuleVO;
use crate::common::taxonomy_common_vo::BooleanVO;
use crate::common::taxonomy_common_vo::Count;
use crate::common::taxonomy_common_vo::PatternList;
use crate::common::taxonomy_common_vo::Score;
pub use crate::common::taxonomy_definition_vo::LayerDefinition;
use crate::common::taxonomy_definition_vo::NamingConfig;
use crate::common::taxonomy_definition_vo::OrphanRuleVO;
use crate::common::taxonomy_error_vo::ErrorCode;
use crate::common::taxonomy_layer_vo::LayerNameVO;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_paths_vo::FilePathList;
use crate::common::taxonomy_suggestion_vo::DescriptionVO;
use crate::config_system::taxonomy_config_language_vo::ConfigLanguage;
use crate::string_value_object;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ─── Architecture definition ─────────────────────────────────────────────

/// Workspace type detected from directory structure (e.g. crates/, packages/, modules/).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkspaceType {
    Rust,
    TypeScript,
    Python,
    Unknown,
}

impl WorkspaceType {
    pub fn as_str(&self) -> &'static str {
        match self {
            WorkspaceType::Rust => "rust",
            WorkspaceType::TypeScript => "typescript",
            WorkspaceType::Python => "python",
            WorkspaceType::Unknown => "unknown",
        }
    }
}

impl std::fmt::Display for WorkspaceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<WorkspaceType> for ConfigLanguage {
    fn from(ws: WorkspaceType) -> Self {
        match ws {
            WorkspaceType::Rust => ConfigLanguage::Rust,
            WorkspaceType::Python => ConfigLanguage::Python,
            WorkspaceType::TypeScript => ConfigLanguage::TypeScript,
            WorkspaceType::Unknown => ConfigLanguage::Rust,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct NamingRuleVO {
    #[serde(default)]
    pub naming_convention: BooleanVO,
    #[serde(default)]
    pub suffix_policy: crate::common::taxonomy_common_vo::SuffixPolicyVO,
    #[serde(default, alias = "allowed_suffix")]
    pub allowed_suffix: PatternList,
    #[serde(default, alias = "forbidden_suffix")]
    pub forbidden_suffix: PatternList,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RoleRuleVO {
    #[serde(default)]
    pub no_domain_logic: BooleanVO,
    #[serde(default)]
    pub must_implement_service_container_aggregate: BooleanVO,
    #[serde(default)]
    pub lazy_eager_initialization_only: BooleanVO,
    #[serde(default)]
    pub stateless_execution: BooleanVO,
    #[serde(default)]
    pub single_execution_goal: BooleanVO,
    #[serde(default)]
    pub high_level_policy_only: BooleanVO,
    #[serde(default)]
    pub coordinates_multiple_orchestrators: BooleanVO,
    #[serde(default)]
    pub crud_only: BooleanVO,
    #[serde(default)]
    pub no_decision_logic: BooleanVO,
    #[serde(default)]
    pub thread_async_safe: BooleanVO,
    #[serde(default)]
    pub no_domain_data_storage: BooleanVO,
    #[serde(default)]
    pub owns_system_health_transitions: BooleanVO,
    #[serde(default)]
    pub lifecycle_tracking_only: BooleanVO,
    #[serde(default)]
    pub no_primitives: BooleanVO,
    #[serde(default)]
    pub forbidden_inheritance: PatternList,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(default)]
pub struct ArchitectureRule {
    pub name: DescriptionVO,
    pub description: DescriptionVO,
    pub rule_type: ErrorCode,
    pub enabled: BooleanVO,
    pub scope: LayerNameVO,
    pub exceptions: PatternList,
    #[serde(default)]
    pub allowed: PatternList,
    #[serde(default)]
    pub forbidden: PatternList,
    #[serde(default)]
    pub mandatory: PatternList,

    #[serde(flatten)]
    pub naming: NamingRuleVO,
    #[serde(flatten)]
    pub code_analysis: CodeAnalysisRuleVO,
    #[serde(flatten)]
    pub role: RoleRuleVO,
    #[serde(flatten)]
    pub orphan: OrphanRuleVO,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct ArchitectureConfig {
    pub enabled: BooleanVO,
    pub layers: HashMap<LayerNameVO, LayerDefinition>,
    pub rules: Vec<ArchitectureRule>,
    pub naming: NamingConfig,
    pub ignored_paths: FilePathList,
    pub mandatory_class_definition: BooleanVO,
}

impl ArchitectureConfig {
    pub fn new(
        enabled: BooleanVO,
        layers: HashMap<LayerNameVO, LayerDefinition>,
        rules: Vec<ArchitectureRule>,
        naming: NamingConfig,
        ignored_paths: FilePathList,
        mandatory_class_definition: BooleanVO,
    ) -> Self {
        Self {
            enabled,
            layers,
            rules,
            naming,
            ignored_paths,
            mandatory_class_definition,
        }
    }
}

impl Default for ArchitectureConfig {
    fn default() -> Self {
        Self {
            enabled: BooleanVO::new(true),
            layers: HashMap::new(),
            rules: Vec::new(),
            naming: NamingConfig::new(Count::new(3)),
            ignored_paths: FilePathList { values: vec![] },
            mandatory_class_definition: BooleanVO::new(false),
        }
    }
}

// ─── Config identifiers ─────────────────────────────────────────────────

string_value_object!(ConfigKey);

impl ConfigKey {
    /// Returns each dot-separated segment of the key.
    pub fn parts(&self) -> Vec<String> {
        self.value.split('.').map(|s| s.to_string()).collect()
    }

    /// Returns the parent key, dropping the last segment. Empty when the
    /// key has no parent (single segment).
    pub fn parent(&self) -> String {
        let parts = self.parts();
        if parts.len() > 1 {
            parts[..parts.len() - 1].join(".")
        } else {
            String::new()
        }
    }

    /// Returns the last segment of the key, or the full value when the
    /// key has no `.` separators.
    pub fn leaf(&self) -> String {
        match self.parts().last() {
            Some(part) => part.clone(),
            None => self.value.clone(),
        }
    }
}

// ─── Project settings ───────────────────────────────────────────────────

string_value_object!(ActualValue);
string_value_object!(ExpectedValue);

/// Scoring thresholds.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Thresholds {
    pub score: Score,
    pub complexity: Count,
    pub max_file_lines: Count,
}

impl Thresholds {
    pub fn new(score: Score, complexity: Count, max_file_lines: Count) -> Self {
        Self {
            score,
            complexity,
            max_file_lines,
        }
    }
}

impl Default for Thresholds {
    fn default() -> Self {
        Self {
            score: Score::new(80.0),
            complexity: Count::new(10),
            max_file_lines: Count::new(500),
        }
    }
}

/// Adapter status enum.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum AdapterStatus {
    #[default]
    Enabled,
    Disabled,
    NotInstalled,
}

impl AdapterStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            AdapterStatus::Enabled => "enabled",
            AdapterStatus::Disabled => "disabled",
            AdapterStatus::NotInstalled => "not_installed",
        }
    }
}

impl std::fmt::Display for AdapterStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Single adapter configuration.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AdapterEntry {
    pub name: AdapterName,
    #[serde(default)]
    pub status: AdapterStatus,
    #[serde(default = "default_weight")]
    pub weight: f64,
    #[serde(default = "default_timeout")]
    pub timeout: f64,
}

fn default_weight() -> f64 {
    1.0
}

fn default_timeout() -> f64 {
    60.0
}

impl AdapterEntry {
    pub fn new(name: AdapterName, status: AdapterStatus, weight: f64) -> Self {
        Self {
            name,
            status,
            weight,
            timeout: 60.0,
        }
    }

    pub fn with_timeout(
        name: AdapterName,
        status: AdapterStatus,
        weight: f64,
        timeout: f64,
    ) -> Self {
        Self {
            name,
            status,
            weight,
            timeout,
        }
    }

    pub fn enabled(name: AdapterName) -> Self {
        Self::new(name, AdapterStatus::Enabled, 1.0)
    }

    pub fn is_active(&self) -> bool {
        matches!(self.status, AdapterStatus::Enabled)
    }
}

/// Project configuration.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ProjectConfig {
    #[serde(default = "default_project_name")]
    pub project_name: DescriptionVO,
    #[serde(default)]
    pub thresholds: Thresholds,
    #[serde(default)]
    pub adapters: Vec<AdapterEntry>,
    #[serde(default)]
    pub ignored_paths: FilePathList,
    #[serde(default)]
    pub ignored_rules: PatternList,
    #[serde(default)]
    pub layer_map: HashMap<String, String>,
    #[serde(default)]
    pub architecture: ArchitectureConfig,
}

fn default_project_name() -> DescriptionVO {
    DescriptionVO::new("lint-arwaky")
}

impl ProjectConfig {
    /// Returns a ProjectConfig with default linter adapters enabled.
    pub fn defaults() -> Self {
        Self {
            project_name: default_project_name(),
            thresholds: Thresholds::default(),
            adapters: vec![
                AdapterEntry::enabled(AdapterName::raw("ruff")),
                AdapterEntry::enabled(AdapterName::raw("mypy")),
                AdapterEntry::enabled(AdapterName::raw("bandit")),
                AdapterEntry::enabled(AdapterName::raw("radon")),
            ],
            ignored_paths: FilePathList::default(),
            ignored_rules: PatternList::default(),
            layer_map: HashMap::new(),
            architecture: ArchitectureConfig::default(),
        }
    }
}

// ─── Config source and load result ──────────────────────────────────────

/// Represents a configuration source with its language, path, and raw content.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ConfigSource {
    pub language: String,
    pub path: FilePath,
    pub raw_content: String,
}

impl ConfigSource {
    pub fn new(
        language: impl Into<String>,
        path: impl Into<String>,
        raw_content: impl Into<String>,
    ) -> Self {
        Self {
            language: language.into(),
            path: FilePath::new(path.into()).unwrap_or_default(),
            raw_content: raw_content.into(),
        }
    }
}

/// Result type for config loading operations containing the parsed config, source info, and warnings.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ConfigResult {
    pub config: ArchitectureConfig,
    pub source: ConfigSource,
    pub warnings: Vec<String>,
}

impl ConfigResult {
    pub fn new(config: ArchitectureConfig, source: ConfigSource, warnings: Vec<String>) -> Self {
        Self {
            config,
            source,
            warnings,
        }
    }
}

// ─── Workspace info ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceInfo {
    pub path: FilePath,
    pub workspace_type: String,
    pub config: ArchitectureConfig,
}

impl WorkspaceInfo {
    pub fn new(path: FilePath, workspace_type: String, config: ArchitectureConfig) -> Self {
        Self {
            path,
            workspace_type,
            config,
        }
    }
}

// ─── Validation result ──────────────────────────────────────────────────

/// Result of a validation operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub reason: Option<String>,
}

impl ValidationResult {
    pub fn ok() -> Self {
        Self {
            is_valid: true,
            reason: None,
        }
    }
    pub fn fail(reason: &str) -> Self {
        Self {
            is_valid: false,
            reason: Some(reason.to_string()),
        }
    }
}
