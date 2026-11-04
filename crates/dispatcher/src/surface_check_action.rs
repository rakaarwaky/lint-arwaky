// PURPOSE: SurfaceCheckAction — check/scan business logic, no formatting.
//
// In-process mode (W10): when `scan_aggregates` is provided, all 7 linters run
// in-process through their aggregate entry points. Subprocess self-invocation
// remains the fallback when aggregates are absent (`scan_aggregates: None`).
use shared_common::FilePath;
use shared_common::ViolationItem;
use shared_config_system::{ConfigRequest, IConfigOrchestratorAggregate};
use shared_doc_rules::IDocRunnerAggregate;
use shared_external_lint::IExternalLintAggregate;
use shared_filesystem::FilesystemRequest;
use shared_filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared_import_rules::IImportRunnerAggregate;
use shared_import_rules::taxonomy_import_rules_request::ImportRequest;
use shared_naming_rules::INamingRunnerAggregate;
use shared_naming_rules::taxonomy_naming_rules_request::NamingRequest;
use shared_orphan_rules::IOrphanAggregate;
use shared_orphan_rules::OrphanRequest;
use shared_quality_rules::CodeAnalysisRequest;
use shared_quality_rules::ICodeAnalysisAggregate;
use shared_role_rules::IRoleRunnerAggregate;
use shared_role_rules::taxonomy_role_rules_request::RoleRequest;
use shared_structure_rules::IStructureAggregate;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Instant;

use crate::surface_ci_action::run_isolated;

// Re-exported so the cancel-aware scan path stays reachable at its historical
// import site (`surface_check_action::collect_scan_with_cancel` /
// `CancellableScanOutcome`) after that block moved to
// `surface_scan_cancel_action`.
pub use crate::surface_scan_cancel_action::{CancellableScanOutcome, collect_scan_with_cancel};

/// Capability seams exposed alongside the filesystem aggregate, so callers can
/// dispatch protocol operations without holding the raw IO protocol (#571).
/// File access goes through `aggregate`; `workspace`/`parser` stay seams because
/// no aggregate facade exposes them.
#[derive(Clone)]
pub struct FilesystemSeam {
    pub workspace: Arc<dyn IFilesystemAggregate>,
    pub parser: Arc<dyn IFilesystemAggregate>,
    pub aggregate: Arc<dyn IFilesystemAggregate>,
}

/// Bundles the 8 scan aggregates + config source + filesystem seam so that
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
    pub structure: Arc<dyn IStructureAggregate>,
    pub doc: Arc<dyn IDocRunnerAggregate>,
    pub config_file_reader: Arc<dyn shared_config_system::IConfigFileReaderAggregate>,
    /// Provides protocol seams + the aggregate per scan run.
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
    collect_scan_with_progress(opts, |_phase, _done, _total| {})
}

/// Scan variant with a progress hook for interactive clients. The dispatcher
/// reports real discovery and aggregate-completion milestones; callers can
/// render these without inventing progress in the UI layer.
pub fn collect_scan_with_progress<F>(
    opts: ScanOptions,
    mut on_progress: F,
) -> Result<Vec<ViolationItem>, String>
where
    F: FnMut(String, usize, usize),
{
    on_progress("Starting scan".to_string(), 0, 0);
    let root = match &opts.path {
        Some(p) => p.value().to_string(),
        None => ".".to_string(),
    };
    if !&opts
        .filesystem
        .aggregate
        .execute(FilesystemRequest::path_exists(std::path::Path::new(&root)))
        .into_path_exists()
    {
        return Err(format!("Error: path '{}' does not exist", root));
    }

    // W10: resolve the target to an absolute path so in-process linters scan
    // the intended scope.
    let root = canonicalize_via(&opts.filesystem.aggregate, Path::new(&root))
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or(root);

    // Validate member against discovered workspaces, then dispatch.
    let target_path = match opts.member.as_deref() {
        Some(m) => validate_member_path(&opts, &root, m)?,
        None => root.clone(),
    };
    // A subfolder or single-file target must still be judged against its whole
    // member dir — the architecture rules read sibling files — so the scan
    // widens to the member dir and only the *report* narrows.
    let scope = shared_cli_commands::classify_scan_scope(&root, &target_path);
    let scan_path = match scope.scan_root() {
        "" => target_path.clone(),
        member_root => member_root.to_string(),
    };

    let violations = match opts.scan_aggregates.as_ref() {
        Some(agg) => run_all_linters_in_process(&scan_path, agg, &mut on_progress),
        None => {
            // Subprocess fallback is forbidden (current_exe self-invocation
            // violates the architecture rules). Callers must provide a
            // ScanAggregates bundle to run the scan in-process.
            return Err(
                "scan_aggregates is required: in-process scan only; subprocess fallback is an architecture violation"
                    .to_string(),
            );
        }
    };
    let violations = retain_scope(violations, &scope);
    let violations = apply_filter(violations, &opts.filter);
    Ok(violations)
}

