// Contract layer — tool resolution protocol trait
// FR-006: External Tool Availability and Resolution
// Responsibilities: PATH checks, JS tool resolution, Cargo tool resolution

use crate::common::taxonomy_path_vo::FilePath;
use crate::filesystem::taxonomy_filesystem_vo::ToolName;
use std::path::Path;

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
