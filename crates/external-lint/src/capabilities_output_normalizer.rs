// PURPOSE: OutputNormalizer — INormalizeProtocol implementation (FR-005)
//
// Converts each external tool's raw output into the unified `LintResult` shape.
// Rule codes keep their tool-native identifiers (`clippy::<lint>`,
// `ruff::<code>`, `eslint::<rule>`, `cargo-audit::<RUSTSEC-ID>`, …) and
// severities follow the per-tool mapping in `crates/external-lint/FRD.md`.

use shared_common::taxonomy_adapter_name_vo::AdapterName;
use shared_common::taxonomy_common_vo::{ColumnNumber, LineNumber};
use shared_common::taxonomy_error_vo::ErrorCode;
use shared_common::taxonomy_lint_result_vo::{LintResult, LintResultList};
use shared_common::taxonomy_lint_vo::LocationList;
use shared_common::taxonomy_message_vo::LintMessage;
use shared_common::taxonomy_path_vo::FilePath;
use shared_common::taxonomy_severity_vo::Severity;
use shared_common::taxonomy_tool_name_vo::ToolName;
use shared_external_lint::contract_external_lint_protocol::INormalizeProtocol;

/// Shared normalizer for the tool-native severity mapping.
pub struct OutputNormalizer;

impl INormalizeProtocol for OutputNormalizer {
    fn normalize(
        &self,
        tool_name: &ToolName,
        raw_output: &str,
        root: &FilePath,
    ) -> (LintResultList, Vec<String>) {
        let mut results = Vec::new();
        let mut warnings = Vec::new();
        let source = AdapterName::raw(tool_name.value());

        let parsed: serde_json::Value = match serde_json::from_str(raw_output) {
            Ok(v) => v,
            Err(e) => {
                warnings.push(format!("{}: could not parse tool output: {}", tool_name, e));
                return (LintResultList::new(results), warnings);
            }
        };

        // Tools disagree on array vs object; normalize to a list of entries.
        let entries: Vec<serde_json::Value> = match parsed {
            serde_json::Value::Array(items) => items,
            serde_json::Value::Object(_) => vec![parsed],
            serde_json::Value::Null => Vec::new(),
            other => {
                warnings.push(format!("{}: unexpected output shape", tool_name));
                let _ = other;
                Vec::new()
            }
        };

        for entry in entries {
            let Some(file) = entry.get("file").and_then(|f| f.as_str()) else {
                continue;
            };
            let file = canonicalize_against(root, file);
            let line = entry
                .get("line")
                .and_then(serde_json::Value::as_i64)
                .unwrap_or(1);
            let column = entry
                .get("column")
                .and_then(serde_json::Value::as_i64)
                .unwrap_or(1);
            let code = entry
                .get("code")
                .and_then(|c| c.as_str())
                .unwrap_or("unknown");
            let code = if code.contains("::") {
                code.to_string()
            } else {
                format!("{}::{}", tool_name.value(), code)
            };
            let message = entry
                .get("message")
                .and_then(|m| m.as_str())
                .unwrap_or_default();
            let tool_severity = entry
                .get("severity")
                .and_then(|s| s.as_str())
                .unwrap_or_default();

            results.push(LintResult {
                file,
                line: LineNumber::new(line),
                column: ColumnNumber::new(column),
                code: ErrorCode::raw(code.clone()),
                message: LintMessage::new(message.to_string()),
                source: Some(source.clone()),
                severity: self.map_severity(
                    tool_name,
                    &ErrorCode::raw(code.clone()),
                    &parse_severity_str(tool_severity),
                ),
                enclosing_scope: None,
                related_locations: LocationList::new(),
            });
        }

        (LintResultList::new(results), warnings)
    }

    fn map_severity(
        &self,
        tool_name: &ToolName,
        code: &ErrorCode,
        tool_severity: &Severity,
    ) -> Severity {
        let code_str = code.code();
        // Tool-native severity labels ("correctness", "High", "2") collapse to
        // the enum here, so map them back to the level each tool's FRD prescribes.
        match tool_name.value() {
            "clippy" => match tool_severity {
                Severity::INFO => Severity::LOW,
                Severity::LOW => Severity::MEDIUM,
                Severity::MEDIUM => Severity::HIGH,
                _ => Severity::CRITICAL,
            },
            "rustfmt" | "prettier" => Severity::MEDIUM,
            "cargo-audit" => match tool_severity {
                Severity::INFO => Severity::LOW,
                Severity::LOW => Severity::MEDIUM,
                Severity::MEDIUM => Severity::HIGH,
                _ => Severity::CRITICAL,
            },
            "ruff" => map_ruff_severity(code_str),
            "mypy" => match tool_severity {
                Severity::HIGH | Severity::CRITICAL => Severity::HIGH,
                Severity::LOW => Severity::LOW,
                // A parse failure blocks all type checking, so escalate.
                _ if code_str.contains("syntax") || code_str.contains("parse") => {
                    Severity::CRITICAL
                }
                _ => Severity::MEDIUM,
            },
            "bandit" => match tool_severity {
                Severity::HIGH | Severity::CRITICAL => Severity::HIGH,
                Severity::MEDIUM => Severity::MEDIUM,
                _ => Severity::LOW,
            },
            "eslint" => match tool_severity {
                Severity::HIGH | Severity::CRITICAL => Severity::HIGH,
                _ => Severity::MEDIUM,
            },
            "tsc" => Severity::HIGH,
            _ => Severity::MEDIUM,
        }
    }
}

/// Carry a tool-native severity token into the `Severity` enum as an ordered
/// rank, so the per-tool mapping can compare it without string literals.
///
/// `error`/`High`/`2` rank highest, `note`/`Low`/`1` lowest, anything
/// unrecognized — which is most tool output — lands on `INFO`.
fn parse_severity_str(raw: &str) -> Severity {
    match raw {
        "error" | "Error" | "ERROR" | "2" | "Critical" | "CRITICAL" => Severity::CRITICAL,
        "High" | "HIGH" | "high" | "warning" | "Warning" | "WARNING" | "correctness" => {
            Severity::HIGH
        }
        "Medium" | "MEDIUM" | "medium" | "style" | "complexity" | "suspicious" | "perf" => {
            Severity::MEDIUM
        }
        "Low" | "LOW" | "low" | "note" | "Note" | "1" | "convention" | "nit" => Severity::LOW,
        _ => Severity::INFO,
    }
}

/// Ruff severity is determined by the rule code, not the tool's severity field.
fn map_ruff_severity(code: &str) -> Severity {
    // Codes are tool-native (`ruff::E501`), so strip the prefix before matching.
    let bare = code.rsplit("::").next().unwrap_or(code);
    if bare == "E999" || bare.starts_with('S') {
        return Severity::CRITICAL;
    }
    if bare.starts_with("F8") || bare.starts_with("B0") {
        return Severity::HIGH;
    }
    if bare == "F401" {
        return Severity::MEDIUM;
    }
    if bare.starts_with('E') || bare.starts_with('W') {
        return Severity::LOW;
    }
    Severity::MEDIUM
}

/// Turn a tool-reported path into an absolute path rooted at `root`.
fn canonicalize_against(root: &FilePath, file: &str) -> FilePath {
    let candidate = std::path::Path::new(file);
    if candidate.is_absolute() {
        return FilePath::new(file.to_string()).unwrap_or_else(|_| root.clone());
    }
    FilePath::new(
        std::path::Path::new(&root.value)
            .join(candidate)
            .to_string_lossy()
            .to_string(),
    )
    .unwrap_or_else(|_| root.clone())
}
