// PURPOSE: Shared helpers for naming checkers — layer detection, exception matching, result construction.
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_adapter_name_vo::AdapterName;
use crate::common::taxonomy_common_vo::{ColumnNumber, LineNumber};
use crate::common::taxonomy_error_vo::ErrorCode;
use crate::common::taxonomy_lint_vo::LocationList;
use crate::common::taxonomy_message_vo::LintMessage;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_severity_vo::Severity;
use crate::common::utility_layer_detector;
use crate::naming_rules::taxonomy_naming_constant::ADAPTER_NAME;
use crate::taxonomy_definition_vo::{LayerDefinition, LayerMapVO};

pub fn layer_keys(layer_map: &LayerMapVO) -> Vec<String> {
    layer_map.values.keys().map(|k| k.to_string()).collect()
}

pub fn detect_layer(file: &str, layer_keys: &[String]) -> Option<String> {
    let filename = utility_layer_detector::extract_filename(file);
    utility_layer_detector::detect_layer_from_prefix(filename)
        .map(|base| utility_layer_detector::resolve_specialized_layer(&base, file, layer_keys))
}

pub fn is_exception(definition: &LayerDefinition, file: &FilePath) -> bool {
    let basename = file.basename();
    definition
        .exceptions
        .values
        .iter()
        .any(|pattern| pattern == &file.value || pattern == &basename)
}

pub fn file_level_result(
    file: &FilePath,
    code: &str,
    message: impl Into<String>,
    severity: Severity,
) -> LintResult {
    LintResult {
        file: file.clone(),
        line: LineNumber::new(1),
        column: ColumnNumber::new(0),
        code: ErrorCode::raw(code),
        message: LintMessage::new(message),
        source: Some(AdapterName::raw(ADAPTER_NAME)),
        severity,
        enclosing_scope: None,
        related_locations: LocationList::new(),
    }
}

/// Construct a file-level LintResult from a string filename.
///
/// This is the shared replacement for duplicated `_make_result` helpers in
/// `NamingConventionChecker` and `SuffixPrefixChecker`.
pub fn string_filename_result(
    file: &str,
    code: &str,
    message: impl Into<String>,
    severity: Severity,
) -> LintResult {
    let file_path = FilePath::new(file).unwrap_or_default();
    LintResult {
        file: file_path,
        line: LineNumber::new(1), // File-level check — not line-specific
        column: ColumnNumber::new(0),
        code: ErrorCode::raw(code),
        message: LintMessage::new(message),
        source: Some(AdapterName::raw(ADAPTER_NAME)),
        severity,
        enclosing_scope: None,
        related_locations: LocationList::new(),
    }
}
