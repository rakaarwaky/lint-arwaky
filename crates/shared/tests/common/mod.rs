#![allow(dead_code)]

use shared_lint_arwaky::common::taxonomy_common_vo::LineNumber;
use shared_lint_arwaky::common::taxonomy_error_vo::ErrorCode;
use shared_lint_arwaky::common::taxonomy_lint_result_vo::LintResult;
use shared_lint_arwaky::common::taxonomy_message_vo::LintMessage;
use shared_lint_arwaky::common::taxonomy_path_vo::FilePath;
use shared_lint_arwaky::common::taxonomy_severity_vo::Severity;

pub fn fp(path: &str) -> FilePath {
    FilePath::new(path.to_string()).expect("valid file path in test")
}

pub fn violation(file: &str, line: usize, code: &str, severity: Severity) -> LintResult {
    LintResult {
        file: fp(file),
        line: LineNumber::new(line as i64),
        column: Default::default(),
        code: ErrorCode::raw(code),
        message: LintMessage::new(format!("violation at {file}:{line}")),
        source: None,
        severity,
        enclosing_scope: None,
        related_locations: Default::default(),
    }
}
