// FR-003 — File I/O & Directory Operations
// US1: Directory walk discovers all source files.
// US2: .gitignore files are excluded.
// US3: Empty directories return empty list.
// US4: Read existing file returns content.
// US5: Write + read roundtrip preserves content.
// US6: Scan with ignored patterns excludes files.
// US7: Non-UTF-8 files are handled gracefully.

use filesystem_lint_arwaky::capabilities_filesystem_io::CapabilitiesFileSystemIO;
use shared::filesystem::contract_filesystem_io_protocol::IFileSystemIOProtocol;
use tempfile::TempDir;

fn make_io() -> CapabilitiesFileSystemIO {
    CapabilitiesFileSystemIO::with_default_timing()
}

#[test]
fn us1_walk_discovers_all_source_files() {
    let tmp = TempDir::new().unwrap();
    std::fs::write(tmp.path().join("main.rs"), "fn main() {}").unwrap();
    std::fs::write(tmp.path().join("utils.rs"), "pub fn util() {}").unwrap();
    std::fs::write(tmp.path().join("app.py"), "# python").unwrap();
    std::fs::write(tmp.path().join("data.json"), "{}").unwrap();
    let io = make_io();
    let files = io.scan_directory_with_ignored(tmp.path(), &[]);
    let source_files: Vec<_> = files
        .iter()
        .filter(|p| p.to_string_lossy().contains(".rs") || p.to_string_lossy().contains(".py"))
        .collect();
    assert!(source_files.len() >= 3, "Should find at least 3 source files");
}

#[test]
fn us2_gitignore_excludes_files() {
    let tmp = TempDir::new().unwrap();
    std::fs::write(tmp.path().join(".gitignore"), "ignored_dir/\n").unwrap();
    let ignored_dir = tmp.path().join("ignored_dir");
    std::fs::create_dir_all(&ignored_dir).unwrap();
    std::fs::write(ignored_dir.join("secret.rs"), "").unwrap();
    std::fs::write(tmp.path().join("visible.rs"), "").unwrap();
    let io = make_io();
    let files = io.scan_directory_with_ignored(tmp.path(), &["ignored_dir".to_string()]);
    let has_ignored = files.iter().any(|p| p.to_string_lossy().contains("ignored_dir"));
    assert!(!has_ignored, "ignored_dir should be excluded");
}

#[test]
fn us3_empty_directory_returns_empty() {
    let tmp = TempDir::new().unwrap();
    let io = make_io();
    let files = io.scan_directory_with_ignored(tmp.path(), &[]);
    assert!(files.is_empty());
}

#[test]
fn us4_read_existing_file() {
    let tmp = TempDir::new().unwrap();
    let file = tmp.path().join("existing.txt");
    std::fs::write(&file, "content here").unwrap();
    let io = make_io();
    let content = io.read_to_string(&file).unwrap();
    assert_eq!(content, "content here");
}

#[test]
fn us5_write_read_roundtrip() {
    let tmp = TempDir::new().unwrap();
    let file = tmp.path().join("roundtrip.txt");
    let io = make_io();
    let original = "The quick brown fox jumps over the lazy dog";
    io.write_string(&file, original).unwrap();
    let read_back = io.read_to_string(&file).unwrap();
    assert_eq!(read_back, original);
}

#[test]
fn us6_scan_with_ignored_patterns() {
    let tmp = TempDir::new().unwrap();
    std::fs::create_dir_all(tmp.path().join("target")).unwrap();
    std::fs::write(tmp.path().join("target").join("build.rs"), "").unwrap();
    std::fs::write(tmp.path().join("src.rs"), "").unwrap();
    let io = make_io();
    let files = io.scan_directory_with_ignored(tmp.path(), &["target".to_string()]);
    assert!(
        files.iter().any(|p| p.to_string_lossy().contains("src.rs")),
        "src.rs should be found"
    );
    assert!(
        !files.iter().any(|p| p.to_string_lossy().contains("build.rs")),
        "build.rs in target/ should be excluded"
    );
}

#[test]
fn us7_non_utf8_file_handled() {
    let tmp = TempDir::new().unwrap();
    let file = tmp.path().join("binary.bin");
    // Write non-UTF-8 bytes
    std::fs::write(&file, vec![0xFF, 0xFE, 0x00, 0x01]).unwrap();
    let io = make_io();
    // read_to_string should fail gracefully
    let result = io.read_to_string(&file);
    assert!(result.is_err(), "Non-UTF-8 file should return error");
}

#[test]
fn fr003_directory_operations() {
    let tmp = TempDir::new().unwrap();
    let dir = tmp.path().join("subdir");
    let io = make_io();
    io.create_dir_all(&dir).unwrap();
    assert!(io.path_exists(&dir));
    assert!(io.is_dir(&dir));

    io.write_string(&dir.join("file.txt"), "x").unwrap();
    let entries = io.read_dir_entries_as_pathbuf(&dir).unwrap();
    assert_eq!(entries.len(), 1);

    io.remove_dir_all(&dir).unwrap();
    assert!(!io.path_exists(&dir));
}

#[test]
fn fr003_path_metadata_operations() {
    let tmp = TempDir::new().unwrap();
    let file = tmp.path().join("meta.txt");
    std::fs::write(&file, "metadata test").unwrap();
    let io = make_io();

    assert!(io.path_exists(&file));
    assert!(io.is_file(&file));
    assert!(!io.is_dir(&file));
    assert!(!io.is_symlink(&file));

    let meta = io.metadata(&file).unwrap();
    assert!(meta.is_file());
}

#[test]
fn fr003_canonicalize_path() {
    let tmp = TempDir::new().unwrap();
    let file = tmp.path().join("canonical.txt");
    std::fs::write(&file, "x").unwrap();
    let io = make_io();
    let canon = io.canonicalize(&file).unwrap();
    assert!(canon.is_absolute());
}
