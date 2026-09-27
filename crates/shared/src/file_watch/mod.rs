// file-watch — taxonomy and contract types
pub mod contract_watch_aggregate;
pub mod contract_watch_protocol;
pub mod taxonomy_service_error;
pub mod taxonomy_watch_config_vo;
pub mod taxonomy_watch_request;
pub mod taxonomy_watch_response;

// ─── Re-exports ────────────────────────────────────────────
// Barrel re-export pattern: allows consumers to import directly

// ── Contract traits ──
pub use contract_watch_aggregate::IWatchAggregate;
pub use contract_watch_protocol::IChangeLintProtocol;
pub use contract_watch_protocol::IEventDedupProtocol;
pub use contract_watch_protocol::ILintableFilterProtocol;
pub use contract_watch_protocol::IWatchBroadcastProtocol;
pub use contract_watch_protocol::IWatchShutdownProtocol;
pub use contract_watch_protocol::IWatchStartProtocol;

// ── Taxonomy types ──
pub use taxonomy_service_error::WatchServiceError;
pub use taxonomy_watch_config_vo::GitDiffResultVO;
pub use taxonomy_watch_config_vo::WatchConfig;
pub use taxonomy_watch_config_vo::WatchEvent;
pub use taxonomy_watch_config_vo::WatchEventKind;
pub use taxonomy_watch_request::WatchRequest;
pub use taxonomy_watch_response::WatchResponse;
