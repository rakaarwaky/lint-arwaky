// PURPOSE: WatchServiceError — structured error type for file watch service failures
use crate::source_parsing::taxonomy_path_vo::FilePath;
use crate::taxonomy_common_error::ErrorMessage;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WatchServiceError {
    pub path: FilePath,
    pub message: String,
}

impl WatchServiceError {
    pub fn new(message: ErrorMessage) -> Self {
        Self {
            path: FilePath::default(),
            message: message.value,
        }
    }
}

impl std::fmt::Display for WatchServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Watch Error on {}: {}", self.path.value, self.message)
    }
}

impl std::error::Error for WatchServiceError {}

#[derive(Debug, Clone)]
pub struct WatchSubscriptionError(pub WatchServiceError);

#[derive(Debug, Clone)]
pub struct WatchEventError(pub WatchServiceError);
