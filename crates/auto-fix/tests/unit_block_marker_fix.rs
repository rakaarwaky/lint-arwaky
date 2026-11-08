// Unit tests — verify BlockMarkerFix rewrites banner lines to canonical form.
use auto_fix_lint_arwaky::IBlockMarkerFixProtocol;
use auto_fix_lint_arwaky::capabilities_block_marker_fix::BlockMarkerFix;
use shared_auto_fix::FixOutcome;
use shared_common::LineNumber;
use shared_common::taxonomy_lint_vo::ContentString;
use shared_common::taxonomy_path_vo::FilePath;
use shared_filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

struct MockIO {
    files: Mutex<HashMap<String, String>>,
}

impl MockIO {
    fn with_files(files: HashMap<String, String>) -> Self {
        Self {
            files: Mutex::new(files),
        }
    }
}

impl IFileSystemIOProtocol for MockIO {
    fn path_exists(&self, path: &Path) -> bool {
        self.files
            .lock()
            .unwrap()
            .contains_key(path.to_string_lossy().as_ref())
    }
    fn read_to_string(&self, path: &Path) -> Result<ContentString, io::Error> {
        self.files
            .lock()
            .unwrap()
            .get(path.to_string_lossy().as_ref())
            .map(|c| ContentString::new(c.clone()))
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "file not found"))
    }
    fn write_string(&self, path: &Path, content: &str) -> Result<(), io::Error> {
        self.files
            .lock()
            .unwrap()
            .insert(path.to_string_lossy().to_string(), content.to_string());
        Ok(())
    }
    // Required IFileSystemIOProtocol methods — all minimal stubs
    fn is_dir(&self, _path: &Path) -> bool {
        false
    }
    fn is_file(&self, _path: &Path) -> bool {
        false
    }
    fn should_ignore(&self, _path: &FilePath, _ignored: &[String]) -> bool {
        false
    }
    fn canonicalize(&self, path: &Path) -> Result<PathBuf, io::Error> {
        Ok(path.to_path_buf())
    }
    fn canonicalize_path_str(&self, path: &FilePath) -> FilePath {
        path.clone()
    }
    fn is_symlink(&self, _path: &Path) -> bool {
        false
    }
    fn metadata(&self, _path: &Path) -> Result<std::fs::Metadata, io::Error> {
        Err(io::Error::new(io::ErrorKind::NotFound, "not found"))
    }
    fn symlink_metadata(&self, _path: &Path) -> Result<std::fs::Metadata, io::Error> {
        Err(io::Error::new(io::ErrorKind::NotFound, "not found"))
    }
    fn get_file_stem<'a>(&self, path: &'a str) -> &'a str {
        Path::new(path).file_stem().unwrap().to_str().unwrap()
    }
    fn is_source_file(&self, _path: &Path) -> bool {
        false
    }
    fn is_source_ext(
        &self,
        _ext: &shared_filesystem::taxonomy_filesystem_vo::FileExtension,
    ) -> bool {
        true
    }
    fn get_basename<'a>(&self, path: &'a str) -> &'a str {
        Path::new(path).file_name().unwrap().to_str().unwrap()
    }
    fn get_parent<'a>(&self, path: &'a str) -> &'a str {
        path
    }
    fn is_python_file(&self, _path: &Path) -> bool {
        false
    }
    fn scan_directory_with_ignored(
        &self,
        _dir: &Path,
        _ignored: &shared_common::taxonomy_common_vo::PatternList,
    ) -> Vec<PathBuf> {
        Vec::new()
    }
    fn is_ignored_dir(
        &self,
        _dir: &Path,
        _ignored: &shared_common::taxonomy_common_vo::PatternList,
    ) -> bool {
        false
    }
    fn read_dir_entries_as_pathbuf(&self, _dir: &Path) -> Result<Vec<PathBuf>, io::Error> {
        Ok(Vec::new())
    }
    fn copy_file(
        &self,
        _src: &Path,
        _dst: &Path,
    ) -> Result<shared_filesystem::taxonomy_filesystem_vo::ByteCount, io::Error> {
        Ok(shared_filesystem::taxonomy_filesystem_vo::ByteCount { bytes: 0 })
    }
    fn create_dir_all(&self, _path: &Path) -> Result<(), io::Error> {
        Ok(())
    }
    fn remove_dir_all(&self, _path: &Path) -> Result<(), io::Error> {
        Ok(())
    }
    fn set_permissions(
        &self,
        _path: &Path,
        _mode: shared_filesystem::taxonomy_filesystem_vo::FileMode,
    ) -> io::Result<()> {
        Ok(())
    }
    fn remove_file(&self, _path: &Path) -> io::Result<()> {
        Ok(())
    }
    fn run_git_command(
        &self,
        _args: &[&str],
        _dir: &str,
    ) -> shared_filesystem::taxonomy_filesystem_vo::GitCommandResult {
        shared_filesystem::taxonomy_filesystem_vo::GitCommandResult::new(
            String::new(),
            String::new(),
            true,
        )
    }
    fn parse_output_lines(
        &self,
        _output: &str,
    ) -> shared_filesystem::taxonomy_filesystem_vo::ParsedLines {
        shared_filesystem::taxonomy_filesystem_vo::ParsedLines::new(vec![])
    }
    fn run_external_command_in(
        &self,
        _name: &shared_common::ToolName,
        _args: &[&str],
        _current_dir: &str,
    ) -> (String, String, bool) {
        (String::new(), String::new(), true)
    }
    fn timing(&self) -> &shared_filesystem::ScanTiming {
        use std::sync::OnceLock;
        static TIMING: OnceLock<shared_filesystem::ScanTiming> = OnceLock::new();
        TIMING.get_or_init(shared_filesystem::ScanTiming::default)
    }
}

