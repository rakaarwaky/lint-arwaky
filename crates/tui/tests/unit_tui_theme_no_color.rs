//! NO_COLOR is a rendering contract, not only a glyph substitution.
use ratatui::style::Color;
use tui_lint_arwaky::utility_tui_theme;

#[test]
fn theme_colors_collapse_to_terminal_defaults_when_no_color_is_requested() {
    assert_eq!(
        utility_tui_theme::color_with_override(Color::Red, true),
        Color::Reset
    );
    assert_eq!(
        utility_tui_theme::color_with_override(Color::Cyan, true),
        Color::Reset
    );
    assert_eq!(
        utility_tui_theme::color_with_override(Color::Red, false),
        Color::Red
    );
}
