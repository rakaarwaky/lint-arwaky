// PURPOSE: External lint scan business logic, no formatting.
//
// Data Flow:
//   CLI → collect_external_direct → filesystem.build_file_index_with_ignored
//         → detect languages → load config → ExternalLintContext
//         → external_lint.scan_all_with_context → violations
//
// The surface layer performs all pre-computation (language detection, config
// loading) and passes an `ExternalLintContext` to the orchestrator, which
// runs adapters with zero filesystem I/O. Config file reading goes through the
// `IConfigFileReaderAggregate` contract (config-system crate) so this surface
// file only imports `contract(aggregate)` traits (AES201).
use std::path::{Path, PathBuf};
use std::sync::Arc;

use shared_common::FilePath;
use shared_config_system::taxonomy_config_system_vo::AdapterEntry;
use shared_external_lint::ExternalLintRequest;
use shared_external_lint::IExternalLintAggregate;
use shared_external_lint::taxonomy_external_lint_vo::ExternalLintContext;
use shared_filesystem::FilesystemRequest;
use shared_filesystem::contract_filesystem_aggregate::IFilesystemAggregate;

use shared_common::ViolationItem;

/// Direct external lint scan — no subprocess. Used by the CLI `external`
/// subcommand so that subprocess self-invocation from `scan` terminates.
pub fn collect_external_direct(
    path: Option<FilePath>,
    external_lint: Arc<dyn IExternalLintAggregate>,
    filesystem: Arc<dyn IFilesystemAggregate>,
    config_file_reader: &dyn shared_config_system::IConfigFileReaderAggregate,
    filter: Option<String>,
    ignored_paths: &[String],
) -> Result<Vec<ViolationItem>, String> {
    let root = match &path {
        Some(p) => p.value().to_string(),
        None => ".".to_string(),
    };
    if !filesystem
        .execute(FilesystemRequest::path_exists(std::path::Path::new(&root)))
        .into_path_exists()
    {
        return Err(format!("Error: path '{}' does not exist", root));
    }
    let root_fp = FilePath::new(root.clone()).map_err(|_| "invalid path".to_string())?;

    // Build file index for target path (respects config ignored_paths)
    let root_path = std::path::Path::new(&root);
    filesystem.execute(FilesystemRequest::build_file_index_with_ignored(
        root_path,
        ignored_paths,
    ));

    // Detect languages from discovered files (extension check only — no file I/O)
    let files = filesystem
        .execute(FilesystemRequest::discover_files(root_path))
        .into_paths();
    let has_rust = files.iter().any(|f| f.ends_with(".rs"));
    let has_python = files.iter().any(|f| f.ends_with(".py"));
    let has_js = files.iter().any(|f| {
        f.ends_with(".js") || f.ends_with(".jsx") || f.ends_with(".ts") || f.ends_with(".tsx")
    });
    let has_markdown = files
        .iter()
        .any(|f| f.ends_with(".md") || f.ends_with(".markdown"));

    // Load adapter entries from config (pre-computed, no orchestrator I/O)
    let config_entries = load_config_entries(root_path, &*filesystem, config_file_reader);

    let context = ExternalLintContext {
        has_rust,
        has_python,
        has_js,
        has_markdown,
        ignored_paths: ignored_paths.to_vec(),
        ignored_rules: load_ignored_rules(root_path, &*filesystem, config_file_reader),
        config_entries,
    };

    // The external adapters shell out to project-wide tools that recursively
    // walk every directory they are handed. When the target is a workspace
    // root, handing them the whole root makes bandit/ruff/mypy walk sibling
    // agent repos and vendored trees that `filter_outside_member_dirs` drops
    // afterwards — pure wasted time (the bandit bottleneck). Instead, hand
    // each adapter the member dirs directly. The post-scan filter still runs:
    // it is the single source of truth for which files count as in scope, and
    // it protects the non-workspace-root path where the target is used as-is.
    let member_dirs: Vec<PathBuf> = ["crates", "packages", "modules"]
        .into_iter()
        .map(|n| root_path.join(n))
        .filter(|p| p.is_dir())
        .collect();
    let targets: Vec<FilePath> = if member_dirs.is_empty() {
        vec![root_fp.clone()]
    } else {
        member_dirs
            .iter()
            .filter_map(|m| FilePath::new(m.to_string_lossy().to_string()).ok())
            .collect()
    };

    let mut violations: Vec<ViolationItem> = Vec::new();
    for target in &targets {
        let scan_results = external_lint
            .execute(
                shared_external_lint::ExternalLintRequest::scan_all_with_context(target, &context),
            )
            .into_violations();
        violations.extend(
            scan_results
                .values
                .iter()
                .map(ViolationItem::from_lint_result),
        );
    }

    // External tools (bandit, ruff, ...) scan the whole target tree, including
    // files outside the workspace member dirs (crates/ packages/ modules/).
    // Internal scanners already filter via build_file_index_impl; mirror that
    // here so root-level files (e.g. setup.py) are not reported.
    filter_outside_member_dirs(&mut violations, &root, filesystem.as_ref());

    if let Some(ref filter_str) = filter {
        let filter_upper = filter_str.to_uppercase();
        violations.retain(|v| v.code.code().contains(&filter_upper));
    }

    Ok(violations)
}

