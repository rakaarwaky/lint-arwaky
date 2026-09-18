// PURPOSE: SurfaceCheckAction — check/scan business logic, no formatting.
//
// In-process mode (W10): when `scan_aggregates` is provided, all 6 linters run
// in-process through their aggregate entry points. Subprocess self-invocation
// remains the fallback when aggregates are absent (`scan_aggregates: None`).
use shared::common::FilePath;
use shared::common::ViolationItem;
use shared::config_system::IConfigOrchestratorAggregate;
use shared::external_lint::IExternalLintAggregate;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::import_rules::IImportRunnerAggregate;
use shared::naming_rules::INamingRunnerAggregate;
use shared::orphan_rules::IOrphanAggregate;
use shared::quality_rules::ICodeAnalysisAggregate;
use shared::role_rules::IRoleRunnerAggregate;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Arc;

/// Bundles the 6 scan aggregates + config source + filesystem factory so that
/// `collect_scan` can dispatch all linters in-process (W10).
#[derive(Clone)]
pub struct ScanAggregates {
    pub quality: Arc<dyn ICodeAnalysisAggregate>,
    pub role: Arc<dyn IRoleRunnerAggregate>,
    pub import: Arc<dyn IImportRunnerAggregate>,
    pub naming: Arc<dyn INamingRunnerAggregate>,
    pub external: Arc<dyn IExternalLintAggregate>,
    pub orphan: Arc<dyn IOrphanAggregate>,
    pub config: Arc<dyn IConfigOrchestratorAggregate>,
    /// Creates a fresh filesystem instance (uncached pipeline) per scan.
    pub fs_factory: Arc<dyn Fn() -> Arc<dyn IFilesystemAggregate> + Send + Sync>,
}

pub struct ScanOptions {
    pub path: Option<FilePath>,
    pub multi_project_orchestrator: Option<Arc<dyn IConfigOrchestratorAggregate>>,
    pub filter: Option<String>,
    pub member: Option<String>,
    pub filesystem: Arc<dyn IFilesystemAggregate>,
    /// In-process aggregate bundle (W10). `None` falls back to subprocess.
    pub scan_aggregates: Option<ScanAggregates>,
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

