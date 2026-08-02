// PURPOSE: Contract layer — aggregate trait for filesystem operations
// Single entry point for rule crates to access filesystem capabilities.
// Implements FR-005 consumer access pattern with granular accessor methods.

use crate::filesystem::taxonomy_filesystem_vo::{FileEntry, FilesystemResult, ImportEntry, ParseWarning, ScanTiming};
use crate::common::taxonomy_source_vo::ContentString;
use crate::common::taxonomy_path_vo::FilePath;
use std::path::{Path, PathBuf};

/// Aggregate trait — combines all filesystem capabilities into one interface.
/// Rule crates depend on this for file I/O, parsing, and dependency queries.
///
/// Pipeline runs once (lazy: triggered on first accessor call).
/// All accessors return references — zero-cost, no clone.
/// Result is immutable after construction (read-only queries only).
pub trait IFilesystemAggregate: Send + Sync {
    // ── Pipeline Trigger ──────────────────────────────────────

    /// Run full pipeline: walk -> cache -> parse -> extract -> graph.
    /// Lazy: pipeline runs on first accessor call. Results cached internally.
    fn run_pipeline(&self, root: &Path, ignored: &[String]);

    /// Run full scan and return FilesystemResult (backward compat).
    fn scan(&self, root: &Path, ignored: &[String]) -> FilesystemResult;

    // ── File Access (FR-001) ─────────────────────────────────

    /// All discovered source files (path, content, language, extension).
    /// Consumers: naming-rules, code-analysis, import-rules.
    fn file_list(&self) -> &[FileEntry];

    // ── Parsed File Access (FR-002) ──────────────────────────

    /// Files enriched with parse metadata and parse_ok flag.
    /// Consumers: role-rules, orphan-detector.
    fn parsed_file_list(&self) -> &[FileEntry];

    // ── Parse Warnings (FR-002) ──────────────────────────────

    /// Parse diagnostics for files that failed to parse.
    /// Consumers: all.
    fn parse_warnings(&self) -> &[ParseWarning];

    // ── Import Access (FR-003) ───────────────────────────────

    /// All extracted import entries across the workspace.
    /// Consumers: import-rules, orphan-detector.
    fn import_list(&self) -> &[ImportEntry];

    // ── Graph Access (FR-004) ────────────────────────────────

    /// Forward import graph (file -> files it imports).
    /// Consumers: import-rules, orphan-detector.
    fn dependency_graph(&self) -> &std::collections::HashMap<PathBuf, Vec<PathBuf>>;

    /// Reverse import map (file -> list of files that import it).
    /// Consumers: orphan-detector.
    fn reverse_import_map(&self) -> &std::collections::HashMap<PathBuf, Vec<PathBuf>>;

    /// Symbol definition map (trait/class/struct/interface name -> defining file).
    /// Consumers: orphan-detector.
    fn symbol_definitions(&self) -> &std::collections::HashMap<String, Vec<PathBuf>>;

    /// Trait implementation map (trait/interface name -> list of implementor files).
    /// Consumers: orphan-detector.
    fn trait_implementations(&self) -> &std::collections::HashMap<String, Vec<PathBuf>>;

    // ── Timing ───────────────────────────────────────────────

    /// Get timing breakdown of last scan.
    fn timing(&self) -> &ScanTiming;

    // ── File Reading (backward compat) ───────────────────────

    /// Read file content. Checks cache first, falls back to disk.
    fn read_file(&self, path: &Path) -> Option<String>;

    /// Read file for linting: checks cache, enforces 2MiB size limit.
    fn read_lintable_file(&self, path: &str) -> Result<Option<String>, String>;

    /// Get cached file content (after scan).
    fn get_file_content(&self, path: &Path) -> Option<String>;

    /// Check if a file is in the cache.
    fn has_file(&self, path: &Path) -> bool;

    // ── File Discovery (backward compat) ─────────────────────

    /// Discover all source files in directory tree.
    fn discover_files(&self, root: &Path, ignored: &[String]) -> Vec<FileEntry>;

    /// Discover source files with workspace restriction.
    fn discover_source_files(&self, root: &Path, ignored: &[String]) -> Vec<FilePath>;

    // ── Import/Dependency (backward compat) ──────────────────

    /// Get imports for a specific file.
    fn imports_for(&self, path: &Path) -> Vec<ImportEntry>;

    /// Get all imports (from last scan).
    fn all_imports(&self) -> &[ImportEntry];