#[test]
fn block_marker_canonical_passes() {
    let io = Arc::new(MockIO::with_files(HashMap::from([(
        "/fake/canonical.rs".to_string(),
        "// ─── Block 1: Module Declaration ───\n".to_string(),
    )])));
    let fix = BlockMarkerFix::new(io.clone());
    let fp = FilePath::new("/fake/canonical.rs").unwrap();
    let outcome = fix.fix_block_marker(fp.value(), LineNumber::new(1));
    assert!(
        matches!(outcome, FixOutcome::Applied { changes } if changes == 0),
        "canonical banner must pass with no change, got {outcome:?}"
    );
    assert_eq!(
        io.files.lock().unwrap().get("/fake/canonical.rs").unwrap(),
        "// ─── Block 1: Module Declaration ───\n"
    );
}

#[test]
fn block_marker_name_mismatch_rewritten() {
    let io = Arc::new(MockIO::with_files(HashMap::from([(
        "/fake/mismatch.rs".to_string(),
        "// ─── Block 1: Module DeclarationTypes ───\n".to_string(),
    )])));
    let fix = BlockMarkerFix::new(io.clone());
    let fp = FilePath::new("/fake/mismatch.rs").unwrap();
    let outcome = fix.fix_block_marker(fp.value(), LineNumber::new(1));
    assert!(
        matches!(outcome, FixOutcome::Applied { changes } if changes == 1),
        "mismatched banner must be rewritten, got {outcome:?}"
    );
    assert_eq!(
        io.files.lock().unwrap().get("/fake/mismatch.rs").unwrap(),
        "// ─── Block 1: Module Declaration ───\n"
    );
}

#[test]
fn block_marker_dry_run_applies() {
    let io = Arc::new(MockIO::with_files(HashMap::from([(
        "/fake/dry.rs".to_string(),
        "// ─── Block 2: Module Contracts ───\n".to_string(),
    )])));
    let fix = BlockMarkerFix::new(io.clone());
    let fp = FilePath::new("/fake/dry.rs").unwrap();
    let outcome = fix.fix_block_marker_dry(fp.value(), LineNumber::new(1), true);
    assert!(
        matches!(outcome, FixOutcome::Applied { .. }),
        "dry-run must report applicable, got {outcome:?}"
    );
    assert_eq!(
        io.files.lock().unwrap().get("/fake/dry.rs").unwrap(),
        "// ─── Block 2: Module Contracts ───\n"
    );
}

#[test]
fn block_marker_python_style_rewritten() {
    let io = Arc::new(MockIO::with_files(HashMap::from([(
        "/fake/py.py".to_string(),
        "# ─── Block 3: Module HelpersExtra ───\n".to_string(),
    )])));
    let fix = BlockMarkerFix::new(io.clone());
    let fp = FilePath::new("/fake/py.py").unwrap();
    let outcome = fix.fix_block_marker(fp.value(), LineNumber::new(1));
    assert!(
        matches!(outcome, FixOutcome::Applied { changes } if changes == 1),
        "python banner must be rewritten, got {outcome:?}"
    );
    assert_eq!(
        io.files.lock().unwrap().get("/fake/py.py").unwrap(),
        "# ─── Block 3: Module Helpers ───\n"
    );
}

#[test]
fn block_marker_not_a_banner_skipped() {
    let io = Arc::new(MockIO::with_files(HashMap::from([(
        "/fake/not.rs".to_string(),
        "// just a comment\n".to_string(),
    )])));
    let fix = BlockMarkerFix::new(io.clone());
    let fp = FilePath::new("/fake/not.rs").unwrap();
    let outcome = fix.fix_block_marker(fp.value(), LineNumber::new(1));
    assert!(
        matches!(outcome, FixOutcome::Skipped(_)),
        "non-banner line must be skipped, got {outcome:?}"
    );
}

#[test]
fn block_marker_out_of_bounds_skipped() {
    let io = Arc::new(MockIO::with_files(HashMap::from([(
        "/fake/oob.rs".to_string(),
        "// ─── Block 1: Module Declaration ───\n".to_string(),
    )])));
    let fix = BlockMarkerFix::new(io.clone());
    let fp = FilePath::new("/fake/oob.rs").unwrap();
    let outcome = fix.fix_block_marker(fp.value(), LineNumber::new(99));
    assert!(
        matches!(outcome, FixOutcome::Skipped(_)),
        "out-of-bounds line must be skipped, got {outcome:?}"
    );
}
