// PURPOSE: File discovery, BFS subdirectory walker, index-build helpers, and
// in-memory linter runner for the check action.
use crate::surface_check_action::{
    FilesystemSeam, ScanAggregates, canonicalize_via, detect_language_from_path,
    run_all_linters_in_process,
};
use shared_common::ViolationItem;
use shared_filesystem::FilesystemRequest;
use shared_filesystem::utility_filesystem_io::is_source_file;
use shared_filesystem::utility_path_filter;
use shared_naming_rules::TESTS_DIR;
use std::path::Path;

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
            // Collect this directory's own top-level source files only.
            // `discover_source_files` in `discover_lintable_files` already
            // walks recursively from scan_root and handles nested trees;
            // per-entry recursive discovery would re-root multi-segment
            // ignored_paths (relative to scan_root) to each entry, breaking
            // patterns such as `internal/lint-arwaky/.worktrees`.
            let n_before = discovered.len();
            discovered.extend(
                seam.aggregate
                    .execute(FilesystemRequest::ScanDirectory {
                        root: entry_path.to_path_buf(),
                    })
                    .into_paths()
                    .into_iter()
                    .filter(|p| is_source_file(std::path::Path::new(p))),
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
///
/// Content and metadata reuse: entries are looked up in the file-index cache
/// the `build_file_index_with_ignored` step filled (`FileListSnapshot`), so
/// file bytes are never re-read from disk and only files not yet in the
/// cache (the `tests/`/`benches/` suites) fall back to a single
/// `read_lintable_file`. `parse_metadata` may be stale from a warm index; the
/// `parse_all` call below refreshes import extraction from the cached tree
/// without a full re-parse, so any metadata drift is corrected in place —
/// cached entries with populated `parse_metadata` are not re-parsed.
pub(crate) fn build_entries(
    seam: &FilesystemSeam,
    discovered: &[String],
) -> Vec<shared_filesystem::taxonomy_filesystem_vo::FileEntry> {
    // Reuse the cached entries (`Vec<FileEntry>` with content) left by the
    // index build (`FileListSnapshot`). The seam API is owned-only, so the full
    // snapshot is materialized via `into_file_list()`; then clone only entries
    // whose path is in `discovered`. Uncached files fall back to a single
    // `read_lintable_file`.
    let snapshot: Vec<shared_filesystem::taxonomy_filesystem_vo::FileEntry> = seam
        .aggregate
        .execute(FilesystemRequest::file_list_snapshot())
        .into_file_list();
    let discovered_set: std::collections::HashSet<&std::path::Path> =
        discovered.iter().map(std::path::Path::new).collect();
    let cached: std::collections::HashMap<
        &std::path::Path,
        &shared_filesystem::taxonomy_filesystem_vo::FileEntry,
    > = snapshot
        .iter()
        .filter(|e| discovered_set.contains(e.path.as_path()))
        .map(|e| (e.path.as_path(), e))
        .collect();
    let mut entries: Vec<shared_filesystem::taxonomy_filesystem_vo::FileEntry> =
        Vec::with_capacity(discovered.len());
    for file_path in discovered {
        if let Some(entry) = cached.get(std::path::Path::new(file_path)) {
            entries.push((*entry).clone());
            continue;
        }
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
        let parse_ok = !content.is_empty();
        entries.push(shared_filesystem::taxonomy_filesystem_vo::FileEntry {
            path: std::path::PathBuf::from(file_path),
            extension,
            language: detect_language_from_path(file_path),
            size: content.len() as u64,
            content,
            parse_ok,
            parse_metadata: None,
        });
    }
    // Fills `parse_metadata` (needed for AES203 unused-import detection) and
    // rewrites the parser's import cache with this target's scoped imports —
    // a single pass over the entries, not one re-parse per caller.
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
    // Snapshot the out-of-scope imports ONCE: the `ImportListSnapshot` is the
    // workspace-wide import cache taken at the last index build, so entries it
    // carries that belong to files outside this target's scope stay in the
    // map for cross-member visibility.
    let ws_snapshot = seam
        .aggregate
        .execute(FilesystemRequest::ImportListSnapshot)
        .into_imports();
    let scoped_keys: std::collections::HashSet<&std::path::Path> =
        entries.iter().map(|e| e.path.as_path()).collect();
    // Reads the parser's scoped import cache ONCE (one read guard on the
    // `imports` RwLock) and groups it in-memory per source path, instead of
    // re-acquiring the read lock via `imports_for` for every entry.
    let scoped_imports: Vec<shared_filesystem::taxonomy_filesystem_vo::ImportEntry> = seam
        .parser
        .execute(FilesystemRequest::import_list_snapshot())
        .into_imports();
    let mut import_map: std::collections::HashMap<
        String,
        Vec<shared_filesystem::taxonomy_filesystem_vo::ImportEntry>,
    > = std::collections::HashMap::new();
    let push_unique =
        |map: &mut std::collections::HashMap<
            String,
            Vec<shared_filesystem::taxonomy_filesystem_vo::ImportEntry>,
        >,
         key: String,
         imp: shared_filesystem::taxonomy_filesystem_vo::ImportEntry| {
            let entry = map.entry(key).or_default();
            if !entry.iter().any(|existing| existing == &imp) {
                entry.push(imp);
            }
        };
    for imp in scoped_imports
        .into_iter()
        .filter(|imp| scoped_keys.contains(imp.source_file.as_path()))
    {
        let key = imp.source_file.to_string_lossy().to_string();
        push_unique(&mut import_map, key, imp);
    }
    for imp in ws_snapshot
        .into_iter()
        .filter(|imp| !scoped_keys.contains(imp.source_file.as_path()))
    {
        let key = imp.source_file.to_string_lossy().to_string();
        push_unique(&mut import_map, key, imp);
    }
    import_map
}

/// Run all 8 linter groups in-memory through their aggregate entry points and
/// collect ViolationItems. No subprocess — the linters execute in-process
/// via `run_all_linters_in_process`.
pub(crate) fn run_all_linters_in_memory(
    path: &str,
    agg: &ScanAggregates,
) -> Result<Vec<ViolationItem>, String> {
    // Canonicalize `path` before deriving the anchors so a relative input
    // (e.g. `"."`) resolves to an absolute canonical path that `starts_with`
    // against the absolute canonicalized `resolved_canon` inside the scope
    // guard (mirrors `collect_scan`'s `canonicalize_via` on `root`).
    let path_canon = canonicalize_via(&agg.fs_seam.aggregate, Path::new(path))
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string());
    let report_target = Path::new(&path_canon);
    let workspace_root = Path::new(&path_canon).parent().unwrap_or(Path::new("."));
    let result = run_all_linters_in_process(
        path,
        report_target,
        workspace_root,
        agg,
        &mut |_phase: String, _done: usize, _total: usize| {},
    );
    Ok(result)
}

