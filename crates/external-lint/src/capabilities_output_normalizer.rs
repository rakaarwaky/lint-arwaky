// PURPOSE: OutputNormalizer — INormalizeProtocol implementation (FR-005)
//
// Converts each external tool's raw output into the unified `LintResult` shape.
// Rule codes keep their tool-native identifiers (`clippy::<lint>`,
// `ruff::<code>`, `eslint::<rule>`, `cargo-audit::<RUSTSEC-ID>`, …) and
// severities follow the per-tool mapping in `crates/external-lint/FRD.md`.

use shared::common::taxonomy_adapter_name_vo::AdapterName;
use shared::common::taxonomy_common_vo::{ColumnNumber, LineNumber};
use shared::common::taxonomy_error_vo::ErrorCode;
use shared::common::taxonomy_lint_result_vo::{LintResult, LintResultList};
use shared::common::taxonomy_lint_vo::LocationList;
use shared::common::taxonomy_message_vo::LintMessage;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_severity_vo::Severity;
use shared::external_lint::contract_external_lint_protocol::INormalizeProtocol;

/// Shared normalizer for the tool-native severity mapping.
pub struct OutputNormalizer;

impl INormalizeProtocol for OutputNormalizer {
    fn normalize(
        &self,
        tool_name: &str,
        raw_output: &str,
        root: &FilePath,
    ) -> (LintResultList, Vec<String>) {
        let mut results = Vec::new();
        let mut warnings = Vec::new();
        let source = AdapterName::raw(tool_name);

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
                format!("{}::{}", tool_name, code)
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
                severity: self.map_severity(tool_name, &code, tool_severity),
                enclosing_scope: None,
                related_locations: LocationList::new(),
            });
        }

        (LintResultList::new(results), warnings)
    }

    fn map_severity(&self, tool_name: &str, code: &str, tool_severity: &str) -> Severity {
        match tool_name {
            "clippy" => match tool_severity {
                "correctness" => Severity::CRITICAL,
                "suspicious" | "perf" => Severity::HIGH,
                "style" | "complexity" => Severity::MEDIUM,
                _ => Severity::LOW,
            },
            "rustfmt" | "prettier" => Severity::MEDIUM,
            "cargo-audit" => match tool_severity {
                "Critical" => Severity::CRITICAL,
                "High" => Severity::HIGH,
                "Medium" => Severity::MEDIUM,
                _ => Severity::LOW,
            },
            "ruff" => map_ruff_severity(code),
            "mypy" => match tool_severity {
                "error" => Severity::HIGH,
                "warning" => Severity::MEDIUM,
                "note" => Severity::LOW,
                // A parse failure blocks all type checking, so escalate.
                _ if code.contains("syntax") || code.contains("parse") => Severity::CRITICAL,
                _ => Severity::MEDIUM,
            },
            "bandit" => match tool_severity {
                "HIGH" => Severity::HIGH,
                "MEDIUM" => Severity::MEDIUM,
                _ => Severity::LOW,
            },
            "eslint" => match tool_severity {
                "2" | "error" => Severity::HIGH,
                _ => Severity::MEDIUM,
            },
            "tsc" => Severity::HIGH,
            _ => Severity::MEDIUM,
        }
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
