use async_trait::async_trait;

use shared::file_system::taxonomy_file_content_vo::FileContent;
use shared::file_system::taxonomy_file_path_vo::FilePath;
use shared::file_system::taxonomy_file_read_error::FileReadError;

#[async_trait]
pub trait IFileSystemPort: Send + Sync {
    async fn read_file(&self, path: &FilePath) -> Result<FileContent, FileReadError>;
}
