// PURPOSE: SurfaceCheckAction — check/scan business logic, no formatting.
//
// In-process mode (W10): when `scan_aggregates` is provided, all 6 linters run
// in-process through their aggregate entry points. Subprocess self-invocation
// remains the fallback when aggregates are absent (`scan_aggregates: None`).
use shared::common::FilePath;
use shared::common::ViolationItem;
use shared::config_system::{ConfigRequest, IConfigOrchestratorAggregate};
use shared::external_lint::IExternalLintAggregate;
use shared::filesystem::FilesystemRequest;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use shared::filesystem::contract_filesystem_protocol::IParserProtocol;
use shared::filesystem::contract_filesystem_protocol::IWorkspaceProtocol;
use shared::import_rules::IImportRunnerAggregate;
use shared::import_rules::taxonomy_import_request::ImportRequest;
use shared::naming_rules::INamingRunnerAggregate;
use shared::naming_rules::taxonomy_naming_request::NamingRequest;
use shared::orphan_rules::IOrphanAggregate;
use shared::orphan_rules::OrphanRequest;
use shared::quality_rules::CodeAnalysisRequest;
use shared::quality_rules::ICodeAnalysisAggregate;
use shared::role_rules::IRoleRunnerAggregate;
use shared::role_rules::taxonomy_role_request::RoleRequest;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Arc;

/// Capability seams exposed alongside the filesystem aggregate, so callers can
/// dispatch individual protocol operations without leaking the aggregate layer.
#[derive(Clone)]
pub struct FilesystemSeam {
    pub io: Arc<dyn IFileSystemIOProtocol>,
    pub workspace: Arc<dyn IWorkspaceProtocol>,
    pub parser: Arc<dyn IParserProtocol>,
    pub aggregate: Arc<dyn IFilesystemAggregate>,
}

/// Bundles the 6 scan aggregates + config source + filesystem seam so that
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
    /// Provides raw protocol seams + aggregate per scan run.
    pub fs_seam: Arc<FilesystemSeam>,
}

pub struct ScanOptions {
    pub path: Option<FilePath>,
    pub multi_project_orchestrator: Option<Arc<dyn IConfigOrchestratorAggregate>>,
    pub filter: Option<String>,
    pub member: Option<String>,
    pub filesystem: Arc<FilesystemSeam>,
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
    if !opts.filesystem.io.path_exists(std::path::Path::new(&root)) {
        return Err(format!("Error: path '{}' does not exist", root));
    }