/// Drop violations whose file is outside any workspace member dir
/// (crates/, packages/, modules/) when the target is (or is inside) a
/// workspace root that defines member dirs. Mirrors `build_file_index_impl`.
pub fn filter_outside_member_dirs(
    violations: &mut Vec<ViolationItem>,
    root: &str,
    fs: &dyn IFilesystemAggregate,
) {
    let root_path = Path::new(root);
    let ws_root = match fs
        .execute(FilesystemRequest::find_workspace_root(root_path))
        .into_root()
    {
        Some(r) => r,
        None => return,
    };
    // External tools (ruff, eslint, …) are handed a canonicalized target and
    // echo back absolute paths, while `find_workspace_root` may resolve to a
    // relative one (e.g. `workspaces-bad`). A relative root makes every
    // `strip_prefix` fail, the fallback keeps the absolute path, and the
    // member-dir check then rejects the entire batch — silently dropping all
    // external violations. Absolutize before comparing.
    let ws_root = if ws_root.is_absolute() {
        ws_root
    } else {
        match std::env::current_dir() {
            Ok(cwd) => cwd.join(&ws_root),
            Err(_) => return,
        }
    };
    let member_dirs: Vec<&str> = ["crates", "packages", "modules"]
        .iter()
        .filter(|d| ws_root.join(d).is_dir())
        .copied()
        .collect();
    if member_dirs.is_empty() {
        return;
    }
    violations.retain(|v| {
        let file_path = Path::new(&v.file.value);
        let rel = file_path.strip_prefix(&ws_root).unwrap_or(file_path);
        member_dirs.iter().any(|d| rel.starts_with(d))
    });
}

/// Walk up from `root_path` looking for lint_arwaky.config.*.yaml files.
/// Returns parsed adapter entries if any config file is found, else empty vec.
///
/// Delegates to the config-system `IConfigFileReaderAggregate` — the surface
/// layer only imports `contract(aggregate)` traits (AES201), so the walk and
/// parse live in the config-system crate, not here.
pub fn load_config_entries(
    root_path: &std::path::Path,
    fs: &dyn IFilesystemAggregate,
    reader: &dyn shared_config_system::IConfigFileReaderAggregate,
) -> Vec<AdapterEntry> {
    reader
        .read_config_files(root_path, &FsView { fs })
        .adapter_entries
}

/// Find the nearest `lint_arwaky.config.yaml` at or above `root_path` and
/// return its `ignored_rules` list.
///
/// Delegates to the config-system `IConfigFileReaderAggregate` (AES201).
/// Read the first `lint_arwaky.config.yaml` found walking up from `root_path`
/// and return its raw content (or `None` when no non-empty config is found).
pub fn find_config_content(
    root_path: &std::path::Path,
    fs: &dyn IFilesystemAggregate,
) -> Option<String> {
    let start = root_path
        .parent()
        .filter(|_| root_path.is_file())
        .unwrap_or(root_path);
    let mut current: Option<&std::path::Path> = Some(start);
    while let Some(dir) = current {
        let cfg_path = dir.join("lint_arwaky.config.yaml");
        let content = fs
            .execute(FilesystemRequest::read_file_result(&cfg_path))
            .into_content_opt();
        if let Some(content) = content
            && !content.is_empty()
        {
            return Some(content);
        }
        current = dir.parent().filter(|&p| p != dir);
    }
    None
}

