// PURPOSE: External lint scan business logic, no formatting.
//
// Data Flow:
//   CLI → collect_external_direct → filesystem.build_file_index_with_ignored → external_lint.scan_all → violations
//
// The external-lint orchestrator receives a pre-built file index and
// performs language detection + adapter execution. All filesystem access
// is handled by the filesystem aggregate before orchestrator delegation.
use std::process::Command;
use std::sync::Arc;

use shared::common::FilePath;
use shared::external_lint::IExternalLintAggregate;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;

use crate::surface_output_component::ViolationItem;

/// Direct external lint scan — no subprocess. Used by the CLI `external`
/// subcommand so that subprocess self-invocation from `scan` terminates.
pub fn collect_external_direct(
    path: Option<FilePath>,
    external_lint: Arc<dyn IExternalLintAggregate>,
    filesystem: Arc<dyn IFilesystemAggregate>,
    filter: Option<String>,
    ignored_paths: &[String],
) -> Result<Vec<ViolationItem>, String> {
    let root = match &path {
        Some(p) => p.value().to_string(),
        None => ".".to_string(),
    };
    if !filesystem.path_exists(std::path::Path::new(&root)) {
        return Err(format!("Error: path '{}' does not exist", root));
    }
    let root_fp = FilePath::new(root.clone()).map_err(|_| "invalid path".to_string())?;

    // Build file index for target path (respects config ignored_paths)
    let root_path = std::path::Path::new(&root);
    filesystem.build_file_index_with_ignored(root_path, ignored_paths);

    let scan_results = external_lint.scan_all(&root_fp);
    let mut violations: Vec<ViolationItem> = scan_results
        .values
        .iter()
        .map(ViolationItem::from_lint_result)
        .collect();

    if let Some(ref filter_str) = filter {
        let filter_upper = filter_str.to_uppercase();
        violations.retain(|v| v.code.code().contains(&filter_upper));
    }

    Ok(violations)
}

pub fn collect_external(
    path: Option<FilePath>,
    _external_lint: Arc<dyn IExternalLintAggregate>,
    filter: Option<String>,
    _filesystem: Arc<dyn IFilesystemAggregate>,
) -> Result<Vec<ViolationItem>, String> {
    let root = match &path {
        Some(p) => p.value().to_string(),
        None => ".".to_string(),
    };
    if !_filesystem.path_exists(std::path::Path::new(&root)) {
        return Err(format!("Error: path '{}' does not exist", root));
    }

    // Use subprocess approach — spawn external linter and parse JSON output
    let exe_path = match std::env::current_exe() {
        Ok(p) => p,
        Err(_) => std::path::PathBuf::from("lint-arwaky-cli"),
    };

    let output = Command::new(&exe_path)
        .args(["external", &root, "--format", "json"])
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
        Err(e) => return Err(format!("[error] failed to run external linter: {e}")),
    };

    if let Some(ref filter_str) = filter {
        let filter_upper = filter_str.to_uppercase();
        violations.retain(|v| v.code.code().contains(&filter_upper));
    }

    Ok(violations)
}
