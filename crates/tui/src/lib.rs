// PURPOSE: Module declarations for tui (Surface-only crate)
// No contract/aggregate/capabilities layers — surfaces call domain aggregates directly.
pub mod root_tui_container;
pub mod taxonomy_tui_event;
pub mod taxonomy_tui_vo;
pub mod surface_event_action;
pub mod surface_file_list_view;
pub mod surface_lint_action;
pub mod surface_logging_controller;
pub mod surface_path_screen;
pub mod surface_preview_view;
pub mod surface_shortcut_component;
pub mod surface_status_component;
pub mod surface_tree_view;
pub mod surface_tui_command;
pub mod utility_file_system;
pub mod utility_report_formatter;
pub mod utility_tui_theme;

// ─── Re-exports ────────────────────────────────────────────
// The TUI taxonomy used to live in the shared kernel crate (shared-tui); it
// moved here because the TUI is its only consumer (issue #572).

pub use taxonomy_tui_event::TuiEvent;
pub use taxonomy_tui_vo::ActionFlags;
pub use taxonomy_tui_vo::AdapterInfo;
pub use taxonomy_tui_vo::AesLayer;
pub use taxonomy_tui_vo::AppState;
pub use taxonomy_tui_vo::BusyKind;
pub use taxonomy_tui_vo::ConfirmState;
pub use taxonomy_tui_vo::FileEntry;
pub use taxonomy_tui_vo::LintExecutionResult;
pub use taxonomy_tui_vo::PanelFocus;
pub use taxonomy_tui_vo::PreviewMode;
pub use taxonomy_tui_vo::ScanUpdate;
pub use taxonomy_tui_vo::WatchMessage;
