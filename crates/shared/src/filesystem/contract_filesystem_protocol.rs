// PURPOSE: filesystem-domain capability contracts (AES102 `_protocol`).
//
// One file for the filesystem feature. Each trait below is one capability
// seam: a trait carries every method that capability implements, with one
// concrete return type each, so a capability implements its trait outright
// and never carries unimplemented stubs.

use crate::taxonomy_filesystem_vo::{
    ByteCount, FileExtension, FileMode, GitCommandResult, ParsedLines, ScanTiming,
};
use crate::taxonomy_filesystem_vo::{
    DefinitionEntry, FileEntry, ImplEntry, ImportEntry, ParseWarning,
};
use shared_common::taxonomy_common_vo::PatternList;
use shared_common::taxonomy_config_language_vo::ConfigLanguage;
use shared_common::taxonomy_language_vo::Language;
use shared_common::taxonomy_path_vo::FilePath;
use shared_common::taxonomy_source_vo::ContentString;
use shared_common::taxonomy_tool_name_vo::ToolName;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

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
        path: &shared_common::taxonomy_path_vo::FilePath,
        ignored: &[String],
    ) -> bool;

    /// Canonicalize path (resolve symlinks).
    fn canonicalize(&self, path: &Path) -> Result<PathBuf, std::io::Error>;

    /// Canonicalize path to absolute string.
    fn canonicalize_path_str(&self, path: &FilePath) -> FilePath;

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
    fn scan_directory_with_ignored(&self, dir: &Path, ignored: &PatternList) -> Vec<PathBuf>;

    /// Check if directory should be ignored.
    fn is_ignored_dir(&self, dir: &Path, ignored: &PatternList) -> bool;

    /// Read directory entries as Vec<PathBuf>.
    fn read_dir_entries_as_pathbuf(&self, dir: &Path) -> Result<Vec<PathBuf>, std::io::Error>;

    // ═══════════════════════════════════════════════════════════
    // File Read/Write
    // ═══════════════════════════════════════════════════════════

    /// Read file content to string.
    fn read_to_string(&self, path: &Path) -> Result<ContentString, std::io::Error>;

    /// Write string to file.
    fn write_string(&self, path: &Path, content: &str) -> Result<(), std::io::Error>;

    /// Copy file from src to dst.
    fn copy_file(&self, src: &Path, dst: &Path) -> Result<ByteCount, std::io::Error>;

    /// Create directory and all parents.
    fn create_dir_all(&self, path: &Path) -> Result<(), std::io::Error>;

    /// Remove directory recursively.
    fn remove_dir_all(&self, path: &Path) -> Result<(), std::io::Error>;

    /// Set file permissions (Unix mode bits).
    fn set_permissions(&self, path: &Path, mode: FileMode) -> std::io::Result<()>;

    /// Remove a file.
    fn remove_file(&self, path: &Path) -> std::io::Result<()>;

    // ═══════════════════════════════════════════════════════════
    // Process Execution
    // ═══════════════════════════════════════════════════════════

    /// Execute a git command and return stdout/stderr/success.
    fn run_git_command(&self, args: &[&str], dir: &str) -> GitCommandResult;

    /// Parse command output into trimmed non-empty lines.
    fn parse_output_lines(&self, output: &str) -> ParsedLines;

    /// Execute an external command with working directory.
    fn run_external_command_in(
        &self,
        name: &ToolName,
        args: &[&str],
        current_dir: &str,
    ) -> (String, String, bool);

    // ═══════════════════════════════════════════════════════════
    // Scan Timing
    // ═══════════════════════════════════════════════════════════

    /// Get timing breakdown of last scan.
    fn timing(&self) -> &ScanTiming;
}

pub trait IGraphProtocol: Send + Sync {
    /// Build graph from imports, file list, definitions, and implementations.
    fn build_graph(
        &self,
        imports: &[ImportEntry],
        files: &[FileEntry],
        definitions: &[DefinitionEntry],
        implementations: &[ImplEntry],
    );
    fn symbol_definitions(&self) -> &HashMap<String, Vec<PathBuf>>;
    fn implementations(&self) -> &HashMap<String, Vec<PathBuf>>;
    fn dependents(&self, path: &Path) -> Vec<PathBuf>;
    fn dependencies(&self, path: &Path) -> Vec<PathBuf>;
    fn reachable(&self, from: &Path, to: &Path) -> bool;
    fn reverse_links(&self) -> &HashMap<PathBuf, Vec<PathBuf>>;
}

