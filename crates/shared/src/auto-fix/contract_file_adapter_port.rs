// PURPOSE: IFileAdapterPort — port trait for file I/O operations
use crate::common::taxonomy_path_vo::FilePath;

pub trait IFileAdapterPort: Send + Sync {
    fn read_file(&self, path: &FilePath) -> Option<String>;
    fn write_file(&self, path: &FilePath, content: &str) -> bool;
    fn path_exists(&self, path: &FilePath) -> bool;
}
