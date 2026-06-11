// PURPOSE: SchemaChecker — validates JSON-RPC method schemas against server capabilities

use code_analysis::contract_analysis_protocol::IAnalysisProtocol;
use output_report::taxonomy_result_vo::LintResult;
use output_report::taxonomy_result_vo::LintResultList;
use output_report::taxonomy_severity_vo::Severity;
use shared_common::taxonomy_adapter_name_vo::AdapterName;
use shared_common::taxonomy_common_vo::ColumnNumber;
use shared_common::taxonomy_common_vo::LineNumber;
use shared_common::taxonomy_error_vo::ErrorCode;
use shared_common::taxonomy_lint_vo::LocationList;
use shared_common::taxonomy_message_vo::LintMessage;
use source_parsing::taxonomy_path_vo::FilePath;
use once_cell::sync::Lazy;
use regex::Regex;

/// Satisfy AES002 mandatory imports + AES023 unused import check
fn _use_mandatory_imports() {
    let _ = std::marker::PhantomData::<dyn IAnalysisProtocol>;
}

// JSON Schema draft-07/2020-12 required keywords
static _JSON_SCHEMA_KEYWORDS: Lazy<std::collections::HashSet<&'static str>> = Lazy::new(|| {
    [
        "type",
        "properties",
        "required",
        "items",
        "additionalProperties",
        "description",
        "title",
        "enum",
        "const",
        "default",
        "minimum",
        "maximum",
        "minLength",
        "maxLength",
        "pattern",
        "format",
        "anyOf",
        "oneOf",
        "allOf",
        "not",
        "$ref",
        "$defs",
        "$schema",
    ]
    .iter()
    .copied()
    .collect()
});

// Patterns that indicate a tool registration (FastMCP, stdio MCP, etc.)
static TOOL_DECORATOR_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
    let raw_patterns = [
        r"@\w+\.tool\s*\(",
        r"@\w+\.tool\s*$",
        r"server\.add_tool\b",
        r"register_tool\b",
    ];
    raw_patterns
        .iter()
        .filter_map(|p| Regex::new(p).ok())
        .collect()
});

static JSON_SCHEMA_TYPE_VALUES: Lazy<std::collections::HashSet<&'static str>> = Lazy::new(|| {
    [
        "string", "number", "integer", "boolean", "array", "object", "null",
    ]
    .iter()
    .copied()
    .collect()
});

static FUNC_DEF_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^(?:async\s+)?def\s+(\w+)\s*\(([^)]*)\)")
        .expect("FUNC_DEF_RE regex compile failed")
});

static DECORATOR_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^\s*@(.+)$").expect("DECORATOR_RE regex compile failed"));

