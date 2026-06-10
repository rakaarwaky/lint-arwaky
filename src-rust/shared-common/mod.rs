// PURPOSE: Module: shared-common module declarations and re-exports
pub mod taxonomy_adapter_error;
pub use taxonomy_adapter_error::{AdapterError, ScanError, ValidationError};
pub mod taxonomy_adapter_vo;
pub use taxonomy_adapter_vo::{AdapterClassMap, AdapterMetadataList, AdapterNameList};
pub mod taxonomy_common_error;
pub use taxonomy_common_error::{
    Cause, Constraint, ErrorMessage, ExitCode, FieldName, ModuleName, PrimitiveTypeName,
};
pub mod taxonomy_common_vo;
pub use taxonomy_common_vo::{
    BooleanVO, ColumnNumber, Count, DataFlowList, IntoPatternListValues, JobIdList,
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
pub mod taxonomy_operation_error;
pub use taxonomy_operation_error::LinterOperationError;
pub mod taxonomy_adapter_registered_event;
pub use taxonomy_adapter_registered_event::AdapterRegistered;
pub mod taxonomy_role_vo;
pub mod taxonomy_source_vo;
pub use taxonomy_source_vo::{ContentString, SourceContentVO};
pub mod taxonomy_fix_applied_event;
pub use taxonomy_fix_applied_event::FixApplied;
pub mod taxonomy_scan_completed_event;
pub use taxonomy_scan_completed_event::ScanCompleted;
pub mod taxonomy_scan_failed_event;
pub use taxonomy_scan_failed_event::ScanFailed;
pub mod taxonomy_fix_vo;
pub use taxonomy_fix_vo::FixResult;
pub mod taxonomy_scan_started_event;
pub use taxonomy_scan_started_event::ScanStarted;
pub mod taxonomy_violation_rs_constant;
pub use taxonomy_violation_rs_constant::*;
pub mod taxonomy_definition_vo;
pub use taxonomy_definition_vo::{LayerDefinition, LayerMapVO, NamingConfig};
pub mod taxonomy_violation_js_constant;
pub mod taxonomy_violation_js_vo;
pub mod taxonomy_violation_py_constant;
pub mod taxonomy_violation_py_vo;
pub mod taxonomy_governance_entity;
pub use taxonomy_governance_entity::ArchitectureGovernanceEntity;
pub mod taxonomy_layer_names_constant;
pub use taxonomy_layer_names_constant::{
    LAYER_AGENT, LAYER_CAPABILITIES, LAYER_CONTRACT, LAYER_GLOBAL, LAYER_INFRASTRUCTURE,
    LAYER_ROOT, LAYER_SURFACES, LAYER_TAXONOMY,
};
pub mod taxonomy_layer_names_vo;
pub use taxonomy_layer_names_vo::LayerNames;
pub mod taxonomy_suffix_vo;
pub use taxonomy_suffix_vo::{SuffixPolicyVO, SuffixVO};
pub mod taxonomy_suggestion_vo;
pub use taxonomy_suggestion_vo::{
    ClassPath, DescriptionVO, LogOutput, MetadataVO, StdError, StdOutput, Suggestion,
};
pub mod taxonomy_rule_vo;
pub use taxonomy_rule_vo::{
    ArchitectureRule, CustomMessageVO, LegacyLayerRule, LegacyLayerRuleList, MandatoryImportRuleVO,
};
