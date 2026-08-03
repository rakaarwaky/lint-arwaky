// PURPOSE: Role rules scan business logic, no formatting.
// AES406 NOTE: Uses subprocess approach (self-invocation) to run role scanning because
// IRoleRunnerAggregate only has run_audit_with_entries, not a single-path variant.
// This is a known gap — IRoleRunnerAggregate should expose a simpler scan method.
// Adapted: IRoleRunnerAggregate no longer has run_audit(path) — only run_audit_with_entries.
use std::process::Command;
use std::sync::Arc;

use shared::common::FilePath;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::role_rules::IRoleRunnerAggregate;

use crate::surface_output_component::ViolationItem;

pub fn collect_role(
    path: Option<FilePath>,
    _role_orchestrator: Arc<dyn IRoleRunnerAggregate>,
    filter: Option<String>,
    _fs_agg: Arc<dyn IFilesystemAggregate>,
) -> Result<Vec<ViolationItem>, String> {
    let root = match &path {
        Some(p) => p.value().to_string(),
        None => ".".to_string(),
    };
    if !std::path::Path::new(&root).exists() {
        return Err(format!("Error: path '{}' does not exist", root));
    }

    let _root_fp = FilePath::new(root.clone()).map_err(|_| "invalid path".to_string())?;

    // Use subprocess approach since IRoleRunnerAggregate only has run_audit_with_entries
    let exe_path = match std::env::current_exe() {
        Ok(p) => p,
        Err(_) => std::path::PathBuf::from("lint-arwaky-cli"),
    };

    let output = Command::new(&exe_path)
        .args(["role", &root, "--format", "json"])
        .output();

    let mut violations: Vec<ViolationItem> = match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(stdout.trim()) {
                if let Some(results) = val.get("results").and_then(|r| r.as_array()) {
                    results
                        .iter()
                        .filter_map(ViolationItem::from_json_obj)
                        .collect()
                } else if let Some(items) = val.as_array() {
                    items
                        .iter()
                        .filter_map(ViolationItem::from_json_obj)
                        .collect()
                } else {
                    Vec::new()
                }
            } else {
                Vec::new()
            }
        }
        Err(e) => return Err(format!("[error] failed to run role linter: {e}")),
    };

    if let Some(ref filter_str) = filter {
        let filter_upper = filter_str.to_uppercase();
        violations.retain(|v| v.code.code().contains(&filter_upper));
    }

    Ok(violations)
}
