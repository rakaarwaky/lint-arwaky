//! NO_COLOR is a rendering contract, not only a glyph substitution.
//!
//! Every design token in the palette must resolve to `Color::Reset` (the
//! terminal's default foreground/background) when NO_COLOR is requested, so
//! no named/RGB/Indexed color can reach the terminal and the forced black
//! background from #360 disappears.
use ratatui::style::{Color, Modifier};
use shared_tui::utility_tui_theme;

/// All design-token colors in the palette table.
const PALETTE: &[Color] = &[
    utility_tui_theme::ACCENT,
    utility_tui_theme::KEY,
    utility_tui_theme::LABEL,
    utility_tui_theme::SEPARATOR,
    utility_tui_theme::VIOLATIONS,
    utility_tui_theme::CLEAN,
    utility_tui_theme::PENDING,
    utility_tui_theme::DIRECTORY,
    utility_tui_theme::CAPABILITIES_BADGE,
    utility_tui_theme::BACKGROUND,
    utility_tui_theme::HIGHLIGHT,
    utility_tui_theme::PATH_INPUT,
    utility_tui_theme::FOCUS_CONFIRM,
    utility_tui_theme::SCROLLBAR,
    utility_tui_theme::HEADER,
];

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
fn whole_palette_yields_only_terminal_defaults_under_no_color() {
    for token in PALETTE {
        assert_eq!(
            utility_tui_theme::color_with_override(*token, true),
            Color::Reset,
            "palette token {token:?} must resolve to Color::Reset under NO_COLOR"
        );
    }
}

#[test]
fn whole_palette_keeps_named_ansi_colors_without_no_color() {
    for token in PALETTE {
        assert_eq!(
            utility_tui_theme::color_with_override(*token, false),
            *token,
            "palette token {token:?} must pass through unchanged when colors are allowed"
        );
        match token {
            Color::Rgb(_, _, _) | Color::Indexed(_) => {
                panic!("palette token {token:?} must be a named ANSI color, never Rgb/Indexed")
            }
            _ => {}
        }
    }
}

#[test]
fn theme_background_closes_to_terminal_default_when_no_color_is_requested() {
    // BACKGROUND (forced Color::Black from #360) resolves to Color::Reset
    // under NO_COLOR, so the terminal's native background shows through.
    assert_eq!(
        utility_tui_theme::color_with_override(utility_tui_theme::BACKGROUND, true),
        Color::Reset
    );
}

// ── high-contrast mode (#562) ─────────────────────────────────────────

#[test]
fn selection_band_paints_the_highlight_token_by_default() {
    let style = utility_tui_theme::highlight_style_with_override(false);

    assert_eq!(style.bg, Some(utility_tui_theme::HIGHLIGHT));
    assert!(!style.add_modifier.contains(Modifier::REVERSED));
    assert!(style.add_modifier.contains(Modifier::BOLD));
}

#[test]
fn selection_band_uses_reversed_video_in_high_contrast_mode() {
    let style = utility_tui_theme::highlight_style_with_override(true);

    assert_eq!(style.bg, None);
    assert!(style.add_modifier.contains(Modifier::REVERSED));
    assert!(style.add_modifier.contains(Modifier::BOLD));
}

#[test]
fn high_contrast_is_opt_in_and_loses_to_no_color() {
    assert!(!utility_tui_theme::high_contrast_with_override(false, false));
    assert!(utility_tui_theme::high_contrast_with_override(false, true));
    // NO_COLOR wins: reversed video still assumes the terminal renders color.
    assert!(!utility_tui_theme::high_contrast_with_override(true, true));
}
