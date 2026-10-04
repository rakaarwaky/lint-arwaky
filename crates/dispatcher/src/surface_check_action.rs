// PURPOSE: SurfaceCheckAction — check/scan business logic, no formatting.
//
// In-process mode (W10): when `scan_aggregates` is provided, all 7 linters run
// in-process through their aggregate entry points. Subprocess self-invocation
// remains the fallback when aggregates are absent (`scan_aggregates: None`).
use shared_common::FilePath;
use shared_common::ViolationItem;
use shared_config_system::{ConfigRequest, IConfigOrchestratorAggregate};
use shared_doc_rules::IDocRunnerAggregate;
use shared_doc_rules::taxonomy_doc_rules_request::DocRequest;
use shared_doc_rules::taxonomy_doc_rules_response::DocResponse;
use shared_external_lint::IExternalLintAggregate;
use shared_filesystem::FilesystemRequest;
use shared_filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared_filesystem::contract_filesystem_protocol::IParserProtocol;
use shared_filesystem::contract_filesystem_protocol::IWorkspaceProtocol;
use shared_filesystem::utility_test_file_discovery;
use shared_import_rules::IImportRunnerAggregate;
use shared_import_rules::taxonomy_import_rules_request::ImportRequest;
use shared_naming_rules::INamingRunnerAggregate;
use shared_naming_rules::taxonomy_naming_rules_request::NamingRequest;
use shared_naming_rules::{BENCHES_DIR, TESTS_DIR};
use shared_orphan_rules::IOrphanAggregate;
use shared_orphan_rules::OrphanRequest;
use shared_quality_rules::CodeAnalysisRequest;
use shared_quality_rules::ICodeAnalysisAggregate;
use shared_role_rules::IRoleRunnerAggregate;
use shared_role_rules::taxonomy_role_rules_request::RoleRequest;
use shared_structure_rules::IStructureAggregate;
use std::path::{Path, PathBuf};
use std::sync::Arc;

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
    pub workspace: Arc<dyn IWorkspaceProtocol>,
    pub parser: Arc<dyn IParserProtocol>,
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
    let violations = match opts.scan_aggregates.as_ref() {
        Some(agg) => run_all_linters_in_process(&target_path, agg, &mut on_progress),
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
    let violations = apply_filter(violations, &opts.filter);
    Ok(violations)
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
pub fn is_member_path(path: &FilePath, ws: &dyn IWorkspaceProtocol) -> bool {
    ws.is_member_path(path)
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
fn run_all_linters_in_process(
    path: &str,
    agg: &ScanAggregates,
    on_progress: &mut dyn FnMut(String, usize, usize),
) -> Vec<ViolationItem> {
    let seam = agg.fs_seam.clone();

    let target = std::path::Path::new(path);
    let target_canon =
        canonicalize_via(&seam.aggregate, target).unwrap_or_else(|| std::path::PathBuf::from(path));
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
    seam.aggregate
        .execute(FilesystemRequest::build_file_index_with_ignored(
            std::path::Path::new(&scan_root),
            &build_index_ignored(&ignored, &scan_root),
        ));

    let discovered = discover_lintable_files(&seam, &scan_root, &ignored);
    let total_files = discovered.len();
    on_progress("Files discovered".to_string(), 0, total_files);
    let entries = build_entries(&seam, &discovered);
    let import_map = build_import_map(&seam, &entries);
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

    all.extend(run_isolated("quality", &mut panics, || {
        agg.quality
            .execute(CodeAnalysisRequest::run_analysis(&entries))
            .into_violations()
            .iter()
            .map(ViolationItem::from_lint_result)
            .collect::<Vec<_>>()
    }));
    on_progress(
        "Quality checks complete".to_string(),
        total_files,
        total_files,
    );
    all.extend(run_isolated("role", &mut panics, || {
        agg.role
            .execute(RoleRequest::audit(&entries))
            .into_violations()
            .iter()
            .map(ViolationItem::from_lint_result)
            .collect::<Vec<_>>()
    }));
    on_progress("Role checks complete".to_string(), total_files, total_files);
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
    on_progress(
        "Import checks complete".to_string(),
        total_files,
        total_files,
    );
    all.extend(run_isolated("naming", &mut panics, || {
        agg.naming
            .execute(NamingRequest::audit_with_tests(&entries, &test_files))
            .into_violations()
            .iter()
            .map(ViolationItem::from_lint_result)
            .collect::<Vec<_>>()
    }));
    on_progress(
        "Naming checks complete".to_string(),
        total_files,
        total_files,
    );
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
        );
        let context = shared_external_lint::taxonomy_external_lint_vo::ExternalLintContext {
            has_rust,
            has_python,
            has_js,
            has_markdown,
            ignored_paths: ignored.clone(),
            ignored_rules: crate::surface_external_action::load_ignored_rules(
                std::path::Path::new(&target_canon_str),
                seam.aggregate.as_ref(),
            ),
            config_entries,
        };
        let mut external: Vec<ViolationItem> = run_isolated("external", &mut panics, || {
            agg.external
                .execute(
                    shared_external_lint::ExternalLintRequest::scan_all_with_context(
                        &ext_target_fp,
                        &context,
                    ),
                )
                .into_violations()
                .values
                .iter()
                .map(ViolationItem::from_lint_result)
                .collect::<Vec<_>>()
        });
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
    all.extend(run_isolated("structure", &mut panics, || {
        structure_violations_in_scope(&target_canon_str, agg)
    }));

    // Doc invariants (AES601–AES605) audit the workspace document chain
    // (FRD/BACKLOG pairs, PRD, AGENTS, ...) which is not part of the
    // source-file index, so it runs against the scan target directly and its
    // findings are appended after the file-scope filter like structure does.
    all.extend(run_isolated("doc", &mut panics, || {
        doc_violations_in_scope(&target_canon_str, agg)
    }));

    on_progress("Scan complete".to_string(), total_files, total_files);
    if !panics.is_empty() {
        for line in &panics {
            eprintln!("{line}");
        }
    }

    all
}

/// Run the structure audit and keep only findings whose folder sits under the
/// scan target. Structure findings carry workspace-root-relative paths, so they
/// resolve against the workspace root (the target itself, or its parent when
/// the target is a member directory).
pub(crate) fn structure_violations_in_scope(
    target: &str,
    agg: &ScanAggregates,
) -> Vec<ViolationItem> {
    let target_path = std::path::Path::new(target);
    // The audit resolves the workspace root the same way; mirror that here so
    // a finding path joins onto the right base.
    let has_members = ["crates", "modules", "packages"]
        .iter()
        .any(|m| target_path.join(m).is_dir());
    let ws_root: std::path::PathBuf = if has_members {
        target_path.to_path_buf()
    } else {
        target_path
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| target_path.to_path_buf())
    };

    crate::surface_structure_action::collect_structure(target, agg.structure.clone())
        .unwrap_or_default()
        .into_iter()
        .filter(|v| {
            // A finding names a folder (AES702) or a file (AES701/AES703); both
            // resolve under the workspace root. The audit always walks every
            // member dir under that root, so a member-dir scan target must
            // additionally keep only the findings inside itself.
            let resolved = ws_root.join(&v.file.value);
            let exists = resolved.is_dir()
                || agg
                    .fs_seam
                    .aggregate
                    .execute(FilesystemRequest::path_exists(&resolved))
                    .into_path_exists();
            exists && resolved.starts_with(target_path)
        })
        .collect::<Vec<_>>()
}

