// PURPOSE: run_all_linters_in_process — execute all 8 linter groups
// in-process against a scan target, then scope/filter the reported violations.
// Extracted from surface_check_action.rs (AES301: file too large).
use crate::surface_check_action::{
    ScanAggregates, build_entries, build_import_map, canonicalize_via, discover_lintable_files,
    discover_test_suite_files, run_single_file_scan,
};
use crate::surface_check_discovery_action::build_index_ignored;
use crate::surface_check_scope_action::{
    doc_violations_in_scope, external_violation_in_scope, structure_violations_in_scope,
    violation_in_scan_scope,
};
use crate::surface_ci_action::run_isolated;
use shared_common::taxonomy_common_vo::PatternList;
use shared_common::{FilePath, ViolationItem};
use shared_config_system::ConfigRequest;
use shared_external_lint::taxonomy_external_lint_vo::ExternalLintContext;
use shared_filesystem::FilesystemRequest;
use shared_import_rules::taxonomy_import_rules_request::ImportRequest;
use shared_naming_rules::taxonomy_naming_rules_request::NamingRequest;
use shared_orphan_rules::OrphanRequest;
use shared_quality_rules::CodeAnalysisRequest;
use shared_role_rules::taxonomy_role_rules_request::RoleRequest;
use std::path::{Path, PathBuf};
use std::time::Instant;

