pub mod taxonomy_tui_event;
pub mod taxonomy_tui_vo;

// ─── Re-exports ────────────────────────────────────────────
// Barrel re-export pattern: allows consumers to import directly

// ── Taxonomy types ──
pub use taxonomy_tui_event::TuiEvent;
pub use taxonomy_tui_vo::ActionFlags;
pub use taxonomy_tui_vo::AdapterInfo;
pub use taxonomy_tui_vo::AesLayer;
pub use taxonomy_tui_vo::AppState;
pub use taxonomy_tui_vo::ConfirmState;
pub use taxonomy_tui_vo::FileEntry;
pub use taxonomy_tui_vo::LintExecutionResult;
pub use taxonomy_tui_vo::PanelFocus;
pub use taxonomy_tui_vo::PreviewMode;
pub use taxonomy_tui_vo::ScanUpdate;

pub use taxonomy_tui_vo::{
    ActionState, NavigationState, PathDialogState, PreviewState, ScanProgressState, SearchState,
};
