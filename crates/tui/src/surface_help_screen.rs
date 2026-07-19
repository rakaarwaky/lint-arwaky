// PURPOSE: HelpScreen — TUI surface component for the help overlay
//
// Provides a static method to check if help is active (is_active)
// and a placeholder render_hint for future inline help hints.
// The actual help content is rendered by PreviewView in HelpOverlay mode.
use ratatui::layout::Rect;
use ratatui::Frame;
use shared::tui::taxonomy_state_vo::AppState;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct HelpScreen;

// ─── Block 3: Constructors & Helpers ──────────────────────
impl HelpScreen {
    pub fn new() -> Self {
        Self
    }

    pub fn is_active(state: &AppState) -> bool {
        state.show_help
    }

    pub fn render_hint(frame: &mut Frame, area: Rect) {
        let _ = frame;
        let _ = area;
    }
}

// ─── Block 2: Public Contract ─────────────────────────────
impl Default for HelpScreen {
    fn default() -> Self {
        Self::new()
    }
}
