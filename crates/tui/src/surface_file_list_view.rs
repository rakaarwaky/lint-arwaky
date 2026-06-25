use crate::taxonomy_file_entry_vo::AesLayer;
use crate::taxonomy_state_vo::{AppState, PanelFocus};
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState};
use ratatui::Frame;

pub struct FileListView;

impl FileListView {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, state: &AppState, frame: &mut Frame, area: Rect) {
        let is_focused = state.panel_focus == PanelFocus::FileList;
        let border_style = if is_focused {
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let block = Block::default()
            .title(" Files ")
            .borders(Borders::ALL)
            .border_style(border_style);

        let items: Vec<ListItem> = state
            .entries
            .iter()
            .enumerate()
            .map(|(i, entry)| {
                let badge_color = layer_color(&entry.layer);
                let badge = entry.layer.badge_label();
                let name = entry.display_name();

                let name_style = if entry.is_dir {
                    Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };

                let line = Line::from(vec![
                    Span::styled(format!("{} ", badge), Style::default().fg(badge_color)),
                    Span::styled(name, name_style),
                ]);

                let item_style = if i == state.selected_index {
                    Style::default()
                        .bg(Color::DarkGray)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };

                ListItem::new(line).style(item_style)
            })
            .collect();

        let mut list_state = ListState::default();
        list_state.select(Some(state.selected_index));

        let list = List::new(items)
            .block(block)
            .highlight_style(
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            );

        frame.render_stateful_widget(list, area, &mut list_state);
    }
}

impl Default for FileListView {
    fn default() -> Self {
        Self::new()
    }
}

fn layer_color(layer: &AesLayer) -> Color {
    match layer {
        AesLayer::Taxonomy => Color::Cyan,
        AesLayer::Contract => Color::Blue,
        AesLayer::Capabilities => Color::Magenta,
        AesLayer::Infrastructure => Color::Yellow,
        AesLayer::Agent => Color::Green,
        AesLayer::Surfaces => Color::Red,
        AesLayer::Root => Color::White,
        AesLayer::None => Color::DarkGray,
    }
}
