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
use shared_naming_rules::INamingRunnerAggregate;
use shared_orphan_rules::IOrphanAggregate;
use shared_quality_rules::ICodeAnalysisAggregate;
use shared_role_rules::IRoleRunnerAggregate;
use shared_structure_rules::IStructureAggregate;
use std::path::{Path, PathBuf};
use std::sync::Arc;

// Re-exported so the cancel-aware scan path stays reachable at its historical
// import site (`surface_check_action::collect_scan_with_cancel` /
// `CancellableScanOutcome`) after that block moved to
// `surface_scan_cancel_action`.
pub use crate::surface_scan_cancel_action::{CancellableScanOutcome, collect_scan_with_cancel};

/// Capability seams exposed alongside the filesystem aggregate, so callers can
/// dispatch protocol operations without holding the raw IO protocol (#571).
/// File access goes through `aggregate`; `workspace`/`parser` stay seams because
/// no aggregate facade exposes them.
/// Shared latency guard threshold used by both the Criterion bench and the
/// regression test so they never drift. Public because benches and integration
/// tests are separate compilation units.
pub const LATENCY_GUARD_MAX_RATIO: f64 = 1.2;

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

/// Run all 8 linter groups via subprocesses, collect JSON, return unified violation list.
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
    // Always scan the full workspace root; the scope only narrows the report.
    let scope = shared_cli_commands::classify_scan_scope(&root, &target_path);
    let scan_root = root.clone();

    let report_target = std::path::PathBuf::from(&target_path);
    let workspace_root = std::path::PathBuf::from(&root);
    let violations = match opts.scan_aggregates.as_ref() {
        Some(agg) => run_all_linters_in_process(
            &scan_root,
            &report_target,
            &workspace_root,
            agg,
            &mut on_progress,
        ),
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
pub(crate) fn retain_scope(
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

/// Run all 8 linter groups in-memory for a given path; return violations.
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

pub(crate) use crate::surface_check_scan_action::run_all_linters_in_process;

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

/// Unit tests for `retain_scope` — scope filtering with prefix edge cases.
#[cfg(test)]
mod retain_scope_tests {
    use super::*;
    use shared_cli_commands::ScanScope;
    use shared_common::{ErrorCode, LineNumber, LintMessage, Severity};

    fn mk_violation(file: &str, code: &str) -> ViolationItem {
        ViolationItem {
            code: ErrorCode::raw(code.to_string()),
            file: FilePath::new(file.to_string()).unwrap(),
            line: LineNumber::new(0),
            column: shared_common::ColumnNumber::new(0),
            message: LintMessage::new("test violation".to_string()),
            severity: Severity::INFO,
            violation_name: String::new(),
            why: String::new(),
            fix: String::new(),
        }
    }

    #[test]
    fn retain_scope_none_prefix_returns_everything() {
        let violations = vec![
            mk_violation("workspaces-bad/crates/alpha/src/lib.rs", "AES101"),
            mk_violation("workspaces-bad/crates/beta/src/lib.rs", "AES102"),
        ];
        let scope = ScanScope::Workspace;
        let result = retain_scope(violations, &scope);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn retain_scope_prefix_keeps_matching_drops_nonmatching() {
        let violations = vec![
            mk_violation(
                "workspaces-bad/crates/naming_violations/src/agent_user_helper.rs",
                "AES101",
            ),
            mk_violation("workspaces-bad/crates/other/src/lib.rs", "AES102"),
        ];
        let scope = ScanScope::Subfolder {
            scan_root: "workspaces-bad/crates".to_string(),
            filter_to: "workspaces-bad/crates/naming_violations/src".to_string(),
        };
        let result = retain_scope(violations, &scope);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].code.code(), "AES101");
        assert_eq!(
            result[0].file.value,
            "workspaces-bad/crates/naming_violations/src/agent_user_helper.rs"
        );
    }

    #[test]
    fn retain_scope_non_canonicalizable_path_kept_not_dropped() {
        // A path that does not exist on disk cannot be canonicalized; per the
        // doc comment on `retain_scope`, it must be kept rather than dropped.
        let violations = vec![mk_violation(
            "/nonexistent/crates/naming_violations/src/agent_user_helper.rs",
            "AES101",
        )];
        let scope = ScanScope::Subfolder {
            scan_root: "workspaces-bad/crates".to_string(),
            filter_to: "/nonexistent/crates/naming_violations/src".to_string(),
        };
        let result = retain_scope(violations, &scope);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].code.code(), "AES101");
    }
}
