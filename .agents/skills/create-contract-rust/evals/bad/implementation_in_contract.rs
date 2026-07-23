// BAD: Contract contains implementation
pub trait IFileSystemPort: Send + Sync {
    async fn read_file(&self, path: &FilePath) -> Result<FileContent, FileReadError>;
}

impl IFileSystemPort for FileAdapter {
    async fn read_file(&self, path: &FilePath) -> Result<FileContent, FileReadError> {
        todo!() // BAD: implementation belongs in capabilities
    }
}
