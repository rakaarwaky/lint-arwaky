// PURPOSE: file-watch-domain capability contracts (AES102 `_protocol`).
//
// One file for the file-watch feature. Each trait below is one capability
// seam: a trait carries every method that capability implements, with one
// concrete return type each, so a capability implements its trait outright
// and never carries unimplemented stubs.

use crate::common::taxonomy_common_vo::BooleanVO;
use crate::file_watch::taxonomy_service_error::WatchServiceError;
use crate::file_watch::taxonomy_watch_config_vo::WatchConfig;
use crate::file_watch::taxonomy_watch_config_vo::WatchEvent;

pub trait IChangeAnalyzerProtocol: Send + Sync {
    fn analyze(&self, events: Vec<WatchEvent>) -> Vec<WatchEvent>;
    fn is_lintable(&self, path: &str) -> bool;
    fn filter_lintable(&self, events: Vec<WatchEvent>) -> Vec<WatchEvent>;
}

#[async_trait::async_trait]
pub trait IWatchProviderProtocol: Send + Sync {
    async fn start(&self, config: &WatchConfig) -> Result<(), WatchServiceError>;
    async fn stop(&self) -> Result<(), WatchServiceError>;
    async fn is_available(&self) -> BooleanVO;
    fn subscribe(&self) -> tokio::sync::broadcast::Receiver<WatchEvent>;
}
