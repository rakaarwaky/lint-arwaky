// PURPOSE: TUI theme — central color palette (design tokens) for all TUI views (I7).
//
// When the NO_COLOR environment variable is set, every color resolved through
// `color()`/`color_with_override()` becomes `Color::Reset`, delegating
// foreground and background to the terminal's own defaults. No palette
// constant may be applied to a `.fg()`/`.bg()` call site without going
// through this accessor; the only remaining NO_COLOR handling in the TUI is
// the ASCII glyph substitution in the status bar (#365).
//
// Contrast limit: every token here names an ANSI colour, and the terminal
// emulator — not this application — decides the RGB each name renders to. A
// low-contrast terminal theme can make `HIGHLIGHT` (`DarkGray`) nearly vanish
// against the canvas. The app cannot sample or override the user's theme, so
// it ships an opt-in mode instead: LINT_ARWAKY_TUI_HIGH_CONTRAST swaps the
// selection band for reversed video, which inverts the colours the terminal is
// already using successfully. See `highlight_style`.
use ratatui::style::{Color, Modifier, Style};

/// Environment variable that opts the TUI into the high-contrast rendering mode.
pub const HIGH_CONTRAST_VAR: &str = "LINT_ARWAKY_TUI_HIGH_CONTRAST";

/// TUI design tokens — central color palette (I7).
pub const ACCENT: Color = Color::Cyan;

/// Emphasis design tokens — central modifier palette (I7).
/// Call sites reference these instead of writing `Modifier::BOLD` inline.
pub const EMPHASIS_SELECTED: Modifier = Modifier::BOLD;
pub const EMPHASIS_HEADING: Modifier = Modifier::BOLD;
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
pub const NARROW_BREAKPOINT_WIDTH: u16 = 100;

/// Returns true when the panel area is wide enough for the full 3-column split.
/// Below `NARROW_BREAKPOINT_WIDTH` the layout collapses to a single active panel.
pub fn is_full_three_column(width: u16) -> bool {
    width >= NARROW_BREAKPOINT_WIDTH
}

/// Returns true when the user has requested no-color output.
pub fn no_color() -> bool {
    std::env::var_os("NO_COLOR").is_some()
}

/// Resolve a design-token color for the current terminal accessibility mode.
/// All renderers call this helper instead of applying raw palette constants.
pub fn color(value: Color) -> Color {
    color_with_override(value, no_color())
}

/// Pure form of [`color`] used by rendering tests without mutating process-wide
/// environment state. `Color::Reset` delegates foreground/background selection
/// to the terminal and is safe for monochrome terminals.
pub fn color_with_override(value: Color, no_color_requested: bool) -> Color {
    if no_color_requested {
        Color::Reset
    } else {
        value
    }
}

/// Returns true when the user has requested the high-contrast rendering mode.
///
/// Opt-in only: `COLORFGBG` is absent on many terminals and misreports on some,
/// so inferring the background would flip the selection band on an unknown
/// basis. `NO_COLOR` wins over this mode; see [`highlight_style`].
pub fn high_contrast() -> bool {
    high_contrast_with_override(no_color(), std::env::var_os(HIGH_CONTRAST_VAR).is_some())
}

/// Pure form of [`high_contrast`] used by rendering tests without mutating
/// process-wide environment state, which the test runner executes in parallel.
pub fn high_contrast_with_override(
    no_color_requested: bool,
    high_contrast_requested: bool,
) -> bool {
    !no_color_requested && high_contrast_requested
}

/// Resolve the style for a selected row, the one element whose contrast the
/// terminal theme can actually break.
///
/// Default mode paints `HIGHLIGHT` as a background, which disappears when the
/// user's theme maps `DarkGray` close to their background. High-contrast mode
/// uses reversed video instead: the terminal swaps the foreground and
/// background colours it is already rendering legibly, so the band stays
/// visible on light and dark themes alike without the app knowing the
/// terminal's palette.
pub fn highlight_style() -> Style {
    highlight_style_with_override(high_contrast())
}

/// Pure form of [`highlight_style`] for rendering tests that cannot set
/// process-wide environment state.
pub fn highlight_style_with_override(high_contrast_requested: bool) -> Style {
    let base = if high_contrast_requested {
        Style::default().add_modifier(Modifier::REVERSED)
    } else {
        Style::default().bg(HIGHLIGHT)
    };
    base.add_modifier(Modifier::BOLD)
}
