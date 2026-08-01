// PURPOSE: shared — all taxonomy types, contract traits, and shared definitions
// No dependencies on other feature crates — this is the foundation layer.

#[path = "common/mod.rs"]
pub mod common;

// Re-export all taxonomy_* and contract_* types from common
// NOTE: widely used by downstream crates as shared::taxonomy_*. Do not remove.

// ── Module re-exports ──
pub use common::contract_executor_protocol;
pub use common::taxonomy_action_vo;
pub use common::taxonomy_adapter_error;
pub use common::taxonomy_adapter_list_vo;
pub use common::taxonomy_adapter_name_vo;
pub use common::taxonomy_common_error;
pub use common::taxonomy_common_vo;
pub use common::taxonomy_definition_vo;
pub use common::taxonomy_display_content_vo;
pub use common::taxonomy_duration_vo;
pub use common::taxonomy_error_vo;
pub use common::taxonomy_filesystem_error;
pub use common::taxonomy_format_vo;
pub use common::taxonomy_git_vo;
pub use common::taxonomy_job_id_vo;
pub use common::taxonomy_job_vo;
pub use common::taxonomy_language_info_vo;
pub use common::taxonomy_language_vo;
pub use common::taxonomy_layer_vo;
pub use common::taxonomy_lint_result_vo;
pub use common::taxonomy_lint_vo;
pub use common::taxonomy_message_vo;
pub use common::taxonomy_name_vo;
pub use common::taxonomy_path_vo;
pub use common::taxonomy_paths_vo;
pub use common::taxonomy_response_data_vo;
pub use common::taxonomy_severity_vo;
pub use common::taxonomy_source_vo;
pub use common::taxonomy_suggestion_vo;
pub use common::taxonomy_threshold_vo;
pub use common::utility_command_runner;
pub use common::utility_compliance_score;
pub use common::utility_language_detector;
pub use common::utility_layer_detector;
pub use common::utility_path_normalization;
pub use common::utility_scope_matcher;
pub use common::utility_signature_parser;
pub use common::utility_value_object_generator;

// ── Type re-exports ──
// Contract traits
pub use common::ICommandExecutorProtocol;

// Taxonomy types
pub use common::taxonomy_action_vo::{ActionName, JobId};
pub use common::taxonomy_adapter_error::{AdapterError, ScanError, ValidationError};
pub use common::taxonomy_adapter_list_vo::AdapterNameList;
pub use common::taxonomy_adapter_name_vo::AdapterName;
pub use common::taxonomy_common_error::{
    Cause, Constraint, ExitCode, FieldName, ModuleName, PrimitiveTypeName,
};
pub use common::taxonomy_common_vo::{
    BooleanVO, ColumnNumber, Count, DataFlowList, ErrorMessage, IntoPatternListValues, JobIdList,
    LanguageVO, LineContentList, LineNumber, PatternList, ResponseDataList, Score, SuffixPolicyVO,
    SuffixVO, Timestamp,
};
pub use common::taxonomy_definition_vo::{LayerDefinition, LayerMapVO, NamingConfig};
pub use common::taxonomy_display_content_vo::DisplayContent;
pub use common::taxonomy_duration_vo::Timeout;
pub use common::taxonomy_error_vo::ErrorCode;
pub use common::taxonomy_filesystem_error::FileSystemError;
pub use common::taxonomy_git_vo::GitBranchName;
pub use common::taxonomy_job_vo::{AdapterMetadata, EnvContentVO, McpConfigVO, SuccessStatus};
pub use common::taxonomy_language_info_vo::LanguageInfo;
pub use common::taxonomy_language_vo::Language;
pub use common::taxonomy_layer_vo::{FileContentVO, Identity, LayerNameVO, LineContentVO};
pub use common::taxonomy_lint_vo::{
    CommandArgs, Location, LocationList, ScopeBounds, ScopeRef, ViolationConstraint,
};
pub use common::taxonomy_message_vo::{ComplianceStatus, LintMessage};
pub use common::taxonomy_name_vo::{NameVariants, SymbolName};
pub use common::taxonomy_path_vo::{DirectoryPath, FilePath};
pub use common::taxonomy_paths_vo::{FilePathList, RenamedFile, RenamedFileList};
pub use common::taxonomy_response_data_vo::ResponseData;
pub use common::taxonomy_severity_vo::Severity;
pub use common::taxonomy_source_vo::{ContentString, SourceContentVO};
pub use common::taxonomy_suggestion_vo::{DescriptionVO, MetadataVO};
pub use common::taxonomy_threshold_vo::Threshold;

#[path = "tui/mod.rs"]
pub mod tui;

// Feature-specific types (in feature folders)
#[path = "auto-fix/mod.rs"]
pub mod auto_fix;
#[path = "cli-commands/mod.rs"]
pub mod cli_commands;
#[path = "code-analysis/mod.rs"]
pub mod code_analysis;
#[path = "config-system/mod.rs"]
pub mod config_system;
#[path = "external-lint/mod.rs"]
pub mod external_lint;
#[path = "file-watch/mod.rs"]
pub mod file_watch;
#[path = "git-hooks/mod.rs"]
pub mod git_hooks;
#[path = "import-rules/mod.rs"]
pub mod import_rules;
#[path = "mcp-server/mod.rs"]
pub mod mcp_server;

#[path = "naming-rules/mod.rs"]
pub mod naming_rules;
#[path = "orphan-detector/mod.rs"]
pub mod orphan_detector;
#[path = "project-setup/mod.rs"]
pub mod project_setup;
#[path = "role-rules/mod.rs"]
pub mod role_rules;

#[path = "report-formatter/mod.rs"]
pub mod report_formatter;

pub mod filesystem;
pub mod maintenance;
