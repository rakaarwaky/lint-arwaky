// file_system_port — Abstract port for file system operations (Contract Layer).
use async_trait::async_trait;

use crate::shared_common::taxonomy_source_vo::ContentString;
use crate::shared_common::taxonomy_common_vo::Count;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use crate::source_parsing::taxonomy_paths_vo::FilePathList;
use crate::file_system::taxonomy_system_error::FileSystemError;
use crate::shared_common::taxonomy_layer_vo::Identity;
use /* UNKNOWN: PatternList */ crate::shared_common::taxonomy_common_vo::PatternList;
use crate::pipeline_jobs::taxonomy_job_vo::SuccessStatus;

/// Abstract interface for file system operations.
/// Implemented by Infrastructure (e.g., OSFileSystemAdapter).
#[async_trait]
pub trait IFileSystemPort: Send + Sync {
    async fn walk(&self, path: &FilePath, ignored_patterns: Option<&PatternList>) -> FilePathList;
    async fn is_directory(&self, path: &FilePath) -> SuccessStatus;
    async fn is_file(&self, path: &FilePath) -> SuccessStatus;
    async fn get_relative_path(&self, path: &FilePath, start: &FilePath) -> FilePath;
    async fn read_text(&self, path: &FilePath) -> Result<ContentString, FileSystemError>;
    async fn get_line_count(&self, path: &FilePath) -> Count;
    async fn exists(&self, path: &FilePath) -> SuccessStatus;
    async fn get_parent(&self, path: &FilePath) -> FilePath;
    async fn write_text(
        &self,
        path: &FilePath,
        content: &ContentString,
        mode: Option<&Identity>,
    ) -> Result<SuccessStatus, FileSystemError>;
    async fn glob(&self, pattern: &Identity) -> FilePathList;
    async fn get_cwd(&self) -> FilePath;
    async fn get_basename(&self, path: &FilePath) -> Identity;
    async fn path_join(&self, parts: &[Identity]) -> FilePath;
    async fn read_file(&self, path: &FilePath) -> Result<ContentString, FileSystemError>;
}
