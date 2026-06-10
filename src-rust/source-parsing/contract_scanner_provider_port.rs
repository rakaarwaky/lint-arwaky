// PURPOSE: Port: Interface for Scanner Provider

use crate::file_system::taxonomy_filesystem_error::FileSystemError;
use crate::source_parsing::taxonomy_path_vo::DirectoryPath;
use crate::source_parsing::taxonomy_paths_vo::FilePathList;

pub trait IScannerProviderPort: Send + Sync {
    fn scan_directory(&self, path: &DirectoryPath) -> Result<FilePathList, FileSystemError>;
    fn get_ignored_files(&self) -> FilePathList;
}