    /// Check if two files have a dependency relationship.
    fn depends_on(&self, from: &Path, to: &Path) -> bool;

    /// Find circular dependencies.
    fn cycles(&self) -> Vec<Vec<PathBuf>>;

    /// Find orphan files (nothing imports them).
    fn orphan_files(&self) -> Vec<PathBuf>;

    // ── Path Queries ─────────────────────────────────────────

    /// Check if path exists.
    fn path_exists(&self, path: &Path) -> bool;

    /// Check if path is a directory.
    fn is_dir(&self, path: &Path) -> bool;

    /// Check if path should be ignored based on ignore patterns.
    fn should_ignore(&self, path: &str, ignored: &[String]) -> bool;

    // ── Workspace ────────────────────────────────────────────

    /// Find workspace root by walking up from start path.
    fn workspace_root(&self, start: &str) -> Option<PathBuf>;

    // ── Directory Operations ─────────────────────────────────

    /// List directory entries (non-recursive).
    fn scan_directory(&self, dir: &Path) -> Vec<PathBuf>;

    /// List directory entries with ignore filter.
    fn scan_directory_with_ignored(&self, dir: &Path, ignored: &[String]) -> Vec<PathBuf>;

    /// Check if directory should be ignored.
    fn is_ignored_dir(&self, dir: &Path, ignored: &[String]) -> bool;

    // ── Path Metadata ────────────────────────────────────────

    /// Check if path is a file.
    fn is_file(&self, path: &Path) -> bool;

    /// Canonicalize path (resolve symlinks).
    fn canonicalize(&self, path: &Path) -> Result<PathBuf, std::io::Error>;

    /// Check if path is a symlink.
    fn is_symlink(&self, path: &Path) -> bool;

    /// Get file metadata.
    fn metadata(&self, path: &Path) -> Result<std::fs::Metadata, std::io::Error>;

    /// Get symlink metadata (does not follow symlinks).
    fn symlink_metadata(&self, path: &Path) -> Result<std::fs::Metadata, std::io::Error>;

    // ── Path Utilities ─────────────────────────────────────────

