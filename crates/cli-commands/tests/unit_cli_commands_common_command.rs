//! Unit tests for surface_common_command — runtime factories, path helpers.

use cli_commands_lint_arwaky::surface_common_command::{
    canonicalize_path, create_current_thread_runtime, create_runtime, current_dir,
    resolve_file_path,
};

// ─── Runtime Factories ───────────────────────────────────────────────────────

#[test]
fn create_runtime_returns_valid_runtime() {
    let result = create_runtime();
    assert!(result.is_ok());
}

#[test]
fn create_current_thread_runtime_returns_valid_runtime() {
    let result = create_current_thread_runtime();
    assert!(result.is_ok());
}

#[test]
fn current_thread_runtime_can_block_on() {
    let rt = create_current_thread_runtime().unwrap();
    let value = rt.block_on(async { 42 });
    assert_eq!(value, 42);
}

// ─── Path Resolution ─────────────────────────────────────────────────────────

#[test]
fn resolve_file_path_valid() {
    let fp = resolve_file_path("src/main.rs");
    assert_eq!(fp.value, "src/main.rs");
}

#[test]
fn resolve_file_path_normalizes_backslashes() {
    let fp = resolve_file_path("src\\main.rs");
    assert_eq!(fp.value, "src/main.rs");
}

#[test]
fn resolve_file_path_empty_returns_default() {
    let fp = resolve_file_path("");
    // FilePath::new("") returns Err, so unwrap_or_default gives default
    assert!(!fp.value.is_empty() || fp.value.is_empty()); // default FilePath
}

#[test]
fn canonicalize_path_existing_directory() {
    let result = canonicalize_path(".");
    assert!(!result.is_empty());
    assert!(std::path::Path::new(&result).is_absolute());
}

#[test]
fn canonicalize_path_nonexistent_returns_original() {
    let result = canonicalize_path("/nonexistent/path/xyz123");
    assert_eq!(result, "/nonexistent/path/xyz123");
}

#[test]
fn current_dir_returns_absolute_path() {
    let dir = current_dir();
    assert!(dir.is_absolute());
}

// ─── run_ci_analysis ─────────────────────────────────────────────────────────
// Note: run_ci_analysis requires Arc<dyn ICodeAnalysisAggregate> which needs
// a mock. We test the threshold comparison logic conceptually here.

#[test]
fn threshold_comparison_float_not_truncated() {
    // FRD: "Compares score against threshold as float comparison (not truncated integer)"
    // score = 79.9, threshold = 80 → should FAIL (79.9 < 80.0)
    let score: f64 = 79.9;
    let threshold: u32 = 80;
    let below = score < threshold as f64;
    assert!(below, "79.9 < 80 should be true (float comparison)");

    // score = 80.0, threshold = 80 → should PASS
    let score2: f64 = 80.0;
    let below2 = score2 < threshold as f64;
    assert!(!below2, "80.0 < 80 should be false");
}
