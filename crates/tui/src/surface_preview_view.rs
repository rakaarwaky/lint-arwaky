// PURPOSE: PreviewView — TUI surface component for the preview panel (right panel)
//
// Renders content modes depending on PreviewMode:
//   - LintResults: output from check/scan/fix/ci/orphan actions
//   - ActionOutput: output from doctor/init/install/version/adapters actions
//   - HelpOverlay: keyboard shortcut reference
//
// The right panel shows command output only — never file content or code.
// Viewing a file selects it in the Files panel instead of opening it here.
//
// Help content is embedded as a static string in help_text().
use crate::{AppState, PanelFocus, PreviewMode};
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::widgets::{
    Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, Wrap,
};
use shared_tui::utility_tui_theme as theme;

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
                .add_modifier(theme::EMPHASIS_HEADING)
        } else {
            Style::default().fg(theme::color(theme::SEPARATOR))
        };

        let title = match state.preview.mode {
            PreviewMode::LintResults => " Lint Results ",
            PreviewMode::ActionOutput => " Action Output ",
            PreviewMode::HelpOverlay => " Help ",
        };
        let content = preview_content(state.preview.mode, &state.preview.text);

        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(border_style);

        let inner_area = area.inner(ratatui::layout::Margin {
            vertical: 1,
            horizontal: 0,
        });
        let content_length = visual_row_count(&content, inner_area.width).max(1);
        let max_scroll = content_length.saturating_sub(inner_area.height as usize);
        let scroll_position = state.preview.scroll.min(max_scroll);
        let paragraph = Paragraph::new(content.clone())
            .block(block)
            .wrap(Wrap { trim: false })
            .scroll((scroll_position.min(u16::MAX as usize) as u16, 0))
            .style(Style::default().fg(theme::color(theme::LABEL)));

        frame.render_widget(paragraph, area);

        if inner_area.width > 0 && inner_area.height > 0 {
            let mut scrollbar_state = ScrollbarState::new(content_length)
                .viewport_content_length(inner_area.height as usize)
                .position(scroll_position);
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

/// Content selected by the preview mode. Renderers and input handling both
/// call this so scrolling never follows stale backing text in the Help mode.
pub fn preview_content(mode: PreviewMode, text: &str) -> String {
    match mode {
        PreviewMode::LintResults | PreviewMode::ActionOutput => text.to_string(),
        PreviewMode::HelpOverlay => help_text(),
    }
}

/// Ratatui's wrapped visual-row count for a preview at the given inner width.
pub fn visual_row_count(content: &str, width: u16) -> usize {
    if width == 0 {
        return content.lines().count().max(1);
    }
    Paragraph::new(content)
        .wrap(Wrap { trim: false })
        .line_count(width)
        .max(1)
}

/// Maximum scroll offset for the preview at the current terminal geometry.
pub fn max_scroll_for_state(state: &AppState) -> usize {
    let width = state.terminal_width.max(1);
    let height = state.terminal_height;
    if height < 5 {
        return 0;
    }
    let layout = crate::surface_tui_layout::compute_panel_layout(width, height);
    let panel = if layout.three_column {
        layout.preview
    } else {
        layout.band
    };
    let inner_width = panel.width.saturating_sub(2);
    let inner_height = panel.height.saturating_sub(2) as usize;
    let content = preview_content(state.preview.mode, &state.preview.text);
    visual_row_count(&content, inner_width).saturating_sub(inner_height)
}

fn help_text() -> String {
    crate::surface_bindings_store::help_text()
}
