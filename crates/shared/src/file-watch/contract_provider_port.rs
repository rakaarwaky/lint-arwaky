// PURPOSE: IWatchProviderPort — port trait for filesystem watch provider
use crate::common::taxonomy_common_vo::bool;
use crate::file_watch::taxonomy_service_error::WatchServiceError;
use crate::file_watch::taxonomy_watch_config_vo::WatchConfig;
use crate::file_watch::taxonomy_watch_event_vo::WatchEvent;

#[async_trait::async_trait]
pub trait IWatchProviderPort: Send + Sync {
    async fn start(&self, config: &WatchConfig) -> Result<(), WatchServiceError>;
    async fn stop(&self) -> Result<(), WatchServiceError>;
    async fn is_available(&self) -> bool;
    fn subscribe(&self) -> tokio::sync::broadcast::Receiver<WatchEvent>;
}
