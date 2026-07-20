// PURPOSE: IFileAdapterPort — protocol trait for file I/O operations
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_source_vo::ContentString;

pub trait IFileAdapterProtocol: Send + Sync {
    fn read_file(&self, path: &FilePath) -> Option<ContentString>;
    fn write_file(&self, path: &FilePath, content: &ContentString) -> bool;
    fn path_exists(&self, path: &FilePath) -> bool;
}
