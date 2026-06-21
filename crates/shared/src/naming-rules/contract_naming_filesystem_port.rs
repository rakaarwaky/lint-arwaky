// PURPOSE: INamingFileSystemPort — Local contract trait for naming-rules filesystem operations
use crate::source_parsing::taxonomy_path_vo::FilePath;
use crate::source_parsing::taxonomy_paths_vo::FilePathList;
use crate::taxonomy_common_vo::PatternList;
use async_trait::async_trait;

#[async_trait]
pub trait INamingFileSystemPort: Send + Sync {
    async fn walk(&self, path: &FilePath, ignored_patterns: Option<&PatternList>) -> FilePathList;
}
