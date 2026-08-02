// PURPOSE: Shared helpers for naming checkers — layer detection, exception matching, result construction.
use shared::common::taxonomy_adapter_name_vo::AdapterName;
use shared::common::taxonomy_common_vo::ColumnNumber;
use shared::common::taxonomy_common_vo::LineNumber;
use shared::common::taxonomy_definition_vo::{LayerDefinition, LayerMapVO};
use shared::common::taxonomy_error_vo::ErrorCode;
use shared::common::taxonomy_lint_result_vo::LintResult;
use shared::common::taxonomy_lint_vo::LocationList;
use shared::common::taxonomy_message_vo::LintMessage;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_severity_vo::Severity;
use shared::common::utility_layer_detector;
use shared::naming_rules::taxonomy_naming_constant::ADAPTER_NAME;

/// Extract the file stem using the last dot (rfind), consistent across all checkers.
///
/// For multi-dot filenames like `foo.spec.rs`, this returns `foo.spec`.
/// For single-dot files like `checker.rs`, this returns `checker`.
/// For dotfiles like `.gitignore`, the entire filename is returned.
/// If there is no dot, the entire filename is returned.
pub fn get_stem(filename: &str) -> Option<&str> {
    std::path::Path::new(filename)
        .file_stem()
        .and_then(|n| n.to_str())
}

/// Extract the suffix (word after the last underscore) from a stem.
pub fn get_suffix(stem: &str) -> Option<&str> {
    stem.rfind('_').map(|pos| &stem[pos + 1..])
}

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
