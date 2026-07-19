// PURPOSE: PathScreen — TUI surface component for the path input dialog (overlay)
//
// Renders a centered popup overlay when show_path_dialog is true.
// User can type a path, confirm, or use current directory.
// Uses centered_rect() helper to calculate popup dimensions.
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};
use ratatui::Frame;
use shared::tui::taxonomy_state_vo::AppState;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct PathScreen;

// ─── Block 3: Constructors & Helpers ──────────────────────
impl PathScreen {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, state: &AppState, frame: &mut Frame, area: Rect) {
        let popup_area = centered_rect(60, 30, area);

        frame.render_widget(Clear, popup_area);

        let block = Block::default()
            .title(" Enter Project Path ")
            .borders(Borders::ALL)
            .border_style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
            .style(Style::default().bg(Color::Black));

        let cwd = std::env::current_dir()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|_| ".".to_string());

        let input_display = if state.path_input.is_empty() {
            format!("[{}]", cwd)
        } else {
            state.path_input.clone()
        };

        let text = vec![
            Line::from(""),
            Line::from(Span::styled(
                "  Type path or press Enter for current dir:",
                Style::default().fg(Color::White),
            )),
            Line::from(""),
            Line::from(vec![
                Span::styled("  > ", Style::default().fg(Color::Green)),
                Span::styled(
                    input_display,
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled("_", Style::default().fg(Color::White)),
            ]),
            Line::from(""),
            Line::from(Span::styled(
                "  [Enter] Confirm   [Tab] Use current dir   [Esc] Quit",
                Style::default().fg(Color::DarkGray),
            )),
        ];

        let paragraph = Paragraph::new(text).block(block).alignment(Alignment::Left);

        frame.render_widget(paragraph, popup_area);
    }
}

// ─── Block 2: Public Contract ─────────────────────────────
impl Default for PathScreen {
    fn default() -> Self {
        Self::new()
    }
}

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
