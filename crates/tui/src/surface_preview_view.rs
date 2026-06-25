use crate::taxonomy_state_vo::{AppState, PanelFocus, PreviewMode};
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;

pub struct PreviewView;

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
            PreviewMode::ActionOutput => (" Action Output ".to_string(), state.preview_text.clone()),
            PreviewMode::HelpOverlay => (" Help ".to_string(), help_text()),
        };

        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(border_style);

        let paragraph = Paragraph::new(content)
            .block(block)
            .wrap(Wrap { trim: false })
            .style(Style::default().fg(Color::White));

        frame.render_widget(paragraph, area);
    }
}

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
  q       Quit
"
    .to_string()
}
