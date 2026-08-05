// Agent layer — IFileSystemIOProtocol implementation for FilesystemOrchestrator
// Extracted from agent_filesystem_orchestrator.rs (AES301: file too large)
// Pure delegation to deps.io — no private field access needed.

use crate::agent_filesystem_orchestrator::FilesystemOrchestrator;
use shared::filesystem::contract_filesystem_io_protocol::IFileSystemIOProtocol;
use shared::filesystem::taxonomy_filesystem_vo::ScanTiming;
use shared::common::taxonomy_path_vo::FilePath;
use std::path::{Path, PathBuf};

// ═══ IFileSystemIOProtocol (29 methods) ════════════════════

impl IFileSystemIOProtocol for FilesystemOrchestrator {
    // ── Path Operations (15) ─────────────────────────────────

    fn path_exists(&self, path: &Path) -> bool {
        self.deps.io.path_exists(path)
    }

    fn is_dir(&self, path: &Path) -> bool {
        self.deps.io.is_dir(path)
    }

    fn is_file(&self, path: &Path) -> bool {
        self.deps.io.is_file(path)
    }

    fn should_ignore(&self, path: &FilePath, ignored: &[String]) -> bool {
        self.deps.io.should_ignore(path, ignored)
    }

    fn canonicalize(&self, path: &Path) -> Result<PathBuf, std::io::Error> {
        self.deps.io.canonicalize(path)
    }

    fn canonicalize_path_str(&self, path: &FilePath) -> String {
        self.deps.io.canonicalize_path_str(path)
    }

    fn is_symlink(&self, path: &Path) -> bool {
        self.deps.io.is_symlink(path)
    }

    fn metadata(&self, path: &Path) -> Result<std::fs::Metadata, std::io::Error> {
        self.deps.io.metadata(path)
    }

    fn symlink_metadata(&self, path: &Path) -> Result<std::fs::Metadata, std::io::Error> {
        self.deps.io.symlink_metadata(path)
    }

    fn get_file_stem<'a>(&self, path: &'a str) -> &'a str {
        self.deps.io.get_file_stem(path)
    }

    fn is_source_file(&self, path: &Path) -> bool {
        self.deps.io.is_source_file(path)
    }

    fn is_source_ext(
        &self,
        ext: &shared::filesystem::taxonomy_filesystem_vo::FileExtension,
    ) -> bool {
        self.deps.io.is_source_ext(ext)
    }

    fn get_basename<'a>(&self, path: &'a str) -> &'a str {
        self.deps.io.get_basename(path)
    }

    fn get_parent<'a>(&self, path: &'a str) -> &'a str {
        self.deps.io.get_parent(path)
    }

    fn is_python_file(&self, path: &Path) -> bool {
        self.deps.io.is_python_file(path)
    }

    // ── Directory Operations (3) ─────────────────────────────

    fn scan_directory_with_ignored(&self, dir: &Path, ignored: &[String]) -> Vec<PathBuf> {
        self.deps.io.scan_directory_with_ignored(dir, ignored)
    }

    fn is_ignored_dir(&self, dir: &Path, ignored: &[String]) -> bool {
        self.deps.io.is_ignored_dir(dir, ignored)
    }

    fn read_dir_entries_as_pathbuf(&self, dir: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
        self.deps.io.read_dir_entries_as_pathbuf(dir)
    }

    // ── File Read/Write (7) ──────────────────────────────────

    fn read_to_string(&self, path: &Path) -> Result<String, std::io::Error> {
        self.deps.io.read_to_string(path)
    }

    fn write_string(&self, path: &Path, content: &str) -> Result<(), std::io::Error> {
        self.deps.io.write_string(path, content)
    }

    fn copy_file(&self, src: &Path, dst: &Path) -> Result<u64, std::io::Error> {
        self.deps.io.copy_file(src, dst)
    }

    fn create_dir_all(&self, path: &Path) -> Result<(), std::io::Error> {
        self.deps.io.create_dir_all(path)
    }

    fn remove_dir_all(&self, path: &Path) -> Result<(), std::io::Error> {
        self.deps.io.remove_dir_all(path)
    }

    fn set_permissions(&self, path: &Path, mode: u32) -> std::io::Result<()> {
        self.deps.io.set_permissions(path, mode)
    }

    fn remove_file(&self, path: &Path) -> std::io::Result<()> {
        self.deps.io.remove_file(path)
    }

    // ── Process Execution (3) ────────────────────────────────

    fn run_git_command(&self, args: &[&str], dir: &str) -> (String, String, bool) {
        self.deps.io.run_git_command(args, dir)
    }

    fn parse_output_lines(&self, output: &str) -> Vec<String> {
        self.deps.io.parse_output_lines(output)
    }

    fn run_external_command_in(
        &self,
        name: &str,
        args: &[&str],
        current_dir: &str,
    ) -> (String, String, bool) {
        self.deps
            .io
            .run_external_command_in(name, args, current_dir)
    }

    // ── Scan Timing ──────────────────────────────────────────

    fn timing(&self) -> &ScanTiming {
        self.deps.io.timing()
    }
}
