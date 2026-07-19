// PURPOSE: FileSystemAdapter — INamingFileSystemPort implementation custom-tailored for naming-rules (crawling/walking only)
use async_trait::async_trait;

use shared::common::taxonomy_path_utils_vo::PathUtils;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::naming_rules::contract_naming_filesystem_port::INamingFileSystemPort;
use shared::taxonomy_common_vo::PatternList;

pub struct OSFileSystemAdapter {}

impl OSFileSystemAdapter {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for OSFileSystemAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl INamingFileSystemPort for OSFileSystemAdapter {
    async fn walk(&self, path: &FilePath, ignored_patterns: Option<&PatternList>) -> FilePathList {
        let root = std::path::Path::new(&path.value);
        let ignored_refs: Vec<&str> = match ignored_patterns {
            Some(p) => p.values.iter().map(|s| s.as_str()).collect(),
            None => Vec::new(),
        };
        let results = PathUtils::walk_recursive(root, &ignored_refs);
        FilePathList {
            values: results
                .into_iter()
                .filter_map(|p| FilePath::new(p.to_string_lossy().to_string()).ok())
                .collect(),
        }
    }
}
