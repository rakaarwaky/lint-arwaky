// PURPOSE: taxonomy_violation_item_vo — shared violation data type for all surface actions.
// Rendering (text/json/sarif/junit) lives in cli-commands (surface_formatting).
// Dispatcher computes violation lists; CLI/MCP surfaces format them themselves.

use crate::taxonomy_common_vo::{ColumnNumber, LineNumber};
use crate::taxonomy_error_vo::ErrorCode;
use crate::taxonomy_lint_vo::LintResult;
use crate::taxonomy_message_vo::LintMessage;
use crate::taxonomy_path_vo::FilePath;
use crate::taxonomy_severity_vo::Severity;

/// Minimal violation item for display. Uses existing VOs — no duplicate String wrappers.
#[derive(Debug, Clone)]
pub struct ViolationItem {
    pub code: ErrorCode,
    pub file: FilePath,
    pub line: LineNumber,
    pub column: ColumnNumber,
    pub message: LintMessage,
    pub severity: Severity,
    /// Short subtype name (e.g. "TAXONOMY_ROLE"). Empty → parsed from `message`.
    pub violation_name: String,
    /// WHY text. Empty → parsed from `message` (`WHY?` / `WHY:`).
    pub why: String,
    /// FIX text. Empty → parsed from `message` (`FIX:` / `HOW TO FIX?`).
    pub fix: String,
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
            violation_name: if r.violation_name.is_empty() {
                parse_violation_name(r.code.code(), &r.message.value)
            } else {
                r.violation_name.clone()
            },
            why: if r.why.is_empty() {
                parse_why(&r.message.value)
            } else {
                r.why.clone()
            },
            fix: if r.fix.is_empty() {
                parse_fix(&r.message.value)
            } else {
                r.fix.clone()
            },
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
            violation_name: item
                .get("violation_name")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            why: item
                .get("why")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            fix: item
                .get("fix")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
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

// ─── Merged from taxonomy_violation_message_vo ────────────────
// Recover the structured violation fields from a LintMessage's embedded text.
// Pure functions over a string, no I/O. This is the backward-compat path for
// a capability that has not yet populated violation_name / why / fix on its
// LintResult: the legacy message format carried the same three pieces, so the
// report can stay complete without every capability having been converted
// first.

/// Extract the violation subtype name from a message.
///
/// Format: `AES401 TAXONOMY_ROLE: description...`
/// Returns the token after the code prefix (e.g. "TAXONOMY_ROLE").
/// Falls back to empty when the message does not match the expected shape.
pub fn parse_violation_name(code: &str, message: &str) -> String {
    let first_line = message.lines().next().unwrap_or("");
    // Only a message that actually starts with the code can yield a name.
    // Without this guard a tool code that itself contains a colon
    // (`markdownlint::MD022`) would split on its own prefix and hand back
    // prose from the message body.
    let Some(rest) = first_line.strip_prefix(code) else {
        return String::new();
    };
    let rest = rest.trim_start_matches([' ', ':']);
    // The name is the first whitespace-delimited token, and it must be a
    // SCREAMING_SNAKE_CASE identifier — anything else is description prose.
    let token = rest.split_whitespace().next().unwrap_or("");
    let name = token.trim_end_matches(':');
    if is_violation_name(name) {
        return name.to_string();
    }
    String::new()
}

/// A violation name is upper-case identifiers joined by underscores
/// (`TAXONOMY_ROLE`, `AGENT_ROLE`, `H2_MISSING`). Rejects anything that
/// carries spaces, lowercase prose, or punctuation.
fn is_violation_name(candidate: &str) -> bool {
    !candidate.is_empty()
        && candidate.len() <= 64
        && candidate
            .chars()
            .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_')
        && candidate
            .chars()
            .next()
            .is_some_and(|c| c.is_ascii_uppercase())
}

/// Extract WHY text from a message.
///
/// Recognizes these line prefixes (anywhere in the message, line by line):
/// - `WHY?`  (legacy format used by capabilities)
/// - `WHY:`
///
/// Returns the text after the prefix, trimmed. Empty string when no WHY
/// line is present.
pub fn parse_why(message: &str) -> String {
    for line in message.lines() {
        let trimmed = line.trim();
        if let Some(text) = trimmed
            .strip_prefix("WHY?")
            .or_else(|| trimmed.strip_prefix("WHY:"))
        {
            return text.trim().to_string();
        }
    }
    String::new()
}

/// Extract FIX text from a message.
///
/// Recognizes:
/// - `FIX:`  (canonical short form)
/// - `HOW TO FIX?`  (legacy long form)
///
/// Returns the text after the prefix, trimmed. Empty string when no FIX
/// line is present.
pub fn parse_fix(message: &str) -> String {
    for line in message.lines() {
        let trimmed = line.trim();
        if let Some(text) = trimmed
            .strip_prefix("HOW TO FIX?")
            .or_else(|| trimmed.strip_prefix("FIX:"))
        {
            return text.trim().to_string();
        }
    }
    String::new()
}
