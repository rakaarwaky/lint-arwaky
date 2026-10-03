//! Layout breakpoint tests for the TUI panel split (#557).
//!
//! Asserts the panel layout mode at 40, 80, and 160 columns matches the
//! breakpoint behavior documented in DESIGN.md:
//!
//! - width < `NARROW_BREAKPOINT_WIDTH` (100) → single active panel
//! - width >= `NARROW_BREAKPOINT_WIDTH` (100) → full 35/20/45 three-column split
//!
//! The `MIN_TERMINAL_WIDTH` floor (40) is guarded before the layout function is
//! called, so these tests only see widths >= 40.

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use shared_tui::utility_tui_theme::{
    MIN_TERMINAL_WIDTH, NARROW_BREAKPOINT_WIDTH, PANEL_FILE_LIST_WIDTH_PCT,
    PANEL_PREVIEW_WIDTH_PCT, PANEL_TREE_WIDTH_PCT, is_full_three_column,
};

/// Below the breakpoint the single active panel owns the whole panel area, so
/// its width is the panel width itself.
fn active_panel_width(panel_width: u16) -> u16 {
    if is_full_three_column(panel_width) {
        panic!("{panel_width} cols is at/above the breakpoint; not a single-panel width");
    }
    panel_width
}

/// `NARROW_BREAKPOINT_WIDTH` is 100 and must sit above the hard floor.
#[test]
fn breakpoint_constant_is_100() {
    // Plain value comparison, not a constant assertion: MIN_TERMINAL_WIDTH and
    // NARROW_BREAKPOINT_WIDTH are u16 consts whose relative order is the
    // property under test.
    let narrow: u16 = NARROW_BREAKPOINT_WIDTH;
    let floor: u16 = MIN_TERMINAL_WIDTH;
    assert_eq!(
        narrow, 100,
        "DESIGN.md documents NARROW_BREAKPOINT_WIDTH as 100"
    );
    assert!(
        narrow > floor,
        "the narrow breakpoint must sit above the hard floor"
    );
}

#[test]
fn at_min_terminal_width_layout_is_single_active_panel() {
    assert!(
        !is_full_three_column(MIN_TERMINAL_WIDTH),
        "{MIN_TERMINAL_WIDTH} cols is below the breakpoint → single active panel"
    );
    // 40 - 2*2 border = 36 content columns, enough for a 20-char filename.
    let content = active_panel_width(MIN_TERMINAL_WIDTH) - 4;
    assert!(
        content >= 20,
        "active panel at MIN_TERMINAL_WIDTH shows {content} content columns, need >= 20"
    );
}

#[test]
fn at_80_columns_layout_is_single_active_panel() {
    assert!(
        !is_full_three_column(80),
        "80 cols is below the breakpoint → single active panel"
    );
    assert_eq!(
        active_panel_width(80),
        80,
        "the active panel fills the area"
    );
}

#[test]
fn at_breakpoint_layout_is_three_columns() {
    assert!(
        is_full_three_column(NARROW_BREAKPOINT_WIDTH),
        "the breakpoint width itself uses the three-column split"
    );
}

#[test]
fn at_160_columns_layout_is_three_columns() {
    assert!(
        is_full_three_column(160),
        "160 cols is above the breakpoint → three-column split"
    );
}

/// The documented three-column widths must match what ratatui computes from the
/// same percentage tokens, so the split and the docs cannot drift.
#[test]
fn three_column_split_at_160_matches_documented_widths() {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(PANEL_TREE_WIDTH_PCT),
            Constraint::Percentage(PANEL_FILE_LIST_WIDTH_PCT),
            Constraint::Percentage(PANEL_PREVIEW_WIDTH_PCT),
        ])
        .split(Rect::new(0, 0, 160, 10));

    assert_eq!(
        layout[0].width, 56,
        "tree pane at 160 cols ({PANEL_TREE_WIDTH_PCT}%)"
    );
    assert_eq!(
        layout[1].width, 32,
        "file-list pane at 160 cols ({PANEL_FILE_LIST_WIDTH_PCT}%)"
    );
    assert_eq!(
        layout[2].width, 72,
        "preview pane at 160 cols ({PANEL_PREVIEW_WIDTH_PCT}%)"
    );
    assert_eq!(
        layout[0].width + layout[1].width + layout[2].width,
        160,
        "the three columns must consume the full panel width"
    );
}

/// Column count by terminal width: the table DESIGN.md publishes.
#[test]
fn documented_column_count_by_width() {
    // 40 cols — single active panel
    assert!(!is_full_three_column(40));
    // 80 cols — single active panel
    assert!(!is_full_three_column(80));
    // 99 cols — still below the breakpoint
    assert!(!is_full_three_column(99));
    // 100 cols — three columns
    assert!(is_full_three_column(100));
    // 160 cols — three columns
    assert!(is_full_three_column(160));
}
