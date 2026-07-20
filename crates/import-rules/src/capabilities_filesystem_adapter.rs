// PURPOSE: FileSystemAdapter — thin delegation layer to shared utility_file functions
// This module delegates all filesystem logic to shared utility functions.
// The adapter struct is kept for backward compatibility but is now a thin wrapper.

use async_trait::async_trait;
use std::path::Path;

use shared::common::contract_system_protocol::IFileSystemProtocol;
use shared::common::taxonomy_filesystem_error::FileSystemError;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::common::utility_file;
use shared::mcp_server::taxonomy_action_vo::ActionName;
use shared::mcp_server::taxonomy_job_vo::SuccessStatus;
use shared::taxonomy_common_error::ErrorMessage;
use shared::taxonomy_common_vo::{Count, PatternList};
use shared::taxonomy_layer_vo::Identity;
use shared::taxonomy_source_vo::ContentString;

pub struct OSFileSystemAdapter;

impl OSFileSystemAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for OSFileSystemAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl IFileSystemProtocol for OSFileSystemAdapter {
    async fn walk(&self, path: &FilePath, ignored_patterns: Option<&PatternList>) -> FilePathList {
        let dir = Path::new(&path.value);
        let ignored = match ignored_patterns {
            Some(p) => p.values.clone(),
            None => Vec::new(),
        };
        let mut results = Vec::new();
        utility_file::walk_source_files(dir, &mut results, &ignored);
        FilePathList { values: results }
    }

    async fn is_directory(&self, path: &FilePath) -> SuccessStatus {
        SuccessStatus::new(utility_file::is_directory(&path.value))
    }

    async fn is_file(&self, path: &FilePath) -> SuccessStatus {
        SuccessStatus::new(utility_file::is_file(&path.value))
    }

    async fn get_relative_path(&self, path: &FilePath, start: &FilePath) -> FilePath {
        let p = Path::new(&path.value);
        let s = Path::new(&start.value);
        p.strip_prefix(s)
            .ok()
            .and_then(|rel| FilePath::new(rel.to_string_lossy().to_string()).ok())
            .unwrap_or_else(|| path.clone())
    }

    async fn read_text(&self, path: &FilePath) -> Result<ContentString, FileSystemError> {
        self.read_file(path).await
    }

    async fn get_line_count(&self, path: &FilePath) -> Count {
        match utility_file::read_file_sync(&path.value) {
            Ok(content) => Count::new(content.lines().count() as i64),
            Err(_) => Count::new(0),
        }
    }

    async fn exists(&self, path: &FilePath) -> SuccessStatus {
        SuccessStatus::new(Path::new(&path.value).exists())
    }

    async fn get_parent(&self, path: &FilePath) -> FilePath {
        FilePath::new(utility_file::get_parent(&path.value).to_string())
            .unwrap_or_else(|_| path.clone())
    }

    async fn write_text(
        &self,
        path: &FilePath,
        content: &ContentString,
        _mode: Option<&Identity>,
    ) -> Result<SuccessStatus, FileSystemError> {
        match std::fs::write(&path.value, &content.value) {
            Ok(_) => Ok(SuccessStatus::new(true)),
            Err(e) => Err(FileSystemError::new(
                path.clone(),
                ErrorMessage::new(e.to_string()),
                ActionName::new("write"),
            )),
        }
    }

    async fn glob(&self, _pattern: &Identity) -> FilePathList {
        FilePathList { values: vec![] }
    }

    async fn get_cwd(&self) -> FilePath {
        match std::env::current_dir() {
            Ok(p) => FilePath::new(p.to_string_lossy().to_string()).unwrap_or_default(),
            Err(_) => FilePath::new(".").unwrap_or_default(),
        }
    }

    async fn get_basename(&self, path: &FilePath) -> Identity {
        Identity::new(utility_file::get_basename(&path.value).to_string())
    }

    async fn path_join(&self, parts: &[Identity]) -> FilePath {
        let mut path = std::path::PathBuf::new();
        for part in parts {
            path.push(&part.value);
        }
        FilePath::new(path.to_string_lossy().to_string()).unwrap_or_default()
    }

    async fn read_file(&self, path: &FilePath) -> Result<ContentString, FileSystemError> {
        match utility_file::read_file_sync(&path.value) {
            Ok(content) => Ok(ContentString::new(content)),
            Err(e) => Err(FileSystemError::new(
                path.clone(),
                ErrorMessage::new(e.to_string()),
                ActionName::new("read"),
            )),
        }
    }
}
