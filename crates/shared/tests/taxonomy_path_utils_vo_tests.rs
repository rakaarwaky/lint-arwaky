// PURPOSE: PathUtils::walk_recursive — tests for flat and path-based ignore patterns
use shared_lint_arwaky::common::taxonomy_path_utils_vo::PathUtils;
use std::fs;

fn make_tree(root: &std::path::Path, files: &[&str]) {
    for f in files {
        let p = root.join(f);
        if let Some(parent) = p.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(&p, "content").unwrap();
    }
}

#[test]
fn flat_pattern_ignores_directory_by_name() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    make_tree(root, &["src/main.rs", "tests/foo.rs", "lib.rs"]);
    let result = PathUtils::walk_recursive(root, &["tests"]);
    let names: Vec<String> = result
        .iter()
        .map(|p| p.strip_prefix(root).unwrap().to_string_lossy().into_owned())
        .collect();
    assert!(names.contains(&"src/main.rs".to_string()));
    assert!(names.contains(&"lib.rs".to_string()));
    assert!(!names.iter().any(|n| n.starts_with("tests")));
}

#[test]
fn flat_pattern_ignores_file_by_name() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    make_tree(root, &["src/main.rs", "debug.log", "lib.rs"]);
    let result = PathUtils::walk_recursive(root, &["debug.log"]);
    let names: Vec<String> = result
        .iter()
        .map(|p| p.strip_prefix(root).unwrap().to_string_lossy().into_owned())
        .collect();
    assert!(names.contains(&"src/main.rs".to_string()));
    assert!(names.contains(&"lib.rs".to_string()));
    assert!(!names.contains(&"debug.log".to_string()));
}

#[test]
fn path_pattern_ignores_nested_directory() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    make_tree(
        root,
        &[
            "src/tests/helper.rs",
            "src/main.rs",
            "tests/other.rs",
            "lib.rs",
        ],
    );
    let result = PathUtils::walk_recursive(root, &["src/tests"]);
    let names: Vec<String> = result
        .iter()
        .map(|p| p.strip_prefix(root).unwrap().to_string_lossy().into_owned())
        .collect();
    assert!(names.contains(&"src/main.rs".to_string()));
    assert!(names.contains(&"lib.rs".to_string()));
    assert!(names.contains(&"tests/other.rs".to_string()));
    assert!(!names.iter().any(|n| n.starts_with("src/tests")));
}

#[test]
fn path_pattern_ignores_deeply_nested_subtree() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    make_tree(
        root,
        &[
            "target/debug/build/a.rs",
            "target/release/b.rs",
            "src/main.rs",
        ],
    );
    let result = PathUtils::walk_recursive(root, &["target/debug"]);
    let names: Vec<String> = result
        .iter()
        .map(|p| p.strip_prefix(root).unwrap().to_string_lossy().into_owned())
        .collect();
    assert!(names.contains(&"src/main.rs".to_string()));
    assert!(names.contains(&"target/release/b.rs".to_string()));
    assert!(!names.iter().any(|n| n.starts_with("target/debug")));
}

#[test]
fn multiple_patterns_combined() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    make_tree(
        root,
        &[
            "src/main.rs",
            "src/tests/fixture.rs",
            "target/debug/x.rs",
            "lib.rs",
        ],
    );
    let result = PathUtils::walk_recursive(root, &["src/tests", "target/debug"]);
    let names: Vec<String> = result
        .iter()
        .map(|p| p.strip_prefix(root).unwrap().to_string_lossy().into_owned())
        .collect();
    assert!(names.contains(&"src/main.rs".to_string()));
    assert!(names.contains(&"lib.rs".to_string()));
    assert!(!names.iter().any(|n| n.starts_with("src/tests")));
    assert!(!names.iter().any(|n| n.starts_with("target/debug")));
}

#[test]
fn no_patterns_returns_all_files() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    make_tree(root, &["a/b.rs", "c.rs"]);
    let result = PathUtils::walk_recursive(root, &[]);
    assert_eq!(result.len(), 2);
}

#[test]
fn empty_directory_returns_empty() {
    let tmp = tempfile::tempdir().unwrap();
    let result = PathUtils::walk_recursive(tmp.path(), &["tests"]);
    assert!(result.is_empty());
}

#[test]
fn single_file_not_ignored() {
    let tmp = tempfile::tempdir().unwrap();
    let file = tmp.path().join("solo.rs");
    fs::write(&file, "fn main() {}").unwrap();
    let result = PathUtils::walk_recursive(&file, &["other"]);
    assert_eq!(result.len(), 1);
}

#[test]
fn single_file_ignored() {
    let tmp = tempfile::tempdir().unwrap();
    let file = tmp.path().join("debug.log");
    fs::write(&file, "log").unwrap();
    let result = PathUtils::walk_recursive(&file, &["debug.log"]);
    assert!(result.is_empty());
}

#[test]
fn collect_paths_convenience_wrapper() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    make_tree(root, &["src/main.rs", "tests/foo.rs"]);
    let result = PathUtils::collect_paths(&root.to_string_lossy(), &["tests"]);
    assert_eq!(result.len(), 1);
    assert!(result[0].to_string_lossy().contains("src/main.rs"));
}
