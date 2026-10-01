// PURPOSE: ShortcutComponent — TUI surface component for the keyboard shortcut bar (bottom area)
//
// Renders 3 rows of keyboard shortcuts with key:label pairs.
// Has two modes:
//   - default_rows: shows navigation/action shortcuts during normal browsing
//   - context_sensitive_rows: shows context-appropriate shortcuts when viewing
//     lint results or action output
//
// format_shortcuts() renders each row as colored spans (yellow keys, white labels).
use crate::utility_tui_theme as theme;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use shared_tui::{AppState, PreviewMode};

pub struct ShortcutComponent;

impl ShortcutComponent {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, state: &AppState, frame: &mut Frame, area: Rect) {
        let rows = crate::utility_shortcuts::rows_for_context(matches!(
            state.preview.mode,
            PreviewMode::LintResults | PreviewMode::ActionOutput | PreviewMode::FileContent
        ));
        let [row1, row2, row3] = rows;

        // I1: while search mode is active, show the live search line instead of shortcuts.
        if state.search.mode {
            let text = format!(
                "search: {} (Enter confirm / Esc cancel)",
                state.search.query
            );
            let paragraph = Paragraph::new(text).style(
                Style::default()
                    .bg(theme::color(theme::BACKGROUND))
                    .fg(theme::color(theme::KEY)),
            );
            frame.render_widget(paragraph, area);
            return;
        }

        let text = vec![
            Line::from(format_shortcuts(&row1)),
            Line::from(format_shortcuts(&row2)),
            Line::from(format_shortcuts(&row3)),
        ];

        let paragraph =
            Paragraph::new(text).style(Style::default().bg(theme::color(theme::BACKGROUND)));
        frame.render_widget(paragraph, area);
    }
}

impl Default for ShortcutComponent {
    fn default() -> Self {
        Self::new()
    }
}

fn format_shortcuts<'a>(shortcuts: &'a [(&'a str, &'a str)]) -> Vec<Span<'a>> {
    let mut spans = Vec::new();
    for (i, (key, label)) in shortcuts.iter().enumerate() {
        if i > 0 {
            spans.push(Span::raw("  "));
        }
        spans.push(Span::styled(
            format!("{}:", key),
            Style::default().fg(theme::color(theme::KEY)),
        ));
        spans.push(Span::styled(
            label.to_string(),
            Style::default().fg(theme::color(theme::LABEL)),
        ));
    }
    spans
}
