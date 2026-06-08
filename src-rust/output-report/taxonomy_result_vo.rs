use serde::{Deserialize, Serialize};

use crate::shared_common::taxonomy_name_vo::AdapterName;
use crate::shared_common::taxonomy_common_vo::ColumnNumber;
use crate::shared_common::taxonomy_error_vo::ErrorCode;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use crate::shared_common::taxonomy_layer_vo::Identity;
use /* UNKNOWN: LineNumber */ crate::shared_common::taxonomy_common_vo::LineNumber;
use /* UNKNOWN: LintMessage */ crate::shared_common::taxonomy_message_vo::LintMessage;
use /* UNKNOWN: LocationList */ crate::shared_common::taxonomy_lint_vo::LocationList;
use crate::output_report::taxonomy_position_vo::Position;
use /* UNKNOWN: ScopeRef */ crate::shared_common::taxonomy_lint_vo::ScopeRef;
use crate::output_report::taxonomy_severity_vo::Severity;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LintResult {
    pub file: FilePath,
    pub line: LineNumber,
    pub column: ColumnNumber,
    pub code: ErrorCode,
    pub message: LintMessage,
    pub source: Option<AdapterName>,
    pub severity: Severity,
    pub enclosing_scope: Option<ScopeRef>,
    pub related_locations: LocationList,
}

impl LintResult {
    pub fn new(
        file: FilePath,
        line: LineNumber,
        column: ColumnNumber,
        code: ErrorCode,
        message: LintMessage,
        source: Option<AdapterName>,
        severity: Severity,
        enclosing_scope: Option<ScopeRef>,
        related_locations: LocationList,
    ) -> Self {
        Self {
            file,
            line,
            column,
            code,
            message,
            source,
            severity,
            enclosing_scope,
            related_locations,
        }
    }

    /// Convenience constructor used by architecture checkers (make_result / mk pattern).
    pub fn new_arch(file: &str, line: usize, code: &str, sev: Severity, msg: &str) -> Self {
        Self {
            file: FilePath::new(file.to_string()).unwrap_or_default(),
            line: LineNumber::new(line as i64),
            column: ColumnNumber::new(0),
            code: ErrorCode::raw(code),
            message: LintMessage::new(msg),
            source: Some(AdapterName::raw("architecture")),
            severity: sev,
            enclosing_scope: Some(crate::shared_common::taxonomy_lint_vo::ScopeRef {
                name: crate::shared_common::taxonomy_suggestion_vo::DescriptionVO::new(String::new()),
                kind: crate::shared_common::taxonomy_suggestion_vo::DescriptionVO::new(String::new()),
                file: None,
                start_line: None,
                end_line: None,
            }),
            related_locations: crate::shared_common::taxonomy_lint_vo::LocationList::new(),
        }
    }

    pub fn position(&self) -> Position {
        Position {
            line: self.line.clone(),
            column: self.column.clone(),
        }
    }
    pub fn identity(&self) -> Identity {
        Identity::new(format!(
            "{}:{}:{}:{:?}",
            self.file, self.line, self.code, self.source
        ))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LintResultList {
    pub values: Vec<LintResult>,
}

impl LintResultList {
    pub fn new(value: Vec<LintResult>) -> Self {
        Self { values: value }
    }
    pub fn iter(&self) -> std::slice::Iter<'_, LintResult> {
        self.values.iter()
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    pub fn push(&mut self, item: LintResult) {
        self.values.push(item);
    }
    pub fn append(&mut self, item: LintResult) {
        self.values.push(item);
    }
}
