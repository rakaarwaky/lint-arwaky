// Unit tests for CapabilitiesFileSystemIO — FR-003: File I/O & Directory Operations.
use filesystem_lint_arwaky::capabilities_filesystem_io::CapabilitiesFileSystemIO;
use shared::filesystem::contract_filesystem_io_protocol::IFileSystemIOProtocol;
use shared::filesystem::taxonomy_filesystem_vo::FileExtension;
use std::path::Path;
use tempfile::TempDir;

fn make_io() -> CapabilitiesFileSystemIO {
    CapabilitiesFileSystemIO::with_default_timing()
}

#[test]
fn path_exists_for_existing_file() {
    let tmp = TempDir::new().unwrap();
    let file = tmp.path().join("test.txt");
    std::fs::write(&file, "hello").unwrap();
    let io = make_io();
    assert!(io.path_exists(&file));
}

#[test]
fn path_exists_for_missing_file() {
    let io = make_io();
    assert!(!io.path_exists(Path::new("/nonexistent_file_12345.txt")));
}

#[test]
fn is_dir_for_directory() {
    let tmp = TempDir::new().unwrap();
    let io = make_io();
    assert!(io.is_dir(tmp.path()));
}

#[test]
fn is_file_for_regular_file() {
    let tmp = TempDir::new().unwrap();
    let file = tmp.path().join("test.txt");
    std::fs::write(&file, "hello").unwrap();
    let io = make_io();
    assert!(io.is_file(&file));
}

#[test]
fn read_and_write_roundtrip() {
    let tmp = TempDir::new().unwrap();
    let file = tmp.path().join("roundtrip.txt");
    let io = make_io();
    io.write_string(&file, "hello world").unwrap();
    let content = io.read_to_string(&file).unwrap();
    assert_eq!(content, "hello world");
}

#[test]
fn write_creates_parent_dirs() {
    let tmp = TempDir::new().unwrap();
    let file = tmp.path().join("deep").join("nested").join("file.txt");
    let io = make_io();
    io.create_dir_all(file.parent().unwrap()).unwrap();
    io.write_string(&file, "nested content").unwrap();
    assert!(io.path_exists(&file));
}

#[test]
fn copy_file_works() {
    let tmp = TempDir::new().unwrap();
    let src = tmp.path().join("src.txt");
    let dst = tmp.path().join("dst.txt");
    let io = make_io();
    io.write_string(&src, "copy me").unwrap();
    let bytes = io.copy_file(&src, &dst).unwrap();
    assert!(bytes > 0);
    assert_eq!(io.read_to_string(&dst).unwrap(), "copy me");
}

#[test]
fn remove_file_works() {
    let tmp = TempDir::new().unwrap();
    let file = tmp.path().join("to_delete.txt");
    let io = make_io();
    io.write_string(&file, "delete me").unwrap();
    assert!(io.path_exists(&file));
    io.remove_file(&file).unwrap();
    assert!(!io.path_exists(&file));
}

#[test]
fn remove_dir_all_works() {
    let tmp = TempDir::new().unwrap();
    let dir = tmp.path().join("to_delete_dir");
    let io = make_io();
    io.create_dir_all(&dir).unwrap();
    io.write_string(&dir.join("inner.txt"), "x").unwrap();
    io.remove_dir_all(&dir).unwrap();
    assert!(!io.path_exists(&dir));
}

#[test]
fn get_file_stem_extracts_name() {
    let io = make_io();
    assert_eq!(io.get_file_stem("main.rs"), "main");
    assert_eq!(io.get_file_stem("config.yaml"), "config");
    assert_eq!(io.get_file_stem("path/to/file.py"), "file");
}

#[test]
fn get_basename_extracts_filename() {
    let io = make_io();
    assert_eq!(io.get_basename("main.rs"), "main.rs");
    assert_eq!(io.get_basename("/path/to/file.py"), "file.py");
}

#[test]
fn get_parent_extracts_dir() {
    let io = make_io();
    assert_eq!(io.get_parent("/path/to/file.rs"), "/path/to");
    assert_eq!(io.get_parent("file.rs"), "");
}

