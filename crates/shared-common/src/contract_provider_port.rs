// PURPOSE: IWatchProviderPort — port trait for filesystem watch provider
use file_watch::taxonomy_service_error::WatchServiceError;
use shared_common::taxonomy_common_vo::BooleanVO;
use source_parsing::taxonomy_path_vo::FilePath;

#[async_trait::async_trait]
pub trait IWatchProviderPort: Send + Sync {
    async fn start(&self, path: &FilePath) -> Result<(), WatchServiceError>;
    async fn stop(&self) -> Result<(), WatchServiceError>;
    async fn is_available(&self) -> BooleanVO;
}
