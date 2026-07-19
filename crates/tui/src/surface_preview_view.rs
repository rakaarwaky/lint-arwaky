// PURPOSE: PreviewView — TUI surface component for the preview panel (right panel)
//
// Renders four content modes depending on PreviewMode:
//   - FileContent: inline file preview (up to 100 lines)
//   - LintResults: output from check/scan/fix/ci/orphan actions
//   - ActionOutput: output from doctor/init/install/version/adapters actions
//   - HelpOverlay: keyboard shortcut reference
//
// Help content is embedded as a static string in help_text().
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{
    Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, Wrap,
};
use ratatui::Frame;
use shared::tui::taxonomy_state_vo::{AppState, PanelFocus, PreviewMode};

// ─── Block 1: Struct Definition ───────────────────────────
pub struct PreviewView;

// ─── Block 3: Constructors & Helpers ──────────────────────
impl PreviewView {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, state: &AppState, frame: &mut Frame, area: Rect) {
        let is_focused = state.panel_focus == PanelFocus::Preview;
        let border_style = if is_focused {
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let (title, content) = match state.preview_mode {
            PreviewMode::FileContent => {
                let title = match state.selected_entry() {
                    Some(entry) if !entry.is_dir => format!(" Preview: {} ", entry.name),
                    _ => " Preview ".to_string(),
                };
                (title, state.preview_text.clone())
            }
            PreviewMode::LintResults => (" Lint Results ".to_string(), state.preview_text.clone()),
            PreviewMode::ActionOutput => {
                (" Action Output ".to_string(), state.preview_text.clone())
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
            .scroll((state.preview_scroll as u16, 0))
            .style(Style::default().fg(Color::White));

        frame.render_widget(paragraph, area);

        let inner_area = area.inner(ratatui::layout::Margin {
            vertical: 1,
            horizontal: 0,
        });
        if inner_area.width > 0 && inner_area.height > 0 {
            let content_length = state.preview_text.lines().count().max(1);
            let max_scroll = content_length.saturating_sub(1);
            let scroll_position = if state.preview_scroll > max_scroll {
                max_scroll
            } else {
                state.preview_scroll
            };
            let mut scrollbar_state = ScrollbarState::new(content_length).position(scroll_position);
            let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓"))
                .thumb_style(Style::default().fg(if is_focused {
                    Color::Cyan
                } else {
                    Color::DarkGray
                }))
                .track_style(Style::default().fg(Color::DarkGray));

            frame.render_stateful_widget(scrollbar, inner_area, &mut scrollbar_state);
        }
    }
}

// ─── Block 2: Public Contract ─────────────────────────────
impl Default for PreviewView {
    fn default() -> Self {
        Self::new()
    }
}

fn help_text() -> String {
    "\
Navigation:
  j/\u{2193}     Move down
  k/\u{2191}     Move up
  h/\u{2190}     Back (parent dir)
  l/\u{2192}/\u{23ce}   Open folder / preview file
  Home    Jump to top
  End     Jump to bottom
  Tab     Cycle panel focus
  /       Search files

Actions (on selected file/folder):
  c       check — AES compliance
  s       scan — multi-adapter scan
  f       fix — auto-fix
  t       ci — CI mode (threshold)
  w       watch — file watch
  o       orphan — dead code check
  Ctrl+S  security — vulnerability scan
  Ctrl+D  duplicates — duplication
  Ctrl+P  dependencies — deps scan

Setup:
  d       doctor — environment diag
  i       init — create config
  I       install — adapter deps
  m       mcp-config — MCP config
  C       config-show — show config
  H       install-hook — git hook
  U       uninstall-hook — remove hook
  a       adapters — list adapters
  v       version — show version

General:
  ?       Toggle this help
  y       Copy preview to clipboard
  Ctrl+Y  Copy preview to clipboard + save to lint-results.txt
  q       Quit
"
    .to_string()
}