#[test]
fn is_source_file_detects_extensions() {
    let io = make_io();
    assert!(io.is_source_file(Path::new("main.rs")));
    assert!(io.is_source_file(Path::new("app.py")));
    assert!(io.is_source_file(Path::new("index.ts")));
    assert!(io.is_source_file(Path::new("utils.js")));
    assert!(!io.is_source_file(Path::new("image.png")));
    assert!(!io.is_source_file(Path::new("data.json")));
}

#[test]
fn is_source_ext_validates() {
    let io = make_io();
    assert!(io.is_source_ext(&FileExtension::new("rs").unwrap()));
    assert!(io.is_source_ext(&FileExtension::new("py").unwrap()));
    assert!(io.is_source_ext(&FileExtension::new("ts").unwrap()));
    assert!(!io.is_source_ext(&FileExtension::new("png").unwrap()));
}

#[test]
fn is_python_file_detects_python() {
    let io = make_io();
    assert!(io.is_python_file(Path::new("module.py")));
    assert!(!io.is_python_file(Path::new("test.pyi")));
    assert!(!io.is_python_file(Path::new("file.rs")));
}

#[test]
fn scan_directory_finds_files() {
    let tmp = TempDir::new().unwrap();
    std::fs::write(tmp.path().join("a.rs"), "fn a() {}").unwrap();
    std::fs::write(tmp.path().join("b.py"), "# b").unwrap();
    std::fs::write(tmp.path().join("c.txt"), "not source").unwrap();
    let io = make_io();
    let files = io.scan_directory_with_ignored(tmp.path(), &[]);
    let names: Vec<String> = files
        .iter()
        .filter_map(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
        .collect();
    assert!(names.contains(&"a.rs".to_string()));
    assert!(names.contains(&"b.py".to_string()));
}

#[test]
fn scan_directory_with_ignored_excludes() {
    let tmp = TempDir::new().unwrap();
    let sub = tmp.path().join("target");
    std::fs::create_dir_all(&sub).unwrap();
    std::fs::write(sub.join("build.rs"), "fn main() {}").unwrap();
    std::fs::write(tmp.path().join("src.rs"), "fn src() {}").unwrap();
    let io = make_io();
    let files = io.scan_directory_with_ignored(tmp.path(), &["target".to_string()]);
    let has_target = files.iter().any(|p| p.to_string_lossy().contains("target"));
    assert!(!has_target, "target/ should be excluded");
}

#[test]
fn read_dir_entries_as_pathbuf() {
    let tmp = TempDir::new().unwrap();
    std::fs::write(tmp.path().join("a.txt"), "a").unwrap();
    std::fs::write(tmp.path().join("b.txt"), "b").unwrap();
    let io = make_io();
    let entries = io.read_dir_entries_as_pathbuf(tmp.path()).unwrap();
    assert_eq!(entries.len(), 2);
}

#[test]
fn canonicalize_resolves_path() {
    let tmp = TempDir::new().unwrap();
    let file = tmp.path().join("test.txt");
    std::fs::write(&file, "x").unwrap();
    let io = make_io();
    let canon = io.canonicalize(&file).unwrap();
    assert!(canon.exists());
}

#[test]
fn metadata_returns_info() {
    let tmp = TempDir::new().unwrap();
    let file = tmp.path().join("meta.txt");
    std::fs::write(&file, "metadata").unwrap();
    let io = make_io();
    let meta = io.metadata(&file).unwrap();
    assert!(meta.is_file());
}

#[test]
fn parse_output_lines_filters() {
    let io = make_io();
    let lines = io.parse_output_lines("line1\n\nline2\n  \nline3\n");
    assert_eq!(lines, vec!["line1", "line2", "line3"]);
}

#[test]
fn timing_returns_default() {
    let io = make_io();
    let timing = io.timing();
    assert_eq!(timing.total_ms, 0);
}

#[test]
fn set_permissions_works() {
    let tmp = TempDir::new().unwrap();
    let file = tmp.path().join("perm.txt");
    std::fs::write(&file, "x").unwrap();
    let io = make_io();
    io.set_permissions(&file, 0o644).unwrap();
    let meta = io.metadata(&file).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        assert_eq!(meta.permissions().mode() & 0o777, 0o644);
    }
}

#[test]
fn read_nonexistent_file_returns_error() {
    let io = make_io();
    let result = io.read_to_string(Path::new("/nonexistent_file_999.txt"));
    assert!(result.is_err());
}
