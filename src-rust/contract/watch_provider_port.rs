use super::*;

#[async_trait::async_trait]
pub trait IWatchProviderPort: Send + Sync {
    async fn start(&self, path: &FilePath) -> Result<(), WatchServiceError>;
    async fn stop(&self) -> Result<(), WatchServiceError>;
    async fn is_available(&self) -> bool;
}
