// PURPOSE: TUI panel layout — single source of truth for panel geometry.
//
// Mirrors the responsive vertical+horizontal split in surface_tui_command.rs
// so the renderer and mouse hit-mapping share one layout (issue #555).
//
// Two modes, selected by `is_full_three_column(panel_width)`:
//   - three_column: tree 35% / file list 20% / preview 45% of the panel band
//   - narrow (panel < NARROW_BREAKPOINT_WIDTH): only the active panel fills the
//     band; `band()` returns the full panel-band rect for that case
//
// The mode is decided from the *panel* band width, not the terminal width, to
// match `is_full_three_column` exactly (the renderer passes
// `main_layout[1].width`).

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use shared_tui::utility_tui_theme::{
    PANEL_FILE_LIST_WIDTH_PCT, PANEL_PREVIEW_WIDTH_PCT, PANEL_TREE_WIDTH_PCT, is_full_three_column,
};

/// Computed panel rects for one frame.
pub struct PanelLayout {
    /// Header row (first row of the vertical split).
    pub header: Rect,
    /// Tree panel rect (left column of the panel band, three-column mode).
    pub tree: Rect,
    /// File list panel rect (middle column, three-column mode).
    pub file_list: Rect,
    /// Preview panel rect (right column, three-column mode).
    pub preview: Rect,
    /// The full panel band (second row of the vertical split). Used to render
    /// the single active panel in narrow mode, and for hit-testing.
    pub band: Rect,
    /// Shortcut bar (third row of the vertical split).
    pub shortcuts: Rect,
    /// Status bar (last row of the vertical split).
    pub status: Rect,
    /// True when the panel band is wide enough for the full 3-column split.
    pub three_column: bool,
}

impl PanelLayout {
    /// The panel-band rect (for narrow-mode rendering / hit-testing).
    pub fn band(&self) -> Rect {
        self.band
    }
}

/// Compute panel rects for a terminal of `width × height`.
///
/// Vertical split: 1 (header) / min(10) panels / 3 (shortcuts) / 1 (status).
/// Horizontal panel split (three-column mode): tree % / file-list % / preview
/// %, using the shared theme constants.
///
/// Callers pass the full terminal area so the rects are in absolute frame
/// coordinates. Both the renderer and `handle_mouse_click` use this, so the
/// two can never drift apart.
pub fn compute_panel_layout(width: u16, height: u16) -> PanelLayout {
    let area = Rect::new(0, 0, width, height);
    let main = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(10),
            Constraint::Length(3),
            Constraint::Length(1),
        ])
        .split(area);
    let band = main[1];
    let three_column = is_full_three_column(band.width);
    let panels = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(PANEL_TREE_WIDTH_PCT),
            Constraint::Percentage(PANEL_FILE_LIST_WIDTH_PCT),
            Constraint::Percentage(PANEL_PREVIEW_WIDTH_PCT),
        ])
        .split(band);
    PanelLayout {
        header: main[0],
        tree: panels[0],
        file_list: panels[1],
        preview: panels[2],
        band,
        shortcuts: main[2],
        status: main[3],
        three_column,
    }
}
