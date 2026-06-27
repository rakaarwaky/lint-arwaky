use naming_rules_lint_arwaky::agent_naming_orchestrator::NamingOrchestrator;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;

fn make_fp(s: &str) -> FilePath {
    FilePath::new(s.to_string()).unwrap_or_default()
}

// ---------------------------------------------------------------------------
// filter_source_files
// ---------------------------------------------------------------------------

#[test]
fn filter_source_files_includes_rs() {
    let files = FilePathList::new(vec![
        make_fp("src/main.rs"),
        make_fp("src/lib.rs"),
    ]);
    let filtered = NamingOrchestrator::filter_source_files(&files);
    assert_eq!(filtered.values.len(), 2);
}

#[test]
fn filter_source_files_includes_py_js_ts() {
    let files = FilePathList::new(vec![
        make_fp("app.py"),
        make_fp("index.js"),
        make_fp("component.ts"),
        make_fp("component.tsx"),
        make_fp("component.jsx"),
    ]);
    let filtered = NamingOrchestrator::filter_source_files(&files);
    assert_eq!(filtered.values.len(), 5);
}

#[test]
fn filter_source_files_excludes_non_source() {
    let files = FilePathList::new(vec![
        make_fp("data.json"),
        make_fp("image.png"),
        make_fp("Cargo.toml"),
        make_fp("README.md"),
        make_fp("main.rs"),
    ]);
    let filtered = NamingOrchestrator::filter_source_files(&files);
    assert_eq!(filtered.values.len(), 1);
    assert_eq!(filtered.values[0].value, "main.rs");
}

#[test]
fn filter_source_files_empty() {
    let files = FilePathList::new(vec![]);
    let filtered = NamingOrchestrator::filter_source_files(&files);
    assert!(filtered.values.is_empty());
}

#[test]
fn filter_source_files_mixed() {
    let files = FilePathList::new(vec![
        make_fp("main.rs"),
        make_fp("data.json"),
        make_fp("lib.py"),
        make_fp("styles.css"),
    ]);
    let filtered = NamingOrchestrator::filter_source_files(&files);
    assert_eq!(filtered.values.len(), 2);
}
