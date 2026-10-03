// Unit tests — collect_scan with nonexistent path returns error.
#[allow(dead_code, unused_imports)]
#[path = "../../shared/tests/common/mock_filesystem.rs"]
mod mock_filesystem;

// Linked from crates/shared/tests/common/mock_filesystem.rs — the canonical
// mock lives in the shared crate's tests dir; this #[path] link keeps a single
// source of truth across the workspace (T1/D2).

use dispatcher_lint_arwaky::surface_check_action::{
    ScanOptions, collect_scan, collect_scan_with_progress,
};
use dispatcher_lint_arwaky::surface_external_action::single_file_external_context;
use shared_common::FilePath;
use std::path::Path;
use std::sync::Arc;

use mock_filesystem::MockFilesystem;

/// Seam bundle for ScanOptions: io + workspace + parser + aggregate.
fn mock_seam() -> Arc<dispatcher_lint_arwaky::surface_check_action::FilesystemSeam> {
    Arc::new(
        dispatcher_lint_arwaky::surface_check_action::FilesystemSeam {
            workspace: Arc::new(MockFilesystem::new()),
            parser: Arc::new(MockFilesystem::new()),
            aggregate: Arc::new(MockFilesystem::new()),
        },
    )
}

#[test]
fn collect_scan_nonexistent_path_returns_error() {
    let opts = ScanOptions {
        path: Some(FilePath::new("/nonexistent/path/that/does/not/exist".to_string()).unwrap()),
        multi_project_orchestrator: None,
        filter: None,
        member: None,
        filesystem: mock_seam(),
        scan_aggregates: None,
    };
    let result = collect_scan(opts);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("does not exist"));
}

#[test]
fn progress_hook_is_called_before_scan_validation() {
    let opts = ScanOptions {
        path: Some(FilePath::new("/nonexistent/path".to_string()).unwrap()),
        multi_project_orchestrator: None,
        filter: None,
        member: None,
        filesystem: mock_seam(),
        scan_aggregates: None,
    };
    let mut phases = Vec::new();
    let result = collect_scan_with_progress(opts, |phase, done, total| {
        phases.push((phase, done, total));
    });
    assert!(result.is_err());
    assert_eq!(phases[0].0, "Starting scan");
}

#[test]
fn collect_scan_with_filter_returns_error_for_nonexistent() {
    let opts = ScanOptions {
        path: Some(FilePath::new("/nonexistent/path".to_string()).unwrap()),
        multi_project_orchestrator: None,
        filter: Some("AES".to_string()),
        member: None,
        filesystem: mock_seam(),
        scan_aggregates: None,
    };
    let result = collect_scan(opts);
    assert!(result.is_err());
}

// ─── Single-file external context ───
//
// `scan README.md` reaches the external adapters through the language flags
// derived from the file's own extension. `has_markdown` is what selects the
// MarkdownLint adapter, so it is the flag this mapping must get right.

#[test]
fn markdown_file_context_selects_the_markdown_adapter() {
    for name in ["README.md", "notes.markdown"] {
        let ctx = single_file_external_context(Path::new(name), &[], Vec::new(), Vec::new());
        assert!(ctx.has_markdown, "{name} must be seen as Markdown");
        assert!(!ctx.has_rust && !ctx.has_python && !ctx.has_js, "{name}");
    }
}

#[test]
fn source_file_context_selects_only_its_own_language() {
    let cases = [
        ("main.rs", "rust"),
        ("tool.py", "python"),
        ("index.ts", "js"),
        ("view.tsx", "js"),
    ];
    for (name, _) in cases {
        let ctx = single_file_external_context(Path::new(name), &[], Vec::new(), Vec::new());
        assert!(!ctx.has_markdown, "{name} must not select markdownlint");
    }
    assert!(
        single_file_external_context(Path::new("main.rs"), &[], Vec::new(), Vec::new()).has_rust,
        "a .rs file is Rust"
    );
    assert!(
        single_file_external_context(Path::new("tool.py"), &[], Vec::new(), Vec::new()).has_python,
        "a .py file is Python"
    );
    assert!(
        single_file_external_context(Path::new("index.ts"), &[], Vec::new(), Vec::new()).has_js,
        "a .ts file is JS/TS"
    );
}

#[test]
fn extensionless_file_context_selects_no_adapter() {
    let ctx = single_file_external_context(Path::new("Makefile"), &[], Vec::new(), Vec::new());
    assert!(
        !ctx.has_rust && !ctx.has_python && !ctx.has_js && !ctx.has_markdown,
        "an extensionless target is not a language project"
    );
}

#[test]
fn single_file_context_carries_ignore_and_config_entries_through() {
    let ctx = single_file_external_context(
        Path::new("README.md"),
        &["target".to_string()],
        Vec::new(),
        vec!["markdownlint::MD013".to_string()],
    );
    assert_eq!(ctx.ignored_paths, vec!["target".to_string()]);
    assert_eq!(ctx.ignored_rules, vec!["markdownlint::MD013".to_string()]);
}
