// PURPOSE: WatchResult — result type for watch operations
use serde::{Deserialize, Serialize};

use shared::file_watch::taxonomy_service_error::WatchServiceError;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub enum WatchResult {
    #[default]
    Started,
    Stopped,
    Changed(Vec<String>),
    Error(WatchServiceError),
}
