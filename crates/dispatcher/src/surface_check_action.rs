// PURPOSE: SurfaceCheckAction — check/scan business logic, no formatting.
//
// Runs all linter subprocesses (self-invocation pattern) with `--format json`,
// collects JSON results, and returns violations as data. CLI/MCP surfaces
// format the returned Vec<ViolationItem> themselves.
// Adapted: std::process::Command replaces tokio::process::Command; sequential
// execution replaces tokio::join!; filesystem aggregate injected via DI.
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use std::process::Command;
use std::sync::Arc;

use shared::common::FilePath;
use shared::config_system::IConfigOrchestratorAggregate;

use shared::common::ViolationItem;

pub struct ScanOptions {
    pub path: Option<FilePath>,
    pub multi_project_orchestrator: Option<Arc<dyn IConfigOrchestratorAggregate>>,
    pub filter: Option<String>,
    pub member: Option<String>,
    pub filesystem: Arc<dyn IFilesystemAggregate>,
}

pub type CheckOptions = ScanOptions;

/// Run all 6 linters via subprocesses, collect JSON, return unified violation list.
/// Err(String) carries a user-facing error message (path not found, bad member, ...).
pub fn collect_scan(opts: ScanOptions) -> Result<Vec<ViolationItem>, String> {
    let root = match &opts.path {
        Some(p) => p.value().to_string(),
        None => ".".to_string(),
    };
    if !opts.filesystem.path_exists(std::path::Path::new(&root)) {
        return Err(format!("Error: path '{}' does not exist", root));
    }

    // Validate member against discovered workspaces
    if let Some(ref m) = opts.member {
        if let Some(ref orchestrator) = opts.multi_project_orchestrator {
            let root_fp = FilePath::new(root.clone()).map_err(|_| "invalid path".to_string())?;
            let workspaces = orchestrator.discover_workspaces(&root_fp);
            if !workspaces.is_empty() {
                let matched = workspaces.iter().any(|ws| {
                    let ws_file = std::path::Path::new(&ws.path.value)
                        .file_name()
                        .map(|n| n.to_string_lossy())
                        .unwrap_or_default();
                    ws_file.as_ref() == m.as_str() || ws.path.value == *m
                });
                if !matched {
                    return Err(format!("[error] no workspace member matching '{m}'"));
                }
            }
        }
        let target_path = {
            let member_path = std::path::Path::new(&root).join(m);
            if member_path.exists() {
                member_path.to_string_lossy().to_string()
            } else {
                root.clone()
            }
        };
        let mut all_violations = run_all_linters_json(&target_path, opts.filesystem.as_ref());

        if let Some(ref filter_str) = opts.filter {
            let filter_upper = filter_str.to_uppercase();
            all_violations.retain(|v| v.code.code().contains(&filter_upper));
        }

        Ok(all_violations)
    } else {
        let mut all_violations = run_all_linters_json(&root, opts.filesystem.as_ref());

        if let Some(ref filter_str) = opts.filter {
            let filter_upper = filter_str.to_uppercase();
            all_violations.retain(|v| v.code.code().contains(&filter_upper));
        }

        Ok(all_violations)
    }
}

pub use collect_scan as collect_check;

/// Check if a path belongs to a workspace member.
pub fn is_member_path(path: &FilePath, fs_agg: &dyn IFilesystemAggregate) -> bool {
    fs_agg.is_member_path(path)
}

/// Run all 6 linters via subprocesses for a given path; return violations.
pub fn collect_scan_json(
    path: &str,
    fs_agg: &dyn IFilesystemAggregate,
) -> Result<Vec<ViolationItem>, String> {
    if !fs_agg.path_exists(std::path::Path::new(path)) {
        return Err(format!("Error: path '{}' does not exist", path));
    }
    Ok(run_all_linters_json(path, fs_agg))
}

/// Default check: subprocess JSON scan of all linters.
pub fn collect_default_check(
    project_root: &str,
    fs_agg: &dyn IFilesystemAggregate,
) -> Result<Vec<ViolationItem>, String> {
    collect_scan_json(project_root, fs_agg)
}

