// PURPOSE: IWorkspaceDetectorProtocol — protocol trait for detecting workspace type from directory structure
use crate::common::taxonomy_path_vo::FilePath;
use crate::config_system::taxonomy_config_language_vo::ConfigLanguage;
use async_trait::async_trait;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkspaceType {
    Rust,
    TypeScript,
    Python,
    Unknown,
}

impl WorkspaceType {
    pub fn as_str(&self) -> &'static str {
        match self {
            WorkspaceType::Rust => "rust",
            WorkspaceType::TypeScript => "typescript",
            WorkspaceType::Python => "python",
            WorkspaceType::Unknown => "unknown",
        }
    }
}

impl std::fmt::Display for WorkspaceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<WorkspaceType> for ConfigLanguage {
    fn from(ws: WorkspaceType) -> Self {
        match ws {
            WorkspaceType::Rust => ConfigLanguage::Rust,
            WorkspaceType::Python => ConfigLanguage::Python,
            WorkspaceType::TypeScript => ConfigLanguage::TypeScript,
            WorkspaceType::Unknown => ConfigLanguage::Rust,
        }
    }
}

#[async_trait]
pub trait IWorkspaceDetectorProtocol: Send + Sync {
    /// Detect workspace type by checking folder structure and config files.
    fn detect(&self, path: &FilePath) -> WorkspaceType;

    /// Check if a path is a workspace root (contains crates/, packages/, or modules/).
    fn is_workspace(&self, path: &FilePath) -> bool;

    /// Discover workspace member directories under the given root.
    async fn discover_workspace_members(&self, root: &FilePath) -> Vec<FilePath>;
}
