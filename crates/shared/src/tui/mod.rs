pub mod taxonomy_action_flags_vo;
pub mod taxonomy_adapter_info_vo;
pub mod taxonomy_file_entry_vo;
pub mod taxonomy_lint_result_vo;
pub mod taxonomy_scan_update_vo;
pub mod taxonomy_state_vo;
pub mod taxonomy_tui_event;
pub mod taxonomy_watch_message_vo;

// ─── Re-exports ────────────────────────────────────────────
// Barrel re-export pattern: allows consumers to import directly

// ── Taxonomy types ──
pub use taxonomy_action_flags_vo::ActionFlags;
pub use taxonomy_adapter_info_vo::AdapterInfo;
pub use taxonomy_file_entry_vo::AesLayer;
pub use taxonomy_file_entry_vo::FileEntry;
pub use taxonomy_lint_result_vo::LintExecutionResult;
pub use taxonomy_scan_update_vo::ScanUpdate;
pub use taxonomy_state_vo::AppState;
pub use taxonomy_state_vo::PanelFocus;
pub use taxonomy_state_vo::PreviewMode;
pub use taxonomy_tui_event::TuiEvent;
pub use taxonomy_watch_message_vo::WatchMessage;
