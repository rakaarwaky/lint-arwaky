use crate::shared_common::taxonomy_common_vo::BooleanVO;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use crate::file_watch::taxonomy_service_error::WatchServiceError;

#[async_trait::async_trait]
pub trait IWatchProviderPort: Send + Sync {
    async fn start(&self, path: &FilePath) -> Result<(), WatchServiceError>;
    async fn stop(&self) -> Result<(), WatchServiceError>;
    async fn is_available(&self) -> BooleanVO;
}