/// Run all 8 linter groups in-process through their aggregate entry points (W10).
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
    scan_root: &str,
    report_target: &Path,
    workspace_root: &Path,
    agg: &ScanAggregates,
    on_progress: &mut dyn FnMut(String, usize, usize),
) -> Vec<ViolationItem> {
    let seam = agg.fs_seam.clone();

    let target = Path::new(scan_root);
    let target_canon =
        canonicalize_via(&seam.aggregate, target).unwrap_or_else(|| PathBuf::from(scan_root));
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
            Path::new(&scan_root),
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
    let parent_workspace = workspace_root.parent().map(|p| p.to_path_buf());
    let workspace_root = Some(workspace_root.to_path_buf());
    let mut all: Vec<ViolationItem> = Vec::new();
    // Capability panics are logged after the scan-scope filters, so an isolated
    // failure is visible without inventing a rule code.
    let mut panics: Vec<String> = Vec::new();
    // Quality, role, import, naming, and orphan groups run on parallel threads
    // (wall time = slowest group). Each thread keeps its own panic collector;
    // findings are re-appended in historical group order for determinism.
    let parallel_start = Instant::now();
    let (quality_v, role_v, import_v, naming_v, orphan_v) = {
        let (quality, role, import, naming, orphan) = std::thread::scope(|s| {
            let h_quality = s.spawn(|| {
                let mut group_panics = Vec::new();
                let group_start = Instant::now();
                let violations = run_isolated("quality", &mut group_panics, || {
                    agg.quality
                        .execute(CodeAnalysisRequest::RunAnalysis {
                            files: entries.clone(),
                        })
                        .into_violations()
                        .iter()
                        .map(ViolationItem::from_lint_result)
                        .collect::<Vec<_>>()
                });
                tracing::info!(
                    target: "lint_arwaky::audit",
                    event = "quality_done",
                    violations = violations.len(),
                    elapsed_ms = group_start.elapsed().as_millis() as u64,
                    "quality linter complete"
                );
                (violations, group_panics)
            });
            let h_role = s.spawn(|| {
                let mut group_panics = Vec::new();
                let group_start = Instant::now();
                let violations = run_isolated("role", &mut group_panics, || {
                    agg.role
                        .execute(RoleRequest::audit(&entries))
                        .into_violations()
                        .iter()
                        .map(ViolationItem::from_lint_result)
                        .collect::<Vec<_>>()
                });
                tracing::info!(
                    target: "lint_arwaky::audit",
                    event = "role_done",
                    violations = violations.len(),
                    elapsed_ms = group_start.elapsed().as_millis() as u64,
                    "role linter complete"
                );
                (violations, group_panics)
            });
            let h_import = s.spawn(|| {
                let mut group_panics = Vec::new();
                let group_start = Instant::now();
                let violations = run_isolated("import", &mut group_panics, || {
                    agg.import
                        .execute(ImportRequest::audit_with_entries_and_imports(
                            &entries,
                            &import_map,
                        ))
                        .into_violations()
                        .iter()
                        .map(ViolationItem::from_lint_result)
                        .collect::<Vec<_>>()
                });
                tracing::info!(
                    target: "lint_arwaky::audit",
                    event = "import_done",
                    violations = violations.len(),
                    elapsed_ms = group_start.elapsed().as_millis() as u64,
                    "import linter complete"
                );
                (violations, group_panics)
            });
            let h_naming = s.spawn(|| {
                let mut group_panics = Vec::new();
                let group_start = Instant::now();
                let violations = run_isolated("naming", &mut group_panics, || {
                    agg.naming
                        .execute(NamingRequest::audit_with_tests(&entries, &test_files))
                        .into_violations()
                });
                tracing::info!(
                    target: "lint_arwaky::audit",
                    event = "naming_done",
                    violations = violations.len(),
                    elapsed_ms = group_start.elapsed().as_millis() as u64,
                    "naming linter complete"
                );
                (violations, group_panics)
            });
            let h_orphan = s.spawn(|| {
                let mut group_panics = Vec::new();
                let group_start = Instant::now();
                let violations = run_isolated("orphan", &mut group_panics, || {
                    let (_graph_ctx, orphan_violations) = agg
                        .orphan
                        .execute(OrphanRequest::scan(
                            &root_fp,
                            &PatternList::new(ignored.clone()),
                        ))
                        .into_scan_outcome();
                    orphan_violations
                        .iter()
                        .map(ViolationItem::from_lint_result)
                        .collect::<Vec<_>>()
                });
                tracing::info!(
                    target: "lint_arwaky::audit",
                    event = "orphan_done",
                    violations = violations.len(),
                    elapsed_ms = group_start.elapsed().as_millis() as u64,
                    "orphan linter complete"
                );
                (violations, group_panics)
            });
            (
                h_quality.join().unwrap_or_default(),
                h_role.join().unwrap_or_default(),
                h_import.join().unwrap_or_default(),
                h_naming.join().unwrap_or_default(),
                h_orphan.join().unwrap_or_default(),
            )
        });
        panics.extend(quality.1);
        panics.extend(role.1);
        panics.extend(import.1);
        panics.extend(naming.1);
        panics.extend(orphan.1);
        (quality.0, role.0, import.0, naming.0, orphan.0)
    };
    tracing::info!(
        target: "lint_arwaky::audit",
        event = "parallel_linters_done",
        elapsed_ms = parallel_start.elapsed().as_millis() as u64,
        "file-scoped linter groups complete"
    );
    all.extend(quality_v);
    all.extend(role_v);
    all.extend(import_v);
    all.extend(naming_v);
    all.extend(orphan_v);
    on_progress(
        "Quality checks complete".to_string(),
        total_files,
        total_files,
    );
    on_progress("Role checks complete".to_string(), total_files, total_files);
    on_progress(
        "Import checks complete".to_string(),
        total_files,
        total_files,
    );
    on_progress(
        "Naming checks complete".to_string(),
        total_files,
        total_files,
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
    let ext_target_fp = FilePath::new(target_canon_str.clone()).unwrap_or_default();
    {
        let ext_files = seam
            .aggregate
            .execute(FilesystemRequest::discover_files(Path::new(
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
            Path::new(&target_canon_str),
            seam.aggregate.as_ref(),
            agg.config_file_reader.as_ref(),
        );
        let adapter_count = config_entries.len();
        let context = ExternalLintContext {
            has_rust,
            has_python,
            has_js,
            has_markdown,
            ignored_paths: ignored.clone(),
            ignored_rules: crate::surface_external_action::load_ignored_rules(
                Path::new(&target_canon_str),
                seam.aggregate.as_ref(),
                agg.config_file_reader.as_ref(),
            ),
            config_entries,
            ..ExternalLintContext::default()
        };
        let ext_start = Instant::now();
        let agg_external = agg.external.clone();
        let mut external: Vec<ViolationItem> = run_isolated("external", &mut panics, || {
            crate::surface_external_action::run_member_external_batches(
                &ext_target_fp,
                &context,
                &ignored,
                &agg_external,
                &member_dirs,
                member_scope.is_some(),
            )
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
                report_target,
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
            report_target,
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
