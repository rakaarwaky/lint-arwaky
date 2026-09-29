// PURPOSE: TUI theme — central color palette (design tokens) for all TUI views (I7).
//
// When the NO_COLOR environment variable is set, all colors become monochrome
// (white on black background) so the TUI remains readable on dumb terminals
// and light-theme setups. This covers the whole TUI — not just the status bar.
use ratatui::style::Color;

/// TUI design tokens — central color palette (I7).
pub const ACCENT: Color = Color::Cyan;
pub const KEY: Color = Color::Yellow;
pub const LABEL: Color = Color::White;
pub const SEPARATOR: Color = Color::DarkGray;
pub const VIOLATIONS: Color = Color::Red;
pub const CLEAN: Color = Color::Green;
pub const PENDING: Color = Color::Yellow;
pub const DIRECTORY: Color = Color::Blue;
pub const CAPABILITIES_BADGE: Color = Color::Magenta;
pub const BACKGROUND: Color = Color::Black;
pub const HIGHLIGHT: Color = Color::DarkGray;
pub const PATH_INPUT: Color = Color::Yellow;
pub const FOCUS_CONFIRM: Color = Color::Green;
pub const SCROLLBAR: Color = Color::DarkGray;
pub const HEADER: Color = Color::Cyan;

/// Layout constants — single source of truth for panel geometry (I9).
/// These replace the duplicated magic numbers in surface_tui_command.rs and
/// surface_file_list_view.rs so mouse hit-maps and panel splits stay in sync.
pub const PANEL_FILE_LIST_WIDTH_PCT: u16 = 20;
pub const PANEL_PREVIEW_WIDTH_PCT: u16 = 45;
pub const PANEL_TREE_WIDTH_PCT: u16 = 35;
pub const SHORTCUT_ROW_COUNT: u16 = 3;
pub const STATUS_BAR_HEIGHT: u16 = 1;
pub const HEADER_HEIGHT: u16 = 1;
pub const MIN_TERMINAL_HEIGHT: u16 = 15;
pub const MIN_TERMINAL_WIDTH: u16 = 40;

/// Returns true when the user has requested no-color output.
pub fn no_color() -> bool {
    std::env::var("NO_COLOR").is_ok()
}