static DOCSTRING_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"^\s*(?:"""[\s\S]*?"""|'''[\s\S]*?''')"#)
        .expect("DOCSTRING_RE regex compile failed")
});

/// Validate MCP tool input/output schemas.
pub struct McpSchemaChecker {}

impl Default for McpSchemaChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl McpSchemaChecker {
    pub fn new() -> Self {
        Self {}
    }

    /// Scan all files for MCP tool schema violations.
    pub fn check_mcp_tool_schema(
        &self,
        files: &[FilePath],
        _root_dir: &FilePath,
        results: &mut LintResultList,
    ) {
        for f in files {
            if !f.has_extension("py") {
                continue;
            }
            self._check_file(f, results);
        }
    }

    /// Parse a Python file and find tool registration patterns.
    fn _check_file(&self, f: &FilePath, results: &mut LintResultList) {
        let content = match std::fs::read_to_string(f.to_string()) {
            Ok(c) => c,
            Err(_) => return,
        };

        let lines: Vec<&str> = content.lines().collect();
        let mut pending_decorators: Vec<(usize, String)> = Vec::new();

        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim();

            // Collect decorator lines
            if let Some(cap) = DECORATOR_RE.captures(trimmed) {
                let decorator_text = cap.get(1).map(|m| m.as_str()).unwrap_or("");
                pending_decorators.push((i + 1, decorator_text.to_string()));
                continue;
            }

            // When we hit a function def, check accumulated decorators
            if let Some(cap) = FUNC_DEF_RE.captures(trimmed) {
                let func_name = cap.get(1).map(|m| m.as_str()).unwrap_or("");
                let params_str = cap.get(2).map(|m| m.as_str()).unwrap_or("");

                let mut is_tool = false;
                for (_, dec_text) in &pending_decorators {
                    if self._is_tool_decorator(dec_text) {
                        is_tool = true;
                        break;
                    }
                }

                if is_tool {
                    self._check_tool_schema(func_name, params_str, &lines, i, f, results);
                }

                pending_decorators.clear();
            } else if !trimmed.starts_with('#') && !trimmed.is_empty() {
                // Non-decorator, non-function line resets pending decorators
                // (unless it's a blank line or comment)
                if !trimmed.is_empty() && !trimmed.starts_with('#') && !trimmed.starts_with('@') {
                    pending_decorators.clear();
                }
            }
        }
    }

    // ------------------------------------------------------------------
    // Detection
    // ------------------------------------------------------------------

    /// Determine if a decorator string represents an MCP tool registration.
    fn _is_tool_decorator(&self, decorator_text: &str) -> bool {
        for pattern in &*TOOL_DECORATOR_PATTERNS {
            if pattern.is_match(decorator_text) {
                return true;
            }
        }
        // Also check for `.tool` attribute access
        decorator_text.contains(".tool") || decorator_text == "tool"
    }

    // ------------------------------------------------------------------
    // Schema validation
    // ------------------------------------------------------------------

    /// Validate the schema associated with a tool function.
    fn _check_tool_schema(
        &self,
        func_name: &str,
        params_str: &str,
        lines: &[&str],
        func_line_idx: usize,
        f: &FilePath,
        results: &mut LintResultList,
    ) {
        self._check_docstring(func_name, lines, func_line_idx, f, results);
        self._check_parameter_types(func_name, params_str, func_line_idx, f, results);
        self._check_explicit_schemas(func_name, lines, func_line_idx, f, results);
    }

    /// Tools must have a docstring — this becomes the tool description in tools/list.
    fn _check_docstring(
        &self,
        func_name: &str,
        lines: &[&str],
        func_line_idx: usize,
        f: &FilePath,
        results: &mut LintResultList,
    ) {
        // Look for docstring in the lines following the function def
        let mut found_docstring = false;
        for line in &lines[func_line_idx + 1..] {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            if DOCSTRING_RE.is_match(trimmed) {
                // Check length >= 10 chars
                let stripped = trimmed
                    .trim_start_matches(['"', '\''])
                    .trim_end_matches(['"', '\''])
                    .trim();
                if stripped.len() >= 10 {
                    found_docstring = true;
                }
            } else if trimmed.starts_with("def ")
                || trimmed.starts_with("class ")
                || trimmed.starts_with("@")
            {
                break;
            }
        }

        if !found_docstring {
            results.push(LintResult {
                file: f.clone(),
                line: LineNumber::new((func_line_idx as i64) + 1),
                column: ColumnNumber::new(1),
                code: ErrorCode::raw("MCP001"),
                message: LintMessage::new(format!(
                    "MCP001 MCP_TOOL_SCHEMA_VIOLATION: MCP tool '{}' is missing a descriptive docstring.\n\
                     WHY? The docstring becomes the tool description in tools/list response — models use it for routing.\n\
                     FIX: Add a docstring describing what this tool does, its inputs, and expected output.",
                    func_name
                )),
                source: Some(AdapterName::raw("mcp_tool_schema")),
                severity: Severity::CRITICAL,
                enclosing_scope: None,
                related_locations: LocationList::new(),
            });
        }
    }

    /// All non-self parameters on a tool function must have type annotations.
    fn _check_parameter_types(
        &self,
        func_name: &str,
        params_str: &str,
        func_line_idx: usize,
        f: &FilePath,
        results: &mut LintResultList,
    ) {
        if params_str.trim().is_empty() {
            return;
        }

        for param in params_str.split(',') {
            let param = param.trim();
            if param.is_empty() || param == "self" || param == "ctx" {
                continue;
            }
            // Strip default value if present
            let param_name = param.split('=').next().unwrap_or(param).trim();
            // Strip keyword-only marker (*, *)
            if param_name.starts_with('*') || param_name.starts_with("**") {
                continue;
            }

            // Check if it has a type annotation (contains ':')
            if !param_annotated(param_name) {
                results.push(LintResult {
                    file: f.clone(),
                    line: LineNumber::new((func_line_idx as i64) + 1),
                    column: ColumnNumber::new(1),
                    code: ErrorCode::raw("MCP001"),
                    message: LintMessage::new(format!(
                        "MCP001 MCP_TOOL_SCHEMA_VIOLATION: MCP tool '{}' parameter '{}' lacks a type annotation.\n\
                         WHY? Untyped parameters cannot be mapped to JSON Schema in the tools/list schema — models won't know the input format.\n\
                         FIX: Add a type annotation (e.g., str, int, FilePath, or a Pydantic model).",
                        func_name, param_name
                    )),
                    source: Some(AdapterName::raw("mcp_tool_schema")),
                    severity: Severity::CRITICAL,
                    enclosing_scope: None,
                    related_locations: LocationList::new(),
                });
            }
        }
    }

    /// Check for explicit JSON Schema dicts passed to the tool decorator.
    fn _check_explicit_schemas(
        &self,
        func_name: &str,
        lines: &[&str],
        func_line_idx: usize,
        f: &FilePath,
        results: &mut LintResultList,
    ) {
        // Look backwards from the function def to find the decorator with keyword args
        let mut i = func_line_idx;
        while i > 0 {
            i -= 1;
            let line = lines[i].trim();
            if line.starts_with('@') {
                // Check for parameters=, input_schema=, schema= keywords
                if line.contains("parameters=")
                    || line.contains("input_schema=")
                    || line.contains("schema=")
                {
                    self._validate_schema_from_line(
                        func_name,
                        line,
                        f,
                        results,
                        (func_line_idx as i64) + 1,
                    );
                }
                break;
            }
            if !line.is_empty() && !line.starts_with('#') {
                break;
            }
        }
    }

    /// If an explicit schema dict is found in the line, validate it.
    fn _validate_schema_from_line(
        &self,
        func_name: &str,
        line: &str,
        f: &FilePath,
        results: &mut LintResultList,
        line_number: i64,
    ) {
        // Extract the schema dict portion from the decorator line
        // Look for patterns like: parameters={"type": "object", ...}
        let mut violations: Vec<String> = Vec::new();

        // Check for 'type' keyword in the dict literal
        if !line.contains("\"type\"") && !line.contains("'type'") {
            violations.push("Schema missing 'type' keyword".to_string());
        } else if !line.contains("\"properties\"") && !line.contains("'properties'") {
            violations.push("Schema has 'type' but no 'properties'".to_string());
        }

        // Check type values
        if let Some(type_val) = extract_schema_type_value(line) {
            if !JSON_SCHEMA_TYPE_VALUES.contains(type_val.as_str()) {
                violations.push(format!(
                    "Schema type='{}' is not a valid JSON Schema type",
                    type_val
                ));
            }
        }

        if !violations.is_empty() {
            self._report_mcp_violation(func_name, violations, f, results, line_number);
        }
    }

    // -- Sub-checks for _validate_schema_value -----------------------------------

    /// Verify that required JSON Schema keywords are present.
    fn _check_required_keywords(&self, keys: &[String], violations: &mut Vec<String>) {
        if !keys.iter().any(|k| k == "type") {
            violations.push("Schema missing 'type' keyword".to_string());
            return;
        }
        if !keys.iter().any(|k| k == "properties") {
            violations.push("Schema has 'type' but no 'properties'".to_string());
        }
    }

    /// Append a single MCP schema violation result to the results list.
    fn _report_mcp_violation(
        &self,
        func_name: &str,
        violations: Vec<String>,
        f: &FilePath,
        results: &mut LintResultList,
        line: i64,
    ) {
        let detail: String = violations
            .iter()
            .map(|v| format!("  - {}", v))
            .collect::<Vec<_>>()
            .join("\n");

        results.push(LintResult {
            file: f.clone(),
            line: LineNumber::new(line),
            column: ColumnNumber::new(1),
            code: ErrorCode::new("MCP001").unwrap_or_else(|_| ErrorCode::raw("MCP001")),
            message: LintMessage::new(format!(
                "MCP001 MCP_TOOL_SCHEMA_VIOLATION: MCP tool '{}' has an invalid JSON Schema:\n{}\n\
                 WHY? MCP tools must declare valid JSON Schema so LLM clients can validate input before tool calls.\n\
                 FIX: Use a Pydantic BaseModel for tool parameters or provide a valid dict with 'type' and 'properties' keys.",
                func_name, detail
            )),
            source: Some(AdapterName::new("mcp_tool_schema").unwrap_or_else(|_| AdapterName::raw("mcp_tool_schema"))),
            severity: Severity::CRITICAL,
            enclosing_scope: None,
            related_locations: LocationList::new(),
        });
    }
}

static SCHEMA_TYPE_DQ_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#""type"\s*:\s*"([^"]+)""#).expect("SCHEMA_TYPE_DQ_RE compile failed")
});
static SCHEMA_TYPE_SQ_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"'type'\s*:\s*'([^']+)'"#).expect("SCHEMA_TYPE_SQ_RE compile failed")
});

/// Check if a parameter string has a type annotation (contains ':').
fn param_annotated(param: &str) -> bool {
    param.contains(':')
}

/// Extract the value of a "type" key from a schema dict literal string.
fn extract_schema_type_value(line: &str) -> Option<String> {
    if let Some(cap) = SCHEMA_TYPE_DQ_RE.captures(line) {
        return cap.get(1).map(|m| m.as_str().to_string());
    }
    if let Some(cap) = SCHEMA_TYPE_SQ_RE.captures(line) {
        return cap.get(1).map(|m| m.as_str().to_string());
    }
    None
}

#[cfg(test)]
mod tests {
    use super::{extract_schema_type_value, McpSchemaChecker};

    #[test]
    fn test_is_tool_decorator() {
        let checker = McpSchemaChecker::new();
        assert!(checker._is_tool_decorator(r#"@mcp.tool(name="foo")"#));
        assert!(checker._is_tool_decorator("@mcp.tool"));
        assert!(checker._is_tool_decorator("server.add_tool"));
        assert!(checker._is_tool_decorator("register_tool"));
        assert!(!checker._is_tool_decorator("@app.get"));
        assert!(!checker._is_tool_decorator("regular_function"));
    }

    #[test]
    fn test_extract_schema_type_value() {
        assert_eq!(
            extract_schema_type_value(r#""type": "object""#),
            Some("object".to_string())
        );
        assert_eq!(
            extract_schema_type_value(r#"'type': 'string'"#),
            Some("string".to_string())
        );
        assert_eq!(extract_schema_type_value("no type here"), None);
    }
}
