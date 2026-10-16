// PURPOSE: ConfirmModal — TUI surface component for the destructive-action confirm gate (#554)
//
// Renders a centered, bordered modal whenever `state.actions.pending_confirm`
// is set, so arming a destructive action (live fix, install, init,
// install-hook, uninstall-hook) reads as a gate rather than another transient
// status-bar line. Renders nothing when no confirmation is pending, so the
// caller can draw it unconditionally after the panels.
//
// Border and glyphs degrade to ASCII when NO_COLOR is set (#365), matching the
// status component; colour comes from theme tokens, never a literal.
use crate::AppState;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::symbols::border;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};
use shared_tui::utility_tui_theme as theme;

/// Modal height in rows: three content lines plus the top and bottom border.
const MODAL_HEIGHT: u16 = 5;

/// ASCII border set used when NO_COLOR is set (#365).
const ASCII_BORDER: border::Set<'static> = border::Set {
    top_left: "+",
    top_right: "+",
    bottom_left: "+",
    bottom_right: "+",
    vertical_left: "|",
    vertical_right: "|",
    horizontal_top: "-",
    horizontal_bottom: "-",
};

/// Returns the warning glyph shown before the action label.
/// Falls back to "!" when NO_COLOR is set (#365).
fn warn() -> &'static str {
    if theme::no_color() { "!" } else { "\u{26a0}" }
}

/// Returns the centered modal area, never taller or wider than the viewport.
///
/// Shares the centering helper with the path-entry screen so both overlays
/// land in the same place.
fn modal_area(area: Rect) -> Rect {
    let popup = crate::surface_path_screen::centered_rect(60, 30, area);
    Rect {
        height: popup.height.min(MODAL_HEIGHT).min(area.height),
        width: popup.width.min(area.width),
        ..popup
    }
}

pub struct ConfirmModal;

impl ConfirmModal {
    pub fn new() -> Self {
        Self
    }

    /// Render the confirm modal over `area`, or nothing when no confirmation is pending.
    pub fn render(&self, state: &AppState, frame: &mut Frame, area: Rect) {
        let Some(confirm) = state.actions.pending_confirm.as_ref() else {
            return;
        };

        let popup_area = modal_area(area);
        frame.render_widget(Clear, popup_area);

        let block = Block::default()
            .title(" Confirm ")
            .borders(Borders::ALL)
            .border_set(if theme::no_color() {
                ASCII_BORDER
            } else {
                border::THICK
            })
            .border_style(
                Style::default()
                    .fg(theme::color(theme::PENDING))
                    .add_modifier(theme::EMPHASIS_SELECTED),
            )
            .style(Style::default().bg(theme::color(theme::BACKGROUND)));

        let lines = vec![
            Line::from(vec![
                Span::styled(
                    format!(" {} ", warn()),
                    Style::default()
                        .fg(theme::color(theme::PENDING))
                        .add_modifier(theme::EMPHASIS_SELECTED),
                ),
                Span::styled(
                    confirm.label.clone(),
                    Style::default()
                        .fg(theme::color(theme::LABEL))
                        .add_modifier(theme::EMPHASIS_SELECTED),
                ),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    " [Enter/y] confirm ",
                    Style::default()
                        .fg(theme::color(theme::KEY))
                        .add_modifier(theme::EMPHASIS_SELECTED),
                ),
                Span::styled(
                    "[Esc/n] cancel",
                    Style::default().fg(theme::color(theme::SEPARATOR)),
                ),
            ]),
        ];

        frame.render_widget(Paragraph::new(lines).block(block), popup_area);
    }
}

impl Default for ConfirmModal {
    fn default() -> Self {
        Self::new()
    }
}
