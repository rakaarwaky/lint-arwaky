use shared_lint_arwaky::common::taxonomy_path_vo::{DirectoryPath, FilePath};

#[test]
fn test_file_path_new() {
    let fp = FilePath::new("test.txt").unwrap_or_default();
    assert_eq!(fp.value, "test.txt");
    assert_eq!(fp.extension(), "txt");
    assert!(fp.has_extension("txt"));
    assert!(!fp.has_extension("md"));

    let fp = FilePath::new("path\\to\\file.txt").unwrap_or_default();
    assert_eq!(fp.value, "path/to/file.txt");

    let fp = FilePath::new("path/to/file/").unwrap_or_default();
    assert_eq!(fp.value, "path/to/file");

    let fp = FilePath::new("/").unwrap_or_default();
    assert_eq!(fp.value, "/");

    let fp = FilePath::new("///").unwrap_or_default();
    assert_eq!(fp.value, "/");
}

#[test]
fn test_file_path_invalid() {
    assert!(FilePath::new("").is_err());
    assert!(FilePath::new("   ").is_err());
}

#[test]
fn test_directory_path_new() {
    let dp = DirectoryPath::new("test/dir").unwrap_or_default();
    assert_eq!(dp.value, "test/dir");

    let dp = DirectoryPath::new("test/dir/").unwrap_or_default();
    assert_eq!(dp.value, "test/dir");

    let dp = DirectoryPath::new("/").unwrap_or_default();
    assert_eq!(dp.value, "/");
}

#[test]
fn test_directory_path_invalid() {
    assert!(DirectoryPath::new("").is_err());
    assert!(DirectoryPath::new("   ").is_err());
}

#[test]
fn test_extension_with_dot_slash_prefix() {
    let fp = FilePath::new("./foo.rs").unwrap_or_default();
    assert_eq!(fp.extension(), "rs");
    let fp = FilePath::new("./nested/foo.py").unwrap_or_default();
    assert_eq!(fp.extension(), "py");
    let fp = FilePath::new(".//foo.ts").unwrap_or_default();
    assert_eq!(fp.extension(), "ts");
}

#[test]
fn test_extension_hidden_basename() {
    let fp = FilePath::new(".bashrc").unwrap_or_default();
    assert_eq!(fp.extension(), "");
    let fp = FilePath::new("/home/user/.gitignore").unwrap_or_default();
    assert_eq!(fp.extension(), "");
}

#[test]
fn test_extension_full_path() {
    let fp = FilePath::new("/tmp/bypass_test/capabilities_unwrap_checker.rs").unwrap_or_default();
    assert_eq!(fp.extension(), "rs");
    let fp = FilePath::new("crates/code-analysis/src/foo.rs").unwrap_or_default();
    assert_eq!(fp.extension(), "rs");
}

#[test]
fn test_extension_special_filenames() {
    let fp = FilePath::new("Makefile").unwrap_or_default();
    assert_eq!(fp.extension(), "");
    let fp = FilePath::new("Dockerfile").unwrap_or_default();
    assert_eq!(fp.extension(), "");
}
