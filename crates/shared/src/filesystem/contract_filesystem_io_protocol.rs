// Contract layer — filesystem IO protocol trait
// File I/O, path operations, directory operations, process execution, scan timing
// Responsibilities: low-level filesystem access, path metadata, process spawning

use crate::common::taxonomy_display_content_vo::DisplayContent;
use crate::common::taxonomy_path_vo::FilePath;
use crate::filesystem::taxonomy_filesystem_vo::{FileEntry, FileExtension, ScanTiming};
use std::path::{Path, PathBuf};

/// Filesystem IO protocol — low-level file I/O, path operations, directory ops, process execution.
/// Consumers import only this trait when they need filesystem access without parse/graph/workspace concerns.
pub trait IFileSystemIOProtocol: Send + Sync {
    // ═══════════════════════════════════════════════════════════
    // Path Operations
    // ═══════════════════════════════════════════════════════════

    /// Check if path exists.
    fn path_exists(&self, path: &Path) -> bool;

    /// Check if path is a directory.
    fn is_dir(&self, path: &Path) -> bool;

    /// Check if path is a file.
    fn is_file(&self, path: &Path) -> bool;

    /// Check if path should be ignored.
    fn should_ignore(
        &self,
        path: &crate::common::taxonomy_path_vo::FilePath,
        ignored: &[String],
    ) -> bool;

    /// Canonicalize path (resolve symlinks).
    fn canonicalize(&self, path: &Path) -> Result<PathBuf, std::io::Error>;

    /// Canonicalize path to absolute string.
    fn canonicalize_path_str(&self, path: &FilePath) -> String;

    /// Check if path is a symlink.
    fn is_symlink(&self, path: &Path) -> bool;

    /// Get file metadata.
    fn metadata(&self, path: &Path) -> Result<std::fs::Metadata, std::io::Error>;

    /// Get symlink metadata (does not follow symlinks).
    fn symlink_metadata(&self, path: &Path) -> Result<std::fs::Metadata, std::io::Error>;

    /// Extract file stem from path.
    fn get_file_stem<'a>(&self, path: &'a str) -> &'a str;

    /// Check if path has a source file extension.
    fn is_source_file(&self, path: &Path) -> bool;

    /// Check if extension string is recognized.
    fn is_source_ext(&self, ext: &FileExtension) -> bool;

    /// Get file basename.
    fn get_basename<'a>(&self, path: &'a str) -> &'a str;

    /// Get parent directory path.
    fn get_parent<'a>(&self, path: &'a str) -> &'a str;

    /// Check if a path is a Python source file.
    fn is_python_file(&self, path: &Path) -> bool;

    // ═══════════════════════════════════════════════════════════
    // Directory Operations
    // ═══════════════════════════════════════════════════════════

    /// List directory entries with ignore filter.
    fn scan_directory_with_ignored(&self, dir: &Path, ignored: &[String]) -> Vec<PathBuf>;

    /// Check if directory should be ignored.
    fn is_ignored_dir(&self, dir: &Path, ignored: &[String]) -> bool;

    /// Read directory entries as Vec<PathBuf>.
    fn read_dir_entries_as_pathbuf(&self, dir: &Path) -> Result<Vec<PathBuf>, std::io::Error>;

    /// List directory entries, skipping hidden files (starting with '.').
    fn list_directory_filtered(&self, path: &FilePath) -> Vec<FileEntry>;

    /// Read up to `max_lines` lines of a file with line-numbered formatting.
    fn read_file_preview(&self, path: &FilePath, max_lines: usize) -> DisplayContent;

    // ═══════════════════════════════════════════════════════════
    // File Read/Write
    // ═══════════════════════════════════════════════════════════

    /// Read file content to string.
    fn read_to_string(&self, path: &Path) -> Result<String, std::io::Error>;

    /// Write string to file.
    fn write_string(&self, path: &Path, content: &str) -> Result<(), std::io::Error>;

    /// Copy file from src to dst.
    fn copy_file(&self, src: &Path, dst: &Path) -> Result<u64, std::io::Error>;

    /// Create directory and all parents.
    fn create_dir_all(&self, path: &Path) -> Result<(), std::io::Error>;

    /// Remove directory recursively.
    fn remove_dir_all(&self, path: &Path) -> Result<(), std::io::Error>;

    /// Set file permissions (Unix mode bits).
    fn set_permissions(&self, path: &Path, mode: u32) -> std::io::Result<()>;

    /// Remove a file.
    fn remove_file(&self, path: &Path) -> std::io::Result<()>;

    // ═══════════════════════════════════════════════════════════
    // Process Execution
    // ═══════════════════════════════════════════════════════════

    /// Execute a git command and return stdout/stderr/success.
    fn run_git_command(&self, args: &[&str], dir: &str) -> (String, String, bool);

    /// Parse command output into trimmed non-empty lines.
    fn parse_output_lines(&self, output: &str) -> Vec<String>;

    /// Execute an external command with working directory.
    fn run_external_command_in(
        &self,
        name: &str,
        args: &[&str],
        current_dir: &str,
    ) -> (String, String, bool);

    // ═══════════════════════════════════════════════════════════
    // Scan Timing
    // ═══════════════════════════════════════════════════════════

    /// Get timing breakdown of last scan.
    fn timing(&self) -> &ScanTiming;
}
