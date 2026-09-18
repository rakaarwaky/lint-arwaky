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

    // Validate member against discovered workspaces, then dispatch.
    let target_path = match opts.member.as_deref() {
        Some(m) => validate_member_path(&opts, &root, m)?,
        None => root.clone(),
    };
    let violations = match opts.scan_aggregates.as_ref() {
        Some(agg) => run_all_linters_in_process(&target_path, agg),
        None => run_all_linters_json(&target_path, opts.filesystem.as_ref()),
    };
    let violations = apply_filter(violations, &opts.filter);
    Ok(violations)
}

/// Resolve the member-scoped scan target, validating it against discovered
/// workspaces when a multi-project orchestrator is available.
fn validate_member_path(opts: &ScanOptions, root: &str, member: &str) -> Result<String, String> {
    if let Some(ref orchestrator) = opts.multi_project_orchestrator {
        let root_fp = FilePath::new(root.to_string()).map_err(|_| "invalid path".to_string())?;
        let workspaces = orchestrator.discover_workspaces(&root_fp);
        if !workspaces.is_empty() {
            let matched = workspaces.iter().any(|ws| {
                let ws_file = std::path::Path::new(&ws.path.value)
                    .file_name()
                    .map(|n| n.to_string_lossy())
                    .unwrap_or_default();
                ws_file.as_ref() == member || ws.path.value == *member
            });
            if !matched {
                return Err(format!("[error] no workspace member matching '{member}'"));
            }
        }
    }
    let member_path = std::path::Path::new(root).join(member);
    Ok(if member_path.exists() {
        member_path.to_string_lossy().to_string()
    } else {
        root.to_string()
    })
}