/// In-memory scan for callers that do not hold a full `ScanAggregates`
/// bundle. Runs all 8 linter groups through their aggregate entry points — no
/// subprocess self-invocation (current_exe is forbidden by the architecture
/// rules).
pub(crate) fn run_all_linters_json(
    path: &str,
    _seam: &FilesystemSeam,
    agg: &ScanAggregates,
) -> Result<Vec<ViolationItem>, String> {
    run_all_linters_in_memory(path, agg)
}

// ─── build_entries / build_import_map unit tests ────────────────────────
#[cfg(test)]
mod discovery_tests {
    use super::*;
    use shared_common::taxonomy_language_vo::Language;
    use shared_filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
    use shared_filesystem::taxonomy_filesystem_vo::{
        FileEntry, ImportEntry, ImportType, ParseMetadata, RustMetadata,
    };
    use std::path::PathBuf;
    use std::sync::Arc;

    use std::collections::HashMap;

    // Minimal in-memory aggregate returning controllable snapshot payloads.
    struct SnapshotAggregate {
        file_list: Vec<FileEntry>,
        import_list: Vec<ImportEntry>,
        /// Controllable content returned by `ReadLintableFile`, keyed by path.
        read_content: HashMap<String, String>,
    }

    impl IFilesystemAggregate for SnapshotAggregate {
        fn execute(
            &self,
            request: shared_filesystem::FilesystemRequest,
        ) -> shared_filesystem::FilesystemResponse {
            match request {
                shared_filesystem::FilesystemRequest::FileListSnapshot => {
                    shared_filesystem::FilesystemResponse::Files {
                        entries: self.file_list.clone(),
                    }
                }
                shared_filesystem::FilesystemRequest::ImportListSnapshot => {
                    shared_filesystem::FilesystemResponse::Imports {
                        entries: self.import_list.clone(),
                    }
                }
                shared_filesystem::FilesystemRequest::ReadLintableFile { path } => {
                    shared_filesystem::FilesystemResponse::ContentOpt {
                        value: self.read_content.get(&path).cloned(),
                    }
                }
                shared_filesystem::FilesystemRequest::ParseAll { mut entries } => {
                    // Re-parse entries that lack cached metadata (production
                    // behavior: `parse_all` skips entries whose
                    // `parse_metadata.is_some()`). Non-empty content parses OK
                    // with populated metadata; empty content stays failed.
                    for entry in &mut entries {
                        if entry.parse_metadata.is_some() {
                            continue;
                        }
                        entry.parse_ok = !entry.content.is_empty();
                        entry.parse_metadata = if entry.content.is_empty() {
                            None
                        } else {
                            Some(ParseMetadata::Rust(RustMetadata::default()))
                        };
                    }
                    shared_filesystem::FilesystemResponse::ParsedEntries { entries }
                }
                _ => shared_filesystem::FilesystemResponse::Paths { paths: Vec::new() },
            }
        }
    }

