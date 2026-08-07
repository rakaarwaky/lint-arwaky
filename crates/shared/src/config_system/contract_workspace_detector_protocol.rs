// PURPOSE: IWorkspaceDetectorProtocol — protocol trait for detecting workspace type from directory structure
use crate::common::taxonomy_path_vo::FilePath;

pub use crate::config_system::taxonomy_config_vo::WorkspaceType;

pub trait IWorkspaceDetectorProtocol: Send + Sync {
    /// Detect workspace type by checking folder structure and config files.
    fn detect(&self, path: &FilePath) -> WorkspaceType;

    /// Check if a path is a workspace root (contains crates/, packages/, or modules/).
    fn is_workspace(&self, path: &FilePath) -> bool;

    /// Discover workspace member directories under the given root.
    fn discover_workspace_members(&self, root: &FilePath) -> Vec<FilePath>;
}