/// Apply the optional case-insensitive code filter to a violation list.
fn apply_filter(mut violations: Vec<ViolationItem>, filter: &Option<String>) -> Vec<ViolationItem> {
    if let Some(filter_str) = filter {
        let filter_upper = filter_str.to_uppercase();
        violations.retain(|v| v.code.code().contains(&filter_upper));
    }
    violations
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

    // W10: build the in-process file index directly from the scan target so
    // member-scoped scans (e.g. `workspaces-bad/crates`) enumerate exactly the
    // files the subprocess linters would see, instead of the parent workspace
    // that `build_file_index_with_ignored` resolves to (which would mix or drop
    // sibling members and produce doubled paths → E902 / missing codes).
    let scan_root = target_canon.clone();

    let root_fp = FilePath::new(scan_root.to_string_lossy().to_string()).unwrap_or_default();

    let ignored = agg
        .config
        .ignored_paths(&root_fp)
        .values
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>();

    // Single-file target: build a one-entry index and run the auditors on it.
    if scan_root.is_file() {
        return run_single_file_scan(&fs, &scan_root, agg, &root_fp, &ignored);
    }

    // Discover source files under the target, matching the per-linter commands
    // that call `discover_source_files(target, &ignored)` internally. Also
    // cache them in the filesystem's file index + import map so
    // `import_list()` / `resolved_import_list()` used by the in-process import
    // auditor are populated for this target.
    //
    // Populate the filesystem's import cache + file index. This walks the
    // discovered workspace root (the parent when the target is a member),
    // which is what the orphan auditor and the import auditor's
    // `import_list()` / `resolved_import_list()` expect — but the entries we
    // pass to quality/role/import/naming below are scoped to the target only,
    // so per-file rules never see sibling members (no doubled-path E902).
    // Skip any config-provided ignore pattern that would exclude the scan
    // target itself (a repo-root scan must always see its own tree), then
    // add the fixture/test-workspace dir names so the workspace-root index
    // walk never picks up `workspaces-bad`/`workspaces-good` as parent
    // source. An explicit scan of a fixture dir is unaffected: the walk
    // roots at that dir and the name filter only applies to entries of the
    // scan target's siblings.
    // An explicit scan of a fixture dir must not ignore the target itself —
    // `build_file_index_with_ignored` uses ignore::WalkBuilder, so an ignore
    // entry matching the scan root's own name suppresses its entire subtree.
    let fixture_names: [&str; 2] = ["workspaces-bad", "workspaces-good"];
    let scan_root_is_fixture = scan_root
        .file_name()
        .and_then(|f| f.to_str())
        .is_some_and(|f| fixture_names.contains(&f));
    let build_ignored: Vec<String> = ignored
        .iter()
        .filter(|p| !p.starts_with('/'))
        .cloned()
        .chain(
            fixture_names
                .iter()
                .filter(|_name| !scan_root_is_fixture)
                .map(|name| name.to_string()),
        )
        .collect();
    fs.build_file_index_with_ignored(std::path::Path::new(&scan_root), &build_ignored);

    let discovered = discover_lintable_files(&fs, &scan_root, &ignored);
    let entries = build_entries(&fs, &discovered);
    let import_map = build_import_map(&fs, &entries);

    let parent_workspace = target_canon
        .parent()
        .and_then(|p| p.parent())
        .map(|p| p.to_path_buf());

    let mut all: Vec<ViolationItem> = Vec::new();

    all.extend(
        agg.quality
            .run_analysis_with_entries(&entries)
            .iter()
            .map(ViolationItem::from_lint_result),
    );
    all.extend(
        agg.role
            .run_audit_with_entries(&entries)
            .iter()
            .map(ViolationItem::from_lint_result),
    );
    // Workspace-wide import map so AES201/202/203/205 see cross-member imports
    // (the dispatcher's fs instance differs from the import orchestrator's own).
    all.extend(
        agg.import
            .run_audit_with_entries_and_imports(&entries, &import_map)
            .iter()
            .map(ViolationItem::from_lint_result),
    );
    all.extend(
        agg.naming
            .run_audit_with_entries(&entries)
            .iter()
            .map(ViolationItem::from_lint_result),
    );
    let (_graph_ctx, orphan_violations) = agg.orphan.scan_orphans(&root_fp, &ignored);
    all.extend(
        orphan_violations
            .iter()
            .map(ViolationItem::from_lint_result),
    );

    // External — adapters run on the target *as given* (relative paths resolve
    // against the process CWD, exactly like the spawned linters did).
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

    // Keep only violations that resolve to a real file under the scan scope
    // (or, for AES205 cycles / AES5xx orphans, within the parent workspace).
    all.retain(|v| {
        let file_path = std::path::Path::new(&v.file.value);
        let resolved = if file_path.is_absolute() {
            file_path.to_path_buf()
        } else {
            target_canon.join(file_path)
        };
        let resolved_canon = fs.canonicalize(&resolved).unwrap_or(resolved.clone());
        if !fs.path_exists(&resolved_canon) {
            return false; // E902 guard: non-existent file — drop, no violation.
        }
        let in_target = resolved_canon.starts_with(&target_canon);
        let in_parent_ws = parent_workspace
            .as_ref()
            .is_some_and(|pw| resolved_canon.starts_with(pw));
        in_target
            || (v.code.code() == "AES205" && in_parent_ws)
            || (v.code.code().starts_with("AES5") && in_parent_ws)
    });

    all
}

