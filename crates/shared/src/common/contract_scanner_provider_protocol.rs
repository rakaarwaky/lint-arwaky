// PURPOSE: IScannerProviderPort — protocol trait for providing language-specific source scanners

use crate::common::taxonomy_filesystem_error::FileSystemError;
use crate::common::taxonomy_path_vo::DirectoryPath;
use crate::common::taxonomy_paths_vo::FilePathList;

pub trait IScannerProviderProtocol: Send + Sync {
    fn scan_directory(&self, path: &DirectoryPath) -> Result<FilePathList, FileSystemError>;
    fn get_ignored_files(&self) -> FilePathList;
}
