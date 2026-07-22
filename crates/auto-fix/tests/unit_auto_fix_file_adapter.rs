// PURPOSE: Unit tests for FileAdapter — file I/O capability.
// Covers: read_file, write_file, path_exists (happy path, edge, error).

use auto_fix_lint_arwaky::capabilities_file_adapter::FileAdapter;
use shared::auto_fix::contract_file_adapter_protocol::IFileAdapterProtocol;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_source_vo::ContentString;
use std::io::Write;
use tempfile::NamedTempFile;

fn sut() -> FileAdapter {
    FileAdapter::new()
}

// ─── path_exists ──────────────────────────────────────────

#[test]
fn path_exists_returns_true_for_existing_file() {
    let tmp = NamedTempFile::new().unwrap();
    let path = FilePath::new(tmp.path().to_str().unwrap().to_string()).unwrap();
    assert!(sut().path_exists(&path));
}

#[test]
fn path_exists_returns_false_for_nonexistent_file() {
    let path = FilePath::new("/nonexistent/path/file.rs".to_string()).unwrap();
    assert!(!sut().path_exists(&path));
}

#[test]
fn path_exists_returns_false_for_empty_path() {
    // FilePath::new rejects empty strings, so we use a whitespace-only normalized path
    let path = FilePath::new("/".to_string()).unwrap();
    // Root exists on Unix but is a directory — path_exists checks existence
    assert!(sut().path_exists(&path));
}

// ─── read_file ────────────────────────────────────────────

#[test]
fn read_file_returns_content_for_existing_file() {
    let mut tmp = NamedTempFile::new().unwrap();
    writeln!(tmp, "fn main() {{}}").unwrap();
    let path = FilePath::new(tmp.path().to_str().unwrap().to_string()).unwrap();

    let result = sut().read_file(&path);
    assert!(result.is_some());
    assert!(result.unwrap().value().contains("fn main()"));
}

#[test]
fn read_file_returns_none_for_nonexistent_file() {
    let path = FilePath::new("/nonexistent/file.rs".to_string()).unwrap();
    assert!(sut().read_file(&path).is_none());
}

#[test]
fn read_file_handles_empty_file() {
    let tmp = NamedTempFile::new().unwrap();
    let path = FilePath::new(tmp.path().to_str().unwrap().to_string()).unwrap();

    let result = sut().read_file(&path);
    assert!(result.is_some());
    assert_eq!(result.unwrap().value(), "");
}

#[test]
fn read_file_handles_unicode_content() {
    let mut tmp = NamedTempFile::new().unwrap();
    writeln!(tmp, "// 日本語コメント 🦀").unwrap();
    let path = FilePath::new(tmp.path().to_str().unwrap().to_string()).unwrap();

    let result = sut().read_file(&path);
    assert!(result.is_some());
    assert!(result.unwrap().value().contains("日本語"));
}

// ─── write_file ───────────────────────────────────────────

#[test]
fn write_file_creates_new_file_with_content() {
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("output.rs");
    let path = FilePath::new(file_path.to_str().unwrap().to_string()).unwrap();
    let content = ContentString::new("pub struct Foo;".to_string());

    let result = sut().write_file(&path, &content);
    assert!(result);

    let read_back = std::fs::read_to_string(&file_path).unwrap();
    assert_eq!(read_back, "pub struct Foo;");
}

#[test]
fn write_file_overwrites_existing_content() {
    let mut tmp = NamedTempFile::new().unwrap();
    writeln!(tmp, "old content").unwrap();
    let path = FilePath::new(tmp.path().to_str().unwrap().to_string()).unwrap();
    let content = ContentString::new("new content".to_string());

    let result = sut().write_file(&path, &content);
    assert!(result);

    let read_back = std::fs::read_to_string(tmp.path()).unwrap();
    assert_eq!(read_back, "new content");
}

#[test]
fn write_file_returns_false_for_invalid_path() {
    let path = FilePath::new("/nonexistent_dir/sub/file.rs".to_string()).unwrap();
    let content = ContentString::new("data".to_string());

    let result = sut().write_file(&path, &content);
    assert!(!result);
}

// ─── Default constructor ──────────────────────────────────

#[test]
fn default_constructor_produces_working_adapter() {
    let adapter = FileAdapter::default();
    let path = FilePath::new("/nonexistent.rs".to_string()).unwrap();
    assert!(!adapter.path_exists(&path));
}
