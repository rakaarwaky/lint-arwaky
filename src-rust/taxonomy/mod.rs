//! # Taxonomy Layer — The Domain Foundation
//!
//! This module is the **innermost layer** of the AES architecture. It contains
//! pure, framework-agnostic domain models, value objects, and business entities.
//! It has zero external dependencies (except `serde` for serialization and `chrono`
//! for timestamp generation) and represents the fundamental vocabulary of the system.
//!
//! ## Layer Rules (AES Compliance)
//! - **Allowed Imports**: Strictly limited to `src/taxonomy/`. Outer imports trigger **AES001**.
//! - **Allowed Suffixes**: `_vo`, `_entity`, `_event`, `_error`, `_constant`
//! - **Primitive Usage**: Forbidden in core entities . Must be wrapped in VOs.
//!
//! ## Import Patterns
//!
//! Import specific types directly from the barrel for explicit dependencies:
//!
//! ```rust
//! use lint_arwaky::taxonomy::{LintResult, Severity, ErrorCode, FilePath};
//! ```
//!
//! Or use factory functions for ergonomic VO construction:
//!
//! ```rust
//! use lint_arwaky::taxonomy::*;
//!
//! let line = line_number(42);
//! let path = file_path("src/main.rs");
//! let score = score(95.5);
//! ```
//!
//! ## Module Index
//!
//! | Domain / Feature | Key Types | Description |
//! |------------------|-----------|-------------|
//! | **Linting** | `LintResult`, `LintResultList`, `Severity`, `ErrorCode`, `Position`, `ComplianceStatus`, `LintMessage` | Core linting results, severity, & compliance |
//! | **Source Code** | `FilePath`, `DirectoryPath`, `ContentString`, `ImportInfo`, `SuffixVO`, `PrimitiveViolation` | Source file, path, & import analysis |
//! | **Architecture** | `ArchitectureConfig`, `ArchitectureRule`, `LayerDefinition`, `LayerMapVO` | Layer-based architecture governance |
//! | **Configuration** | `AppConfig`, `ProjectConfig`, `Thresholds`, `ConfigKey` | Application & project configuration |
//! | **Adapter** | `AdapterName`, `AdapterMetadata`, `AdapterRegistered`, `AdapterClassMap` | Plugin adapter registration & lifecycle |
//! | **Capability** | `CapabilityRoutingContext`, `ClassDefinitionMap`, `CapabilityReference` | Capability-based dispatch routing |
//! | **Common** | `LineNumber`, `ColumnNumber`, `Score`, `Timestamp`, `Count`, `Duration`, `Timeout` | Shared primitive value objects |
//! | **Git** | `GitRef`, `GitDiffResultVO`, `GitHookError` | Version control integration |
//! | **Job** | `JobId`, `JobStatus`, `ActionArgs`, `JobError` | Background job scheduling |
//! | **Transport** | `TransportProtocol`, `TransportEndpoint`, `TransportUrlVO`, `TransportError` | Inter-process communication protocol |
//! | **Plugin** | `PluginGroup`, `PluginError`, `DiscoveryError` | Plugin discovery & lifecycle |
//! | **Naming** | `SymbolName`, `NameVariants`, `PrimitiveTypeList`, `SymbolNameList` | Naming convention validation |
//! | **Semantic** | `SemanticError`, `CallChainError`, `ScopeResolutionError` | Code semantic & scope tracing |
//! | **Fix** | `FixResult`, `FixApplied` | Auto-fix results & events |
//! | **Watch** | `WatchResult`, `WatchServiceError` | File system watcher models |
//! | **Doctor** | `DoctorResultVO` | System diagnostics & health check |

// ═══════════════════════════════════════════════════════════════════════════════
// MODULE DECLARATIONS
// ═══════════════════════════════════════════════════════════════════════════════