/// Keep only the violations the user's path selects. `Workspace` and
/// `TopLevel` keep everything; the other two narrow to the requested prefix.
///
/// A failed canonicalization keeps the violation: dropping a real finding
/// because a path did not resolve would hide a defect, which is worse than
/// reporting one extra line.
fn retain_scope(
    violations: Vec<ViolationItem>,
    scope: &shared_cli_commands::ScanScope,
) -> Vec<ViolationItem> {
    let Some(prefix) = scope.filter_prefix() else {
        return violations;
    };
    let prefix_canon = std::path::PathBuf::from(prefix)
        .canonicalize()
        .unwrap_or_else(|_| std::path::PathBuf::from(prefix));
    violations
        .into_iter()
        .filter(|v| {
            std::path::PathBuf::from(&v.file.value)
                .canonicalize()
                .unwrap_or_else(|_| std::path::PathBuf::from(&v.file.value))
                .starts_with(&prefix_canon)
        })
        .collect()
}

/// Canonicalize a path via the filesystem aggregate (`None` on failure).
pub(crate) fn canonicalize_via(fs: &Arc<dyn IFilesystemAggregate>, path: &Path) -> Option<PathBuf> {
    match fs
        .execute(FilesystemRequest::canonicalize(path))
        .into_paths()
    {
        paths if paths.is_empty() => None,
        paths => Some(PathBuf::from(&paths[0])),
    }
}