/// Run the doc-invariant audit (AES601–AES605) and convert its findings into
/// violation items. Findings carry relative doc paths and resolve against the
/// scan target; a finding names a document that must exist inside the target,
/// so out-of-scope or stale findings are dropped like structure findings.
/// All doc rules are HIGH-severity invariant failures → `Severity::HIGH`.
pub(crate) fn doc_violations_in_scope(target: &str, agg: &ScanAggregates) -> Vec<ViolationItem> {
    let target_path = std::path::Path::new(target);
    let target_canon = canonicalize_via(&agg.fs_seam.aggregate, target_path)
        .unwrap_or_else(|| target_path.to_path_buf());

    let DocResponse::Findings { findings } = agg.doc.execute(DocRequest::audit_all(&target_canon));
    findings
        .into_iter()
        .filter_map(|finding| {
            let file = shared_common::FilePath::new(finding.doc.clone()).ok()?;
            let doc_path = target_canon.join(&finding.doc);
            let exists = doc_path.is_file()
                || agg
                    .fs_seam
                    .aggregate
                    .execute(FilesystemRequest::path_exists(&doc_path))
                    .into_path_exists();
            if !exists {
                return None;
            }
            let message = shared_common::LintMessage::new(format!(
                "[{}] {}",
                finding.violation_type, finding.message
            ));
            Some(ViolationItem {
                code: shared_common::ErrorCode::raw(finding.code),
                file,
                line: shared_common::LineNumber::new(1),
                column: shared_common::ColumnNumber::new(1),
                message,
                severity: shared_common::Severity::HIGH,
            })
        })
        .collect::<Vec<_>>()
}