/// Run all 6 linters on a single-file target, returning in-scope violations.
fn run_single_file_scan(
    fs: &Arc<dyn IFilesystemAggregate>,
    scan_root: &std::path::Path,
    agg: &ScanAggregates,
    root_fp: &shared::common::taxonomy_path_vo::FilePath,
    ignored: &[String],
) -> Vec<ViolationItem> {
    let content = fs
        .read_lintable_file(&scan_root.to_string_lossy())
        .unwrap_or_default();
    let extension = scan_root
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_string();
    let language = match fs.detect_language_from_path(scan_root.to_string_lossy().as_ref()) {
        shared::common::taxonomy_config_language_vo::ConfigLanguage::Rust => {
            shared::filesystem::taxonomy_filesystem_vo::Language::Rust
        }
        shared::common::taxonomy_config_language_vo::ConfigLanguage::Python => {
            shared::filesystem::taxonomy_filesystem_vo::Language::Python
        }
        shared::common::taxonomy_config_language_vo::ConfigLanguage::TypeScript => {
            shared::filesystem::taxonomy_filesystem_vo::Language::TypeScript
        }
    };
    let mut entries = vec![shared::filesystem::taxonomy_filesystem_vo::FileEntry {
        path: scan_root.to_path_buf(),
        extension,
        language,
        size: content.len() as u64,
        content: content.clone(),
        parse_ok: !content.is_empty(),
        parse_metadata: None,
    }];
    fs.parse_all(&mut entries);
    let import_map: std::collections::HashMap<
        String,
        Vec<shared::filesystem::taxonomy_filesystem_vo::ImportEntry>,
    > = std::collections::HashMap::new();
    let mut all: Vec<ViolationItem> = Vec::new();
    all.extend(
        agg.quality
            .run_analysis_with_entries(&entries)
            .iter()
            .map(ViolationItem::from_lint_result),
    );
    all.extend(
        agg.role
            .run_audit_with_entries(&entries)
            .iter()
            .map(ViolationItem::from_lint_result),
    );
    all.extend(
        agg.import
            .run_audit_with_entries_and_imports(&entries, &import_map)
            .iter()
            .map(ViolationItem::from_lint_result),
    );
    all.extend(
        agg.naming
            .run_audit_with_entries(&entries)
            .iter()
            .map(ViolationItem::from_lint_result),
    );
    let (_graph_ctx, orphan_violations) = agg.orphan.scan_orphans(root_fp, ignored);
    all.extend(
        orphan_violations
            .iter()
            .map(ViolationItem::from_lint_result),
    );
    // Drop violations naming files outside the target.
    all.retain(|v| {
        let p = std::path::Path::new(&v.file.value);
        p.is_absolute() && p.starts_with(scan_root)
    });
    all
}

/// Discover lintable source files under `scan_root` matching the index walk:
/// under a workspace root only member dirs carry source; elsewhere member-dir
/// names and fixture dirs are skipped. Returns the discovered file paths.
fn discover_lintable_files(
    fs: &Arc<dyn IFilesystemAggregate>,
    scan_root: &std::path::Path,
    ignored: &[String],
) -> Vec<String> {
    // Discover the target's own files with the config ignore list (a dir
    // named `workspaces-bad` never matches a file pattern, so fixture
    // targets are unaffected by the index build's fixture-dir exclusion).
    let mut discovered = fs.discover_source_files(scan_root, ignored);
    // Recurse into subdirs so nested source trees (crates/<name>/src/*) are
    // fully covered, matching what the subprocess linters walk.
    let is_ws_root = ["crates", "packages", "modules"]
        .iter()
        .any(|name| scan_root.join(name).is_dir());
    let mut skip_dirs: std::collections::HashSet<&str> =
        shared::common::taxonomy_default_constant::DEFAULT_IGNORED_PATHS
            .iter()
            .copied()
            .collect();
    // A member-scoped scan never leaks into sibling members (E902 /
    // missing-code bug).
    if !is_ws_root {
        skip_dirs.insert("crates");
        skip_dirs.insert("packages");
        skip_dirs.insert("modules");
    }
    // Fixture dirs are skipped only when they are NOT the scan target itself;
    // a `check .` of the repo root must never lint `workspaces-bad/good`.
    let scan_root_is_fixture = scan_root
        .file_name()
        .and_then(|f| f.to_str())
        .is_some_and(|f| f == "workspaces-bad" || f == "workspaces-good");
    if !scan_root_is_fixture {
        skip_dirs.insert("workspaces-bad");
        skip_dirs.insert("workspaces-good");
    }
    let member_names: &[&str] = &["crates", "packages", "modules"];
    let mut queue: Vec<(std::path::PathBuf, usize)> = vec![(scan_root.to_path_buf(), 0)];
    let mut seen: std::collections::HashSet<std::path::PathBuf> = std::collections::HashSet::new();
    // `depth` tracks BFS level: 0 = scan root, 1 = inside a top-level member
    // dir (crates/packages/modules), 2+ = member subdirs (the actual member
    // names, e.g. code_analysis). At a workspace root the depth-0 level is
    // gated to member dirs only; deeper levels enter every subdir.
    while let Some((dir, depth)) = queue.pop() {
        let gate_members = is_ws_root && depth == 0;
        for entry in fs.scan_directory(dir.as_path()) {
            let entry_path = std::path::Path::new(&entry);
            if !entry_path.is_dir() {
                continue;
            }
            let name = entry_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("");
            if skip_dirs.contains(name) {
                continue;
            }
            let is_member_dir = member_names.contains(&name);
            if gate_members && !is_member_dir {
                continue;
            }
            if !seen.insert(entry_path.to_path_buf()) {
                continue;
            }
            discovered.extend(fs.discover_source_files(entry_path, ignored));
            let next_depth = if is_ws_root && is_member_dir {
                1
            } else {
                depth + 1
            };
            queue.push((entry_path.to_path_buf(), next_depth));
        }
    }
    discovered
}

