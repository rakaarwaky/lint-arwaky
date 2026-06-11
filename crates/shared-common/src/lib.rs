// PURPOSE: shared-common — all taxonomy types, contract traits, and shared definitions
// No dependencies on other feature crates — this is the foundation layer.

// === Taxonomy: Value Objects ===
pub mod taxonomy_name_vo;
pub use taxonomy_name_vo::{NameVariants, SymbolName};

pub mod taxonomy_common_vo;
pub use taxonomy_common_vo::{
    BooleanVO, ColumnNumber, Count, DataFlowList, ErrorMessage, IntoPatternListValues, JobIdList,
    LineContentList, LineNumber, PatternList, ResponseDataList, Score, Timestamp,
};

pub mod taxonomy_duration_vo;
pub use taxonomy_duration_vo::{Duration, Timeout};

pub mod taxonomy_error_vo;
pub use taxonomy_error_vo::ErrorCode;

pub mod taxonomy_layer_vo;
pub use taxonomy_layer_vo::{FileContentVO, Identity, LayerNameVO, LineContentVO};

pub mod taxonomy_lint_vo;
pub use taxonomy_lint_vo::{
    CommandArgs, Location, LocationList, ScopeBounds, ScopeRef, ViolationConstraint,
};

pub mod taxonomy_message_vo;
pub use taxonomy_message_vo::{ComplianceStatus, LintMessage};

pub mod taxonomy_adapter_name_vo;
pub use taxonomy_adapter_name_vo::AdapterName;

pub mod taxonomy_source_vo;
pub use taxonomy_source_vo::{ContentString, SourceContentVO};

pub mod taxonomy_suggestion_vo;
pub use taxonomy_suggestion_vo::{
    ClassPath, DescriptionVO, LogOutput, MetadataVO, StdError, StdOutput, Suggestion,
};

pub mod taxonomy_path_vo;
pub use taxonomy_path_vo::{DirectoryPath, FilePath};

pub mod taxonomy_paths_vo;

pub mod taxonomy_definition_vo;
pub use taxonomy_definition_vo::{LayerDefinition, LayerMapVO, NamingConfig};

pub mod taxonomy_suffix_vo;
pub use taxonomy_suffix_vo::{SuffixPolicyVO, SuffixVO};

pub mod taxonomy_rule_vo;
pub use taxonomy_rule_vo::{
    ArchitectureRule, CustomMessageVO, LegacyLayerRule, LegacyLayerRuleList, MandatoryImportRuleVO,
};

pub mod taxonomy_action_vo;
pub use taxonomy_action_vo::{ActionArgs, ActionName, JobId};

pub mod taxonomy_job_vo;
pub use taxonomy_job_vo::{
    AdapterMetadata, EnvContentVO, JobStatus, LintStatusActionArgs, McpConfigVO, ResponseData,
    SuccessStatus,
};

pub mod taxonomy_severity_vo;
pub use taxonomy_severity_vo::Severity;

pub mod taxonomy_result_vo;
pub use taxonomy_result_vo::{LintResult, LintResultList};

pub mod taxonomy_position_vo;

pub mod taxonomy_score_vo;
pub use taxonomy_score_vo::compute_score;

pub mod taxonomy_score_constant;

pub mod taxonomy_metadata_vo;

pub mod taxonomy_protocol_vo;

pub mod taxonomy_analysis_vo;

pub mod taxonomy_import_source_vo;

pub mod taxonomy_governance_entity;

pub mod taxonomy_operation_error;

pub mod taxonomy_adapter_vo;

pub mod taxonomy_adapter_error;

pub mod taxonomy_app_vo;

pub mod taxonomy_config_error;

pub mod taxonomy_config_vo;
pub use taxonomy_config_vo::{default_aes_config, default_config_for_language};

pub mod taxonomy_identifier_vo;

pub mod taxonomy_setting_vo;

pub mod taxonomy_validation_vo;

pub mod taxonomy_filesystem_error;

pub mod taxonomy_fix_applied_event;
pub use taxonomy_fix_applied_event::FixApplied;

pub mod taxonomy_fix_vo;
pub use taxonomy_fix_vo::FixResult;

pub mod taxonomy_symbol_renamer_utility;
pub use taxonomy_symbol_renamer_utility::SymbolRenamer;

pub mod taxonomy_diff_result_vo;

pub mod taxonomy_hook_error;

pub mod taxonomy_installed_event;

pub mod taxonomy_ref_vo;

pub mod taxonomy_removed_event;

pub mod taxonomy_naming_error;

pub mod taxonomy_naming_list_vo;

pub mod taxonomy_semantic_error;

pub mod taxonomy_layer_names_vo;
pub use taxonomy_layer_names_vo::*;

pub mod taxonomy_layer_names_constant;
pub use taxonomy_layer_names_constant::*;

pub mod taxonomy_catalog_constant;

pub mod taxonomy_command_catalog_vo;

pub mod taxonomy_group_vo;

pub mod taxonomy_manager_error;

pub mod taxonomy_doctor_vo;

pub mod taxonomy_language_vo;

pub mod taxonomy_stats_vo;

pub mod taxonomy_agent_status_vo;

pub mod taxonomy_multi_project_vo;

pub mod taxonomy_summary_vo;

pub mod taxonomy_metrics_error;

pub mod taxonomy_service_error;

pub mod taxonomy_watch_vo;

pub mod taxonomy_server_constant;

pub mod taxonomy_transport_error;

pub mod taxonomy_common_error;
pub use taxonomy_common_error::{
    Cause, Constraint, ExitCode, FieldName, ModuleName, PrimitiveTypeName,
};

pub mod taxonomy_violation_message_rs_error;
pub use taxonomy_violation_message_rs_error::*;
pub mod taxonomy_violation_message_js_error;
pub mod taxonomy_violation_message_py_error;


