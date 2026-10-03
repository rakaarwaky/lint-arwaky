//! NO_COLOR is a rendering contract, not only a glyph substitution.
use ratatui::style::{Color, Modifier};
use shared_tui::utility_tui_theme;

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

#[test]
fn selection_band_paints_the_highlight_token_by_default() {
    let style = utility_tui_theme::highlight_style_with_override(false);

    assert_eq!(style.bg, Some(utility_tui_theme::HIGHLIGHT));
    assert!(!style.add_modifier.contains(Modifier::REVERSED));
    assert!(style.add_modifier.contains(Modifier::BOLD));
}

#[test]
fn high_contrast_selection_band_uses_reversed_video_instead_of_a_palette_colour() {
    let style = utility_tui_theme::highlight_style_with_override(true);

    // Reversed video inverts the colours the terminal already renders legibly,
    // so the band survives a theme where DarkGray sits near the background.
    assert!(style.add_modifier.contains(Modifier::REVERSED));
    assert_eq!(style.bg, None);
    assert!(style.add_modifier.contains(Modifier::BOLD));
}

#[test]
fn high_contrast_mode_activates_on_request_and_yields_a_distinct_band() {
    assert!(!utility_tui_theme::high_contrast_with_override(
        false, false
    ));
    assert!(utility_tui_theme::high_contrast_with_override(false, true));
    assert_ne!(
        utility_tui_theme::highlight_style_with_override(false),
        utility_tui_theme::highlight_style_with_override(true)
    );
}

#[test]
fn no_color_outranks_high_contrast_when_both_are_requested() {
    // A request for no colour is a stricter instruction than a request for
    // more contrast, so the mode must not re-introduce a palette colour.
    assert!(!utility_tui_theme::high_contrast_with_override(true, true));
    assert_eq!(
        utility_tui_theme::color_with_override(Color::Red, true),
        Color::Reset
    );
}