    fn seam_with(file_list: Vec<FileEntry>, import_list: Vec<ImportEntry>) -> FilesystemSeam {
        seam_with_content(file_list, import_list, HashMap::new())
    }

    fn seam_with_content(
        file_list: Vec<FileEntry>,
        import_list: Vec<ImportEntry>,
        read_content: HashMap<String, String>,
    ) -> FilesystemSeam {
        let snap = SnapshotAggregate {
            file_list,
            import_list,
            read_content,
        };
        let aggregate: Arc<dyn IFilesystemAggregate> = Arc::new(snap);
        FilesystemSeam {
            workspace: aggregate.clone(),
            parser: aggregate.clone(),
            aggregate,
        }
    }

    fn rust_entry(
        path: &str,
        content: &str,
        parse_ok: bool,
        metadata: Option<ParseMetadata>,
    ) -> FileEntry {
        FileEntry {
            path: PathBuf::from(path),
            extension: "rs".to_string(),
            language: Language::Rust,
            size: content.len() as u64,
            content: content.to_string(),
            parse_ok,
            parse_metadata: metadata,
        }
    }

    fn import_entry(source: &str) -> ImportEntry {
        ImportEntry {
            source_file: PathBuf::from(source),
            raw_path: "foo".to_string(),
            resolved_path: None,
            import_type: ImportType::Use,
            language: Language::Rust,
            is_dynamic: false,
            is_resolved: false,
            symbols: vec![],
            is_reexport: false,
            is_wildcard: false,
            line: 0,
        }
    }