pub mod adapter_collection_vo;
pub mod adapter_name_vo;
pub mod adapter_registered_event;
pub mod agent_status_vo;
pub mod architecture_analysis_vo;
pub mod architecture_config_vo;
pub mod architecture_governance_entity;
pub mod architecture_rule_vo;
pub mod capability_routing_vo;
pub mod command_catalog_constant;
pub mod command_metadata_vo;
pub mod common_collection_vo;
pub mod common_duration_vo;
pub mod common_error_vo;
pub mod config_app_vo;
pub mod config_identifier_vo;
pub mod config_provider_error;
pub mod config_setting_vo;
pub mod config_source_vo;
pub mod doctor_result_vo;
pub mod error_code_vo;
pub mod file_system_error;
pub mod fix_applied_event;
pub mod fix_result_vo;
pub mod git_diff_vo;
pub mod git_hook_error;
pub mod git_ref_vo;
pub mod hook_installed_event;
pub mod hook_removed_event;
pub mod job_action_vo;
pub mod job_registry_error;
pub mod layer_content_vo;
pub mod layer_definition_vo;
pub mod layer_names_constant;
pub mod layer_names_vo;
pub mod lint_adapter_error;
pub mod lint_domain_vo;
pub mod lint_operation_error;
pub mod lint_position_vo;
pub mod lint_result_vo;
pub mod lint_score_constant;
pub mod lint_score_vo;
pub mod lint_severity_vo;
pub mod lint_status_vo;
pub mod log_suggestion_vo;
pub mod maintenance_stats_vo;
pub mod mcp_server_constant;
pub mod message_status_vo;
pub mod metrics_provider_error;
pub mod naming_provider_error;
pub mod naming_symbol_vo;
pub mod naming_symbols_constant;
pub mod naming_symbols_vo;
pub mod plugin_group_vo;
pub mod plugin_manager_error;
pub mod project_language_vo;
pub mod project_summary_vo;
pub mod scan_completed_event;
pub mod scan_failed_event;
pub mod scan_started_event;
pub mod semantic_tracer_error;
pub mod source_analysis_vo;
pub mod source_content_vo;
pub mod source_parser_error;
pub mod source_path_vo;
pub mod source_paths_vo;
pub mod source_suffix_vo;
pub mod transport_client_error;
pub mod transport_protocol_vo;
pub mod watch_result_vo;
pub mod watch_service_error;

// ═══════════════════════════════════════════════════════════════════════════════
// PUBLIC RE-EXPORTS (Flat Access via Barrel)
// ═══════════════════════════════════════════════════════════════════════════════

// --- Adapter Domain ---
pub use adapter_collection_vo::{AdapterClassMap, AdapterMetadataList, AdapterNameList};
pub use adapter_name_vo::AdapterName;
pub use agent_status_vo::{AgentStatus, AgentStatusVO};

// --- Architecture Domain ---
pub use architecture_analysis_vo::{
    FileDefinitionMap, FilePathSet, GraphAnalysisContext, ImportGraph, InboundLinkMap,
    InheritanceMap, ModuleToFileMap, OrphanIndicatorResult, ReachabilityResult,
};
pub use architecture_config_vo::{default_aes_config, ArchitectureConfig};
pub use architecture_governance_entity::ArchitectureGovernanceEntity;
pub use architecture_rule_vo::{
    ArchitectureRule, CustomMessageVO, LegacyLayerRule, LegacyLayerRuleList, MandatoryImportRuleVO,
};

// --- Capability Domain ---
pub use capability_routing_vo::{
    CapabilityReference, CapabilityReferenceList, CapabilityRoutingContext, ClassDefinitionMap,
    ClassFileMap, ClassMethodsVO, ClassNameVO, ClassUsageItem, ClassUsageItemList, ClassUsageMap,
};

// --- Command Domain ---
pub use command_catalog_constant::COMMAND_CATALOG;
pub use command_metadata_vo::CommandMetadataVO;

// --- Common Collections ---
pub use common_collection_vo::{
    BooleanVO, ColumnNumber, Count, DataFlowList, IntoPatternListValues, JobIdList,
    LineContentList, LineNumber, PatternList, ResponseDataList, Score, Timestamp,
};
pub use common_duration_vo::{Duration, Timeout};
pub use common_error_vo::{
    Cause, Constraint, ErrorMessage, ExitCode, FieldName, ModuleName, PrimitiveTypeName,
};

// --- Config Domain ---
pub use config_app_vo::AppConfig;
pub use config_identifier_vo::ConfigKey;
pub use config_provider_error::ConfigError;
pub use config_setting_vo::{
    ActualValue, AdapterEntry, AdapterStatus, ExpectedValue, ProjectConfig, Thresholds,
};
pub use config_source_vo::{ConfigResult, ConfigSource};

// --- Doctor Domain ---
pub use doctor_result_vo::DoctorResultVO;

// --- Error Codes ---
pub use error_code_vo::ErrorCode;