/// Parser protocol — AST parse results and import extraction queries.
/// Consumers import only this trait when they need parse warnings or import data.
pub trait IParserProtocol: Send + Sync {
    fn parse_warnings(&self) -> &[ParseWarning];
    fn import_list(&self) -> Vec<ImportEntry>;
    fn parse_all(&self, files: &mut [FileEntry]);
    fn imports_for(&self, path: &Path) -> Vec<ImportEntry>;
    fn extract(&self, path: &Path, content: &str, language: Language) -> Vec<ImportEntry>;

    /// Resolve all stored imports through barrel files (__init__.py, mod.rs, etc.).
    /// Populates `resolved_path` and `is_resolved` fields.
    /// Call after `parse_all` with the project root directory.
    fn resolve_barrel_imports(&self, root_dir: &Path);
}

/// Tool resolution protocol — external tool availability and command resolution.
/// Consumers import only this trait when they need tool detection or command building.
pub trait IToolResolutionProtocol: Send + Sync {
    /// Check if an executable exists in PATH.
    fn is_executable_in_path(&self, executable: &ToolName) -> bool;

    /// Check if a binary is available in system PATH.
    fn is_binary_available(&self, bin_name: &ToolName) -> bool;

    /// Check if an executable exists in local node_modules/.bin.
    fn has_local_bin(&self, working_dir: &Path, executable: &ToolName) -> bool;

    /// Resolve JS tool command from local node_modules/.bin.
    fn resolve_js_cmd(
        &self,
        executable: &ToolName,
        args: Vec<String>,
        working_dir: &FilePath,
    ) -> Option<Vec<String>>;

    /// Walk up to find JS project root.
    fn resolve_js_working_dir(&self, path: &FilePath) -> FilePath;

    /// Find parent dir with Cargo.toml.
    fn resolve_cargo_working_dir(&self, path: &FilePath) -> FilePath;

    /// Find parent dir with Cargo.lock.
    fn resolve_cargo_lock_working_dir(&self, path: &FilePath) -> FilePath;

    /// Check if directory contains a config file (.eslintrc, .prettierrc, tsconfig.json, etc).
    fn has_config_file(&self, dir: &Path) -> bool;

    /// Find Cargo.toml in the given path.
    fn has_cargo_toml(&self, path: &FilePath) -> Option<FilePath>;

    /// Find Cargo.lock in the given path.
    fn has_cargo_lock(&self, path: &FilePath) -> Option<FilePath>;

    /// Check if path contains Python files (recursive, handles files too).
    fn is_python_file_recursive(&self, path: &FilePath) -> bool;

    /// Create default working directory.
    fn default_working_dir(&self, path: &FilePath) -> FilePath;
}

/// Workspace protocol — workspace structure detection and navigation.
/// Consumers import only this trait when they need workspace-level queries.
pub trait IWorkspaceProtocol: Send + Sync {
    /// FR-005: Find workspace root by walking up from start path.
    fn workspace_root(&self, start: &FilePath) -> Option<PathBuf>;

    /// FR-005: Find workspace root (Result variant).
    fn find_workspace_root_from_path(&self, start: &Path) -> Result<PathBuf, std::io::Error>;

    /// FR-005: Detect if a path is a workspace member.
    fn is_member_path(&self, path: &FilePath) -> bool;

    /// FR-005: Detect if a path is a leaf member.
    fn is_leaf_member_path(&self, path: &FilePath) -> bool;

    /// FR-005: Detect source directory from project root.
    fn detect_source_dir(&self, project_root: &Path) -> PathBuf;

    /// Detect ConfigLanguage from a file system path.
    fn detect_language_from_path(&self, path: &str) -> ConfigLanguage;

    /// FR-005: Check if any container/entry file under workspace root references identifiers.
    fn check_wired_in_container(&self, workspace_root: &Path, identifiers: &PatternList) -> bool;

    /// Resolve a module path relative to base_dir, confined under root.
    fn resolve_orphan_module_path(
        &self,
        root: &Path,
        base_dir: &Path,
        module_path: &str,
    ) -> Option<PathBuf>;
}
