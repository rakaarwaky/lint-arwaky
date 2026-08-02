// Contract layer — aggregate trait for filesystem operations
// Organized by FR per FRD v3.0.0

use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_source_vo::ContentString;
use crate::filesystem::taxonomy_filesystem_vo::{
    FileEntry, FilesystemResult, ImportEntry, ParseWarning, ScanTiming,
};
use std::path::{Path, PathBuf};

/// Aggregate trait — combines all filesystem capabilities into one interface.
/// Organized by FR per FRD v3.0.0.
pub trait IFilesystemAggregate: Send + Sync {
    // ═══════════════════════════════════════════════════════════
    // FR-001: File Discovery
    // ═══════════════════════════════════════════════════════════

    /// Discover source files with content (full mode).
    /// Consumer: code-analysis
    fn discover_files(&self, root: &Path, ignored: &[String]) -> Vec<FileEntry>;

    /// Discover source file paths only (lightweight mode).
    /// Consumer: naming-rules
    fn discover_source_files(&self, root: &Path, ignored: &[String]) -> Vec<FilePath>;

    /// All discovered source files (after pipeline).
    fn file_list(&self) -> &[FileEntry];

    // ═══════════════════════════════════════════════════════════
    // FR-002: AST Parsing
    // ═══════════════════════════════════════════════════════════

    /// Files enriched with parse metadata and parse_ok flag.
    /// Consumer: role-rules
    fn parsed_file_list(&self) -> &[FileEntry];

    /// Parse diagnostics for files that failed to parse.
    fn parse_warnings(&self) -> &[ParseWarning];

    // ═══════════════════════════════════════════════════════════
    // FR-003: Import Data Extraction
    // ═══════════════════════════════════════════════════════════

    /// All extracted import entries across the workspace.
    /// Consumer: import-rules
    fn import_list(&self) -> &[ImportEntry];

    /// Get imports for a specific file.
    fn imports_for(&self, path: &Path) -> Vec<ImportEntry>;

    /// Get all imports (from last scan).
    fn all_imports(&self) -> &[ImportEntry];

    // ═══════════════════════════════════════════════════════════
    // FR-004: Graph Data Construction
    // ═══════════════════════════════════════════════════════════

    /// Forward import graph (file -> files it imports).
    fn dependency_graph(&self) -> &std::collections::HashMap<PathBuf, Vec<PathBuf>>;

    /// Reverse import map (file -> list of files that import it).
    fn reverse_import_map(&self) -> &std::collections::HashMap<PathBuf, Vec<PathBuf>>;

    /// Symbol definition map (trait/class/struct/interface name -> defining file).
    fn symbol_definitions(&self) -> &std::collections::HashMap<String, Vec<PathBuf>>;

    /// Trait implementation map (trait/interface name -> list of implementor files).
    fn trait_implementations(&self) -> &std::collections::HashMap<String, Vec<PathBuf>>;

    /// Check if two files have a dependency relationship.
    fn depends_on(&self, from: &Path, to: &Path) -> bool;

    /// Find circular dependencies.
    fn cycles(&self) -> Vec<Vec<PathBuf>>;

    /// Find orphan files (nothing imports them).
    fn orphan_files(&self) -> Vec<PathBuf>;

    // ═══════════════════════════════════════════════════════════
    // FR-005: Workspace Detection
    // ═══════════════════════════════════════════════════════════

    /// Find workspace root by walking up from start path.
    fn workspace_root(&self, start: &str) -> Option<PathBuf>;

    /// Find workspace root from Path (Result variant).
    fn find_workspace_root_from_path(&self, start: &Path) -> Result<PathBuf, std::io::Error>;

    /// Detect if a path is a single workspace member.
    fn is_member_path(&self, path: &str) -> bool;

    /// Detect if a path is a leaf member.
    fn is_leaf_member_path(&self, path: &str) -> bool;

    /// Detect source directory from project root.
    fn detect_source_dir(&self, project_root: &Path) -> PathBuf;

    /// Detect ConfigLanguage from a file system path.
    fn detect_language_from_path(
        &self,
        path: &str,
    ) -> crate::common::taxonomy_config_language_vo::ConfigLanguage;

    /// Check if any container/entry file under workspace root references identifiers.
    fn check_wired_in_container(&self, workspace_root: &Path, identifiers: &[String]) -> bool;

    /// Resolve a module path relative to base_dir, confined under root.
    fn resolve_orphan_module_path(
        &self,
        root: &Path,
        base_dir: &Path,
        module_path: &str,
    ) -> Option<PathBuf>;

    // ═══════════════════════════════════════════════════════════
    // FR-006: Tool Resolution
    // ═══════════════════════════════════════════════════════════

    /// Check if an executable exists in PATH.
    fn is_executable_in_path(&self, executable: &str) -> bool;

    /// Check if a binary is available in system PATH.
    fn is_binary_available(&self, bin_name: &str) -> bool;

    /// Check if an executable exists in local node_modules/.bin.
    fn has_local_bin(&self, working_dir: &Path, executable: &str) -> bool;

    /// Resolve JS tool command from local node_modules/.bin.
    fn resolve_js_cmd(
        &self,
        executable: &str,
        args: Vec<String>,
        working_dir: &str,
    ) -> Option<Vec<String>>;

    /// Walk up to find JS project root.
    fn resolve_js_working_dir(
        &self,
        path: &crate::common::taxonomy_path_vo::FilePath,
    ) -> crate::common::taxonomy_path_vo::FilePath;