// --- File System Errors ---
pub use file_system_error::{AccessDeniedError, FileSystemError, PathNotFoundError};

// --- Fix Domain ---
pub use fix_result_vo::FixResult;

// --- Git Domain ---
pub use git_diff_vo::GitDiffResultVO;
pub use git_hook_error::GitHookError;
pub use git_ref_vo::GitRef;

// --- Job Domain ---
pub use job_action_vo::{ActionArgs, ActionName, JobId};
pub use job_registry_error::JobError;

// --- Layer Domain ---
pub use layer_content_vo::{FileContentVO, Identity, LayerNameVO, LineContentVO};
pub use layer_definition_vo::{LayerDefinition, LayerMapVO, NamingConfig};
pub use layer_names_constant::{
    LAYER_AGENT, LAYER_CAPABILITIES, LAYER_CONTRACT, LAYER_GLOBAL, LAYER_INFRASTRUCTURE,
    LAYER_ROOT, LAYER_SURFACES, LAYER_TAXONOMY,
};
pub use layer_names_vo::{
    all_core_layers, core_layer_names, layer_agent, layer_capabilities, layer_contract,
    layer_global, layer_infrastructure, layer_root, layer_surfaces, layer_taxonomy,
};

// --- Lint Domain ---
pub use adapter_registered_event::AdapterRegistered;
pub use fix_applied_event::FixApplied;
pub use hook_installed_event::HookInstalled;
pub use hook_removed_event::HookRemoved;
pub use lint_adapter_error::{AdapterError, ScanError, ValidationError};
pub use lint_domain_vo::{
    CommandArgs, Location, LocationList, ScopeBounds, ScopeRef, ViolationConstraint,
};
pub use lint_operation_error::LinterOperationError;
pub use lint_position_vo::Position;
pub use lint_result_vo::{LintResult, LintResultList};
pub use lint_score_constant::{FORMAT_JSON, FORMAT_JUNIT, FORMAT_SARIF, FORMAT_TEXT};
pub use lint_score_vo::FileFormat;
pub use lint_severity_vo::Severity;
pub use lint_status_vo::{
    AdapterMetadata, EnvContentVO, JobStatus, LintStatusActionArgs, McpConfigVO, ResponseData,
    SuccessStatus,
};
pub use scan_completed_event::ScanCompleted;
pub use scan_failed_event::ScanFailed;
pub use scan_started_event::ScanStarted;

// --- Log/Suggestion Domain ---
pub use log_suggestion_vo::{
    ClassPath, DescriptionVO, LogOutput, MetadataVO, StdError, StdOutput, Suggestion,
};

// --- Maintenance Domain ---
pub use maintenance_stats_vo::MaintenanceStatsVO;

// --- MCP Domain ---
pub use mcp_server_constant::{
    AUTO_LINT_VERSION, MAX_BATCH_SIZE, MAX_PATH_DEPTH, MAX_PATH_LENGTH, MAX_STRING_LENGTH,
    MCP_SERVER_VERSION,
};

// --- Message Domain ---
pub use message_status_vo::{ComplianceStatus, LintMessage};

// --- Provider Errors ---
pub use metrics_provider_error::MetricsError;
pub use naming_provider_error::NamingError;

// --- Naming Domain ---
pub use naming_symbol_vo::{NameVariants, SymbolName};
pub use naming_symbols_constant::CORE_PRIMITIVE_TYPES;
pub use naming_symbols_vo::{
    primitive_type_list, CallChainList, ImportNameList, PrimitiveTypeList, SymbolNameList,
};

// --- Plugin Domain ---
pub use plugin_group_vo::PluginGroup;
pub use plugin_manager_error::{DiscoveryError, PluginError, RegistrationError};

// --- Project Domain ---
pub use project_language_vo::{LanguageSource, ProjectLanguage};
pub use project_summary_vo::{AggregatedResults, ProjectResult};

// --- Semantic Domain ---
pub use semantic_tracer_error::{CallChainError, ScopeResolutionError, SemanticError};

// --- Source Domain ---
pub use source_analysis_vo::{
    ImportInfo, ImportInfoList, PrimitiveViolation, PrimitiveViolationList,
};
pub use source_content_vo::ContentString;
pub use source_parser_error::{SourceParserError, SyntaxErrorVO};
pub use source_path_vo::{DirectoryPath, FilePath};
pub use source_paths_vo::{FilePathList, RenamedFile, RenamedFileList};
pub use source_suffix_vo::{SuffixPolicyVO, SuffixVO};

