// PURPOSE: TUI theme — central color palette (design tokens) for all TUI views (I7).
// Views reference these semantic constants instead of hardcoding Color:: values.
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
