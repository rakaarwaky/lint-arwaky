// PURPOSE: FileSystemAdapter — IFileSystemProtocol implementation using std::fs
use async_trait::async_trait;
use std::fs;
use std::path::{Path, PathBuf};

use shared::common::contract_system_protocol::IFileSystemProtocol;
use shared::common::taxonomy_filesystem_error::FileSystemError;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::mcp_server::taxonomy_action_vo::ActionName;
use shared::mcp_server::taxonomy_job_vo::SuccessStatus;
use shared::taxonomy_common_error::ErrorMessage;
use shared::taxonomy_common_vo::Count;
use shared::taxonomy_common_vo::PatternList;
use shared::taxonomy_layer_vo::Identity;
use shared::taxonomy_source_vo::ContentString;

/// Returns the inner `FilePath` if `result` is `Ok`, otherwise returns `FilePath::default()`.
/// Private helper — uses `Result::match` to avoid inline match patterns.
fn filepath_or_default(result: Result<FilePath, impl std::fmt::Debug>) -> FilePath {
    result.unwrap_or_default()
}

/// Returns the inner `FilePath` if `result` is `Ok`, otherwise clones `fallback`.
fn filepath_or_clone(
    result: Result<FilePath, impl std::fmt::Debug>,
    fallback: &FilePath,
) -> FilePath {
    match result {
        Ok(fp) => fp,
        Err(_) => fallback.clone(),
    }
}

/// Returns the `&str` slice from an `OsStr` option, falling back to `""`.
fn os_str_to_str(opt: Option<&std::ffi::OsStr>) -> &str {
    opt.and_then(|o| o.to_str()).map_or("", |s| s)
}

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
                let name = os_str_to_str(path.file_name());
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
impl IFileSystemProtocol for OSFileSystemAdapter {
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

    async fn is_directory(&self, path: &FilePath) -> SuccessStatus {
        SuccessStatus::new(Path::new(&path.value).is_dir())
    }

    async fn is_file(&self, path: &FilePath) -> SuccessStatus {
        SuccessStatus::new(Path::new(&path.value).is_file())
    }

    async fn get_relative_path(&self, path: &FilePath, start: &FilePath) -> FilePath {
        let p = Path::new(&path.value);
        let s = Path::new(&start.value);
        p.strip_prefix(s).ok().map_or_else(
            || path.clone(),
            |rel| filepath_or_clone(FilePath::new(rel.to_string_lossy().to_string()), path),
        )
    }

    async fn read_text(&self, path: &FilePath) -> Result<ContentString, FileSystemError> {
        self.read_file(path).await
    }

    async fn get_line_count(&self, path: &FilePath) -> Count {
        if let Ok(content) = fs::read_to_string(&path.value) {
            Count::new(content.lines().count() as i64)
        } else {
            Count::new(0)
        }
    }

    async fn exists(&self, path: &FilePath) -> SuccessStatus {
        SuccessStatus::new(Path::new(&path.value).exists())
    }

    async fn get_parent(&self, path: &FilePath) -> FilePath {
        let p = Path::new(&path.value);
        p.parent().map_or_else(
            || path.clone(),
            |parent| filepath_or_clone(FilePath::new(parent.to_string_lossy().to_string()), path),
        )
    }

    async fn write_text(
        &self,
        path: &FilePath,
        content: &ContentString,
        _mode: Option<&Identity>,
    ) -> Result<SuccessStatus, FileSystemError> {
        match fs::write(&path.value, &content.value) {
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
        let cwd = match std::env::current_dir() {
            Ok(p) => p,
            Err(_) => PathBuf::from("."),
        };
        let primary = filepath_or_default(FilePath::new(cwd.to_string_lossy().to_string()));
        if primary != FilePath::default() {
            primary
        } else {
            filepath_or_default(FilePath::new(".".to_string()))
        }
    }

    async fn get_basename(&self, path: &FilePath) -> Identity {
        let p = Path::new(&path.value);
        let name = os_str_to_str(p.file_name());
        Identity::new(name.to_string())
    }

    async fn path_join(&self, parts: &[Identity]) -> FilePath {
        let mut path = PathBuf::new();
        for part in parts {
            path.push(&part.value);
        }
        let primary = filepath_or_default(FilePath::new(path.to_string_lossy().to_string()));
        if primary != FilePath::default() {
            primary
        } else {
            filepath_or_default(FilePath::new(".".to_string()))
        }
    }

    async fn read_file(&self, path: &FilePath) -> Result<ContentString, FileSystemError> {
        match fs::read_to_string(&path.value) {
            Ok(content) => Ok(ContentString::new(content)),
            Err(e) => Err(FileSystemError::new(
                path.clone(),
                ErrorMessage::new(e.to_string()),
                ActionName::new("read"),
            )),
        }
    }
}