/// Resolve the member-scoped scan target, validating it against discovered
/// workspaces when a multi-project orchestrator is available.
pub(crate) fn validate_member_path(
    opts: &ScanOptions,
    root: &str,
    member: &str,
) -> Result<String, String> {
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
pub(crate) fn apply_filter(
    mut violations: Vec<ViolationItem>,
    filter: &Option<String>,
) -> Vec<ViolationItem> {
    if let Some(filter_str) = filter {
        let filter_upper = filter_str.to_uppercase();
        violations.retain(|v| v.code.code().contains(&filter_upper));
    }
    violations
}

/// Check if a path belongs to a workspace member.
pub fn is_member_path(path: &FilePath, ws: &dyn IFilesystemAggregate) -> bool {
    ws.execute(FilesystemRequest::is_member_path(path.clone()))
        .into_is_member()
}

/// Run all 6 linters in-memory for a given path; return violations.
pub fn collect_scan_json(
    path: &str,
    seam: &FilesystemSeam,
    agg: &ScanAggregates,
) -> Result<Vec<ViolationItem>, String> {
    if !&seam
        .aggregate
        .execute(FilesystemRequest::path_exists(std::path::Path::new(path)))
        .into_path_exists()
    {
        return Err(format!("Error: path '{}' does not exist", path));
    }
    run_all_linters_json(path, seam, agg)
}

/// Default check: in-memory scan of all linters.
pub fn collect_default_check(
    project_root: &str,
    seam: &FilesystemSeam,
    agg: &ScanAggregates,
) -> Result<Vec<ViolationItem>, String> {
    collect_scan_json(project_root, seam, agg)
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
///
/// Each capability runs through `run_isolated`, so one panicking linter is
/// logged to stderr while the others continue.
pub(crate) fn run_all_linters_in_process(
    path: &str,
    agg: &ScanAggregates,
    on_progress: &mut dyn FnMut(String, usize, usize),
) -> Vec<ViolationItem> {
    let seam = agg.fs_seam.clone();

    let target = std::path::Path::new(path);
    let target_canon =
        canonicalize_via(&seam.aggregate, target).unwrap_or_else(|| std::path::PathBuf::from(path));
    let target_canon_str = target_canon.to_string_lossy().to_string();
    let target_canon_str_ref = target_canon_str.as_str();

    tracing::info!(
        target: "lint_arwaky::audit",
        event = "scan_start",
        path = target_canon_str_ref,
        "scan starting"
    );
    let scan_start = Instant::now();

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

    tracing::debug!(
        target: "lint_arwaky::audit",
        event = "ignored_paths_loaded",
        count = ignored.len(),
        patterns = ignored.join(","),
        "config ignored_paths loaded"
    );

    // Single-file target: build a one-entry index and run the auditors on it.
    if scan_root.is_file() {
        on_progress("Scanning file".to_string(), 0, 1);
        let result = run_single_file_scan(&seam, &scan_root, agg, &root_fp, &ignored);
        on_progress("Scan complete".to_string(), 1, 1);
        return result;
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
    let index_build_start = Instant::now();
    seam.aggregate
        .execute(FilesystemRequest::build_file_index_with_ignored(
            std::path::Path::new(&scan_root),
            &build_index_ignored(&ignored, &scan_root),
        ));
    tracing::debug!(
        target: "lint_arwaky::audit",
        event = "file_index_built",
        elapsed_ms = index_build_start.elapsed().as_millis() as u64,
        "file index build complete"
    );

    let discover_start = Instant::now();
    let discovered = discover_lintable_files(&seam, &scan_root, &ignored);
    let total_files = discovered.len();
    tracing::info!(
        target: "lint_arwaky::audit",
        event = "files_discovered",
        count = total_files,
        elapsed_ms = discover_start.elapsed().as_millis() as u64,
        "lintable files discovered under scan root"
    );
    on_progress("Files discovered".to_string(), 0, total_files);
    let entries_start = Instant::now();
    let entries = build_entries(&seam, &discovered);
    let import_map = build_import_map(&seam, &entries);
    tracing::info!(
        target: "lint_arwaky::audit",
        event = "index_built",
        files = total_files,
        elapsed_ms = entries_start.elapsed().as_millis() as u64,
        "entry + import-map index built"
    );
    on_progress("Index built".to_string(), 0, total_files);

    // AES103's subject is the files the default walk prunes, so they need their
    // own discovery. The root here is the scan target rather than the
    // workspace root, because a member-dir scan must judge its own test files
    // and no sibling's.
    let test_files = build_entries(
        &seam,
        &discover_test_suite_files(&seam, &scan_root, &ignored),
    );

    let parent_workspace = target_canon
        .parent()
        .and_then(|p| p.parent())
        .map(|p| p.to_path_buf());
    // AES5xx orphan (and AES205 cycle) violation paths are relative to the
    // workspace root the orphan scanner resolved — for a member-dir scan target
    // that is one level up (the target's parent), not two.
    let workspace_root = target_canon.parent().map(|p| p.to_path_buf());

    let mut all: Vec<ViolationItem> = Vec::new();
    // Capability panics are logged after the scan-scope filters, so an isolated
    // failure is visible without inventing a rule code.
    let mut panics: Vec<String> = Vec::new();

    let quality_start = Instant::now();
    all.extend(run_isolated("quality", &mut panics, || {
        agg.quality
            .execute(CodeAnalysisRequest::RunAnalysis {
                files: entries.clone(),
            })
            .into_violations()
            .iter()
            .map(ViolationItem::from_lint_result)
            .collect::<Vec<_>>()
    }));
    tracing::info!(
        target: "lint_arwaky::audit",
        event = "quality_done",
        violations = all.len(),
        elapsed_ms = quality_start.elapsed().as_millis() as u64,
        "quality linter complete"
    );
    on_progress(
        "Quality checks complete".to_string(),
        total_files,
        total_files,
    );
    let role_start = Instant::now();
    all.extend(run_isolated("role", &mut panics, || {
        agg.role
            .execute(RoleRequest::audit(&entries))
            .into_violations()
            .iter()
            .map(ViolationItem::from_lint_result)
            .collect::<Vec<_>>()
    }));
    tracing::info!(
        target: "lint_arwaky::audit",
        event = "role_done",
        elapsed_ms = role_start.elapsed().as_millis() as u64,
        "role linter complete"
    );
    on_progress("Role checks complete".to_string(), total_files, total_files);
    let import_start = Instant::now();
    // Workspace-wide import map so AES201/202/203/205 see cross-member imports
    // (the dispatcher's fs instance differs from the import orchestrator's own).
    all.extend(run_isolated("import", &mut panics, || {
        agg.import
            .execute(ImportRequest::audit_with_entries_and_imports(
                &entries,
                &import_map,
            ))
            .into_violations()
            .iter()
            .map(ViolationItem::from_lint_result)
            .collect::<Vec<_>>()
    }));
    tracing::info!(
        target: "lint_arwaky::audit",
        event = "import_done",
        elapsed_ms = import_start.elapsed().as_millis() as u64,
        "import linter complete"
    );
    on_progress(
        "Import checks complete".to_string(),
        total_files,
        total_files,
    );
    let naming_start = Instant::now();
    all.extend(run_isolated("naming", &mut panics, || {
        agg.naming
            .execute(NamingRequest::audit_with_tests(&entries, &test_files))
            .into_violations()
    }));
    tracing::info!(
        target: "lint_arwaky::audit",
        event = "naming_done",
        elapsed_ms = naming_start.elapsed().as_millis() as u64,
        "naming linter complete"
    );
    on_progress(
        "Naming checks complete".to_string(),
        total_files,
        total_files,
    );
    let orphan_start = Instant::now();
    all.extend(run_isolated("orphan", &mut panics, || {
        let (_graph_ctx, orphan_violations) = agg
            .orphan
            .execute(OrphanRequest::scan(
                &root_fp,
                &shared_common::taxonomy_common_vo::PatternList::new(ignored.clone()),
            ))
            .into_scan_outcome();
        orphan_violations
            .iter()
            .map(ViolationItem::from_lint_result)
            .collect::<Vec<_>>()
    }));
    tracing::info!(
        target: "lint_arwaky::audit",
        event = "orphan_done",
        elapsed_ms = orphan_start.elapsed().as_millis() as u64,
        "orphan linter complete"
    );
    on_progress(
        "Orphan checks complete".to_string(),
        total_files,
        total_files,
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
        let has_markdown = ext_files
            .iter()
            .any(|f| f.ends_with(".md") || f.ends_with(".markdown"));
        // Honor the project's `adapters:` SSOT exactly like the `external`
        // subcommand does — an empty list would run every language adapter,
        // including tools the config never enables (markdownlint), which is
        // where the workspaces-good false positives came from.
        let config_entries = crate::surface_external_action::load_config_entries(
            std::path::Path::new(&target_canon_str),
            seam.aggregate.as_ref(),
            agg.config_file_reader.as_ref(),
        );
        let adapter_count = config_entries.len();
        let context = shared_external_lint::taxonomy_external_lint_vo::ExternalLintContext {
            has_rust,
            has_python,
            has_js,
            has_markdown,
            ignored_paths: ignored.clone(),
            ignored_rules: crate::surface_external_action::load_ignored_rules(
                std::path::Path::new(&target_canon_str),
                seam.aggregate.as_ref(),
                agg.config_file_reader.as_ref(),
            ),
            config_entries,
        };
        let ext_start = Instant::now();
        let mut external: Vec<ViolationItem> = run_isolated("external", &mut panics, || {
            // The external adapters shell out to project-wide tools that
            // recursively walk every directory they are handed. When the scan
            // target is a workspace root, handing them the whole root makes
            // bandit/ruff/mypy walk sibling agent repos and vendored trees
            // that are later dropped by the scope filter below — pure wasted
            // time (the bandit bottleneck). Instead, hand each adapter the
            // member dirs directly; the adapters resolve their own working
            // dirs (Cargo.toml parent, etc.) so per-member invocations stay
            // correct. Non-workspace-root targets keep the single-target scan.
            let targets: Vec<FilePath> = if member_scope.is_some() {
                member_dirs
                    .iter()
                    .filter_map(|m| FilePath::new(m.to_string_lossy().to_string()).ok())
                    .collect()
            } else {
                vec![ext_target_fp]
            };
            let mut out: Vec<ViolationItem> = Vec::new();
            for target in &targets {
                out.extend(
                    agg.external
                        .execute(
                            shared_external_lint::ExternalLintRequest::scan_all_with_context(
                                target, &context,
                            ),
                        )
                        .into_violations()
                        .values
                        .iter()
                        .map(ViolationItem::from_lint_result),
                );
            }
            out
        });
        tracing::info!(
            target: "lint_arwaky::audit",
            event = "external_done",
            adapters = adapter_count,
            elapsed_ms = ext_start.elapsed().as_millis() as u64,
            "external adapters complete"
        );
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

    // Structure — folder-layout audit (AES701–AES703). Its findings name a
    // folder or a file inside one, and a folder path is not itself a file, so
    // these are scoped separately and appended after the file-scope filter.
    let structure_start = Instant::now();
    all.extend(run_isolated("structure", &mut panics, || {
        structure_violations_in_scope(&target_canon_str, agg)
    }));
    tracing::info!(
        target: "lint_arwaky::audit",
        event = "structure_done",
        elapsed_ms = structure_start.elapsed().as_millis() as u64,
        "structure audit complete"
    );

    // Doc invariants (AES601–AES606) audit the workspace document chain
    // (FRD/BACKLOG pairs, PRD, AGENTS, ...) which is not part of the
    // source-file index, so it runs against the scan target directly and its
    // findings are appended after the file-scope filter like structure does.
    let doc_start = Instant::now();
    all.extend(run_isolated("doc", &mut panics, || {
        doc_violations_in_scope(&target_canon_str, agg)
    }));
    tracing::info!(
        target: "lint_arwaky::audit",
        event = "doc_done",
        elapsed_ms = doc_start.elapsed().as_millis() as u64,
        "doc audit complete"
    );

    on_progress("Scan complete".to_string(), total_files, total_files);
    tracing::info!(
        target: "lint_arwaky::audit",
        event = "scan_complete",
        total_violations = all.len(),
        elapsed_ms = scan_start.elapsed().as_millis() as u64,
        "scan finished"
    );
    if !panics.is_empty() {
        for line in &panics {
            eprintln!("{line}");
        }
    }

    all
}

// Re-export scope and discovery helpers so existing call sites keep their
// import paths unchanged.
pub(crate) use crate::surface_check_discovery_action::{
    build_entries, build_import_map, build_index_ignored, discover_lintable_files,
    discover_test_suite_files, run_all_linters_json,
};
pub(crate) use crate::surface_check_scope_action::{
    doc_violations_in_scope, external_violation_in_scope, run_single_file_scan,
    structure_violations_in_scope, violation_in_scan_scope,
};

/// Detect the source language from a file path extension. This is a
/// surface-layer helper that avoids importing IWorkspaceProtocol (AES201).
pub(crate) fn detect_language_from_path(
    path: &str,
) -> shared_filesystem::taxonomy_filesystem_vo::Language {
    let ext = std::path::Path::new(path)
        .extension()
        .map(|e| e.to_str().unwrap_or(""))
        .unwrap_or("");
    match ext {
        "rs" => shared_filesystem::taxonomy_filesystem_vo::Language::Rust,
        "py" => shared_filesystem::taxonomy_filesystem_vo::Language::Python,
        "ts" | "tsx" | "js" | "jsx" => {
            shared_filesystem::taxonomy_filesystem_vo::Language::TypeScript
        }
        _ => shared_filesystem::taxonomy_filesystem_vo::Language::Unknown,
    }
}

#[cfg(test)]
mod bfs_tests {
    use super::*;
    use crate::surface_check_discovery_action::{
        bfs_enter_subdirs, build_skip_dirs, discover_lintable_files,
    };
    use std::collections::HashSet;

    /// Build a `FilesystemSeam` from the real `FilesystemContainer` so the BFS
    /// drives actual directory entries (the `MockFilesystem` returns the same
    /// list for every `ScanDirectory` call, which would loop).
    fn real_seam() -> FilesystemSeam {
        let container = filesystem::root_filesystem_container::FilesystemContainer::new();
        let orchestrator = container.orchestrator();
        FilesystemSeam {
            workspace: orchestrator.clone(),
            parser: orchestrator.clone(),
            aggregate: orchestrator,
        }
    }

    /// Exact-path predicate: true if `discovered` contains the canonicalized
    /// form of `expected` (a path under the temp root).
    fn has_path(discovered: &[String], expected: &std::path::Path) -> bool {
        let canon = expected
            .canonicalize()
            .unwrap_or_else(|_| expected.to_path_buf());
        discovered.iter().any(|p| {
            std::path::Path::new(p)
                .canonicalize()
                .map(|c| c == canon)
                .unwrap_or(false)
        })
    }

    #[test]
    fn bfs_skips_nested_git_repo_with_dotgit_file() {
        let tmp = tempfile::TempDir::new().unwrap();
        let root = tmp.path();
        // Nested repo: dir with a `.git` FILE (not dir) — the submodule / worktree marker.
        let nested = root.join("submodule");
        std::fs::create_dir_all(nested.join("src")).unwrap();
        std::fs::write(nested.join(".git"), "gitdir: /elsewhere").unwrap();
        std::fs::write(nested.join("src").join("main.rs"), "fn main() {}").unwrap();
        // Normal dir: no `.git` file.
        let normal = root.join("normal");
        std::fs::create_dir_all(normal.join("src")).unwrap();
        std::fs::write(normal.join("src").join("lib.rs"), "fn lib() {}").unwrap();

        let seam = real_seam();
        let mut discovered = Vec::new();
        let ignored: Vec<String> = Vec::new();
        bfs_enter_subdirs(&seam, root, &HashSet::new(), &ignored, &mut discovered);
        assert!(
            !discovered.iter().any(|p| p.contains("submodule")),
            "BFS must skip a dir carrying its own .git file: {discovered:?}"
        );
        assert!(
            has_path(&discovered, &normal.join("src").join("lib.rs")),
            "BFS must keep a plain dir without a .git marker: {discovered:?}"
        );
    }

    #[test]
    fn bfs_skips_multi_segment_ignored_path() {
        let tmp = tempfile::TempDir::new().unwrap();
        let root = tmp.path();
        // Multi-segment path: internal/lint-arwaky/.worktrees
        let wt = root.join("internal").join("lint-arwaky").join(".worktrees");
        std::fs::create_dir_all(wt.join("feature-x")).unwrap();
        std::fs::write(wt.join("feature-x").join("main.rs"), "fn main() {}").unwrap();
        // A dir that should survive.
        let src = root.join("internal").join("lint-arwaky").join("src");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(src.join("lib.rs"), "fn lib() {}").unwrap();

        let seam = real_seam();
        let mut discovered = Vec::new();
        let ignored = vec!["internal/lint-arwaky/.worktrees".to_string()];
        bfs_enter_subdirs(&seam, root, &HashSet::new(), &ignored, &mut discovered);
        assert!(
            !discovered.iter().any(|p| p.contains(".worktrees")),
            "BFS must prune the multi-segment ignored subtree: {discovered:?}"
        );
        assert!(
            has_path(&discovered, &src.join("lib.rs")),
            "BFS must keep the non-ignored sibling dir: {discovered:?}"
        );
    }

    /// At a workspace root the BFS must enter a non-member subdirectory (e.g.
    /// `docs`) so source files nested under it are discovered. The asserted
    /// file lives at `docs/sub/root_doc.rs` — one level deeper than the
    /// `discover_source_files` root scan reaches, so its discovery depends on
    /// `bfs_enter_subdirs` entering `docs` and then `docs/sub`. A `target/`
    /// subdir confirms `DEFAULT_IGNORED_PATHS` pruning still applies at depth 0.
    #[test]
    fn bfs_enters_non_member_subdir_at_workspace_root() {
        let tmp = tempfile::TempDir::new().unwrap();
        let root = tmp.path();
        // Workspace root marker.
        let members = root.join("crates").join("alpha").join("src");
        std::fs::create_dir_all(&members).unwrap();
        std::fs::write(members.join("lib.rs"), "fn lib() {}").unwrap();
        // Non-member subdir, one level deeper than the root scan reaches.
        let docs_sub = root.join("docs").join("sub");
        std::fs::create_dir_all(&docs_sub).unwrap();
        std::fs::write(docs_sub.join("root_doc.rs"), "fn root_doc() {}").unwrap();
        // `target/` must be pruned even though it holds a source file.
        let target = root.join("target");
        std::fs::create_dir_all(&target).unwrap();
        std::fs::write(target.join("build.rs"), "fn build() {}").unwrap();

        let seam = real_seam();
        let mut discovered = Vec::new();
        let ignored: Vec<String> = Vec::new();
        bfs_enter_subdirs(
            &seam,
            root,
            &build_skip_dirs(true, root),
            &ignored,
            &mut discovered,
        );

        assert!(
            has_path(&discovered, &members.join("lib.rs")),
            "BFS must still enter the member dir at a workspace root: {discovered:?}"
        );
        assert!(
            has_path(&discovered, &docs_sub.join("root_doc.rs")),
            "BFS must enter a non-member subdir at a workspace root: {discovered:?}"
        );
        assert!(
            !discovered.iter().any(|p| p.contains("target")),
            "BFS must prune the default-ignored `target/` subdir: {discovered:?}"
        );
    }

    /// A root-level `.md` doc inside a non-member subdir of a workspace root is
    /// not a source file, so `discover_lintable_files` must not surface it —
    /// the doc-rules audit (a separate collector) is what handles root docs.
    #[test]
    fn discover_lintable_files_prunes_md_docs_in_non_member_subdir() {
        let tmp = tempfile::TempDir::new().unwrap();
        let root = tmp.path();
        let member = root.join("crates").join("alpha").join("src");
        std::fs::create_dir_all(&member).unwrap();
        std::fs::write(member.join("lib.rs"), "fn lib() {}").unwrap();
        let docs = root.join("docs");
        std::fs::create_dir_all(&docs).unwrap();
        std::fs::write(docs.join("root_doc.md"), "# doc\n").unwrap();

        let seam = real_seam();
        let ignored: Vec<String> = Vec::new();
        let discovered = discover_lintable_files(&seam, root, &ignored);

        assert!(
            has_path(&discovered, &member.join("lib.rs")),
            "member-dir source files must still be discovered: {discovered:?}"
        );
        assert!(
            !discovered.iter().any(|p| p.ends_with("root_doc.md")),
            "`discover_lintable_files` must not surface non-source `.md` docs: {discovered:?}"
        );
    }

    /// A `check .`-style scan of a repo root must not descend into the
    /// `workspaces-bad` / `workspaces-good` fixture siblings, while a direct
    /// fixture scan keeps its own tree.
    #[test]
    fn bfs_skips_fixture_dirs_at_non_fixture_root() {
        let tmp = tempfile::TempDir::new().unwrap();
        let root = tmp.path();
        // Workspace root marker.
        let member = root.join("crates").join("alpha").join("src");
        std::fs::create_dir_all(&member).unwrap();
        std::fs::write(member.join("lib.rs"), "fn lib() {}").unwrap();
        // Fixture sibling dirs holding source files.
        let bad = root.join("workspaces-bad");
        std::fs::create_dir_all(&bad).unwrap();
        std::fs::write(bad.join("bad.rs"), "fn bad() {}").unwrap();
        let good = root.join("workspaces-good");
        std::fs::create_dir_all(&good).unwrap();
        std::fs::write(good.join("good.rs"), "fn good() {}").unwrap();

        let seam = real_seam();
        let mut discovered = Vec::new();
        let ignored: Vec<String> = Vec::new();
        // `scan_target_is_ws_root = true` keeps member dirs; the fixture dirs
        // are added to the skip set because the target is not a fixture.
        bfs_enter_subdirs(
            &seam,
            root,
            &build_skip_dirs(true, root),
            &ignored,
            &mut discovered,
        );
        assert!(
            has_path(&discovered, &member.join("lib.rs")),
            "member-dir source files must still be discovered: {discovered:?}"
        );
        assert!(
            !discovered.iter().any(|p| p.contains("workspaces-bad")),
            "BFS must not descend into workspaces-bad at a non-fixture root: {discovered:?}"
        );
        assert!(
            !discovered.iter().any(|p| p.contains("workspaces-good")),
            "BFS must not descend into workspaces-good at a non-fixture root: {discovered:?}"
        );
    }

    /// A degenerate workspace root (marker dirs, no source files) must yield
    /// an empty discovery without panicking.
    #[test]
    fn bfs_handles_empty_workspace_root() {
        let tmp = tempfile::TempDir::new().unwrap();
        let root = tmp.path();
        std::fs::create_dir_all(root.join("crates")).unwrap();
        std::fs::create_dir_all(root.join("packages")).unwrap();

        let seam = real_seam();
        let mut discovered = Vec::new();
        let ignored: Vec<String> = Vec::new();
        bfs_enter_subdirs(
            &seam,
            root,
            &build_skip_dirs(true, root),
            &ignored,
            &mut discovered,
        );
        assert!(
            discovered.is_empty(),
            "an empty workspace root must discover no files: {discovered:?}"
        );
    }

    /// `discover_lintable_files` surfaces root-level source files directly
    /// under the scan target — a regression guard for the root scan that runs
    /// before the BFS.
    #[test]
    fn discover_lintable_files_walks_root_source_at_workspace_root() {
        let tmp = tempfile::TempDir::new().unwrap();
        let root = tmp.path();
        // Root-level source file directly under the scan target.
        let root_src = root.join("root_src.rs");
        std::fs::write(&root_src, "fn root_src() {}").unwrap();
        // Member dir so the target is a workspace root.
        let member = root.join("crates").join("alpha").join("src");
        std::fs::create_dir_all(&member).unwrap();
        std::fs::write(member.join("lib.rs"), "fn lib() {}").unwrap();

        let seam = real_seam();
        let ignored: Vec<String> = Vec::new();
        let discovered = discover_lintable_files(&seam, root, &ignored);

        assert!(
            has_path(&discovered, &root_src),
            "root-level source files must be discovered at a workspace root: {discovered:?}"
        );
    }
}
