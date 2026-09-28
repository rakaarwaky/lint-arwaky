// PURPOSE: ConfirmScreen — TUI surface component for the destructive-action
// confirmation dialog (I5 confirm gate, UX-1-01).
//
// Renders a centered modal popup whenever `state.pending_confirm` is set,
// naming the pending action and its answer keys. Input routing lives in
// surface_tui_command::from_key_event (confirm gate); execution after
// confirmation lives in surface_event_action (ConfirmAction arm).
use crate::utility_tui_theme as theme;
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Wrap};
use shared::tui::AppState;

pub struct ConfirmScreen;

impl ConfirmScreen {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, state: &AppState, frame: &mut Frame, area: Rect) {
        let Some(confirm) = state.pending_confirm.as_ref() else {
            return;
        };

        let popup_area = centered_rect(50, 25, area);

        frame.render_widget(Clear, popup_area);

        let block = Block::default()
            .title(" Confirm Action ")
            .borders(Borders::ALL)
            .border_style(
                Style::default()
                    .fg(theme::VIOLATIONS)
                    .add_modifier(Modifier::BOLD),
            )
            .style(Style::default().bg(theme::BACKGROUND));

        let text = vec![
            Line::from(""),
            Line::from(Span::styled(
                format!("  {}", confirm.label),
                Style::default()
                    .fg(theme::LABEL)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "  This action changes system or repository state.",
                Style::default().fg(theme::SEPARATOR),
            )),
            Line::from(""),
            Line::from(vec![
                Span::styled("  [y/Enter] ", Style::default().fg(theme::FOCUS_CONFIRM)),
                Span::styled("Confirm   ", Style::default().fg(theme::LABEL)),
                Span::styled("[n/Esc] ", Style::default().fg(theme::KEY)),
                Span::styled("Cancel", Style::default().fg(theme::LABEL)),
            ]),
        ];

        let paragraph = Paragraph::new(text)
            .block(block)
            .wrap(Wrap { trim: false })
            .alignment(Alignment::Left);

        frame.render_widget(paragraph, popup_area);
    }
}

impl Default for ConfirmScreen {
    fn default() -> Self {
        Self::new()
    }
}

/// Same centering math as the path dialog; duplicated locally so the two
/// overlays stay decoupled (AES: surfaces own their rendering).
fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
