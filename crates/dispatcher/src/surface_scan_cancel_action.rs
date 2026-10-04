// PURPOSE: utility-tier surface — cooperative scan cancellation.
//
// Carries the cancel-aware scan path: `collect_scan_with_cancel` stops between
// linter phases when the token is observed set and reports partial results so
// callers can distinguish a user-interrupted scan from a completion (fixes
// #564 / re-closes #366).
//
// `CancellableScanOutcome` and `collect_scan_with_cancel` are re-exported from
// `surface_check_action` so existing import paths stay valid.
use shared_common::FilePath;
use shared_common::ViolationItem;
use std::sync::atomic::{AtomicBool, Ordering};

use shared_config_system::ConfigRequest;
use shared_filesystem::FilesystemRequest;
use shared_import_rules::taxonomy_import_rules_request::ImportRequest;
use shared_naming_rules::taxonomy_naming_rules_request::NamingRequest;
use shared_orphan_rules::OrphanRequest;
use shared_quality_rules::CodeAnalysisRequest;
use shared_role_rules::taxonomy_role_rules_request::RoleRequest;

use crate::surface_check_action::{
    ScanAggregates, ScanOptions, apply_filter, build_entries, build_import_map,
    build_index_ignored, canonicalize_via, discover_lintable_files, discover_test_suite_files,
    doc_violations_in_scope, external_violation_in_scope, run_single_file_scan,
    structure_violations_in_scope, validate_member_path, violation_in_scan_scope,
};

/// Outcome of a cancellable scan. `stopped_early` is set when the dispatcher
/// returned before every linter phase ran because the cancel token was observed
/// set — distinct from a full run that happened to end with the flag set.
pub struct CancellableScanOutcome {
    pub violations: Vec<ViolationItem>,
    pub stopped_early: bool,
}

/// Scan variant with a cooperative cancel token. Checks `cancel` between
/// linter phases and stops early when set, returning partial results so
/// the caller can distinguish "stopped because the user hit Esc" from a
/// normal completion.
pub fn collect_scan_with_cancel<F>(
    opts: ScanOptions,
    cancel: &AtomicBool,
    mut on_progress: F,
) -> Result<CancellableScanOutcome, String>
where
    F: FnMut(String, usize, usize),
{
    on_progress("Starting scan".to_string(), 0, 0);
    if cancel.load(Ordering::Relaxed) {
        return Ok(CancellableScanOutcome {
            violations: Vec::new(),
            stopped_early: true,
        });
    }
    let root = match &opts.path {
        Some(p) => p.value().to_string(),
        None => ".".to_string(),
    };
    if !opts
        .filesystem
        .aggregate
        .execute(FilesystemRequest::path_exists(std::path::Path::new(&root)))
        .into_path_exists()
    {
        return Err(format!("Error: path '{root}' does not exist"));
    }

    let root = canonicalize_via(&opts.filesystem.aggregate, std::path::Path::new(&root))
        .unwrap_or_else(|| std::path::PathBuf::from(&root))
        .to_string_lossy()
        .to_string();

    let target_path = match opts.member.as_deref() {
        Some(m) => validate_member_path(&opts, &root, m)?,
        None => root.clone(),
    };
    let (violations, stopped_early) = match opts.scan_aggregates.as_ref() {
        Some(agg) => run_all_linters_in_process_cancel(&target_path, agg, cancel, &mut on_progress),
        None => {
            if cancel.load(Ordering::Relaxed) {
                return Ok(CancellableScanOutcome {
                    violations: Vec::new(),
                    stopped_early: true,
                });
            }
            // Subprocess self-invocation via current_exe() is forbidden by the
            // architecture rules. Callers must supply a ScanAggregates bundle
            // to run the scan in-process.
            return Err(
                "scan_aggregates is required: in-process scan only; subprocess fallback is an architecture violation"
                    .to_string(),
            );
        }
    };
    let violations = apply_filter(violations, &opts.filter);
    Ok(CancellableScanOutcome {
        violations,
        stopped_early,
    })
}

