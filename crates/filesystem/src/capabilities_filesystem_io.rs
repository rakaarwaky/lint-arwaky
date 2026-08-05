// FR: Filesystem IO — Capabilities layer
// Implements IFileSystemIOProtocol by delegating to utility_filesystem_io stateless functions.
// 3-block structure per AES skill.

use crate::utility_filesystem_io;
use shared::common::taxonomy_display_content_vo::DisplayContent;
use shared::common::taxonomy_path_vo::FilePath;
use shared::filesystem::contract_filesystem_io_protocol::IFileSystemIOProtocol;
use shared::filesystem::taxonomy_filesystem_vo::{FileEntry, FileExtension, ScanTiming};
use std::path::{Path, PathBuf};
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct CapabilitiesFileSystemIO {
    timing: Arc<ScanTiming>,
}

impl CapabilitiesFileSystemIO {
    pub fn new(timing: Arc<ScanTiming>) -> Self {
        Self { timing }
    }
}

// ─── Block 2: Public Contract (domain protocol ONLY) ──────

impl IFileSystemIOProtocol for CapabilitiesFileSystemIO {
    // ── Path Operations (15) ─────────────────────────────────

    fn path_exists(&self, path: &Path) -> bool {
        utility_filesystem_io::path_exists(path)
    }

    fn is_dir(&self, path: &Path) -> bool {
        utility_filesystem_io::is_dir(path)
    }

    fn is_file(&self, path: &Path) -> bool {
        utility_filesystem_io::is_file(path)
    }

    fn should_ignore(&self, path: &FilePath, ignored: &[String]) -> bool {
        utility_filesystem_io::is_path_ignored(&path.value, ignored)
    }

    fn canonicalize(&self, path: &Path) -> Result<PathBuf, std::io::Error> {
        utility_filesystem_io::canonicalize(path)
    }

    fn canonicalize_path_str(&self, path: &FilePath) -> String {
        utility_filesystem_io::canonicalize_path_str(&path.value)
    }

    fn is_symlink(&self, path: &Path) -> bool {
        utility_filesystem_io::is_symlink(path)
    }

    fn metadata(&self, path: &Path) -> Result<std::fs::Metadata, std::io::Error> {
        utility_filesystem_io::metadata(path)
    }

    fn symlink_metadata(&self, path: &Path) -> Result<std::fs::Metadata, std::io::Error> {
        utility_filesystem_io::symlink_metadata(path)
    }

    fn get_file_stem<'a>(&self, path: &'a str) -> &'a str {
        utility_filesystem_io::get_file_stem(path)
    }

    fn is_source_file(&self, path: &Path) -> bool {
        utility_filesystem_io::is_source_file(path)
    }

    fn is_source_ext(&self, ext: &FileExtension) -> bool {
        utility_filesystem_io::is_source_ext(&ext.value)
    }

    fn get_basename<'a>(&self, path: &'a str) -> &'a str {
        utility_filesystem_io::get_basename(path)
    }

    fn get_parent<'a>(&self, path: &'a str) -> &'a str {
        utility_filesystem_io::get_parent(path)
    }

    fn is_python_file(&self, path: &Path) -> bool {
        utility_filesystem_io::is_python_file(path)
    }

    // ── Directory Operations (3) ─────────────────────────────

    fn scan_directory_with_ignored(&self, dir: &Path, ignored: &[String]) -> Vec<PathBuf> {
        utility_filesystem_io::scan_directory_with_ignored(dir, ignored)
    }

    fn is_ignored_dir(&self, dir: &Path, ignored: &[String]) -> bool {
        let name = dir.file_name().and_then(|n| n.to_str()).unwrap_or("");
        utility_filesystem_io::is_path_ignored(name, ignored)
    }

    fn read_dir_entries_as_pathbuf(&self, dir: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
        utility_filesystem_io::read_dir_entries_as_pathbuf(dir)
    }

    fn list_directory_filtered(&self, path: &FilePath) -> Vec<FileEntry> {
        let dir_path = Path::new(path.value());
        let paths = self
            .read_dir_entries_as_pathbuf(dir_path)
            .unwrap_or_default();

        let mut entries = Vec::new();
        for entry_path in paths {
            let name = match entry_path.file_name().and_then(|n| n.to_str()) {
                Some(n) => n.to_string(),
                None => continue,
            };
            if name.starts_with('.') {
                continue;
            }
            if let Some(file_entry) = FileEntry::from_path(&entry_path) {
                entries.push(file_entry);
            }
        }
        entries
    }

    fn read_file_preview(&self, path: &FilePath, max_lines: usize) -> DisplayContent {
        let file_path = Path::new(path.value());
        let content = match self.read_to_string(file_path) {
            Ok(c) => c,
            Err(e) => return DisplayContent::new(format!("Cannot read file: {e}")),
        };

        let lines: Vec<&str> = content.lines().take(max_lines).collect();
        let mut output = String::new();
        for (i, line) in lines.iter().enumerate() {
            output.push_str(&format!("{:>4} │ {}\n", i + 1, line));
        }
        let total_lines = content.lines().count();
        if total_lines > max_lines {
            output.push_str(&format!("\n... ({} more lines)", total_lines - max_lines));
        }
        DisplayContent::new(output)
    }

    // ── File Read/Write (7) ──────────────────────────────────

    fn read_to_string(&self, path: &Path) -> Result<String, std::io::Error> {
        utility_filesystem_io::read_to_string(path)
    }

    fn write_string(&self, path: &Path, content: &str) -> Result<(), std::io::Error> {
        utility_filesystem_io::write_string(path, content)
    }

    fn copy_file(&self, src: &Path, dst: &Path) -> Result<u64, std::io::Error> {
        utility_filesystem_io::copy_file(src, dst)
    }

    fn create_dir_all(&self, path: &Path) -> Result<(), std::io::Error> {
        utility_filesystem_io::create_dir_all(path)
    }

    fn remove_dir_all(&self, path: &Path) -> Result<(), std::io::Error> {
        utility_filesystem_io::remove_dir_all(path)
    }

    fn set_permissions(&self, path: &Path, mode: u32) -> std::io::Result<()> {
        utility_filesystem_io::set_permissions(path, mode)
    }

    fn remove_file(&self, path: &Path) -> std::io::Result<()> {
        utility_filesystem_io::remove_file(path)
    }

    // ── Process Execution (3) ────────────────────────────────

    fn run_git_command(&self, args: &[&str], dir: &str) -> (String, String, bool) {
        utility_filesystem_io::run_git_command(args, dir)
    }

    fn parse_output_lines(&self, output: &str) -> Vec<String> {
        utility_filesystem_io::parse_output_lines(output)
    }

    fn run_external_command_in(
        &self,
        name: &str,
        args: &[&str],
        current_dir: &str,
    ) -> (String, String, bool) {
        utility_filesystem_io::run_external_command_in(name, args, current_dir)
    }

    // ── Scan Timing ──────────────────────────────────────────

    fn timing(&self) -> &ScanTiming {
        &self.timing
    }
}

// ─── Block 3: Constructors, Std Traits & Helpers ─────────

impl CapabilitiesFileSystemIO {
    pub fn with_default_timing() -> Self {
        Self {
            timing: Arc::new(ScanTiming {
                cache_ms: 0,
                walk_ms: 0,
                parse_ms: 0,
                extract_ms: 0,
                graph_ms: 0,
                total_ms: 0,
            }),
        }
    }
}