/// Build parsed `FileEntry`s for the discovered file paths, running the
/// tree-sitter parse so `parse_metadata` is populated for the auditors.
fn build_entries(
    fs: &Arc<dyn IFilesystemAggregate>,
    discovered: &[String],
) -> Vec<shared::filesystem::taxonomy_filesystem_vo::FileEntry> {
    let mut entries: Vec<shared::filesystem::taxonomy_filesystem_vo::FileEntry> = Vec::new();
    for file_path in discovered {
        let content = fs.read_lintable_file(file_path).unwrap_or_default();
        let extension = std::path::Path::new(file_path)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_string();
        let language = fs.detect_language_from_path(file_path);
        let language = match language {
            shared::common::taxonomy_config_language_vo::ConfigLanguage::Rust => {
                shared::filesystem::taxonomy_filesystem_vo::Language::Rust
            }
            shared::common::taxonomy_config_language_vo::ConfigLanguage::Python => {
                shared::filesystem::taxonomy_filesystem_vo::Language::Python
            }
            shared::common::taxonomy_config_language_vo::ConfigLanguage::TypeScript => {
                shared::filesystem::taxonomy_filesystem_vo::Language::TypeScript
            }
        };
        entries.push(shared::filesystem::taxonomy_filesystem_vo::FileEntry {
            path: std::path::PathBuf::from(file_path),
            extension,
            language,
            size: content.len() as u64,
            content: content.clone(),
            parse_ok: !content.is_empty(),
            parse_metadata: None,
        });
    }
    // Fills `parse_metadata` (needed for AES203 unused-import detection) and
    // rewrites the parser's import cache with this target's scoped imports.
    fs.parse_all(&mut entries);
    entries
}

/// Merge the out-of-scope import snapshot with in-scope parser imports into
/// a workspace-wide import map keyed by absolute source path, so
/// AES201/202/203/205 see cross-member imports.
fn build_import_map(
    fs: &Arc<dyn IFilesystemAggregate>,
    entries: &[shared::filesystem::taxonomy_filesystem_vo::FileEntry],
) -> std::collections::HashMap<String, Vec<shared::filesystem::taxonomy_filesystem_vo::ImportEntry>>
{
    let ws_snapshot = fs.import_list_snapshot();
    let scoped_keys: std::collections::HashSet<&std::path::Path> =
        entries.iter().map(|e| e.path.as_path()).collect();
    let out_of_scope: Vec<_> = ws_snapshot
        .into_iter()
        .filter(|e| !scoped_keys.contains(e.source_file.as_path()))
        .collect();

    let mut import_map: std::collections::HashMap<
        String,
        Vec<shared::filesystem::taxonomy_filesystem_vo::ImportEntry>,
    > = std::collections::HashMap::new();
    for entry in entries {
        for imp in fs.imports_for(&entry.path) {
            import_map
                .entry(entry.path.to_string_lossy().to_string())
                .or_default()
                .push(imp);
        }
    }
    for entry in out_of_scope {
        let key = entry.source_file.to_string_lossy().to_string();
        import_map.entry(key).or_default().push(entry);
    }
    import_map
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
