// Barrel exports for the Taxonomy layer (AES Architecture)
// All public domain types are re-exported here for clean cross-layer imports.

pub mod adapter_collection_vo;
pub mod adapter_name_vo;
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
pub mod doctor_result_vo;
pub mod error_code_vo;
pub mod file_system_error;
pub mod fix_result_vo;
pub mod git_diff_vo;
pub mod git_hook_error;
pub mod git_ref_vo;
pub mod job_action_vo;
pub mod job_registry_error;
pub mod layer_content_vo;
pub mod layer_definition_vo;
pub mod layer_names_vo;
pub mod lint_adapter_error;
pub mod lint_code_vo;
pub mod lint_domain_vo;
pub mod lint_fix_vo;
pub mod lint_operation_error;
pub mod lint_position_vo;
pub mod lint_result_vo;
pub mod lint_scan_event;
pub mod lint_score_vo;
pub mod lint_severity_vo;
pub mod lint_status_vo;
pub mod log_suggestion_vo;
pub mod maintenance_doctor_vo;
pub mod maintenance_stats_vo;
pub mod message_status_vo;
pub mod metrics_provider_error;
pub mod naming_provider_error;
pub mod naming_symbol_vo;
pub mod naming_symbols_vo;
pub mod plugin_group_vo;
pub mod plugin_manager_error;
pub mod project_summary_vo;
pub mod semantic_tracer_error;
pub mod source_analysis_vo;
pub mod source_content_vo;
pub mod source_parser_error;
pub mod source_path_vo;
pub mod source_paths_vo;
pub mod source_suffix_vo;
pub mod source_system_error;
pub mod transport_client_error;
pub mod transport_protocol_vo;
pub mod watch_result_vo;
pub mod watch_service_error;

// ---- Re-exports (canonical sources) ----
// Modules without conflicts use wildcard re-export
pub use adapter_collection_vo::*;
pub use adapter_name_vo::*;
pub use agent_status_vo::*;
pub use architecture_analysis_vo::*;
pub use architecture_config_vo::*;
pub use architecture_governance_entity::*;
pub use architecture_rule_vo::*;
pub use capability_routing_vo::*;
pub use command_catalog_constant::*;
pub use command_metadata_vo::*;
pub use common_duration_vo::*;
pub use common_error_vo::*;
pub use config_app_vo::*;
pub use config_identifier_vo::*;
pub use config_provider_error::*;
pub use config_setting_vo::*;
pub use doctor_result_vo::*;
pub use error_code_vo::*;
pub use file_system_error::*;
pub use fix_result_vo::*;
pub use git_diff_vo::*;
pub use git_hook_error::*;
pub use git_ref_vo::*;
pub use job_action_vo::*;
pub use job_registry_error::*;
pub use layer_content_vo::*;
pub use layer_definition_vo::*;
pub use layer_names_vo::*;
pub use lint_adapter_error::*;
pub use lint_domain_vo::*;
pub use lint_operation_error::*;
pub use lint_result_vo::*;
pub use lint_scan_event::*;
pub use lint_score_vo::*;
pub use lint_severity_vo::*;
pub use log_suggestion_vo::*;
pub use maintenance_stats_vo::*;
pub use message_status_vo::*;
pub use metrics_provider_error::*;
pub use naming_provider_error::*;
pub use naming_symbol_vo::*;
pub use naming_symbols_vo::*;
pub use plugin_group_vo::*;
pub use plugin_manager_error::*;
pub use project_summary_vo::*;
pub use semantic_tracer_error::*;
pub use source_analysis_vo::*;
pub use source_content_vo::*;
pub use source_parser_error::*;
pub use source_path_vo::*;
pub use source_paths_vo::*;
pub use source_suffix_vo::*;
pub use transport_client_error::*;
pub use transport_protocol_vo::*;
pub use watch_result_vo::*;
pub use watch_service_error::*;

// ---- Constants and defaults ----
pub const MAX_STRING_LENGTH: usize = 1_000_000;

/// Default AES configuration loader (stub).
pub fn default_aes_config() -> ArchitectureConfig {
    ArchitectureConfig::default()
}

// ---- Modules with conflicts: explicit re-exports only ----
// common_collection_vo: Timestamp excluded (canonical in common_duration_vo)
pub use common_collection_vo::{
    BooleanVO, ColumnNumber, Count, DataFlowList, IntoPatternListValues, JobIdList,
    LineContentList, LineNumber, PatternList, ResponseDataList, Score,
};

// lint_code_vo: ErrorCode excluded (canonical in error_code_vo)
// lint_code_vo has no other public types

// lint_fix_vo: FixResult excluded (canonical in fix_result_vo)
// lint_fix_vo has no other public types

// lint_position_vo: LineNumber/ColumnNumber excluded (canonical in common_collection_vo)
pub use lint_position_vo::Position;

// lint_status_vo: TransportUrlVO excluded (canonical in transport_protocol_vo)
pub use lint_status_vo::{
    AdapterMetadata, EnvContentVO, JobStatus, LintStatusActionArgs, McpConfigVO, ResponseData,
    SuccessStatus,
};

// maintenance_doctor_vo: DoctorResultVO excluded (canonical in doctor_result_vo)
// maintenance_doctor_vo has no other public types

// source_system_error: FileSystemError/PathNotFoundError/AccessDeniedError excluded
//   (canonical in file_system_error)
// source_system_error has no other public types
