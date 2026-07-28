extern crate shared_lint_arwaky as shared;

use shared::code_analysis::utility_target_resolver::collect_source_files;
use shared::common::taxonomy_path_vo::DirectoryPath;
use std::path::Path;

// ─── Regression Tests for Phase 3.4 fixes ─────────────────────────────────────

/// Single file scan: a .py file should be returned as a single-element vec.
#[test]
fn collect_source_files_single_py_file() {
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("test_script.py");
    std::fs::write(&file_path, "import os\n").unwrap();

    let dir_path = DirectoryPath::new(file_path.to_string_lossy().to_string()).unwrap();
    let files = collect_source_files(&file_path, &dir_path, &[]);

    assert_eq!(files.len(), 1);
    assert!(files[0].value.ends_with("test_script.py"));
}

/// Single file scan: a .rs file should be returned as a single-element vec.
#[test]
fn collect_source_files_single_rs_file() {
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("capabilities_checker.rs");
    std::fs::write(&file_path, "fn main() {}\n").unwrap();

    let dir_path = DirectoryPath::new(file_path.to_string_lossy().to_string()).unwrap();
    let files = collect_source_files(&file_path, &dir_path, &[]);

    assert_eq!(files.len(), 1);
    assert!(files[0].value.ends_with("capabilities_checker.rs"));
}

/// Single file scan: a .ts file should be returned as a single-element vec.
#[test]
fn collect_source_files_single_ts_file() {
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("index.ts");
    std::fs::write(&file_path, "export const x = 1;\n").unwrap();

    let dir_path = DirectoryPath::new(file_path.to_string_lossy().to_string()).unwrap();
    let files = collect_source_files(&file_path, &dir_path, &[]);

    assert_eq!(files.len(), 1);
    assert!(files[0].value.ends_with("index.ts"));
}

/// Single file scan: non-source file (.txt) should return empty vec.
#[test]
fn collect_source_files_non_source_file_returns_empty() {
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("readme.txt");
    std::fs::write(&file_path, "Hello\n").unwrap();

    let dir_path = DirectoryPath::new(file_path.to_string_lossy().to_string()).unwrap();
    let files = collect_source_files(&file_path, &dir_path, &[]);

    assert_eq!(files.len(), 0);
}

/// Single file scan: a source file matching an ignored pattern should be excluded.
#[test]
fn collect_source_files_ignored_file_returns_empty() {
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("test_script.py");
    std::fs::write(&file_path, "import os\n").unwrap();

    let dir_path = DirectoryPath::new(file_path.to_string_lossy().to_string()).unwrap();
    let ignored = vec!["test_script.py".to_string()];
    let files = collect_source_files(&file_path, &dir_path, &ignored);

    assert_eq!(files.len(), 0);
}

/// Single file scan: a source file matching a glob ignore pattern should be excluded.
#[test]
fn collect_source_files_ignored_glob_returns_empty() {
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("test_script.py");
    std::fs::write(&file_path, "import os\n").unwrap();

    let dir_path = DirectoryPath::new(file_path.to_string_lossy().to_string()).unwrap();
    let ignored = vec!["*.py".to_string()];
    let files = collect_source_files(&file_path, &dir_path, &ignored);

    assert_eq!(files.len(), 0);
}

/// Nonexistent path should return empty vec.
#[test]
fn collect_source_files_nonexistent_path_returns_empty() {
    let path = Path::new("/nonexistent/path/to/file.rs");
    let dir_path = DirectoryPath::new(path.to_string_lossy().to_string()).unwrap();
    let files = collect_source_files(path, &dir_path, &[]);

    assert_eq!(files.len(), 0);
}

/// Directory scan still works as before (regression test).
#[test]
fn collect_source_files_directory_returns_files() {
    let dir = tempfile::tempdir().unwrap();
    let src_dir = dir.path().join("src");
    std::fs::create_dir_all(&src_dir).unwrap();

    let py_file = src_dir.join("capabilities_handler.py");
    std::fs::write(&py_file, "def handle(): pass\n").unwrap();
    let rs_file = src_dir.join("capabilities_handler.rs");
    std::fs::write(&rs_file, "fn handle() {}\n").unwrap();

    let dir_path = DirectoryPath::new(src_dir.to_string_lossy().to_string()).unwrap();
    let files = collect_source_files(&src_dir, &dir_path, &[]);

    assert_eq!(files.len(), 2);
    let names: Vec<&str> = files.iter().map(|f| {
        std::path::Path::new(&f.value)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
    }).collect();
    assert!(names.contains(&"capabilities_handler.py"));
    assert!(names.contains(&"capabilities_handler.rs"));
}