// --- Transport Domain ---
pub use transport_client_error::TransportError;
pub use transport_protocol_vo::{TransportEndpoint, TransportProtocol, TransportUrlVO};

// --- Watch Domain ---
pub use watch_result_vo::WatchResult;
pub use watch_service_error::{WatchEventError, WatchServiceError, WatchSubscriptionError};

// ═══════════════════════════════════════════════════════════════════════════════
// TYPE ALIASES (Ergonomic Shortcuts for Complex Types)
// ═══════════════════════════════════════════════════════════════════════════════

/// Map of layer names to their definitions — the core of architecture configuration.
pub type LayerDefinitionMap = std::collections::HashMap<LayerNameVO, LayerDefinition>;

/// Map of file paths to their import targets.
pub type ImportGraphMap = std::collections::HashMap<String, Vec<String>>;

/// Map of file paths to their inbound link sources.
pub type InboundLinkGraphMap = std::collections::HashMap<String, Vec<String>>;

/// Map of class names to their file locations.
pub type ClassFileGraphMap = std::collections::HashMap<String, FilePath>;

/// Map of class names to their method lists.
pub type ClassMethodGraphMap = std::collections::HashMap<String, ClassMethodsVO>;

/// Map of adapter names to their class paths.
pub type AdapterClassGraphMap = std::collections::HashMap<String, String>;

/// Map of config keys to their JSON values.
pub type ConfigValueMap = std::collections::HashMap<String, serde_json::Value>;

/// Map of metadata keys to their JSON values.
pub type MetadataMap = std::collections::HashMap<String, serde_json::Value>;

// ═══════════════════════════════════════════════════════════════════════════════
// FACTORY FUNCTIONS (Ergonomic VO Construction)
// ═══════════════════════════════════════════════════════════════════════════════

/// Create a `LineNumber` from an integer.
#[inline]
pub fn line_number(value: i64) -> LineNumber {
    LineNumber::new(value)
}

/// Create a `ColumnNumber` from an integer.
#[inline]
pub fn column_number(value: i64) -> ColumnNumber {
    ColumnNumber::new(value)
}

/// Create a `Score` from a floating-point value.
#[inline]
pub fn score(value: f64) -> Score {
    Score::new(value)
}

/// Create a `Count` from an integer.
#[inline]
pub fn count(value: i64) -> Count {
    Count::new(value)
}

/// Create a `BooleanVO` from a boolean.
#[inline]
pub fn boolean(value: bool) -> BooleanVO {
    BooleanVO::new(value)
}

/// Create an `ErrorMessage` from a string-like value.
#[inline]
pub fn error_message(value: impl Into<String>) -> ErrorMessage {
    ErrorMessage::new(value)
}

/// Create a `DescriptionVO` from a string-like value.
#[inline]
pub fn description(value: impl Into<String>) -> DescriptionVO {
    DescriptionVO::new(value)
}

/// Create a `Suggestion` from a string-like value.
#[inline]
pub fn suggestion(value: impl Into<String>) -> Suggestion {
    Suggestion::new(value)
}

/// Create a `FilePath` from a string-like value.
///
/// # Errors
/// Returns an error if the path is empty or only whitespace.
#[inline]
pub fn file_path(value: impl Into<String>) -> Result<FilePath, String> {
    FilePath::new(value)
}

/// Create a `DirectoryPath` from a string-like value.
///
/// # Errors
/// Returns an error if the path is empty or only whitespace.
#[inline]
pub fn directory_path(value: impl Into<String>) -> Result<DirectoryPath, String> {
    DirectoryPath::new(value)
}

/// Create a `Timestamp` representing the current UTC time.
#[inline]
pub fn timestamp_now() -> Timestamp {
    Timestamp::now()
}

/// Create a `Timestamp` from a string-like value.
#[inline]
pub fn timestamp(value: impl Into<String>) -> Timestamp {
    Timestamp::new(value)
}

/// Create a `Duration` from a floating-point value (milliseconds).
#[inline]
pub fn duration_ms(value: f64) -> Duration {
    Duration::new(value)
}

/// Create a `Timeout` from a floating-point value (seconds).
#[inline]
pub fn timeout_secs(value: f64) -> Timeout {
    Timeout::new(value)
}

