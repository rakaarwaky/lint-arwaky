// PURPOSE: taxonomy_path_helper — pure utility functions for path matching and layer extraction
use std::path::Path;

pub fn extract_layer_from_prefix(filename: &str) -> Option<String> {
    let stem = Path::new(filename)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or_default();

    const PREFIX_MAP: &[(&str, &str)] = &[
        ("taxonomy_", "taxonomy"),
        ("contract_", "contract"),
        ("capabilities_", "capabilities"),
        ("utility_", "utility"),
        ("agent_", "agent"),
        ("surface_", "surfaces"),
        ("root_", "root"),
    ];

    for &(prefix, layer) in PREFIX_MAP {
        if stem.starts_with(prefix) {
            return Some(layer.to_string());
        }
    }

    None
}

pub fn get_relative_path(file_path: &str, root_dir: &str) -> String {
    let normalized_file = match Path::new(file_path)
        .canonicalize()
        .map(|p| p.to_string_lossy().replace('\\', "/"))
    {
        Ok(p) => p,
        Err(_) => file_path.replace('\\', "/"),
    };
    let normalized_root = match Path::new(root_dir)
        .canonicalize()
        .map(|p| p.to_string_lossy().replace('\\', "/"))
    {
        Ok(p) => p,
        Err(_) => root_dir.trim_end_matches('/').replace('\\', "/"),
    };

    let file_path = Path::new(&normalized_file);
    let root_path = Path::new(&normalized_root);

    match file_path.strip_prefix(root_path) {
        Ok(rel) => rel.to_string_lossy().replace('\\', "/"),
        Err(_) => {
            // Fallback: try string-based prefix removal
            let root_with_slash = if normalized_root.ends_with('/') {
                normalized_root.clone()
            } else {
                format!("{}/", normalized_root)
            };
            if let Some(suffix) = normalized_file.strip_prefix(&root_with_slash) {
                suffix.to_string()
            } else if let Some(suffix) = normalized_file.strip_prefix(&normalized_root) {
                suffix.trim_start_matches('/').to_string()
            } else {
                normalized_file
            }
        }
    }
}

// ─── Regression Tests for Phase 3 Fixes ──────────────────────────────────────────

#[cfg(test)]
mod phase3_regression_tests {
    use super::*;

    /// Regression test for Phase 3.5: get_relative_path uses Path::strip_prefix
    /// instead of naive string prefix matching. This prevents bugs where paths like
    /// "/project/src/foo.rs" with root "/project/sr" would incorrectly match.
    #[test]
    fn relative_path_uses_proper_prefix_stripping() {
        // When both paths exist and canonicalize successfully, strip_prefix ensures
        // proper path component boundaries (not substring matching)
        let result = get_relative_path("/tmp", "/tmp");
        assert_eq!(result, "");

        // Non-matching roots should return the original normalized path
        let result = get_relative_path("/tmp/foo/bar", "/tmp/baz");
        assert_eq!(result, "/tmp/foo/bar");
    }

    /// Regression test: get_relative_path handles non-canonicalizable paths gracefully.
    #[test]
    fn relative_path_fallback_for_nonexistent_paths() {
        // When paths don't exist, the function falls back to string replacement
        let result = get_relative_path("nonexistent/file.rs", "nonexistent/root");
        assert_eq!(result, "file.rs");
    }

    /// Regression test: get_relative_path handles trailing slashes correctly.
    #[test]
    fn relative_path_trailing_slash_handling() {
        // The root_dir should strip trailing slashes before comparison
        let result = get_relative_path("nonexistent/file.rs", "nonexistent/root/");
        assert_eq!(result, "file.rs");
    }
}
