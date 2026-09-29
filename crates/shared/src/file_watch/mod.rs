// file-watch — taxonomy and contract types
pub mod contract_watch_aggregate;
pub mod contract_watch_protocol;
pub mod taxonomy_file_watch_error;
pub mod taxonomy_file_watch_request;
pub mod taxonomy_file_watch_response;
pub mod taxonomy_file_watch_vo;

// ─── Re-exports ────────────────────────────────────────────
// Barrel re-export pattern: allows consumers to import directly

// ── Contract traits ──
pub use contract_watch_aggregate::IWatchAggregate;
pub use contract_watch_protocol::IChangeFilterProtocol;
pub use contract_watch_protocol::IChangeLintProtocol;
pub use contract_watch_protocol::IWatchLifecycleProtocol;

// ── Taxonomy types ──
pub use taxonomy_file_watch_error::WatchServiceError;
pub use taxonomy_file_watch_request::WatchRequest;
pub use taxonomy_file_watch_response::WatchResponse;
pub use taxonomy_file_watch_vo::GitDiffResultVO;
pub use taxonomy_file_watch_vo::WatchConfig;
pub use taxonomy_file_watch_vo::WatchEvent;
pub use taxonomy_file_watch_vo::WatchEventKind;
