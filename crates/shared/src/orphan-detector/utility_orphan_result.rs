// PURPOSE: taxonomy_orphan_result_utility — pure data construction for orphan LintResult
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::cli_commands::taxonomy_severity_vo::Severity;
use crate::common::taxonomy_lint_vo::LocationList;
use crate::common::taxonomy_path_vo::FilePath;
use crate::taxonomy_adapter_name_vo::AdapterName;
use crate::taxonomy_common_vo::ColumnNumber;
use crate::taxonomy_common_vo::LineNumber;
use crate::taxonomy_error_vo::ErrorCode;
use crate::taxonomy_message_vo::LintMessage;

pub fn mk_orphan_result(file: &str, msg: &str, sev: Severity, code: &str) -> LintResult {
    LintResult {
        file: FilePath {
            value: file.to_string(),
        },
        line: LineNumber::new(0),
        column: ColumnNumber::new(0),
        code: ErrorCode::raw(code),
        message: LintMessage::new(msg),
        source: Some(AdapterName::raw("architecture")),
        severity: sev,
        enclosing_scope: None,
        related_locations: LocationList::new(),
    }
}