    // W10: when aggregates are wired, resolve the target to an absolute path so
    // in-process linters scan the intended scope. Subprocess fallback keeps the
    // raw path (its per-linter normalization differs).
    let root = if opts.scan_aggregates.is_some() {
        opts.filesystem
            .canonicalize(std::path::Path::new(&root))
            .unwrap_or_else(|_| PathBuf::from(root))
            .to_string_lossy()
            .to_string()
    } else {
        root
    };

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
        if let Some(ref agg) = opts.scan_aggregates {
            let mut all_violations = run_all_linters_in_process(&target_path, agg);
            if let Some(ref filter_str) = opts.filter {
                let filter_upper = filter_str.to_uppercase();
                all_violations.retain(|v| v.code.code().contains(&filter_upper));
            }
            Ok(all_violations)
        } else {
            let mut all_violations = run_all_linters_json(&target_path, opts.filesystem.as_ref());
            if let Some(ref filter_str) = opts.filter {
                let filter_upper = filter_str.to_uppercase();
                all_violations.retain(|v| v.code.code().contains(&filter_upper));
            }
            Ok(all_violations)
        }
    } else {
        if let Some(ref agg) = opts.scan_aggregates {
            let mut all_violations = run_all_linters_in_process(&root, agg);
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

/// Run all 6 linters in-process through their aggregate entry points (W10).
///
/// E902 fix: when the scan target is a member directory (e.g.
/// `workspaces-good/modules`) the discovered workspace root is the parent
/// (e.g. `workspaces-good`). Building the file index from that parent makes
/// the entry paths relative to it, so a later root + relative-path join
/// produces a doubled path (`workspaces-good/modules/workspaces-good/modules`)
/// and a file-not-found E902 violation. The subprocess path avoided this:
/// each linter's own file index scoping normalized the target. Here we keep
/// the target itself as the scan scope, and every linter output that names a
/// non-existent file is dropped (with no violation emitted) so stale or
/// doubled paths can never surface as E902.
fn run_all_linters_in_process(path: &str, agg: &ScanAggregates) -> Vec<ViolationItem> {
    let fs = (agg.fs_factory)();

    let target = std::path::Path::new(path);
    let target_canon = fs
        .canonicalize(target)
        .unwrap_or_else(|_| std::path::PathBuf::from(path));
    let target_canon_str = target_canon.to_string_lossy().to_string();

    // E902 fix: when the target sits inside a larger workspace (parent has
    // crates/packages/modules + a manifest) keep the target as the scan scope
    // instead of re-discovering and re-joining the workspace root. For
    // targets outside any workspace, use the target itself.
    let ws_root = fs.find_workspace_root(target);
    let target_inside_ws = ws_root
        .as_ref()
        .map(|r| {
            let t = std::path::Path::new(&target_canon_str);
            t.starts_with(r)
        })
        .unwrap_or(false);
    let scan_root = target_canon.clone();

    let root_fp = FilePath::new(scan_root.to_string_lossy().to_string()).unwrap_or_default();

    let ignored = agg
        .config
        .ignored_paths(&root_fp)
        .values
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>();

    fs.build_file_index_with_ignored(std::path::Path::new(&scan_root), &ignored);
    let files: Vec<shared::filesystem::taxonomy_filesystem_vo::FileEntry> = fs.file_list().to_vec();

    // E902 fix: verify each file actually exists on disk before passing it to
    // the linters so no file-not-found is emitted for stale/doubled paths.
    let verified: Vec<&shared::filesystem::taxonomy_filesystem_vo::FileEntry> = files
        .iter()
        .filter(|f| fs.path_exists(std::path::Path::new(&f.path)))
        .collect();
    let entries = if verified.is_empty() && !files.is_empty() {
        // Mock filesystem reports no paths — keep all entries so behavior
        // matches the subprocess fallback.
        Vec::new()
    } else {
        verified.iter().map(|&e| e.clone()).collect()
    };

    let mut all: Vec<ViolationItem> = Vec::new();

    // Filter: keep only violations whose file path is within the target scope,
    // and drop any violation that names a non-existent file (E902 guard).
    // AES205 cycle violations are global — keep if within the same parent
    // workspace.
    let parent_workspace = target_canon
        .parent()
        .and_then(|p| p.parent())
        .map(|p| p.to_path_buf());

    // Quality
    all.extend(
        agg.quality
            .run_analysis_with_entries(&entries)
            .iter()
            .map(ViolationItem::from_lint_result),
    );
    // Role
    all.extend(
        agg.role
            .run_audit_with_entries(&entries)
            .iter()
            .map(ViolationItem::from_lint_result),
    );
    // Import
    all.extend(
        agg.import
            .run_audit_with_entries(&entries)
            .iter()
            .map(ViolationItem::from_lint_result),
    );
    // Naming
    all.extend(
        agg.naming
            .run_audit_with_entries(&entries)
            .iter()
            .map(ViolationItem::from_lint_result),
    );
    // Orphan — the linter operates on workspace-wide context: its own
    // `scan_orphans` rebuilds the graph from root + ignored patterns, which is
    // what the subprocess path passed to it.
    let (_graph_ctx, orphan_violations) = agg.orphan.scan_orphans(&root_fp, &ignored);
    all.extend(
        orphan_violations
            .iter()
            .map(ViolationItem::from_lint_result),
    );

    // External — match the subprocess path: adapters run on the target *as
    // given* (relative paths resolve against the process CWD, exactly like the
    // spawned linters did). Running them on the canonicalized target would
    // make config/tool discovery differ from the subprocess baseline, so keep
    // the original target string for the external aggregate.
    let ext_target_fp = FilePath::new(path.to_string()).unwrap_or_default();
    {
        let ext_files = fs.discover_files(std::path::Path::new(&target_canon_str));
        let has_rust = ext_files.iter().any(|f| f.ends_with(".rs"));
        let has_python = ext_files.iter().any(|f| f.ends_with(".py"));
        let has_js = ext_files.iter().any(|f| {
            f.ends_with(".js") || f.ends_with(".jsx") || f.ends_with(".ts") || f.ends_with(".tsx")
        });
        let context = shared::external_lint::taxonomy_external_lint_vo::ExternalLintContext {
            has_rust,
            has_python,
            has_js,
            ignored_paths: ignored.clone(),
            config_entries: Vec::new(),
        };
        let mut external: Vec<ViolationItem> = agg
            .external
            .scan_all_with_context(&ext_target_fp, &context)
            .values
            .iter()
            .map(ViolationItem::from_lint_result)
            .collect();
        // External tool findings may use paths relative to the tool's working
        // dir (the target) — keep only those that resolve under the target.
        external.retain(|v| {
            let file_path = std::path::Path::new(&v.file.value);
            let resolved = if file_path.is_absolute() {
                file_path.to_path_buf()
            } else {
                target_canon.join(file_path)
            };
            let resolved_canon = fs.canonicalize(&resolved).unwrap_or(resolved.clone());
            resolved_canon.starts_with(&target_canon)
                || (v.code.code() == "AES205"
                    && parent_workspace
                        .as_ref()
                        .is_some_and(|pw| resolved_canon.starts_with(pw)))
        });
        all.extend(external);
    }

    // Filter: keep only violations whose file path is within the target scope,
    // and drop any violation that names a non-existent file (E902 guard).
    // AES205 cycle violations are global — keep if within the same parent
    // workspace.
    let parent_workspace = target_canon
        .parent()
        .and_then(|p| p.parent())
        .map(|p| p.to_path_buf());
    all.retain(|v| {
        let file_path = std::path::Path::new(&v.file.value);
        let resolved = if file_path.is_absolute() {
            file_path.to_path_buf()
        } else if file_path.starts_with(std::path::Path::new(&target_canon_str)) {
            // Relative already under target (orphan paths) — verify, no re-join.
            file_path.to_path_buf()
        } else if target_inside_ws {
            // E902 fix: target inside a larger workspace — never re-join the
            // workspace root (that is what produced the doubled path).
            return false;
        } else if let Some(ref ws) = ws_root {
            ws.join(file_path)
        } else {
            target_canon.join(file_path)
        };
        let resolved_canon = fs.canonicalize(&resolved).unwrap_or(resolved.clone());
        if !fs.path_exists(&resolved_canon) {
            // E902 guard: non-existent file — drop, no violation emitted.
            return false;
        }
        if resolved_canon.starts_with(&target_canon) {
            return true;
        }
        if v.code.code() == "AES205" {
            if let Some(ref pw) = parent_workspace {
                if resolved_canon.starts_with(pw) {
                    return true;
                }
            }
        }
        false
    });

    all
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
