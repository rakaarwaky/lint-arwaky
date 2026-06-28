// PURPOSE: FileCollector — taxonomy utility for collecting lintable source files from a directory tree
use crate::common::taxonomy_language_detector_helper::LanguageDetector;
use crate::common::taxonomy_path_vo::DirectoryPath;
use crate::common::taxonomy_path_vo::FilePath;

/// Return true if `rel_path` should be skipped based on `ignored` patterns.
///
/// Each pattern is matched as a **path segment** rather than a free-text substring. This
/// fixes a long-standing bug where patterns like `/test-workspaces` failed to match the
/// absolute path `/home/.../test-workspaces/crates/...` because the old substring-based
/// matcher was tripped up by leading slashes, leading paths, and unrelated prefixes. The
/// result was that all of `test-workspaces/**` and `packages/vscode-extension/src/**`
/// leaked into `lint-arwaky check .` results even though they were listed in
/// `ignored_paths`.
///
/// Three forms of pattern are supported:
///   1. Absolute-style prefix `"/foo"`, `"/foo/bar"` — matches any path that contains
///      the segments `foo` or `foo/bar` in order, at any depth. The leading slash is
///      dropped before comparison; this works on both absolute paths
///      (`/home/.../test-workspaces/crates/foo.rs`) and relative paths
///      (`test-workspaces/crates/foo.rs`).
///   2. Single segment `"foo"` — matches any path segment equal to `foo`
///      (catches both `foo` at root and `nested/foo` mid-tree).
///   3. Suffix glob `".min.js"`, `"*.bak"` — matches any path whose basename ends with the
///      suffix. Used for vendor minified files like `cytoscape.min.js`.
pub fn is_path_ignored(rel_path: &str, ignored: &[String]) -> bool {
    if rel_path.is_empty() {
        return false;
    }
    let segments: Vec<&str> = rel_path
        .split(['/', '\\'])
        .filter(|s| !s.is_empty())
        .collect();
    for pat in ignored {
        if pat.is_empty() {
            continue;
        }
        // (1) Absolute-style prefix "/foo" or "/foo/bar"
        if let Some(stripped) = pat.strip_prefix('/') {
            if stripped.is_empty() {
                continue;
            }
            let pat_segments: Vec<&str> = stripped
                .split(['/', '\\'])
                .filter(|s| !s.is_empty())
                .collect();
            if pat_segments.is_empty() {
                continue;
            }
            // Match if pat_segments appear contiguously in `segments` at any depth.
            // We do NOT use `starts_with` here because `rel_path` may be absolute
            // (`/home/.../test-workspaces/...`) — the pattern segments can appear
            // anywhere along the path, not just at the very beginning.
            let n_pat = pat_segments.len();
            let n_seg = segments.len();
            if n_seg < n_pat {
                continue;
            }
            for start in 0..=(n_seg - n_pat) {
                if segments[start..start + n_pat] == pat_segments[..] {
                    return true;
                }
            }
            continue;
        }
        // (2) Suffix glob "*.ext" or ".ext" (used for minified vendor files)
        if pat.starts_with("*.") || (pat.starts_with('.') && pat.contains('.')) {
            let suffix = if let Some(s) = pat.strip_prefix('*') {
                s.trim_start_matches('.')
            } else {
                pat.trim_start_matches('.')
            };
            if suffix.is_empty() {
                continue;
            }
            let basename = segments.last().copied().unwrap_or_default();
            if basename.ends_with(suffix) {
                return true;
            }
            continue;
        }
        // (3) Bare segment/pattern — match single segment or multi-segment subpath.
        let pat_segments: Vec<&str> = pat.split(['/', '\\']).filter(|s| !s.is_empty()).collect();
        if pat_segments.len() == 1 {
            if segments.contains(&pat_segments[0]) {
                return true;
            }
        } else if pat_segments.len() > 1 {
            let n_pat = pat_segments.len();
            let n_seg = segments.len();
            if n_seg >= n_pat {
                for start in 0..=(n_seg - n_pat) {
                    if segments[start..start + n_pat] == pat_segments[..] {
                        return true;
                    }
                }
            }
        }
    }
    false
}

/// Collect lintable source files (.rs, .py, .ts, .js, .tsx, .jsx) from a directory tree.
pub fn collect_source_files(
    root_dir: &std::path::Path,
    dir_path: &DirectoryPath,
    ignored: &[String],
) -> Vec<FilePath> {
    let mut files = Vec::new();
    let path = std::path::Path::new(&dir_path.value);
    if path.is_file() {
        let relative_path = match path.strip_prefix(root_dir) {
            Ok(p) => p,
            Err(_) => path,
        };
        let rel_str = relative_path.to_string_lossy();
        if !is_path_ignored(&rel_str, ignored) {
            if let Ok(fp) = FilePath::new(path.to_string_lossy().to_string()) {
                let detector = LanguageDetector::new();
                if detector.is_lintable(&fp) {
                    files.push(fp);
                }
            }
        }
        return files;
    }

    if let Ok(entries) = std::fs::read_dir(&dir_path.value) {
        for entry in entries.flatten() {
            let path = entry.path();
            let relative_path = match path.strip_prefix(root_dir) {
                Ok(p) => p,
                Err(_) => &path,
            };
            let rel_str = relative_path.to_string_lossy();
            if is_path_ignored(&rel_str, ignored) {
                continue;
            }
            if path.is_dir() {
                // Skip Rust integration test directories — tests live in tests/ and
                // should not be scanned by the AES linter.
                let dir_name = path
                    .file_name()
                    .map(|n| n.to_string_lossy())
                    .unwrap_or_default();
                if dir_name == "tests" {
                    continue;
                }
                let sub_dir =
                    DirectoryPath::new(path.to_string_lossy().to_string()).unwrap_or_default();
                files.extend(collect_source_files(root_dir, &sub_dir, ignored));
            } else if let Some(path_str) = path.to_str() {
                if let Ok(fp) = FilePath::new(path_str.to_string()) {
                    let detector = LanguageDetector::new();
                    if detector.is_lintable(&fp) {
                        files.push(fp);
                    }
                }
            }
        }
    }
    files
}