/// Create an `ErrorCode` from a string-like value.
///
/// # Errors
/// Returns an error if the code is empty.
#[inline]
pub fn error_code(value: impl Into<String>) -> Result<ErrorCode, String> {
    ErrorCode::new(value)
}

/// Create an `ErrorCode` without validation (for compile-time safe inputs).
#[inline]
pub fn error_code_raw(value: impl Into<String>) -> ErrorCode {
    ErrorCode::raw(value)
}

/// Create an `AdapterName` from a string-like value.
///
/// # Errors
/// Returns an error if the name is empty or only whitespace.
#[inline]
pub fn adapter_name(value: impl Into<String>) -> Result<AdapterName, String> {
    AdapterName::new(value)
}

/// Create an `AdapterName` without validation (for compile-time safe inputs).
#[inline]
pub fn adapter_name_raw(value: impl Into<String>) -> AdapterName {
    AdapterName::raw(value)
}

/// Create a `LayerNameVO` from a string-like value.
#[inline]
pub fn layer_name(value: impl Into<String>) -> LayerNameVO {
    LayerNameVO::new(value)
}

/// Create a `LintMessage` from a string-like value.
#[inline]
pub fn lint_message(value: impl Into<String>) -> LintMessage {
    LintMessage::new(value)
}

/// Create a `PatternList` from various input types.
#[inline]
pub fn patterns(value: impl IntoPatternListValues) -> PatternList {
    PatternList::new(value)
}

/// Create a `FilePathList` from a vector of `FilePath`.
#[inline]
pub fn file_path_list(paths: Vec<FilePath>) -> FilePathList {
    FilePathList::new(paths)
}

/// Create a `ComplianceStatus` from a boolean.
#[inline]
pub fn compliance(passing: bool) -> ComplianceStatus {
    ComplianceStatus::new(passing)
}

/// Create a `ConfigKey` from a string-like value.
#[inline]
pub fn config_key(value: impl Into<String>) -> ConfigKey {
    ConfigKey::new(value)
}

/// Create a `GitRef` from a string-like value.
#[inline]
pub fn git_ref(value: impl Into<String>) -> GitRef {
    GitRef::new(value)
}

/// Create a `SymbolName` from a string-like value.
#[inline]
pub fn symbol_name(value: impl Into<String>) -> SymbolName {
    SymbolName::new(value)
}

/// Create a `SuffixVO` from a string-like value.
#[inline]
pub fn suffix(value: impl Into<String>) -> SuffixVO {
    SuffixVO::new(value)
}

/// Create a `Position` from line and column numbers.
#[inline]
pub fn position(line: i64, column: i64) -> Position {
    Position {
        line: LineNumber::new(line),
        column: ColumnNumber::new(column),
    }
}