/// Run all 6 linters as subprocesses with `--format json`, collect ViolationItems.
fn run_all_linters_json(path: &str, fs_agg: &dyn IFilesystemAggregate) -> Vec<ViolationItem> {
    let exe_path = match std::env::current_exe() {
        Ok(p) => p,
        Err(_) => std::path::PathBuf::from("lint-arwaky-cli"),
    };

    let linter_names = ["quality", "role", "import", "naming", "orphan", "external"];

    let mut all: Vec<ViolationItem> = Vec::new();

    for linter_name in &linter_names {
        let output = Command::new(&exe_path)
            .args([linter_name, path, "--format", "json"])
            .output();

        if let Ok(out) = output {
            let stdout = String::from_utf8_lossy(&out.stdout);
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(stdout.trim()) {
                if let Some(results) = val.get("results").and_then(|r| r.as_array()) {
                    for item in results {
                        if let Some(v) = ViolationItem::from_json_obj(item) {
                            all.push(v);
                        }
                    }
                } else if let Some(items) = val.as_array() {
                    for item in items {
                        if let Some(v) = ViolationItem::from_json_obj(item) {
                            all.push(v);
                        }
                    }
                }
            }
        }
    }

    // Normalize relative paths to absolute before filtering.
    let target_canonical = fs_agg.canonicalize(std::path::Path::new(path)).ok();
    // Detect workspace root for resolving relative paths from orphan scan
    // (orphan scan returns paths like "crates/calculator/src/foo.rs" relative to workspace root)
    let ws_root = fs_agg.find_workspace_root(std::path::Path::new(path));
    {
        let cwd = std::env::current_dir().ok();
        let target_parent = target_canonical.as_ref().and_then(|t| t.parent());
        for v in &mut all {
            if std::path::Path::new(&v.file.value).is_absolute() {
                continue;
            }
            let rel = v.file.value.clone();
            let file_path = std::path::Path::new(&rel);

            // Try workspace root first (orphan scan paths are relative to workspace root)
            if let Some(ref ws) = ws_root {
                if let Ok(canon) = fs_agg.canonicalize(&ws.join(file_path)) {
                    v.file = FilePath::new(canon.to_string_lossy().to_string())
                        .unwrap_or_else(|_| v.file.clone());
                    continue;
                }
            }
            if let Some(ref cwd) = cwd {
                if let Ok(canon) = fs_agg.canonicalize(&cwd.join(file_path)) {
                    v.file = FilePath::new(canon.to_string_lossy().to_string())
                        .unwrap_or_else(|_| v.file.clone());
                    continue;
                }
            }
            if let Some(ref target) = target_canonical {
                if let Ok(canon) = fs_agg.canonicalize(&target.join(file_path)) {
                    v.file = FilePath::new(canon.to_string_lossy().to_string())
                        .unwrap_or_else(|_| v.file.clone());
                    continue;
                }
            }
            if let Some(parent) = target_parent {
                if let Ok(canon) = fs_agg.canonicalize(&parent.join(file_path)) {
                    v.file = FilePath::new(canon.to_string_lossy().to_string())
                        .unwrap_or_else(|_| v.file.clone());
                }
            }
        }
    }

    // Filter: only keep violations whose file path is within the target directory.
    // Exception: AES205 cycle violations are global — keep them if the file is
    // within the same parent workspace (e.g., workspaces-bad/).
    if let Some(canonical_target) = &target_canonical {
        let parent_workspace = canonical_target
            .parent()
            .and_then(|p| p.parent())
            .map(|p| p.to_path_buf());
        all.retain(|v| {
            let file_path = std::path::Path::new(&v.file.value);
            // Always retain AES205 cycle violations if within the same parent workspace
            if v.code.code() == "AES205" {
                if let Some(ref pw) = parent_workspace {
                    if let Ok(canonical) = fs_agg.canonicalize(file_path) {
                        if canonical.starts_with(pw) {
                            return true;
                        }
                    }
                }
            }
            if let Ok(canonical) = fs_agg.canonicalize(file_path) {
                return canonical.starts_with(canonical_target);
            }
            if let Ok(cwd) = std::env::current_dir() {
                let joined = cwd.join(file_path);
                let cwd_joined = fs_agg.canonicalize(&joined).unwrap_or(joined);
                if cwd_joined.starts_with(canonical_target) {
                    return true;
                }
            }
            if let Ok(target_joined) = fs_agg.canonicalize(&canonical_target.join(file_path)) {
                return target_joined.starts_with(canonical_target);
            }
            false
        });
    }

    all
}
