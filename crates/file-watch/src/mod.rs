// PURPOSE: Module declarations and re-exports for file-watch (aggregates, provider, VOs, errors)
pub mod contract_provider_port;
pub use contract_provider_port::IWatchProviderPort;
pub mod taxonomy_watch_vo;
pub use taxonomy_watch_vo::DirectoryWatchVO;
pub mod infrastructure_watch_provider;
pub use infrastructure_watch_provider::WatchServiceProvider;
pub mod taxonomy_result_vo;
pub use taxonomy_result_vo::WatchResult;
pub mod taxonomy_service_error;
pub use taxonomy_service_error::{WatchEventError, WatchServiceError, WatchSubscriptionError};