    #[test]
    fn build_entries_cache_hit_reuses_entry_verbatim() {
        let cached = rust_entry(
            "/a.rs",
            "fn main() {}",
            true,
            Some(ParseMetadata::Rust(RustMetadata::default())),
        );
        let snap = SnapshotAggregate {
            file_list: vec![cached.clone()],
            import_list: Vec::new(),
            read_content: HashMap::new(),
        };
        let seam = seam_with(snap.file_list.clone(), snap.import_list.clone());
        let discovered = vec!["/a.rs".to_string()];

        let entries = build_entries(&seam, &discovered);

        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].path, PathBuf::from("/a.rs"));
        assert_eq!(entries[0].content, "fn main() {}");
        assert!(entries[0].parse_ok);
        assert!(matches!(
            entries[0].parse_metadata,
            Some(ParseMetadata::Rust(_))
        ));
    }

    #[test]
    fn build_entries_cache_miss_falls_back_to_disk_read() {
        let snap = SnapshotAggregate {
            file_list: Vec::new(),
            import_list: Vec::new(),
            read_content: HashMap::new(),
        };
        let seam = seam_with(snap.file_list.clone(), snap.import_list.clone());
        let discovered = vec!["/b.rs".to_string()];

        let entries = build_entries(&seam, &discovered);

        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].path, PathBuf::from("/b.rs"));
        assert_eq!(entries[0].content, "");
        assert!(!entries[0].parse_ok);
        assert!(entries[0].parse_metadata.is_none());
        assert_eq!(entries[0].size, 0);

        // Non-empty disk read produces parsed content.
        let seam = seam_with_content(
            Vec::new(),
            Vec::new(),
            HashMap::from([("/b.rs".to_string(), "fn main() {}".to_string())]),
        );
        let entries = build_entries(&seam, &discovered);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].content, "fn main() {}");
        assert!(entries[0].parse_ok);
        assert_eq!(entries[0].size, "fn main() {}".len() as u64);
    }

    #[test]
    fn build_entries_mixed_cache() {
        let cached = rust_entry(
            "/a.rs",
            "fn a() {}",
            true,
            Some(ParseMetadata::Rust(RustMetadata::default())),
        );
        let snap = SnapshotAggregate {
            file_list: vec![cached.clone()],
            import_list: Vec::new(),
            read_content: HashMap::new(),
        };
        let seam = seam_with(snap.file_list.clone(), snap.import_list.clone());
        let discovered = vec!["/a.rs".to_string(), "/b.rs".to_string()];

        let entries = build_entries(&seam, &discovered);

        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].path, PathBuf::from("/a.rs"));
        assert!(entries[0].parse_ok);
        assert!(matches!(
            entries[0].parse_metadata,
            Some(ParseMetadata::Rust(_))
        ));
        assert_eq!(entries[1].path, PathBuf::from("/b.rs"));
        assert!(!entries[1].parse_ok);
        assert!(entries[1].parse_metadata.is_none());
    }

    #[test]
    fn build_entries_empty_discovered() {
        let snap = SnapshotAggregate {
            file_list: Vec::new(),
            import_list: Vec::new(),
            read_content: HashMap::new(),
        };
        let seam = seam_with(snap.file_list.clone(), snap.import_list.clone());

        let entries = build_entries(&seam, &[]);

        assert!(entries.is_empty());
    }

    #[test]
    fn build_entries_cached_failed_parse_reparsed_by_parse_all() {
        // Entry has parse_metadata=None → parse_all re-parses it: non-empty
        // content flips parse_ok=true and populates metadata.
        let failed = rust_entry("/c.rs", "fn broken( { }", false, None);
        let snap = SnapshotAggregate {
            file_list: vec![failed.clone()],
            import_list: Vec::new(),
            read_content: HashMap::new(),
        };
        let seam = seam_with(snap.file_list.clone(), snap.import_list.clone());
        let discovered = vec!["/c.rs".to_string()];

        let entries = build_entries(&seam, &discovered);

        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].content, "fn broken( { }");
        assert!(entries[0].parse_ok, "re-parse flips parse_ok to true");
        assert!(
            entries[0].parse_metadata.is_some(),
            "re-parse populates parse_metadata"
        );
    }

    #[test]
    fn build_entries_cached_metadata_populated_cloned_verbatim() {
        // Entry already has parse_metadata populated → parse_all skips it;
        // build_entries clones it verbatim.
        let cached = rust_entry(
            "/d.rs",
            "fn main() {}",
            true,
            Some(ParseMetadata::Rust(RustMetadata::default())),
        );
        let snap = SnapshotAggregate {
            file_list: vec![cached.clone()],
            import_list: Vec::new(),
            read_content: HashMap::new(),
        };
        let seam = seam_with(snap.file_list.clone(), snap.import_list.clone());
        let discovered = vec!["/d.rs".to_string()];

        let entries = build_entries(&seam, &discovered);

        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].path, cached.path);
        assert_eq!(entries[0].content, cached.content);
        assert_eq!(entries[0].parse_ok, cached.parse_ok);
        assert_eq!(entries[0].size, cached.size);
        assert!(entries[0].parse_metadata.is_some());
        assert!(matches!(
            entries[0].parse_metadata,
            Some(ParseMetadata::Rust(_))
        ));
    }

    #[test]
    fn build_import_map_scoped_and_out_of_scope_present_once() {
        let scoped_imp = import_entry("/a.rs");
        let out_imp = import_entry("/other.rs");
        let snap = SnapshotAggregate {
            file_list: Vec::new(),
            import_list: vec![scoped_imp.clone(), out_imp.clone()],
            read_content: HashMap::new(),
        };
        let seam = seam_with(snap.file_list.clone(), snap.import_list.clone());
        let entries = vec![rust_entry("/a.rs", "use foo;", true, None)];

        let map = build_import_map(&seam, &entries);

        assert_eq!(map.len(), 2);
        let scoped = map.get("/a.rs").expect("scoped file present");
        assert_eq!(scoped.len(), 1);
        assert_eq!(scoped[0].source_file, PathBuf::from("/a.rs"));
        let out = map
            .get("/other.rs")
            .expect("out-of-scope file present once");
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].source_file, PathBuf::from("/other.rs"));
    }

    #[test]
    fn build_import_map_empty_snapshot_yields_empty_map() {
        let snap = SnapshotAggregate {
            file_list: Vec::new(),
            import_list: Vec::new(),
            read_content: HashMap::new(),
        };
        let seam = seam_with(snap.file_list.clone(), snap.import_list.clone());
        let entries = vec![rust_entry("/a.rs", "use foo;", true, None)];

        let map = build_import_map(&seam, &entries);

        assert!(map.is_empty());
    }

    #[test]
    fn build_import_map_empty_ws_snapshot_includes_scoped_imports() {
        // `entries` carries `/a.rs` (scoped_keys = {"/a.rs"}), `import_list`
        // (ws_snapshot) has one `/a.rs` import. The ws_snapshot import is
        // filtered out by `!scoped_keys.contains`, and the scoped import is
        // kept: map contains exactly `{"a.rs": [that import]}` and nothing
        // else — pins that scoped imports are included when ws_snapshot
        // contributes nothing for that file.
        let snap = SnapshotAggregate {
            file_list: Vec::new(),
            import_list: vec![import_entry("/a.rs")],
            read_content: HashMap::new(),
        };
        let seam = seam_with(snap.file_list.clone(), snap.import_list.clone());
        let entries = vec![rust_entry(
            "/a.rs",
            "use foo;",
            true,
            Some(ParseMetadata::Rust(RustMetadata::default())),
        )];

        let map = build_import_map(&seam, &entries);

        assert_eq!(map.len(), 1);
        let scoped = map.get("/a.rs").expect("scoped file present");
        assert_eq!(scoped.len(), 1);
        assert_eq!(scoped[0].source_file, PathBuf::from("/a.rs"));
    }

    #[test]
    fn build_import_map_no_doubling_when_scoped_duplicated_in_snapshot() {
        // Two identical scoped ImportEntries (same source_file) in ws_snapshot
        // (the out-of-scope snapshot). `/a.rs` is not in `entries` so
        // `scoped_keys` is empty and both ws_snapshot duplicates pass the
        // `!scoped_keys.contains` filter and reach the map — the production
        // de-dup path must collapse them to one, else len == 2.
        let dup_imp = import_entry("/a.rs");
        let snap = SnapshotAggregate {
            file_list: Vec::new(),
            import_list: vec![dup_imp.clone(), dup_imp.clone()],
            read_content: HashMap::new(),
        };
        let seam = seam_with(snap.file_list.clone(), snap.import_list.clone());
        let entries: Vec<shared_filesystem::taxonomy_filesystem_vo::FileEntry> = Vec::new();

        let map = build_import_map(&seam, &entries);

        assert_eq!(map.len(), 1);
        let scoped = map.get("/a.rs").expect("scoped file present");
        assert_eq!(scoped.len(), 1);
        assert_eq!(scoped[0].source_file, PathBuf::from("/a.rs"));
    }
}

// ─── BFS unit tests ──────────────────────────────────────────────

pub struct _Aes303Placeholder {
    _placeholder: (),
}
