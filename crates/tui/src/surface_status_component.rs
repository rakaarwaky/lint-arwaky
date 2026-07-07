// PURPOSE: StatusComponent — TUI surface component for the status bar (bottom line)
//
// Displays current status message, selected file name, and violation count.
// Violation count is colored red when > 0, green when 0.
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
        let line = if state.scanning {
            // Scan progress indicator: progress bar + phase + counts
            let bar_width = 20;
            let (filled, _) = if state.scan_files_total > 0 {
                let ratio = state.scan_files_done as f64 / state.scan_files_total as f64;
                let filled = (ratio * bar_width as f64) as usize;
                (filled.min(bar_width), bar_width)
            } else {
                (0, bar_width)
            };
            let empty = bar_width - filled;
            let bar: String = "█".repeat(filled) + &"░".repeat(empty);

            let phase_display = if state.scan_phase.is_empty() {
                "Scanning...".to_string()
            } else {
                state.scan_phase.clone()
            };

            let progress_detail = if state.scan_files_total > 0 {
                format!(
                    " {} {}/{}",
                    bar, state.scan_files_done, state.scan_files_total,
                )
            } else {
                format!(" {}", bar)
            };

            let violation_style = if state.scan_violations > 0 {
                Style::default().fg(Color::Red)
            } else {
                Style::default().fg(Color::Yellow)
            };

            Line::from(vec![
                Span::styled(
                    format!(" {} ", phase_display),
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(ratatui::style::Modifier::BOLD),
                ),
                Span::styled(progress_detail, Style::default().fg(Color::White)),
                Span::styled(" \u{2502} ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    format!("{} violations", state.scan_violations),
                    violation_style,
                ),
            ])
        } else {
            let selected_name = match state.selected_entry() {
                Some(entry) => entry.display_name(),
                None => "(none)".to_string(),
            };

            let violation_style = if state.violation_count > 0 {
                Style::default().fg(Color::Red)
            } else {
                Style::default().fg(Color::Green)
            };

            Line::from(vec![
                Span::styled(" Status: ", Style::default().fg(Color::DarkGray)),
                Span::styled(&state.status_message, Style::default().fg(Color::White)),
                Span::styled(" \u{2502} ", Style::default().fg(Color::DarkGray)),
                Span::styled("Selected: ", Style::default().fg(Color::DarkGray)),
                Span::styled(selected_name, Style::default().fg(Color::Cyan)),
                Span::styled(" \u{2502} ", Style::default().fg(Color::DarkGray)),
                Span::styled(format!("{} viol.", state.violation_count), violation_style),
            ])
        };

        let paragraph = Paragraph::new(line).style(Style::default().bg(Color::Black));
        frame.render_widget(paragraph, area);
    }
}

impl Default for StatusComponent {
    fn default() -> Self {
        Self::new()
    }
}
