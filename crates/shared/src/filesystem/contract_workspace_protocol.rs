use crate::common::taxonomy_common_vo::PatternList;
// Contract layer — workspace protocol trait
// FR-005: Workspace Structure Detection
// Responsibilities: root detection, member detection, source dir, language

use crate::common::taxonomy_config_language_vo::ConfigLanguage;
use crate::common::taxonomy_path_vo::FilePath;
use std::path::{Path, PathBuf};

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
