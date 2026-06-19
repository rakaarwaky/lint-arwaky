// PURPOSE: FileWatchContainer — wiring for file-watch feature (root layer, wiring only)
// Wiring: IWatchProviderPort → WatchServiceProvider (infrastructure layer)
use crate::infrastructure_watch_provider::WatchServiceProvider;
use std::sync::Arc;

pub struct FileWatchContainer {
    watch_provider: Arc<WatchServiceProvider>,
}

impl FileWatchContainer {
    pub fn new() -> Self {
        Self {
            watch_provider: Arc::new(WatchServiceProvider::new()),
        }
    }

    pub fn watch_provider(&self) -> Arc<WatchServiceProvider> {
        self.watch_provider.clone()
    }
}

impl Default for FileWatchContainer {
    fn default() -> Self {
        Self::new()
    }
}
