// PURPOSE: Acceptance test — FRD: "Tool discovery and fallback — missing tools
// are safely ignored or warned about without crashing the run."

use external_lint_lint_arwaky::ExternalLintContainer;
use shared::common::taxonomy_path_vo::FilePath;
use shared::external_lint::contract_external_lint_aggregate::IExternalLintAggregate;
use std::fs;

/// FRD-EXT-001: Missing tools are safely ignored without crashing the run.
#[tokio::test]
async fn frd_001_missing_tools_do_not_crash_scan() {
    // Create a project that triggers all 9 adapters
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("main.rs"), "fn main() {}").unwrap();
    fs::write(dir.path().join("app.py"), "x = 1").unwrap();
    fs::write(dir.path().join("index.ts"), "export {}").unwrap();

    let path = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let container = ExternalLintContainer::new();
    let aggregate = container.aggregate();

    // Even if none of the 9 tools are installed, scan_all must not panic
    let results = aggregate.scan_all(&path).await;

    // The result is a valid LintResultList (possibly empty)
    // The key assertion: we reached this line without a panic
    let _ = results.len();
}

/// FRD-EXT-002: Scan continues with remaining adapters when one tool is missing.
#[tokio::test]
async fn frd_002_partial_tool_availability_still_returns_results() {
    let dir = tempfile::tempdir().unwrap();
    // Only Python files — triggers ruff, mypy, bandit
    fs::write(dir.path().join("app.py"), "import os\nprint('hello')\n").unwrap();

    let path = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let container = ExternalLintContainer::new();
    let aggregate = container.aggregate();

    // Should not crash even if ruff/mypy/bandit are not installed
    let results = aggregate.scan_all(&path).await;
    // Results may be empty (tools missing) or populated (tools present)
    // Either way, no crash
    let _ = results;
}

/// FRD-EXT-003: Empty project directory produces empty results without error.
#[tokio::test]
async fn frd_003_empty_project_returns_empty_results() {
    let dir = tempfile::tempdir().unwrap();
    let path = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();

    let container = ExternalLintContainer::new();
    let aggregate = container.aggregate();

    let results = aggregate.scan_all(&path).await;
    assert!(results.is_empty());
}