    /// Find parent dir with Cargo.toml.
    fn resolve_cargo_working_dir(
        &self,
        path: &crate::common::taxonomy_path_vo::FilePath,
    ) -> crate::common::taxonomy_path_vo::FilePath;

    /// Find parent dir with Cargo.lock.
    fn resolve_cargo_lock_working_dir(
        &self,
        path: &crate::common::taxonomy_path_vo::FilePath,
    ) -> crate::common::taxonomy_path_vo::FilePath;

    /// Check if directory contains a config file.
    fn has_config_file(&self, dir: &Path) -> bool;

    /// Find Cargo.toml in the given path.
    fn has_cargo_toml(&self, path_str: &str) -> Option<String>;

    /// Find Cargo.lock in the given path.
    fn has_cargo_lock(&self, path_str: &str) -> Option<String>;

    /// Check if path contains Python files (recursive).
    fn has_python_files_recursive(&self, path: &crate::common::taxonomy_path_vo::FilePath) -> bool;

    /// Create default working directory.
    fn default_working_dir(
        &self,
        path: &crate::common::taxonomy_path_vo::FilePath,
    ) -> crate::common::taxonomy_path_vo::FilePath;

    /// No-op apply_fix for linters that cannot auto-fix.
    fn noop_apply_fix(
        &self,
    ) -> Result<
        crate::common::taxonomy_message_vo::ComplianceStatus,
        crate::common::taxonomy_operation_error::LinterOperationError,
    >;

    // ═══════════════════════════════════════════════════════════
    // FR-007: File Cache
    // ═══════════════════════════════════════════════════════════

    /// Read file content from bounded cache.
    fn read_cached(&self, path: &crate::common::taxonomy_path_vo::FilePath) -> ContentString;

    // ═══════════════════════════════════════════════════════════
    // File I/O (utility)
    // ═══════════════════════════════════════════════════════════

    /// Run full pipeline: walk -> parse -> extract -> graph.
    fn run_pipeline(&self, root: &Path, ignored: &[String]);

    /// Run full scan and return FilesystemResult.
    fn scan(&self, root: &Path, ignored: &[String]) -> FilesystemResult;

    /// Get timing breakdown of last scan.
    fn timing(&self) -> &ScanTiming;

    /// Read file content.
    fn read_file(&self, path: &Path) -> Option<String>;

    /// Read file for linting: checks cache, enforces 2MiB size limit.
    fn read_lintable_file(&self, path: &str) -> Result<Option<String>, String>;

    /// Get cached file content (after scan).
    fn get_file_content(&self, path: &Path) -> Option<String>;

    /// Check if a file is in the cache.
    fn has_file(&self, path: &Path) -> bool;

    /// Collect source files from a directory tree.
    fn collect_source_files(&self, root_dir: &Path, ignored: &[String]) -> Vec<FilePath>;

    /// Recursively scan directory for files.
    fn scan_directory_recursive(&self, dir: &Path) -> Vec<String>;

    /// Walk directory and collect paths of all source files.
    fn collect_source_files_from_path(&self, dir: &Path, files: &mut Vec<String>);

    /// Collect file entries (path, content) for each lintable file.
    fn collect_file_entries(&self, files: &[String]) -> Vec<(PathBuf, String)>;

    // ═══════════════════════════════════════════════════════════
    // Path Operations (utility)
    // ═══════════════════════════════════════════════════════════

    /// Check if path exists.
    fn path_exists(&self, path: &Path) -> bool;

    /// Check if path is a directory.
    fn is_dir(&self, path: &Path) -> bool;

    /// Check if path is a file.
    fn is_file(&self, path: &Path) -> bool;

    /// Check if path should be ignored.
    fn should_ignore(&self, path: &str, ignored: &[String]) -> bool;

    /// Canonicalize path (resolve symlinks).
    fn canonicalize(&self, path: &Path) -> Result<PathBuf, std::io::Error>;

    /// Canonicalize path to absolute string.
    fn canonicalize_path_str(&self, path_str: &str) -> String;

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
    fn is_source_ext(&self, ext: &str) -> bool;

    /// Get file basename.
    fn get_basename<'a>(&self, path: &'a str) -> &'a str;

    /// Get parent directory path.
    fn get_parent<'a>(&self, path: &'a str) -> &'a str;

    /// Check if directory contains Python source files.
    fn has_python_files(&self, dir: &Path) -> bool;

    // ═══════════════════════════════════════════════════════════
    // Directory Operations (utility)
    // ═══════════════════════════════════════════════════════════

    /// List directory entries (non-recursive).
    fn scan_directory(&self, dir: &Path) -> Vec<PathBuf>;

    /// List directory entries with ignore filter.
    fn scan_directory_with_ignored(&self, dir: &Path, ignored: &[String]) -> Vec<PathBuf>;

    /// Check if directory should be ignored.
    fn is_ignored_dir(&self, dir: &Path, ignored: &[String]) -> bool;

    /// Read directory entries as Vec<PathBuf>.
    fn read_dir_entries_as_pathbuf(&self, dir: &Path) -> Result<Vec<PathBuf>, std::io::Error>;

    // ═══════════════════════════════════════════════════════════
    // File Write Operations (utility)
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

    /// Write text content to a file.
    fn write_text_to_file(&self, path: &Path, text: &str) -> Result<(), String>;

    // ═══════════════════════════════════════════════════════════
    // Process Execution (utility)
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
}
