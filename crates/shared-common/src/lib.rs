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

// === Contract: source-parsing ===
pub mod contract_parser_port;
pub use contract_parser_port::ISourceParserPort;

pub mod contract_path_normalization_port;
pub use contract_path_normalization_port::IPathNormalizationPort;

pub mod contract_scanner_provider_port;
pub use contract_scanner_provider_port::IScannerProviderPort;

// === Contract: code-analysis ===
pub mod contract_lint_protocol;
pub use contract_lint_protocol::IArchLintProtocol;

pub mod contract_analysis_protocol;

pub mod contract_adapter_port;

pub mod contract_bypass_checker_protocol;

pub mod contract_class_protocol;

pub mod contract_code_metric_analyzer_protocol;

pub mod contract_cycle_protocol;

pub mod contract_dead_inheritance_protocol;

pub mod contract_fix_runner_aggregate;

pub mod contract_inline_unused_protocol;

pub mod contract_layer_detection_aggregate;

pub mod contract_line_protocol;

pub mod contract_mandatory_inheritance_protocol;

pub mod contract_target_resolver_protocol;

pub mod contract_unused_protocol;

// === Contract: file-system ===
pub mod contract_system_port;

// === Contract: cli-transport ===
pub mod contract_executor_port;

// === Contract: import-rules ===
pub mod contract_import_parser_port;

pub mod contract_import_runner_aggregate;

pub mod contract_rule_protocol;

// === Contract: naming-rules ===
pub mod contract_naming_runner_aggregate;

// === Contract: config-system ===
pub mod contract_discovery_port;

pub mod contract_orchestration_aggregate;

pub mod contract_config_parser_port;

pub mod contract_reader_port;

pub mod contract_validator_protocol;

pub mod contract_detector_port;

// === Contract: file-watch ===
pub mod contract_provider_port;

// === Contract: git-hooks ===
pub mod contract_git_commands_aggregate;

pub mod contract_manager_port;

pub mod contract_hook_orchestrator_aggregate;

// === Contract: mcp-server ===
pub mod contract_server_port;

// === Contract: metrics-service ===
pub mod contract_metrics_port;

// === Contract: output-report ===
pub mod contract_output_aggregate;

pub mod contract_report_aggregate;

pub mod contract_client_aggregate;

// === Contract: pipeline-jobs ===
pub mod contract_registry_aggregate;

pub mod contract_registry_port;

pub mod contract_dispatcher_aggregate;

pub mod contract_extended_aggregate;

pub mod contract_pipeline_output_aggregate;

// === Contract: auto-fix ===
pub mod contract_fix_aggregate;

pub mod contract_fix_protocol;

// === Contract: plugin-system ===
pub mod contract_plugin_commands_aggregate;

pub mod contract_plugin_manager_port;

// === Contract: project-setup ===
pub mod contract_setup_aggregate;

pub mod contract_setup_protocol;

// === Contract: lifecycle-state ===
pub mod contract_lifecycle_aggregate;

// === Contract: multi-project ===
pub mod contract_multi_project_orchestrator;

// === Contract: role-rules ===
pub mod contract_role_aggregate;

pub mod contract_role_protocol;

pub mod contract_role_runner_aggregate;

pub mod contract_agent_role_protocol;

pub mod contract_capabilities_role_protocol;

pub mod contract_surface_role_protocol;

pub mod contract_taxonomy_role_protocol;

// === Contract: di-containers ===
pub mod contract_service_aggregate;

// === Contract: orphan-detector ===
pub mod contract_orphan_aggregate;

pub mod contract_orphan_protocol;
