// PURPOSE: taxonomy_violation_item_vo — shared violation data type for all surface actions.
// Rendering (text/json/sarif/junit) lives in cli-commands (surface_formatting).
// Dispatcher computes violation lists; CLI/MCP surfaces format them themselves.

use crate::common::taxonomy_common_vo::{ColumnNumber, LineNumber};
use crate::common::taxonomy_error_vo::ErrorCode;
use crate::common::taxonomy_lint_result_vo::LintResult;
use crate::common::taxonomy_message_vo::LintMessage;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_severity_vo::Severity;

/// Minimal violation item for display. Uses existing VOs — no duplicate String wrappers.
#[derive(Debug, Clone)]
pub struct ViolationItem {
    pub code: ErrorCode,
    pub file: FilePath,
    pub line: LineNumber,
    pub column: ColumnNumber,
    pub message: LintMessage,
    pub severity: Severity,
}

impl ViolationItem {
    pub fn from_lint_result(r: &LintResult) -> Self {
        Self {
            code: r.code.clone(),
            file: r.file.clone(),
            line: r.line.clone(),
            column: r.column.clone(),
            message: r.message.clone(),
            severity: r.severity.clone(),
        }
    }

    pub fn from_json_obj(item: &serde_json::Value) -> Option<Self> {
        Some(Self {
            code: ErrorCode::raw(item.get("code")?.as_str()?),
            file: FilePath::new(item.get("file")?.as_str()?.to_string()).ok()?,
            line: LineNumber::new(item.get("line").and_then(|v| v.as_i64()).unwrap_or(0)),
            column: ColumnNumber::new(item.get("column").and_then(|v| v.as_i64()).unwrap_or(0)),
            message: LintMessage::new(item.get("message")?.as_str()?),
            severity: parse_severity(
                item.get("severity")
                    .and_then(|v| v.as_str())
                    .unwrap_or("INFO"),
            ),
        })
    }

    pub fn severity_level(&self) -> u8 {
        match self.severity {
            Severity::CRITICAL => 4,
            Severity::HIGH => 3,
            Severity::MEDIUM => 2,
            Severity::LOW => 1,
            Severity::INFO => 0,
        }
    }
}

fn parse_severity(s: &str) -> Severity {
    match s.to_uppercase().as_str() {
        "CRITICAL" => Severity::CRITICAL,
        "HIGH" => Severity::HIGH,
        "MEDIUM" => Severity::MEDIUM,
        "LOW" => Severity::LOW,
        _ => Severity::INFO,
    }
}
