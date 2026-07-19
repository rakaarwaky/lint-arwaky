// PURPOSE: IFileReaderPort — port trait for file reading operations
pub trait IFileReaderPort: Send + Sync {
    fn read_file(&self, path: &str) -> Option<String>;
}
