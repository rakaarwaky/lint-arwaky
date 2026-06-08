//! Port trait for scanning the filesystem.
//!
//! Defines the outbound interface for recursively scanning
//! directories and retrieving git-ignored file patterns.

use crate::taxonomy::{DirectoryPath, FilePathList, FileSystemError};

pub trait IScannerProviderPort: Send + Sync {
    fn scan_directory(&self, path: &DirectoryPath) -> Result<FilePathList, FileSystemError>;
    fn get_ignored_files(&self) -> FilePathList;
}
