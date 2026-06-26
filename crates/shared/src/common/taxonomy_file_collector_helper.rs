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
        // (3) Bare segment — match any segment anywhere in the path.
        if segments.contains(&pat.as_str()) {
            return true;
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
                let dir_name = path.file_name().map(|n| n.to_string_lossy()).unwrap_or_default();
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

#[cfg(test)]
mod tests {
    use super::*;

    fn ignored(patterns: &[&str]) -> Vec<String> {
        patterns.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn absolute_prefix_matches_at_any_depth() {
        // The pattern must match anywhere along the path, not only at the
        // beginning — `rel_path` is typically absolute
        // (`/home/raka/.../test-workspaces/crates/foo.rs`).
        let ig = ignored(&["/test-workspaces"]);
        assert!(is_path_ignored(
            "/home/raka/mcp-arwaky/lint-arwaky/test-workspaces",
            &ig
        ));
        assert!(is_path_ignored(
            "/home/raka/mcp-arwaky/lint-arwaky/test-workspaces/crates/foo.rs",
            &ig
        ));
        // Bare (no leading slash) and relative forms must also match.
        assert!(is_path_ignored("test-workspaces", &ig));
        assert!(is_path_ignored("test-workspaces/crates/foo.rs", &ig));
    }

    #[test]
    fn absolute_prefix_does_not_match_unrelated_segment() {
        let ig = ignored(&["/test-workspaces"]);
        // Unrelated paths must NOT match.
        assert!(!is_path_ignored("/home/not-test-workspaces/foo.rs", &ig));
        assert!(!is_path_ignored(
            "/home/raka/lint-arwaky/crates/test.rs",
            &ig
        ));
        // Identical name as a mid-segment of an unrelated path must NOT match
        // (`not-test-workspaces` is its own segment, not `test-workspaces`).
        assert!(!is_path_ignored("/home/not-test-workspaces", &ig));
    }

    #[test]
    fn absolute_prefix_nested_path() {
        let ig = ignored(&["/packages/vscode-extension"]);
        assert!(is_path_ignored(
            "packages/vscode-extension/src/extension.ts",
            &ig
        ));
        assert!(!is_path_ignored("packages/some-other/src/foo.ts", &ig));
    }

    #[test]
    fn bare_segment_matches_anywhere() {
        // `node_modules` (no leading slash) should match anywhere.
        let ig = ignored(&["node_modules"]);
        assert!(is_path_ignored("node_modules/lodash/index.js", &ig));
        assert!(is_path_ignored("frontend/node_modules/react/index.js", &ig));
    }

    #[test]
    fn suffix_glob_matches_minified_vendor_files() {
        let ig = ignored(&[".min.js", ".min.css"]);
        assert!(is_path_ignored(
            "packages/vscode-extension/media/cytoscape.min.js",
            &ig
        ));
        assert!(is_path_ignored("static/style.min.css", &ig));
        // Must NOT match a regular `.js` file.
        assert!(!is_path_ignored("packages/foo/index.js", &ig));
    }

    #[test]
    fn empty_pattern_ignored() {
        let ig = ignored(&[""]);
        assert!(!is_path_ignored("anything.rs", &ig));
    }

    #[test]
    fn multiple_patterns_any_match() {
        let ig = ignored(&["/target", "/test-workspaces", ".min.js"]);
        assert!(is_path_ignored("/home/raka/target/debug/foo.rs", &ig));
        assert!(is_path_ignored("/home/raka/test-workspaces/foo.rs", &ig));
        assert!(is_path_ignored("/home/raka/lib/vendor.min.js", &ig));
        assert!(!is_path_ignored("/home/raka/crates/foo.rs", &ig));
    }

    #[test]
    fn packages_pattern_excludes_only_packages_segment() {
        let ig = ignored(&["/packages"]);
        // Unrelated paths must not match.
        assert!(!is_path_ignored("/home/raka/crates/foo.rs", &ig));
        // Path that DOES contain `packages` segment must match (at any depth).
        assert!(is_path_ignored("/home/raka/packages/foo.ts", &ig));
        assert!(is_path_ignored(
            "/home/raka/packages/vscode-extension/src/extension.ts",
            &ig
        ));
        // Same name as mid-segment of an unrelated path must not match.
        assert!(!is_path_ignored(
            "/home/raka/crates/packages-fake/foo.ts",
            &ig
        ));
    }
}
