use super::*;

pub trait IWatchProviderPort: Send + Sync {
    fn start(&self, path: &FilePath) -> Result<(), WatchServiceError>;
    fn stop(&self) -> Result<(), WatchServiceError>;
    fn is_available(&self) -> bool;
}
