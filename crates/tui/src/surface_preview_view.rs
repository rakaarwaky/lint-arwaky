// PURPOSE: PreviewView — TUI surface component for the preview panel (right panel)
//
// Renders four content modes depending on PreviewMode:
//   - FileContent: inline file preview (up to 100 lines)
//   - LintResults: output from check/scan/fix/ci/orphan actions
//   - ActionOutput: output from doctor/init/install/version/adapters actions
//   - HelpOverlay: keyboard shortcut reference
//
// Help content is embedded as a static string in help_text().
use crate::utility_tui_theme as theme;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::widgets::{
    Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, Wrap,
};
use shared_tui::{AppState, PanelFocus, PreviewMode};

pub struct PreviewView;

impl PreviewView {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, state: &AppState, frame: &mut Frame, area: Rect) {
        let is_focused = state.navigation.panel_focus == PanelFocus::Preview;
        let border_style = if is_focused {
            Style::default()
                .fg(theme::color(theme::ACCENT))
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(theme::color(theme::SEPARATOR))
        };

        let (title, content) = match state.preview.mode {
            PreviewMode::FileContent => (" Preview ".to_string(), state.preview.text.clone()),
            PreviewMode::LintResults => (" Lint Results ".to_string(), state.preview.text.clone()),
            PreviewMode::ActionOutput => {
                (" Action Output ".to_string(), state.preview.text.clone())
            }
            PreviewMode::HelpOverlay => (" Help ".to_string(), help_text()),
        };

        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(border_style);

        let paragraph = Paragraph::new(content)
            .block(block)
            .wrap(Wrap { trim: false })
            .scroll((state.preview.scroll as u16, 0))
            .style(Style::default().fg(theme::color(theme::LABEL)));

        frame.render_widget(paragraph, area);

        let inner_area = area.inner(ratatui::layout::Margin {
            vertical: 1,
            horizontal: 0,
        });
        if inner_area.width > 0 && inner_area.height > 0 {
            let content_length = state.preview.text.lines().count().max(1);
            let max_scroll = content_length.saturating_sub(1);
            let scroll_position = if state.preview.scroll > max_scroll {
                max_scroll
            } else {
                state.preview.scroll
            };
            let mut scrollbar_state = ScrollbarState::new(content_length).position(scroll_position);
            let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓"))
                .thumb_style(Style::default().fg(if is_focused {
                    theme::color(theme::ACCENT)
                } else {
                    theme::color(theme::SCROLLBAR)
                }))
                .track_style(Style::default().fg(theme::color(theme::SCROLLBAR)));

            frame.render_stateful_widget(scrollbar, inner_area, &mut scrollbar_state);
        }
    }
}

impl Default for PreviewView {
    fn default() -> Self {
        Self::new()
    }
}

fn help_text() -> String {
    crate::surface_shortcut_bindings::help_text()
}
