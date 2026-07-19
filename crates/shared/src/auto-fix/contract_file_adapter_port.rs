// PURPOSE: IFileAdapterPort — port trait for file I/O operations
pub trait IFileAdapterPort: Send + Sync {
    fn read_file(&self, path: &str) -> Option<String>;
    fn write_file(&self, path: &str, content: &str) -> bool;
    fn path_exists(&self, path: &str) -> bool;
}
