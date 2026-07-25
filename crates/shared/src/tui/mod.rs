pub mod taxonomy_action_flags_vo;
pub mod taxonomy_adapter_info_vo;
pub mod taxonomy_file_entry_vo;
pub mod taxonomy_lint_result_vo;
pub mod taxonomy_scan_update_vo;
pub mod taxonomy_state_vo;
pub mod taxonomy_tui_event;
pub mod taxonomy_watch_message_vo;

pub mod contract_action_handler_protocol;
pub mod contract_lint_executor_protocol;
pub mod contract_report_formatter_protocol;
pub mod contract_tui_aggregate;

pub mod utility_tui_io;

// ─── Re-exports ────────────────────────────────────────────
// Barrel re-export pattern: allows consumers to import directly

// ── Contract traits ──
pub use contract_action_handler_protocol::IActionHandlerProtocol;
pub use contract_lint_executor_protocol::ILintExecutorProtocol;
pub use contract_report_formatter_protocol::IReportFormatterProtocol;
pub use contract_tui_aggregate::ITuiAggregate;

// ── Taxonomy types ──
pub use taxonomy_action_flags_vo::ActionFlags;
pub use taxonomy_adapter_info_vo::AdapterInfo;
pub use taxonomy_file_entry_vo::FileEntry;
pub use taxonomy_file_entry_vo::AesLayer;
pub use taxonomy_lint_result_vo::LintExecutionResult;
pub use taxonomy_scan_update_vo::ScanUpdate;
pub use taxonomy_state_vo::AppState;
pub use taxonomy_state_vo::PanelFocus;
pub use taxonomy_state_vo::PreviewMode;
pub use taxonomy_tui_event::TuiEvent;
pub use taxonomy_watch_message_vo::WatchMessage;
