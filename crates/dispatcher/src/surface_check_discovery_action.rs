// PURPOSE: File discovery, BFS subdirectory walker, index-build helpers, and
// in-memory linter runner for the check action.
use crate::surface_check_action::{
    FilesystemSeam, ScanAggregates, detect_language_from_path, run_all_linters_in_process,
};
use shared_common::ViolationItem;
use shared_filesystem::FilesystemRequest;
use shared_filesystem::utility_path_filter;
use shared_naming_rules::TESTS_DIR;

const BENCHES_DIR: &str = "benches";

pub(crate) fn discover_lintable_files(
    seam: &FilesystemSeam,
    scan_root: &std::path::Path,
    ignored: &[String],
) -> Vec<String> {
    tracing::debug!(
        target: "lint_arwaky::audit",
        event = "walker_start",
        root = scan_root.to_string_lossy().as_ref(),
        "discovering lintable files"
    );
    // Discover the target's own files with the config ignore list (a dir
    // named `workspaces-bad` never matches a file pattern, so fixture
    // targets are unaffected by the index build's fixture-dir exclusion).
    let mut discovered = seam
        .aggregate
        .execute(FilesystemRequest::discover_source_files(scan_root, ignored))
        .into_paths();
    tracing::debug!(
        target: "lint_arwaky::audit",
        event = "walker_root_files",
        count = discovered.len(),
        "root-level source files discovered"
    );
    // Recurse into subdirs so nested source trees (crates/<name>/src/*) are
    // fully covered, matching what the subprocess linters walk.
    let is_ws_root = ["crates", "packages", "modules"]
        .iter()
        .any(|name| scan_root.join(name).is_dir());
    let skip_dirs = build_skip_dirs(is_ws_root, scan_root);
    bfs_enter_subdirs(seam, scan_root, &skip_dirs, ignored, &mut discovered);
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
/// plus member dirs when the scan target is not a workspace root, plus fixture
/// dirs when they are not the scan target itself (a `check .` of the repo root
/// must never lint `workspaces-bad/good`, but a direct fixture scan must lint
/// its files).
pub(crate) fn build_skip_dirs(
    scan_target_is_ws_root: bool,
    scan_root: &std::path::Path,
) -> std::collections::HashSet<&'static str> {
    let mut skip_dirs: std::collections::HashSet<&str> =
        shared_common::taxonomy_default_constant::DEFAULT_IGNORED_PATHS
            .iter()
            .copied()
            .collect();
    if !scan_target_is_ws_root {
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

/// BFS into every subdirectory of `scan_root`, collecting lintable source
/// files (`discovered` is extended in place).
pub(crate) fn bfs_enter_subdirs(
    seam: &FilesystemSeam,
    scan_root: &std::path::Path,
    skip_dirs: &std::collections::HashSet<&str>,
    ignored: &[String],
    discovered: &mut Vec<String>,
) {
    let mut seen: std::collections::HashSet<std::path::PathBuf> = std::collections::HashSet::new();
    let mut queue: Vec<std::path::PathBuf> = vec![scan_root.to_path_buf()];
    while let Some(dir) = queue.pop() {
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
                tracing::trace!(
                    target: "lint_arwaky::audit",
                    event = "walker_skip",
                    reason = "default_skip_dir",
                    dir = name,
                    "skipping default-ignored directory"
                );
                continue;
            }
            // Multi-segment ignored_paths (e.g. `internal/lint-arwaky/.worktrees`)
            // must match the full relative path from scan_root, not just the base
            // name — the base-name check in `skip_dirs` cannot see across segments.
            let rel_str = entry_path
                .strip_prefix(scan_root)
                .map(|rel| rel.to_string_lossy().to_string())
                .unwrap_or_else(|_| entry_path.to_string_lossy().to_string());
            if utility_path_filter::is_path_ignored(&rel_str, ignored) {
                tracing::trace!(
                    target: "lint_arwaky::audit",
                    event = "walker_skip",
                    reason = "ignored_path_pattern",
                    dir = rel_str.as_str(),
                    "skipping directory matched by config ignored_paths"
                );
                continue;
            }
            // Nested git repos (submodules / worktrees) carry their own `.git`
            // file (not directory) and are linted by their own config — skip them.
            if entry_path.join(".git").is_file() {
                tracing::trace!(
                    target: "lint_arwaky::audit",
                    event = "walker_skip",
                    reason = "nested_git_repo",
                    dir = rel_str.as_str(),
                    "skipping nested git repo (has .git file)"
                );
                continue;
            }
            if !seen.insert(entry_path.to_path_buf()) {
                continue;
            }
            let n_before = discovered.len();
            discovered.extend(
                seam.aggregate
                    .execute(FilesystemRequest::discover_source_files(
                        entry_path, ignored,
                    ))
                    .into_paths(),
            );
            tracing::debug!(
                target: "lint_arwaky::audit",
                event = "walker_enter",
                dir = entry_path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default().as_str(),
                new_files = discovered.len() - n_before,
                total_files = discovered.len(),
                "entering subdirectory"
            );
            queue.push(entry_path.to_path_buf());
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
        let language = detect_language_from_path(file_path);
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
    seam.parser
        .execute(FilesystemRequest::parse_all(entries.clone()))
        .into_parsed_entries()
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
        for imp in seam
            .parser
            .execute(FilesystemRequest::imports_for(&entry.path))
            .into_imports()
        {
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

// ─── BFS unit tests ──────────────────────────────────────────────
