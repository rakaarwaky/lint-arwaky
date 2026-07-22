extern crate shared_lint_arwaky as shared;

use shared::import_rules::utility_path_normalizer::{extract_layer_from_prefix, get_relative_path};

// ─── Regression Tests for Phase 3 Fixes ──────────────────────────────────────────

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
    let result = get_relative_path("nonexistent/root/file.rs", "nonexistent/root");
    assert_eq!(result, "file.rs");
}

/// Regression test: get_relative_path handles trailing slashes correctly.
#[test]
fn relative_path_trailing_slash_handling() {
    // The root_dir should strip trailing slashes before comparison
    let result = get_relative_path("nonexistent/root/file.rs", "nonexistent/root/");
    assert_eq!(result, "file.rs");
}

/// Regression test: extract_layer_from_prefix extracts layer from filename prefix.
#[test]
fn extract_layer_from_prefix_agent() {
    assert_eq!(
        extract_layer_from_prefix("agent_my_file.rs"),
        Some("agent".to_string())
    );
}

#[test]
fn extract_layer_from_prefix_surface() {
    assert_eq!(
        extract_layer_from_prefix("surface_check.rs"),
        Some("surfaces".to_string())
    );
}

#[test]
fn extract_layer_from_prefix_capabilities() {
    assert_eq!(
        extract_layer_from_prefix("capabilities_analyzer.rs"),
        Some("capabilities".to_string())
    );
}

#[test]
fn extract_layer_from_prefix_unknown() {
    assert_eq!(extract_layer_from_prefix("unknown_file.rs"), None);
}
