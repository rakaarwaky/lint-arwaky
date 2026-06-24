// PURPOSE: FileSystemAdapter — INamingFileSystemPort implementation custom-tailored for naming-rules (crawling/walking only)
use async_trait::async_trait;
use std::fs;
use std::path::Path;

use shared::naming_rules::contract_naming_filesystem_port::INamingFileSystemPort;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use shared::source_parsing::taxonomy_paths_vo::FilePathList;
use shared::taxonomy_common_vo::PatternList;

pub struct OSFileSystemAdapter {}

impl OSFileSystemAdapter {
    pub fn new() -> Self {
        Self {}
    }

    fn walk_recursive(&self, dir: &Path, ignored: &[String], results: &mut Vec<FilePath>) {
        if dir.is_file() {
            if let Ok(fp) = FilePath::new(dir.to_string_lossy().to_string()) {
                results.push(fp);
            }
            return;
        }
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                let name = Option::unwrap_or_default(path.file_name().and_then(|n| n.to_str()));
                if ignored.contains(&name.to_string()) {
                    continue;
                }
                if path.is_dir() {
                    self.walk_recursive(&path, ignored, results);
                } else if let Ok(fp) = FilePath::new(path.to_string_lossy().to_string()) {
                    results.push(fp);
                }
            }
        }
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
        let root = Path::new(&path.value);
        let ignored = match ignored_patterns {
            Some(p) => p.values.clone(),
            None => Vec::new(),
        };
        let mut results = Vec::new();
        self.walk_recursive(root, &ignored, &mut results);
        FilePathList { values: results }
    }
}