/// Whether an external-lint violation falls inside the scan scope: its file
/// resolves under the target, or it is an AES205 cycle resolved under the
/// parent workspace. When the target is a workspace root (`member_dirs` is
/// `Some`), a violation must additionally resolve inside one of the member
/// dirs — the AES linters only lint `crates`/`packages`/`modules`, so root
/// level files (e.g. a packaging `setup.py`) are out of scope for the external
/// adapters too.
pub(crate) fn external_violation_in_scope(
    v: &ViolationItem,
    seam: &FilesystemSeam,
    target_canon: &std::path::Path,
    parent_workspace: Option<&std::path::Path>,
    member_dirs: Option<&[&std::path::Path]>,
) -> bool {
    let resolved_canon = resolve_violation_path(v, seam, target_canon, parent_workspace);
    if let Some(members) = member_dirs {
        return members.iter().any(|m| {
            resolved_canon.starts_with(m)
                && seam
                    .aggregate
                    .execute(FilesystemRequest::path_exists(&resolved_canon))
                    .into_path_exists()
        });
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
pub(crate) fn violation_in_scan_scope(
    v: &ViolationItem,
    seam: &FilesystemSeam,
    target_canon: &std::path::Path,
    parent_workspace: Option<&std::path::Path>,
    workspace_root: Option<&std::path::Path>,
    member_dirs: Option<&[&std::path::Path]>,
) -> bool {
    let resolved_canon = resolve_violation_path(v, seam, target_canon, workspace_root);
    if !&seam
        .aggregate
        .execute(FilesystemRequest::path_exists(&resolved_canon))
        .into_path_exists()
    {
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
pub(crate) fn resolve_violation_path(
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
        let exists_under_target = &seam
            .aggregate
            .execute(FilesystemRequest::path_exists(&under_target))
            .into_path_exists();
        match workspace_root {
            Some(pw) if !exists_under_target => pw.join(file_path),
            _ => under_target,
        }
    };
    canonicalize_via(&seam.aggregate, &resolved).unwrap_or(resolved)
}

/// Run all 6 linters on a single-file target, returning in-scope violations.
pub(crate) fn run_single_file_scan(
    seam: &FilesystemSeam,
    scan_root: &std::path::Path,
    agg: &ScanAggregates,
    root_fp: &shared_common::taxonomy_path_vo::FilePath,
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
        shared_common::taxonomy_config_language_vo::ConfigLanguage::Rust => {
            shared_filesystem::taxonomy_filesystem_vo::Language::Rust
        }
        shared_common::taxonomy_config_language_vo::ConfigLanguage::Python => {
            shared_filesystem::taxonomy_filesystem_vo::Language::Python
        }
        shared_common::taxonomy_config_language_vo::ConfigLanguage::TypeScript => {
            shared_filesystem::taxonomy_filesystem_vo::Language::TypeScript
        }
    };
    let mut entries = vec![shared_filesystem::taxonomy_filesystem_vo::FileEntry {
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
        Vec<shared_filesystem::taxonomy_filesystem_vo::ImportEntry>,
    > = std::collections::HashMap::new();
    let mut all: Vec<ViolationItem> = Vec::new();
    // Capability panics are logged after the scope filter, so an isolated
    // failure is visible without inventing a rule code.
    let mut panics: Vec<String> = Vec::new();
    all.extend(run_isolated("quality", &mut panics, || {
        agg.quality
            .execute(CodeAnalysisRequest::run_analysis(&entries))
            .into_violations()
            .iter()
            .map(ViolationItem::from_lint_result)
            .collect::<Vec<_>>()
    }));
    all.extend(run_isolated("role", &mut panics, || {
        agg.role
            .execute(RoleRequest::audit(&entries))
            .into_violations()
            .iter()
            .map(ViolationItem::from_lint_result)
            .collect::<Vec<_>>()
    }));
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
    // AES103 judges the test/bench files, so a one-file scan of a misnamed or
    // nested test file has to reach it. The file itself is the only evidence
    // that scan target can offer, so it is passed as the test set when it sits
    // in one of those directories.
    let test_entries: Vec<shared_filesystem::taxonomy_filesystem_vo::FileEntry> =
        if utility_test_file_discovery::is_inside_any(
            &scan_root.to_string_lossy(),
            &[TESTS_DIR, BENCHES_DIR],
        ) {
            entries.clone()
        } else {
            Vec::new()
        };
    all.extend(run_isolated("naming", &mut panics, || {
        agg.naming
            .execute(NamingRequest::audit_with_tests(&entries, &test_entries))
            .into_violations()
            .iter()
            .map(ViolationItem::from_lint_result)
            .collect::<Vec<_>>()
    }));
    all.extend(run_isolated("orphan", &mut panics, || {
        let (_graph_ctx, orphan_violations) = agg
            .orphan
            .execute(OrphanRequest::scan(
                root_fp,
                &shared_common::taxonomy_common_vo::PatternList::new(ignored.to_vec()),
            ))
            .into_scan_outcome();
        orphan_violations
            .iter()
            .map(ViolationItem::from_lint_result)
            .collect::<Vec<_>>()
    }));
    // External — the MarkdownLint adapter is the only one that lints a single
    // file meaningfully: every other adapter shells out to a project-wide tool
    // (cargo, ruff, eslint) whose findings name other files, so on a one-file
    // target they would all be dropped by the scope filter below anyway. The
    // adapter itself already early-returns empty for a non-Markdown file, so
    // this is safe to call unconditionally.
    all.extend(run_isolated("external", &mut panics, || {
        crate::surface_external_action::collect_single_file_external(
            scan_root,
            ignored,
            &agg.external,
            crate::surface_external_action::load_config_entries(scan_root, seam.aggregate.as_ref()),
            seam.aggregate.as_ref(),
        )
    }));
    all.retain(|v| {
        let p = std::path::Path::new(&v.file.value);
        p.is_absolute() && p.starts_with(scan_root)
    });
    if !panics.is_empty() {
        for line in &panics {
            eprintln!("{line}");
        }
    }
    all
}

/// Discover lintable source files under `scan_root` matching the index walk:
/// under a workspace root only member dirs carry source; elsewhere member-dir
/// names and fixture dirs are skipped. Returns the discovered file paths.
pub(crate) fn discover_lintable_files(
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

/// Discover the test and bench files under *scan_root* for AES103.
///
/// The walk lives in the filesystem foundation because it is the one discovery
/// path that does not prune `tests/` and `benches`; this only asks for it by
/// the names the AES103 vocabulary fixes, so a change to the layout rules
/// cannot leave this call pointing at the old names.
pub(crate) fn discover_test_suite_files(
    seam: &FilesystemSeam,
    scan_root: &std::path::Path,
    ignored: &[String],
) -> Vec<String> {
    seam.aggregate
        .execute(FilesystemRequest::discover_files_in_directories(
            scan_root,
            &[TESTS_DIR, BENCHES_DIR],
            ignored,
        ))
        .into_paths()
}

/// Build the set of directory names the BFS skips: always `DEFAULT_IGNORED_PATHS`
/// plus member dirs when not at a workspace root, plus fixture dirs when they
/// are not the scan target itself (a `check .` of the repo root must never
/// lint `workspaces-bad/good`, but a direct fixture scan must lint its files).
pub(crate) fn build_skip_dirs(
    is_ws_root: bool,
    scan_root: &std::path::Path,
) -> std::collections::HashSet<&'static str> {
    let mut skip_dirs: std::collections::HashSet<&str> =
        shared_common::taxonomy_default_constant::DEFAULT_IGNORED_PATHS
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
pub(crate) fn bfs_enter_subdirs(
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
pub(crate) fn build_index_ignored(ignored: &[String], scan_root: &std::path::Path) -> Vec<String> {
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
        .collect::<Vec<_>>()
}

/// Build parsed `FileEntry`s for the discovered file paths, running the
/// tree-sitter parse so `parse_metadata` is populated for the auditors.
pub(crate) fn build_entries(
    seam: &FilesystemSeam,
    discovered: &[String],
) -> Vec<shared_filesystem::taxonomy_filesystem_vo::FileEntry> {
    let mut entries: Vec<shared_filesystem::taxonomy_filesystem_vo::FileEntry> = Vec::new();
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
            shared_common::taxonomy_config_language_vo::ConfigLanguage::Rust => {
                shared_filesystem::taxonomy_filesystem_vo::Language::Rust
            }
            shared_common::taxonomy_config_language_vo::ConfigLanguage::Python => {
                shared_filesystem::taxonomy_filesystem_vo::Language::Python
            }
            shared_common::taxonomy_config_language_vo::ConfigLanguage::TypeScript => {
                shared_filesystem::taxonomy_filesystem_vo::Language::TypeScript
            }
        };
        entries.push(shared_filesystem::taxonomy_filesystem_vo::FileEntry {
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
pub(crate) fn build_import_map(
    seam: &FilesystemSeam,
    entries: &[shared_filesystem::taxonomy_filesystem_vo::FileEntry],
) -> std::collections::HashMap<String, Vec<shared_filesystem::taxonomy_filesystem_vo::ImportEntry>>
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
        Vec<shared_filesystem::taxonomy_filesystem_vo::ImportEntry>,
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

/// Run all 6 linters in-memory through their aggregate entry points and
/// collect ViolationItems. No subprocess — the linters execute in-process
/// via `run_all_linters_in_process`.
pub(crate) fn run_all_linters_in_memory(
    path: &str,
    agg: &ScanAggregates,
) -> Result<Vec<ViolationItem>, String> {
    let result = run_all_linters_in_process(
        path,
        agg,
        &mut |_phase: String, _done: usize, _total: usize| {},
    );
    Ok(result)
}

/// In-memory scan for callers that do not hold a full `ScanAggregates`
/// bundle. Runs all 6 linters through their aggregate entry points — no
/// subprocess self-invocation (current_exe is forbidden by the architecture
/// rules).
pub(crate) fn run_all_linters_json(
    path: &str,
    _seam: &FilesystemSeam,
    agg: &ScanAggregates,
) -> Result<Vec<ViolationItem>, String> {
    run_all_linters_in_memory(path, agg)
}
