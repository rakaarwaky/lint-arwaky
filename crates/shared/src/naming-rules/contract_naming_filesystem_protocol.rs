// PURPOSE: INamingFileSystemProtocol — Local contract trait for naming-rules filesystem operations
use crate::common::taxonomy_common_vo::PatternList;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_paths_vo::FilePathList;
use async_trait::async_trait;

#[async_trait]
pub trait INamingFileSystemProtocol: Send + Sync {
    async fn walk(&self, path: &FilePath, ignored_patterns: Option<&PatternList>) -> FilePathList;
}
