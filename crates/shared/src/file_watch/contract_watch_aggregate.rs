// PURPOSE: IWatchAggregate — single entry point over the watch domain
// The agent behind the aggregate dispatches each WatchRequest to the right
// capability operation. Consumers never see the provider or analyzer protocols.
use crate::taxonomy_file_watch_request::WatchRequest;
use crate::taxonomy_file_watch_response::WatchResponse;

/// Single entry point over the file-watch domain; the agent dispatches internally.
pub trait IWatchAggregate: Send + Sync {
    fn execute(&self, request: WatchRequest) -> WatchResponse;
}