/// Create a `Location` with file, line, and column information.
#[inline]
pub fn location(file: impl Into<String>, line: i64, column: i64) -> Location {
    Location {
        file: FilePath::new(file).ok(),
        line: Some(LineNumber::new(line)),
        column: Some(ColumnNumber::new(column)),
        description: DescriptionVO::new(String::new()),
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// UTILITY FUNCTIONS (Common Operations on Taxonomy Types)
// ═══════════════════════════════════════════════════════════════════════════════

/// Compute the overall compliance score from a list of lint results.
///
/// Starts at 100.0 and deducts points based on severity:
/// - INFO: 0 points
/// - LOW: 1 point
/// - MEDIUM: 2 points
/// - HIGH: 3 points
/// - CRITICAL: 5 points
pub fn compute_score(results: &[LintResult]) -> f64 {
    let penalty: f64 = results.iter().map(|r| r.severity.score_impact()).sum();
    (100.0_f64 - penalty).max(0.0)
}

/// Check if any result in the list has CRITICAL severity.
pub fn has_critical(results: &[LintResult]) -> bool {
    results.iter().any(|r| r.severity == Severity::CRITICAL)
}

/// Check if the results are passing (no CRITICAL and score >= threshold).
pub fn is_passing(results: &[LintResult], threshold: f64) -> bool {
    !has_critical(results) && compute_score(results) >= threshold
}

/// Count violations by severity level.
pub fn count_by_severity(results: &[LintResult]) -> std::collections::HashMap<Severity, usize> {
    let mut counts = std::collections::HashMap::new();
    for r in results {
        *counts.entry(r.severity.clone()).or_insert(0) += 1;
    }
    counts
}

/// Group lint results by file path.
pub fn group_by_file(
    results: &[LintResult],
) -> std::collections::HashMap<String, Vec<&LintResult>> {
    let mut groups: std::collections::HashMap<String, Vec<&LintResult>> =
        std::collections::HashMap::new();
    for r in results {
        groups
            .entry(r.file.value.clone())
            .or_insert_with(Vec::new)
            .push(r);
    }
    groups
}

/// Get the worst (highest) severity from a list of results.
pub fn worst_severity(results: &[LintResult]) -> Severity {
    results
        .iter()
        .map(|r| &r.severity)
        .max_by_key(|s| match s {
            Severity::INFO => 0,
            Severity::LOW => 1,
            Severity::MEDIUM => 2,
            Severity::HIGH => 3,
            Severity::CRITICAL => 4,
        })
        .cloned()
        .unwrap_or(Severity::INFO)
}

// ═══════════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::{
        boolean, column_number, compute_score, count, count_by_severity, error_message, file_path,
        has_critical, is_passing, line_number, position, score, worst_severity, LintResult,
        Severity,
    };

    #[test]
    fn test_factory_line_number() {
        let ln = line_number(42);
        assert_eq!(ln.value, 42);
    }

    #[test]
    fn test_factory_column_number() {
        let cn = column_number(10);
        assert_eq!(cn.value, 10);
    }

    #[test]
    fn test_factory_score() {
        let s = score(95.5);
        assert_eq!(s.value, 95.5);
    }

    #[test]
    fn test_factory_count() {
        let c = count(7);
        assert_eq!(c.value, 7);
    }

    #[test]
    fn test_factory_boolean() {
        let b = boolean(true);
        assert!(b.value);
    }

    #[test]
    fn test_factory_error_message() {
        let em = error_message("test error");
        assert_eq!(em.value, "test error");
    }

    #[test]
    fn test_factory_file_path() {
        let fp = file_path("src/main.rs").unwrap_or_default();
        assert_eq!(fp.value, "src/main.rs");
    }

    #[test]
    fn test_factory_file_path_invalid() {
        assert!(file_path("").is_err());
    }

    #[test]
    fn test_factory_position() {
        let pos = position(10, 5);
        assert_eq!(pos.line.value, 10);
        assert_eq!(pos.column.value, 5);
    }

    #[test]
    fn test_compute_score_empty() {
        let results: Vec<LintResult> = vec![];
        assert_eq!(compute_score(&results), 100.0);
    }

    #[test]
    fn test_compute_score_with_violations() {
        let results = vec![
            LintResult {
                severity: Severity::HIGH,
                ..Default::default()
            },
            LintResult {
                severity: Severity::MEDIUM,
                ..Default::default()
            },
        ];
        assert_eq!(compute_score(&results), 95.0);
    }

    #[test]
    fn test_has_critical() {
        let results = vec![LintResult {
            severity: Severity::CRITICAL,
            ..Default::default()
        }];
        assert!(has_critical(&results));
    }

    #[test]
    fn test_has_critical_false() {
        let results = vec![LintResult {
            severity: Severity::HIGH,
            ..Default::default()
        }];
        assert!(!has_critical(&results));
    }

    #[test]
    fn test_is_passing() {
        let results: Vec<LintResult> = vec![];
        assert!(is_passing(&results, 80.0));
    }

    #[test]
    fn test_worst_severity() {
        let results = vec![
            LintResult {
                severity: Severity::LOW,
                ..Default::default()
            },
            LintResult {
                severity: Severity::HIGH,
                ..Default::default()
            },
            LintResult {
                severity: Severity::MEDIUM,
                ..Default::default()
            },
        ];
        assert_eq!(worst_severity(&results), Severity::HIGH);
    }

    #[test]
    fn test_group_by_severity() {
        let results = vec![
            LintResult {
                severity: Severity::HIGH,
                ..Default::default()
            },
            LintResult {
                severity: Severity::HIGH,
                ..Default::default()
            },
            LintResult {
                severity: Severity::LOW,
                ..Default::default()
            },
        ];
        let counts = count_by_severity(&results);
        assert_eq!(counts[&Severity::HIGH], 2);
        assert_eq!(counts[&Severity::LOW], 1);
    }
}
