//! Shared helper utilities for inline checker methods.

use crate::output_report::taxonomy_result_vo::LintResult;
use crate::output_report::taxonomy_severity_vo::Severity;
use crate::shared_common::taxonomy_adapter_name_vo::AdapterName;
use crate::shared_common::taxonomy_common_vo::ColumnNumber;
use crate::shared_common::taxonomy_common_vo::LineNumber;
use crate::shared_common::taxonomy_error_vo::ErrorCode;
use crate::shared_common::taxonomy_lint_vo::LocationList;
use crate::shared_common::taxonomy_message_vo::LintMessage;
use crate::source_parsing::taxonomy_path_vo::FilePath;

/// Create a LintResult — shared by all inline checkers.
pub fn mk_result(file: &str, line: usize, code: &str, sev: Severity, msg: &str) -> LintResult {
    LintResult {
        file: FilePath::new(file.to_string()).unwrap_or_default(),
        line: LineNumber::new(line as i64),
        column: ColumnNumber::new(0),
        code: ErrorCode::raw(code),
        message: LintMessage::new(msg),
        source: Some(AdapterName::raw("architecture")),
        severity: sev,
        enclosing_scope: None,
        related_locations: LocationList::new(),
    }
}
