// PURPOSE: WatchServiceError — structured error type for file watch service failures
use crate::common::taxonomy_message_vo::LintMessage;
use crate::source_parsing::taxonomy_path_vo::FilePath;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WatchServiceError {
    pub path: FilePath,
    pub message: LintMessage,
}

impl WatchServiceError {
    pub fn new(message: LintMessage) -> Self {
        Self {
            path: FilePath::default(),
            message,
        }
    }
}

impl std::fmt::Display for WatchServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Watch Error on {}: {}", self.path.value, self.message)
    }
}

impl std::error::Error for WatchServiceError {}