    // W10: when aggregates are wired, resolve the target to an absolute path so
    // in-process linters scan the intended scope. Subprocess fallback keeps the
    // raw path (its per-linter normalization differs).
    let root = if opts.scan_aggregates.is_some() {
        canonicalize_scan_root(&opts.filesystem.io, &root)
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

/// Canonicalize a scan root (falling back to the raw path when the filesystem
/// cannot canonicalize it, e.g. the path does not exist yet).
fn canonicalize_scan_root(io: &Arc<dyn IFileSystemIOProtocol>, root: &str) -> String {
    io.canonicalize(std::path::Path::new(root))
        .unwrap_or_else(|_| PathBuf::from(root))
        .to_string_lossy()
        .to_string()
}

/// Resolve the member-scoped scan target, validating it against discovered
/// workspaces when a multi-project orchestrator is available.
fn validate_member_path(opts: &ScanOptions, root: &str, member: &str) -> Result<String, String> {
    if let Some(ref orchestrator) = opts.multi_project_orchestrator {
        let root_fp = FilePath::new(root.to_string()).map_err(|_| "invalid path".to_string())?;
        let workspaces = orchestrator
            .execute(ConfigRequest::discover_workspaces(&root_fp))
            .into_workspaces();
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
pub fn is_member_path(path: &FilePath, ws: &dyn IWorkspaceProtocol) -> bool {
    ws.is_member_path(path)
}

/// Run all 6 linters via subprocesses for a given path; return violations.
pub fn collect_scan_json(path: &str, seam: &FilesystemSeam) -> Result<Vec<ViolationItem>, String> {
    if !seam.io.path_exists(std::path::Path::new(path)) {
        return Err(format!("Error: path '{}' does not exist", path));
    }
    Ok(run_all_linters_json(path, seam))
}

/// Default check: subprocess JSON scan of all linters.
pub fn collect_default_check(
    project_root: &str,
    seam: &FilesystemSeam,
) -> Result<Vec<ViolationItem>, String> {
    collect_scan_json(project_root, seam)
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
    let seam = agg.fs_seam.clone();

    let target = std::path::Path::new(path);
    let target_canon = seam
        .io
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
        .execute(ConfigRequest::ignored_paths(&root_fp))
        .into_patterns()
        .values
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>();

    // Single-file target: build a one-entry index and run the auditors on it.
    if scan_root.is_file() {
        return run_single_file_scan(&seam, &scan_root, agg, &root_fp, &ignored);
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
    seam.aggregate
        .execute(FilesystemRequest::build_file_index_with_ignored(
            std::path::Path::new(&scan_root),
            &build_index_ignored(&ignored, &scan_root),
        ));

    let discovered = discover_lintable_files(&seam, &scan_root, &ignored);
    let entries = build_entries(&seam, &discovered);
    let import_map = build_import_map(&seam, &entries);

    let parent_workspace = target_canon
        .parent()
        .and_then(|p| p.parent())
        .map(|p| p.to_path_buf());
    // AES5xx orphan (and AES205 cycle) violation paths are relative to the
    // workspace root the orphan scanner resolved — for a member-dir scan target
    // that is one level up (the target's parent), not two.
    let workspace_root = target_canon.parent().map(|p| p.to_path_buf());

    let mut all: Vec<ViolationItem> = Vec::new();

    all.extend(
        agg.quality
            .execute(CodeAnalysisRequest::run_analysis(&entries))
            .into_violations()
            .iter()
            .map(ViolationItem::from_lint_result),
    );
    all.extend(
        agg.role
            .execute(RoleRequest::audit(&entries))
            .into_violations()
            .iter()
            .map(ViolationItem::from_lint_result),
    );
    // Workspace-wide import map so AES201/202/203/205 see cross-member imports
    // (the dispatcher's fs instance differs from the import orchestrator's own).
    all.extend(
        agg.import
            .execute(ImportRequest::audit_with_entries_and_imports(
                &entries,
                &import_map,
            ))
            .into_violations()
            .iter()
            .map(ViolationItem::from_lint_result),
    );
    all.extend(
        agg.naming
            .execute(NamingRequest::audit(&entries))
            .into_violations()
            .iter()
            .map(ViolationItem::from_lint_result),
    );
    let (_graph_ctx, orphan_violations) = agg
        .orphan
        .execute(OrphanRequest::scan(
            &root_fp,
            &shared::common::taxonomy_common_vo::PatternList::new(ignored.clone()),
        ))
        .into_scan_outcome();
    all.extend(
        orphan_violations
            .iter()
            .map(ViolationItem::from_lint_result),
    );

    // External — adapters run on the target *as given* (relative paths resolve
    // against the process CWD, exactly like the spawned linters did).
    // When the scan target is a workspace root (has crates/packages/modules
    // siblings), gate external violations to those member dirs only so root-
    // level packaging files (setup.py at the workspace root) are not picked
    // up — matching the gating the AES-quality / role / import / naming /
    // orphan linters apply via `discover_lintable_files`.
    let member_dirs: Vec<std::path::PathBuf> = ["crates", "packages", "modules"]
        .into_iter()
        .map(|n| target_canon.join(n))
        .filter(|p| p.is_dir())
        .collect();
    let member_refs: Vec<&std::path::Path> = member_dirs.iter().map(|p| p.as_path()).collect();
    let member_scope: Option<&[&std::path::Path]> = if member_refs.is_empty() {
        None
    } else {
        Some(&member_refs)
    };
    let ext_target_fp = FilePath::new(path.to_string()).unwrap_or_default();
    {
        let ext_files = seam
            .aggregate
            .execute(FilesystemRequest::discover_files(std::path::Path::new(
                &target_canon_str,
            )))
            .into_paths();
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
            .execute(
                shared::external_lint::ExternalLintRequest::scan_all_with_context(
                    &ext_target_fp,
                    &context,
                ),
            )
            .into_violations()
            .values
            .iter()
            .map(ViolationItem::from_lint_result)
            .collect();
        external.retain(|v| {
            external_violation_in_scope(
                v,
                &seam,
                &target_canon,
                parent_workspace.as_deref(),
                member_scope,
            )
        });
        all.extend(external);
    }

    // Keep only violations that resolve to a real file under the scan scope
    // (or, for AES205 cycles / AES5xx orphans, within the parent workspace).
    // At a workspace root the scope is the member dirs only, so a root-level
    // packaging file (e.g. setup.py) never surfaces from any linter.
    all.retain(|v| {
        violation_in_scan_scope(
            v,
            &seam,
            &target_canon,
            parent_workspace.as_deref(),
            workspace_root.as_deref(),
            member_scope,
        )
    });

    all
}

/// Whether an external-lint violation falls inside the scan scope: its file
/// resolves under the target, or it is an AES205 cycle resolved under the
/// parent workspace. When the target is a workspace root (`member_dirs` is
/// `Some`), a violation must additionally resolve inside one of the member
/// dirs — the AES linters only lint `crates`/`packages`/`modules`, so root
/// level files (e.g. a packaging `setup.py`) are out of scope for the external
/// adapters too.
fn external_violation_in_scope(
    v: &ViolationItem,
    seam: &FilesystemSeam,
    target_canon: &std::path::Path,
    parent_workspace: Option<&std::path::Path>,
    member_dirs: Option<&[&std::path::Path]>,
) -> bool {
    let resolved_canon = resolve_violation_path(v, seam, target_canon, parent_workspace);
    if let Some(members) = member_dirs {
        return members
            .iter()
            .any(|m| resolved_canon.starts_with(m) && seam.io.path_exists(&resolved_canon));
    }
    resolved_canon.starts_with(target_canon)
        || (v.code.code() == "AES205"
            && parent_workspace.is_some_and(|pw| resolved_canon.starts_with(pw)))
}

/// Whether a violation stays in the scan scope: the file exists under the
/// target, or it is an AES205 cycle / AES5xx orphan under the parent workspace.
/// Non-existent files are dropped so doubled/stale paths never surface as E902.
/// When `member_dirs` is `Some` (workspace root target), the violation must
/// additionally resolve inside one of the member dirs — root-level packaging
/// files (e.g. setup.py) are out of scope for every linter.
fn violation_in_scan_scope(
    v: &ViolationItem,
    seam: &FilesystemSeam,
    target_canon: &std::path::Path,
    parent_workspace: Option<&std::path::Path>,
    workspace_root: Option<&std::path::Path>,
    member_dirs: Option<&[&std::path::Path]>,
) -> bool {
    let resolved_canon = resolve_violation_path(v, seam, target_canon, workspace_root);
    if !seam.io.path_exists(&resolved_canon) {
        return false; // E902 guard: non-existent file — drop, no violation.
    }
    let in_target = resolved_canon.starts_with(target_canon);
    let in_parent_ws = parent_workspace.is_some_and(|pw| resolved_canon.starts_with(pw));
    // At a workspace root, scope to member dirs only so root-level packaging
    // files are never reported — matching what the AES linters lint.
    if let Some(members) = member_dirs {
        return members.iter().any(|m| resolved_canon.starts_with(m));
    }
    in_target
        || (v.code.code() == "AES205" && in_parent_ws)
        || (v.code.code().starts_with("AES5") && in_parent_ws)
}

/// Resolve a violation's file path to a canonical absolute path under the
/// target (relative paths join the target; absolute paths are canonicalized as
/// is, falling back to the path itself when canonicalization fails).
///
/// Orphan (AES5xx) and cycle (AES205) violations carry paths relative to the
/// workspace root, which is the target's parent when a member directory is the
/// scan target. Those do not resolve under the target, so a relative path that
/// misses there is retried against the parent workspace before being kept.
fn resolve_violation_path(
    v: &ViolationItem,
    seam: &FilesystemSeam,
    target_canon: &std::path::Path,
    workspace_root: Option<&std::path::Path>,
) -> std::path::PathBuf {
    let file_path = std::path::Path::new(&v.file.value);
    let resolved = if file_path.is_absolute() {
        file_path.to_path_buf()
    } else {
        let under_target = target_canon.join(file_path);
        let exists_under_target = seam.io.path_exists(&under_target);
        match workspace_root {
            Some(pw) if !exists_under_target => pw.join(file_path),
            _ => under_target,
        }
    };
    seam.io.canonicalize(&resolved).unwrap_or(resolved)
}

/// Run all 6 linters on a single-file target, returning in-scope violations.
fn run_single_file_scan(
    seam: &FilesystemSeam,
    scan_root: &std::path::Path,
    agg: &ScanAggregates,
    root_fp: &shared::common::taxonomy_path_vo::FilePath,
    ignored: &[String],
) -> Vec<ViolationItem> {
    let content = seam
        .aggregate
        .execute(FilesystemRequest::read_lintable_file(
            &scan_root.to_string_lossy(),
        ))
        .into_content_opt()
        .unwrap_or_default();
    let extension = scan_root
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_string();
    let language = match seam
        .workspace
        .detect_language_from_path(scan_root.to_string_lossy().as_ref())
    {
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
    seam.parser.parse_all(&mut entries);
    let import_map: std::collections::HashMap<
        String,
        Vec<shared::filesystem::taxonomy_filesystem_vo::ImportEntry>,
    > = std::collections::HashMap::new();
    let mut all: Vec<ViolationItem> = Vec::new();
    all.extend(
        agg.quality
            .execute(CodeAnalysisRequest::run_analysis(&entries))
            .into_violations()
            .iter()
            .map(ViolationItem::from_lint_result),
    );
    all.extend(
        agg.role
            .execute(RoleRequest::audit(&entries))
            .into_violations()
            .iter()
            .map(ViolationItem::from_lint_result),
    );
    all.extend(
        agg.import
            .execute(ImportRequest::audit_with_entries_and_imports(
                &entries,
                &import_map,
            ))
            .into_violations()
            .iter()
            .map(ViolationItem::from_lint_result),
    );
    all.extend(
        agg.naming
            .execute(NamingRequest::audit(&entries))
            .into_violations()
            .iter()
            .map(ViolationItem::from_lint_result),
    );
    let (_graph_ctx, orphan_violations) = agg
        .orphan
        .execute(OrphanRequest::scan(
            root_fp,
            &shared::common::taxonomy_common_vo::PatternList::new(ignored.to_vec()),
        ))
        .into_scan_outcome();
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
    seam: &FilesystemSeam,
    scan_root: &std::path::Path,
    ignored: &[String],
) -> Vec<String> {
    // Discover the target's own files with the config ignore list (a dir
    // named `workspaces-bad` never matches a file pattern, so fixture
    // targets are unaffected by the index build's fixture-dir exclusion).
    let mut discovered = seam
        .aggregate
        .execute(FilesystemRequest::discover_source_files(scan_root, ignored))
        .into_paths();
    // Recurse into subdirs so nested source trees (crates/<name>/src/*) are
    // fully covered, matching what the subprocess linters walk.
    let is_ws_root = ["crates", "packages", "modules"]
        .iter()
        .any(|name| scan_root.join(name).is_dir());
    let skip_dirs = build_skip_dirs(is_ws_root, scan_root);
    bfs_enter_subdirs(
        seam,
        scan_root,
        &skip_dirs,
        is_ws_root,
        ignored,
        &mut discovered,
    );
    discovered
}

/// Build the set of directory names the BFS skips: always `DEFAULT_IGNORED_PATHS`
/// plus member dirs when not at a workspace root, plus fixture dirs when they
/// are not the scan target itself (a `check .` of the repo root must never
/// lint `workspaces-bad/good`, but a direct fixture scan must lint its files).
fn build_skip_dirs(
    is_ws_root: bool,
    scan_root: &std::path::Path,
) -> std::collections::HashSet<&'static str> {
    let mut skip_dirs: std::collections::HashSet<&str> =
        shared::common::taxonomy_default_constant::DEFAULT_IGNORED_PATHS
            .iter()
            .copied()
            .collect();
    if !is_ws_root {
        skip_dirs.insert("crates");
        skip_dirs.insert("packages");
        skip_dirs.insert("modules");
    }
    let scan_root_is_fixture = scan_root
        .file_name()
        .and_then(|f| f.to_str())
        .is_some_and(|f| f == "workspaces-bad" || f == "workspaces-good");
    if !scan_root_is_fixture {
        skip_dirs.insert("workspaces-bad");
        skip_dirs.insert("workspaces-good");
    }
    skip_dirs
}

/// Breadth-first walk into subdirectories of `scan_root`, collecting lintable
/// source files. At a workspace root the depth-0 level is gated to member dirs
/// only; deeper levels enter every subdir. `discovered` is extended in place.
fn bfs_enter_subdirs(
    seam: &FilesystemSeam,
    scan_root: &std::path::Path,
    skip_dirs: &std::collections::HashSet<&str>,
    is_ws_root: bool,
    ignored: &[String],
    discovered: &mut Vec<String>,
) {
    let member_names: &[&str] = &["crates", "packages", "modules"];
    let mut queue: Vec<(std::path::PathBuf, usize)> = vec![(scan_root.to_path_buf(), 0)];
    let mut seen: std::collections::HashSet<std::path::PathBuf> = std::collections::HashSet::new();
    while let Some((dir, depth)) = queue.pop() {
        let gate_members = is_ws_root && depth == 0;
        for entry in seam
            .aggregate
            .execute(FilesystemRequest::scan_directory(dir.as_path()))
            .into_paths()
        {
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
            discovered.extend(
                seam.aggregate
                    .execute(FilesystemRequest::discover_source_files(
                        entry_path, ignored,
                    ))
                    .into_paths(),
            );
            let next_depth = if is_ws_root && is_member_dir {
                1
            } else {
                depth + 1
            };
            queue.push((entry_path.to_path_buf(), next_depth));
        }
    }
}

/// Build the ignore list for the in-process file index: config ignore entries
/// that do not suppress the scan target itself (absolute patterns are dropped —
/// a repo-root scan must always see its own tree), plus the fixture dir names
/// so the index walk never picks up `workspaces-bad`/`workspaces-good` as
/// sibling source. An explicit fixture scan keeps its own tree: the fixture
/// names are omitted then, since an ignore entry matching the scan root's own
/// name would suppress the entire subtree.
fn build_index_ignored(ignored: &[String], scan_root: &std::path::Path) -> Vec<String> {
    let fixture_names: [&str; 2] = ["workspaces-bad", "workspaces-good"];
    let scan_root_is_fixture = scan_root
        .file_name()
        .and_then(|f| f.to_str())
        .is_some_and(|f| fixture_names.contains(&f));
    ignored
        .iter()
        .filter(|p| !p.starts_with('/'))
        .cloned()
        .chain(
            fixture_names
                .iter()
                .filter(|_name| !scan_root_is_fixture)
                .map(|name| name.to_string()),
        )
        .collect()
}

/// Build parsed `FileEntry`s for the discovered file paths, running the
/// tree-sitter parse so `parse_metadata` is populated for the auditors.
fn build_entries(
    seam: &FilesystemSeam,
    discovered: &[String],
) -> Vec<shared::filesystem::taxonomy_filesystem_vo::FileEntry> {
    let mut entries: Vec<shared::filesystem::taxonomy_filesystem_vo::FileEntry> = Vec::new();
    for file_path in discovered {
        let content = seam
            .aggregate
            .execute(FilesystemRequest::read_lintable_file(file_path))
            .into_content_opt()
            .unwrap_or_default();
        let extension = std::path::Path::new(file_path)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_string();
        let language = seam.workspace.detect_language_from_path(file_path);
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
    seam.parser.parse_all(&mut entries);
    entries
}

/// Merge the out-of-scope import snapshot with in-scope parser imports into
/// a workspace-wide import map keyed by absolute source path, so
/// AES201/202/203/205 see cross-member imports.
fn build_import_map(
    seam: &FilesystemSeam,
    entries: &[shared::filesystem::taxonomy_filesystem_vo::FileEntry],
) -> std::collections::HashMap<String, Vec<shared::filesystem::taxonomy_filesystem_vo::ImportEntry>>
{
    let ws_snapshot = seam
        .aggregate
        .execute(FilesystemRequest::ImportListSnapshot)
        .into_imports();
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
        for imp in seam.parser.imports_for(&entry.path) {
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
fn run_all_linters_json(path: &str, seam: &FilesystemSeam) -> Vec<ViolationItem> {
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
    let target_canonical = seam.io.canonicalize(std::path::Path::new(path)).ok();
    // Detect workspace root for resolving relative paths from orphan scan
    // (orphan scan returns paths like "crates/calculator/src/foo.rs" relative to workspace root)
    let ws_root = seam
        .workspace
        .workspace_root(&FilePath::new(path.to_string()).unwrap_or_default());
    normalize_violation_paths(&mut all, seam, &target_canonical, &ws_root);

    // Filter: only keep violations whose file path is within the target directory.
    // Exception: AES205 cycle violations are global — keep them if the file is
    // within the same parent workspace (e.g., workspaces-bad/).
    if let Some(canonical_target) = &target_canonical {
        let parent_workspace = canonical_target
            .parent()
            .and_then(|p| p.parent())
            .map(|p| p.to_path_buf());
        all.retain(|v| {
            json_violation_in_target(v, seam, canonical_target, parent_workspace.as_deref())
        });
    }

    all
}

/// Rewrite each violation's relative file path to an absolute, canonicalized
/// path, trying (in order) the workspace root, the process CWD, the scan
/// target, then the target's parent. Absolute paths are left untouched.
fn normalize_violation_paths(
    violations: &mut [ViolationItem],
    seam: &FilesystemSeam,
    target_canonical: &Option<std::path::PathBuf>,
    ws_root: &Option<std::path::PathBuf>,
) {
    let cwd = std::env::current_dir().ok();
    let target_parent = target_canonical.as_deref().and_then(|t| t.parent());
    for v in violations.iter_mut() {
        if std::path::Path::new(&v.file.value).is_absolute() {
            continue;
        }
        let file_path = std::path::Path::new(&v.file.value);
        let bases: [Option<&std::path::Path>; 4] = [
            ws_root.as_deref(),
            cwd.as_deref(),
            target_canonical.as_deref(),
            target_parent,
        ];
        for base in bases.into_iter().flatten() {
            if let Ok(canon) = seam.io.canonicalize(&base.join(file_path)) {
                v.file = FilePath::new(canon.to_string_lossy().to_string())
                    .unwrap_or_else(|_| v.file.clone());
                break;
            }
        }
    }
}

/// Whether a subprocess-collected violation stays inside the scan target
/// (or, for AES205 cycles, inside the parent workspace).
fn json_violation_in_target(
    v: &ViolationItem,
    seam: &FilesystemSeam,
    canonical_target: &std::path::Path,
    parent_workspace: Option<&std::path::Path>,
) -> bool {
    let file_path = std::path::Path::new(&v.file.value);
    // Always retain AES205 cycle violations if within the same parent workspace
    if v.code.code() == "AES205" {
        if let Some(pw) = parent_workspace {
            if let Ok(canonical) = seam.io.canonicalize(file_path) {
                if canonical.starts_with(pw) {
                    return true;
                }
            }
        }
    }
    if let Ok(canonical) = seam.io.canonicalize(file_path) {
        return canonical.starts_with(canonical_target);
    }
    if let Ok(cwd) = std::env::current_dir() {
        let joined = cwd.join(file_path);
        let cwd_joined = seam.io.canonicalize(&joined).unwrap_or(joined);
        if cwd_joined.starts_with(canonical_target) {
            return true;
        }
    }
    if let Ok(target_joined) = seam.io.canonicalize(&canonical_target.join(file_path)) {
        return target_joined.starts_with(canonical_target);
    }
    false
}
