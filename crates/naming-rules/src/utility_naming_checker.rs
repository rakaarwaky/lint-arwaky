// PURPOSE: Shared helpers for naming checkers — stem/suffix extraction, result construction.
use shared::common::taxonomy_adapter_name_vo::AdapterName;
use shared::common::taxonomy_common_vo::ColumnNumber;
use shared::common::taxonomy_common_vo::LineNumber;
use shared::common::taxonomy_error_vo::ErrorCode;
use shared::common::taxonomy_lint_result_vo::LintResult;
use shared::common::taxonomy_lint_vo::LocationList;
use shared::common::taxonomy_message_vo::LintMessage;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_severity_vo::Severity;
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

/// Construct a file-level LintResult from a string filename.
pub fn string_filename_result(
    file: &str,
    code: &str,
    message: impl Into<String>,
    severity: Severity,
) -> LintResult {
    let file_path = FilePath::new(file).unwrap_or_default();
    LintResult {
        file: file_path,
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
