// PURPOSE: Module declarations and re-exports for all shared-common VOs, errors, events, constants, entities
pub mod taxonomy_name_vo;
pub use taxonomy_name_vo::{NameVariants, SymbolName};


pub mod taxonomy_common_error;
pub use taxonomy_common_error::{
    Cause, Constraint, ExitCode, FieldName, ModuleName, PrimitiveTypeName,
};
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


pub mod taxonomy_violation_message_rs_error;
pub use taxonomy_violation_message_rs_error::*;
pub mod taxonomy_definition_vo;
pub use taxonomy_definition_vo::{LayerDefinition, LayerMapVO, NamingConfig};
pub mod taxonomy_violation_message_js_error;
pub mod taxonomy_violation_message_py_error;


pub mod taxonomy_suggestion_vo;
pub use taxonomy_suggestion_vo::{
    ClassPath, DescriptionVO, LogOutput, MetadataVO, StdError, StdOutput, Suggestion,
};

