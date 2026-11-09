// PURPOSE (surface layer — the `_view` half names MCP JSON rendering): MCP surface helpers — JSON response builders and path guards.
//
// Extracted from `surface_mcp_action_command.rs` (AES301: file too large).
// These pure helpers build JSON envelopes, normalize responses, and guard
// client paths against workspace escapes. No dispatcher logic lives here.
use serde_json::Value;
use serde_json::json;
use shared_common::taxonomy_violation_item_vo::ViolationItem;
use std::path::{Component, Path, PathBuf};

/// Serialize violations to JSON (mirrors old execute_* shape + severity).
pub fn violations_to_json(violations: &[ViolationItem]) -> Vec<Value> {
    violations
        .iter()
        .map(|v| {
            json!({
                "file": v.file.value,
                "code": v.code.code(),
                "message": v.message.value,
                "line": v.line.value(),
                "column": v.column.value(),
                "severity": format!("{}", v.severity),
            })
        })
        .collect()
}

/// Standard scan response envelope.
pub(crate) fn violations_response(action: &str, path: &str, violations: &[ViolationItem]) -> Value {
    let exit_code = if violations.is_empty() { 0 } else { 1 };
    json!({
        "status": if exit_code == 0 { "ok" } else { "warning" },
        "result": if exit_code == 0 { "clean" } else { "violations" },
        "action": action,
        "path": path,
        "exit_code": exit_code,
        "violation_count": violations.len(),
        "results": violations_to_json(violations),
    })
}

/// Resolve a client path beneath the configured root. Lexical parent segments
/// are rejected before canonicalization, and canonicalization then protects
/// against symlink escapes.
pub fn resolve_confined_path(root: &Path, requested: &str) -> Result<PathBuf, Value> {
    if requested.trim().is_empty() {
        return Err(error_response("Invalid path"));
    }
    let canonical_root = root
        .canonicalize()
        .map_err(|_| error_response("Workspace root does not exist"))?;
    let requested_path = Path::new(requested);
    if requested_path
        .components()
        .any(|part| matches!(part, Component::ParentDir))
    {
        return Err(error_response("Path escapes workspace root"));
    }
    let candidate = if requested_path.is_absolute() {
        requested_path.to_path_buf()
    } else {
        canonical_root.join(requested_path)
    };
    let resolved = candidate
        .canonicalize()
        .map_err(|_| error_response("Path does not exist"))?;
    if !resolved.starts_with(&canonical_root) {
        return Err(error_response("Path escapes workspace root"));
    }
    Ok(resolved)
}

pub fn is_mutating_action(action: &str) -> bool {
    matches!(
        action,
        "fix" | "install-hook" | "uninstall-hook" | "init" | "install" | "docs_scaffold"
    )
}

pub fn normalize_response(mut response: Value) -> Value {
    let exit_code = response
        .get("exit_code")
        .and_then(Value::as_i64)
        .unwrap_or(2);
    if let Some(object) = response.as_object_mut() {
        let normalized = if exit_code == 0 {
            "ok"
        } else if exit_code == 1 || exit_code == 3 {
            "warning"
        } else {
            "error"
        };
        object.insert("status".to_string(), json!(normalized));
    }
    response
}

pub fn error_response(message: &str) -> Value {
    json!({"status": "error", "error": message, "exit_code": 2})
}
