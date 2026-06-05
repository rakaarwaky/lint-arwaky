/// watch_service_provider — Provides file system watching capabilities.
use crate::contract::watch_provider_port::IWatchProviderPort;
use crate::taxonomy::{BooleanVO, ErrorMessage, FilePath, WatchServiceError};

pub struct WatchServiceProvider {
    callback: Option<Box<dyn Fn(FilePath) + Send + Sync>>,
    running: bool,
}

impl WatchServiceProvider {
    pub fn new(callback: Option<Box<dyn Fn(FilePath) + Send + Sync>>) -> Self {
        Self {
            callback,
            running: false,
        }
    }

    pub fn is_available(&self) -> BooleanVO {
        BooleanVO::new(cfg!(feature = "watch"))
    }

    pub fn start(&mut self, path: &FilePath) -> Result<(), WatchServiceError> {
        if !std::path::Path::new(&path.value).exists() {
            return Err(WatchServiceError::new(ErrorMessage::new(format!(
                "Path does not exist: {}",
                &path.value
            ))));
        }
        self.running = true;
        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), WatchServiceError> {
        self.running = false;
        Ok(())
    }
}

#[async_trait::async_trait]
impl IWatchProviderPort for WatchServiceProvider {
    async fn start(&self, _path: &FilePath) -> Result<(), WatchServiceError> {
        Ok(())
    }

    async fn stop(&self) -> Result<(), WatchServiceError> {
        Ok(())
    }

    async fn is_available(&self) -> bool {
        cfg!(feature = "watch")
    }
}
