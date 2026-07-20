// PURPOSE: IFileCollectionProtocol — protocol for collecting source files from a directory
// Agents use this via DI to avoid direct I/O.
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_paths_vo::FilePathList;

pub trait IFileCollectionProtocol: Send + Sync {
    /// Walk a directory recursively and return all source files.
    fn collect_files(&self, target: &FilePath) -> FilePathList;

    /// Walk a directory recursively and return source files matching specific extensions.
    fn collect_files_with_extensions(&self, target: &FilePath, extensions: &[&str]) -> FilePathList;
}
