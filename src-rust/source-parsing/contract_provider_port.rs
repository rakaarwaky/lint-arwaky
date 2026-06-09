//! Port trait for scanning the filesystem.
//!
//! Defines the outbound interface for recursively scanning
//! directories and retrieving git-ignored file patterns.

use crate::file_system::taxonomy_system_error::FileSystemError;
use crate::source_parsing::taxonomy_path_vo::DirectoryPath;
use crate::source_parsing::taxonomy_paths_vo::FilePathList;

pub trait IScannerProviderPort: Send + Sync {
    fn scan_directory(&self, path: &DirectoryPath) -> Result<FilePathList, FileSystemError>;
    fn get_ignored_files(&self) -> FilePathList;
}