/// In-process linters with a cooperative cancel token. Checks `cancel` after
/// each linter phase; when set, stops and returns the partial results
/// collected so far plus `stopped_early = true`.
fn run_all_linters_in_process_cancel(
    path: &str,
    agg: &ScanAggregates,
    cancel: &AtomicBool,
    on_progress: &mut dyn FnMut(String, usize, usize),
) -> (Vec<ViolationItem>, bool) {
    let seam = agg.fs_seam.clone();

    let target = std::path::Path::new(path);
    let target_canon =
        canonicalize_via(&seam.aggregate, target).unwrap_or_else(|| std::path::PathBuf::from(path));
    let target_canon_str = target_canon.to_string_lossy().to_string();

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

    if scan_root.is_file() {
        on_progress("Scanning file".to_string(), 0, 1);
        if cancel.load(Ordering::Relaxed) {
            return (Vec::new(), true);
        }
        let result = run_single_file_scan(&seam, &scan_root, agg, &root_fp, &ignored);
        on_progress("Scan complete".to_string(), 1, 1);
        return (result, false);
    }

    seam.aggregate
        .execute(FilesystemRequest::build_file_index_with_ignored(
            std::path::Path::new(&scan_root),
            &build_index_ignored(&ignored, &scan_root),
        ));

    let discovered = discover_lintable_files(&seam, &scan_root, &ignored);
    let total_files = discovered.len();
    on_progress("Files discovered".to_string(), 0, total_files);
    if cancel.load(Ordering::Relaxed) {
        return (Vec::new(), true);
    }

    let entries = build_entries(&seam, &discovered);
    let import_map = build_import_map(&seam, &entries);
    on_progress("Index built".to_string(), 0, total_files);

    let test_files = build_entries(
        &seam,
        &discover_test_suite_files(&seam, &scan_root, &ignored),
    );

    let parent_workspace = target_canon
        .parent()
        .and_then(|p| p.parent())
        .map(|p| p.to_path_buf());
    let workspace_root = target_canon.parent().map(|p| p.to_path_buf());

    let mut all: Vec<ViolationItem> = Vec::new();

    all.extend(
        agg.quality
            .execute(CodeAnalysisRequest::run_analysis(&entries))
            .into_violations()
            .iter()
            .map(ViolationItem::from_lint_result),
    );
    on_progress(
        "Quality checks complete".to_string(),
        total_files,
        total_files,
    );
    if cancel.load(Ordering::Relaxed) {
        return (all, true);
    }

    all.extend(
        agg.role
            .execute(RoleRequest::audit(&entries))
            .into_violations()
            .iter()
            .map(ViolationItem::from_lint_result),
    );
    on_progress("Role checks complete".to_string(), total_files, total_files);
    if cancel.load(Ordering::Relaxed) {
        return (all, true);
    }

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
    on_progress(
        "Import checks complete".to_string(),
        total_files,
        total_files,
    );
    if cancel.load(Ordering::Relaxed) {
        return (all, true);
    }

    all.extend(
        agg.naming
            .execute(NamingRequest::audit_with_tests(&entries, &test_files))
            .into_violations()
            .iter()
            .map(ViolationItem::from_lint_result),
    );
    on_progress(
        "Naming checks complete".to_string(),
        total_files,
        total_files,
    );
    if cancel.load(Ordering::Relaxed) {
        return (all, true);
    }

    let (_graph_ctx, orphan_violations) = agg
        .orphan
        .execute(OrphanRequest::scan(
            &root_fp,
            &shared_common::taxonomy_common_vo::PatternList::new(ignored.clone()),
        ))
        .into_scan_outcome();
    all.extend(
        orphan_violations
            .iter()
            .map(ViolationItem::from_lint_result),
    );
    on_progress(
        "Orphan checks complete".to_string(),
        total_files,
        total_files,
    );
    if cancel.load(Ordering::Relaxed) {
        return (all, true);
    }

    // External
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
        let mut external: Vec<ViolationItem> = agg
            .external
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
    if cancel.load(Ordering::Relaxed) {
        return (all, true);
    }

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

    all.extend(structure_violations_in_scope(&target_canon_str, agg));
    all.extend(doc_violations_in_scope(&target_canon_str, agg));
    on_progress("Scan complete".to_string(), total_files, total_files);

    (all, false)
}