    /// Extract file stem from path (filename without extension).
    fn get_file_stem<'a>(&self, path: &'a str) -> &'a str;

    // ── Path Discovery Helpers ────────────────────────────────

    /// Check if directory contains Python source files.
    fn has_python_files(&self, dir: &Path) -> bool;

    /// Check if directory contains a config file (.eslintrc, .prettierrc, tsconfig.json, etc).
    fn has_config_file(&self, dir: &Path) -> bool;

    /// Find Cargo.toml in the given path. Returns path string if found.
    fn has_cargo_toml(&self, path_str: &str) -> Option<String>;

    /// Find Cargo.lock in the given path. Returns path string if found.
    fn has_cargo_lock(&self, path_str: &str) -> Option<String>;

    /// Check if an executable exists in PATH environment variable.
    fn is_executable_in_path(&self, executable: &str) -> bool;

    /// Check if an executable exists in local node_modules/.bin directory.
    fn has_local_bin(&self, working_dir: &Path, executable: &str) -> bool;

    // ── Write Operations (setup/hooks) ───────────────────────

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

    // ── Workspace Member Detection ─────────────────────────

    /// Detect if a path is a single workspace member (crate/module/package).
    fn is_member_path(&self, path: &str) -> bool;

    /// Detect if a path is a leaf member (not a group of members).
    fn is_leaf_member_path(&self, path: &str) -> bool;

    // ── Source Detection ───────────────────────────────────

    /// Detect source directory from project root (packages/, crates/, modules/).
    fn detect_source_dir(&self, project_root: &Path) -> PathBuf;

    /// Collect source files from a directory tree or single file.
    fn collect_source_files(&self, root_dir: &Path, ignored: &[String]) -> Vec<FilePath>;

    /// Recursively scan directory for files (non-source-aware, raw paths).
    fn scan_directory_recursive(&self, dir: &Path) -> Vec<String>;

    // ── Path Metadata Helpers ─────────────────────────────

    /// Check if a path has a source file extension (.rs, .py, .ts, .js, .tsx, .jsx).
    fn is_source_file(&self, path: &Path) -> bool;

    /// Check if an extension string is a recognized source file extension.
    fn is_source_ext(&self, ext: &str) -> bool;

    /// Get file basename (filename without directory).
    fn get_basename<'a>(&self, path: &'a str) -> &'a str;

    /// Get parent directory path.
    fn get_parent<'a>(&self, path: &'a str) -> &'a str;

    // ── Canonicalize (String variant) ─────────────────────────

    /// Canonicalize path to absolute string.
    fn canonicalize_path_str(&self, path_str: &str) -> String;

    // ── Path Resolution (external-lint) ───────────────────────

    /// Resolve JS tool command from local node_modules/.bin.
    fn resolve_js_cmd(&self, executable: &str, args: Vec<String>, working_dir: &str) -> Option<Vec<String>>;

    /// Walk up to find JS project root.
    fn resolve_js_working_dir(&self, path: &crate::common::taxonomy_path_vo::FilePath) -> crate::common::taxonomy_path_vo::FilePath;

    /// Find parent dir with Cargo.toml.
    fn resolve_cargo_working_dir(&self, path: &crate::common::taxonomy_path_vo::FilePath) -> crate::common::taxonomy_path_vo::FilePath;

    /// Find parent dir with Cargo.lock.
    fn resolve_cargo_lock_working_dir(&self, path: &crate::common::taxonomy_path_vo::FilePath) -> crate::common::taxonomy_path_vo::FilePath;

    /// Create default working directory.
    fn default_working_dir(&self, path: &crate::common::taxonomy_path_vo::FilePath) -> crate::common::taxonomy_path_vo::FilePath;

    // ── Python Detection (recursive) ──────────────────────────

    /// Check if path contains Python files (recursive, handles files too).
    fn has_python_files_recursive(&self, path: &crate::common::taxonomy_path_vo::FilePath) -> bool;

    // ── File Mutations ────────────────────────────────────────

    /// Set file permissions (Unix mode bits).
    fn set_permissions(&self, path: &Path, mode: u32) -> std::io::Result<()>;

    /// Remove a file.
    fn remove_file(&self, path: &Path) -> std::io::Result<()>;

    // ── Cache ─────────────────────────────────────────────────

    /// Read file content from bounded cache (returns ContentString).
    fn read_cached(&self, path: &crate::common::taxonomy_path_vo::FilePath) -> ContentString;

    // ── Workspace Detection ───────────────────────────────────

    /// Check if any container/entry file under workspace root references identifiers.
    fn check_wired_in_container(&self, workspace_root: &Path, identifiers: &[String]) -> bool;

    /// Find workspace root from Path (Result variant).
    fn find_workspace_root_from_path(&self, start: &Path) -> Result<PathBuf, std::io::Error>;

    // ── Orphan Detection ──────────────────────────────────────

    /// Resolve a module path relative to base_dir, confined under root.
    fn resolve_orphan_module_path(&self, root: &Path, base_dir: &Path, module_path: &str) -> Option<PathBuf>;

    // ── Language Detection ────────────────────────────────────

    /// Detect ConfigLanguage from a file system path.
    fn detect_language_from_path(&self, path: &str) -> crate::config_system::taxonomy_config_language_vo::ConfigLanguage;

    // ── File Entry Collection ─────────────────────────────────

    /// Collect file entries (path, content) for each lintable file.
    fn collect_file_entries(&self, files: &[String]) -> Vec<(PathBuf, String)>;

    // ── Process Execution (git) ───────────────────────────────

    /// Execute a git command and return stdout/stderr/success.
    fn run_git_command(&self, args: &[&str], dir: &str) -> (String, String, bool);

    /// Parse command output into trimmed non-empty lines.
    fn parse_output_lines(&self, output: &str) -> Vec<String>;

    // ── Process Execution (external) ──────────────────────────

    /// Execute an external command with working directory.
    fn run_external_command_in(&self, name: &str, args: &[&str], current_dir: &str) -> (String, String, bool);

    // ── TUI I/O ───────────────────────────────────────────────

    /// Write text content to a file.
    fn write_text_to_file(&self, path: &Path, text: &str) -> Result<(), String>;

    /// Check if a binary is available in system PATH.
    fn is_binary_available(&self, bin_name: &str) -> bool;

    /// Read directory entries as Vec<PathBuf>.
    fn read_dir_entries_as_pathbuf(&self, dir: &Path) -> Result<Vec<PathBuf>, std::io::Error>;

    // ── Noop (linter compatibility) ───────────────────────────

    /// No-op apply_fix for linters that cannot auto-fix.
    fn noop_apply_fix(&self) -> Result<crate::common::taxonomy_message_vo::ComplianceStatus, crate::code_analysis::taxonomy_operation_error::LinterOperationError>;

}