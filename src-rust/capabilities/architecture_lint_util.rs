// architecture_lint_util — Centralized utility helper functions for architectural linter rules.

use crate::taxonomy::{
    AdapterName, ColumnNumber, ErrorCode, FilePath, LintMessage, LintResult, 
    LineNumber, LocationList, ScopeRef, Severity
};

pub fn make_adapter(name: &str) -> AdapterName {
    AdapterName::raw(name)
}

pub fn make_lint_result(
    file: &str, line: i64, col: i64, code: &str, msg: &str, sev: Severity,
) -> LintResult {
    LintResult {
        file: FilePath::new(file.to_string()),
        line: LineNumber::new(line),
        column: ColumnNumber::new(col),
        code: ErrorCode::raw(code),
        message: LintMessage::new(msg.to_string()),
        source: make_adapter("architecture"),
        severity: sev,
        enclosing_scope: ScopeRef::default(),
        related_locations: LocationList::new(),
    }
}

pub fn is_barrel_file(filename: &str) -> bool {
    matches!(filename, "__init__.py" | "mod.rs" | "index.ts" | "index.js")
}

pub fn is_entry_point(filename: &str) -> bool {
    matches!(filename, "__init__.py" | "main.py" | "py.typed" | "app.py" | "lib.rs")
}