pub fn load_ignored_rules(
    root_path: &std::path::Path,
    fs: &dyn IFilesystemAggregate,
    reader: &dyn shared_config_system::IConfigFileReaderAggregate,
) -> Vec<String> {
    reader
        .read_config_files(root_path, &FsView { fs })
        .ignored_rules
}

/// Adapts an `IFilesystemAggregate` to the `IConfigFileReadFs` view the
/// config reader needs, so surface callers keep passing the aggregate they
/// already hold.
struct FsView<'a> {
    fs: &'a dyn IFilesystemAggregate,
}

impl shared_config_system::IConfigFileReadFs for FsView<'_> {
    fn path_exists(&self, path: &std::path::Path) -> bool {
        self.fs
            .execute(FilesystemRequest::path_exists(path))
            .into_path_exists()
    }

    fn read_file(&self, path: &std::path::Path) -> Option<String> {
        self.fs
            .execute(FilesystemRequest::read_file_result(path))
            .into_content_opt()
    }
}

pub fn collect_external(
    path: Option<FilePath>,
    external_lint: Arc<dyn IExternalLintAggregate>,
    filter: Option<String>,
    filesystem: Arc<dyn IFilesystemAggregate>,
    config_file_reader: &dyn shared_config_system::IConfigFileReaderAggregate,
) -> Result<Vec<ViolationItem>, String> {
    let ignored: Vec<String> = Vec::new();
    collect_external_direct(
        path,
        external_lint,
        filesystem,
        config_file_reader,
        filter,
        &ignored,
    )
}

/// Run the external adapters against a single-file target.
///
/// The language flags come from the file's own extension rather than from a
/// project walk: a lone `.md` file *is* a Markdown-only project, which is what
/// makes the MarkdownLint adapter fire on `scan README.md`. Every other adapter
/// shells out to a project-wide tool whose findings name other files, so the
/// caller's scope filter drops them anyway.
pub fn collect_single_file_external(
    scan_root: &Path,
    ignored: &[String],
    external_lint: &Arc<dyn IExternalLintAggregate>,
    config_entries: Vec<AdapterEntry>,
    fs: &dyn IFilesystemAggregate,
    config_file_reader: &dyn shared_config_system::IConfigFileReaderAggregate,
) -> Vec<ViolationItem> {
    let context = single_file_external_context(
        scan_root,
        ignored,
        config_entries,
        load_ignored_rules(scan_root, fs, config_file_reader),
    );
    let Ok(target) = FilePath::new(scan_root.to_string_lossy().to_string()) else {
        return Vec::new();
    };
    external_lint
        .execute(ExternalLintRequest::scan_all_with_context(
            &target, &context,
        ))
        .into_violations()
        .values
        .iter()
        .map(ViolationItem::from_lint_result)
        .collect()
}

/// Build the external-lint context for a single-file target from the file's own
/// extension. Public so the extension-to-language mapping is directly testable:
/// it is the whole reason `scan README.md` reaches the MarkdownLint adapter.
pub fn single_file_external_context(
    scan_root: &Path,
    ignored: &[String],
    config_entries: Vec<AdapterEntry>,
    ignored_rules: Vec<String>,
) -> ExternalLintContext {
    let extension = scan_root.extension().and_then(|e| e.to_str()).unwrap_or("");
    ExternalLintContext {
        has_rust: extension == "rs",
        has_python: extension == "py",
        has_js: matches!(extension, "js" | "jsx" | "ts" | "tsx"),
        has_markdown: matches!(extension, "md" | "markdown"),
        ignored_paths: ignored.to_vec(),
        ignored_rules,
        config_entries,
    }
}
