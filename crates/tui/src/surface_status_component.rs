use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame;
use shared::tui::taxonomy_state_vo::AppState;

pub struct StatusComponent;

impl StatusComponent {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, state: &AppState, frame: &mut Frame, area: Rect) {
        let selected_name = match state.selected_entry() {
            Some(entry) => entry.display_name(),
            None => "(none)".to_string(),
        };

        let violation_style = if state.violation_count > 0 {
            Style::default().fg(Color::Red)
        } else {
            Style::default().fg(Color::Green)
        };

        let line = Line::from(vec![
            Span::styled(" Status: ", Style::default().fg(Color::DarkGray)),
            Span::styled(&state.status_message, Style::default().fg(Color::White)),
            Span::styled(" \u{2502} ", Style::default().fg(Color::DarkGray)),
            Span::styled("Selected: ", Style::default().fg(Color::DarkGray)),
            Span::styled(selected_name, Style::default().fg(Color::Cyan)),
            Span::styled(" \u{2502} ", Style::default().fg(Color::DarkGray)),
            Span::styled(format!("{} viol.", state.violation_count), violation_style),
        ]);

        let paragraph = Paragraph::new(line).style(Style::default().bg(Color::Black));
        frame.render_widget(paragraph, area);
    }
}

impl Default for StatusComponent {
    fn default() -> Self {
        Self::new()
    }
}
