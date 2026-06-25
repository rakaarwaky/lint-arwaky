use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem};
use ratatui::Frame;
use shared::tui::taxonomy_state_vo::{AppState, PanelFocus};
use std::path::Path;

pub struct TreeView;

impl TreeView {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, state: &AppState, frame: &mut Frame, area: Rect) {
        let is_focused = state.panel_focus == PanelFocus::Tree;
        let border_style = if is_focused {
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let block = Block::default()
            .title(" Tree ")
            .borders(Borders::ALL)
            .border_style(border_style);

        let mut items = Vec::new();
        let components = build_path_components(&state.current_dir, &state.project_root);

        let root_line = Line::from(vec![
            Span::styled("[*] ", Style::default().fg(Color::Yellow)),
            Span::styled(
                shorten_path(&state.project_root),
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]);
        items.push(ListItem::new(root_line));

        for (i, component) in components.iter().enumerate() {
            let indent = "  ".repeat(i + 1);
            let is_current = i == components.len().saturating_sub(1);
            let style = if is_current {
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Blue)
            };
            let dir_line = Line::from(vec![
                Span::raw(indent.clone()),
                Span::styled("[-] ", Style::default().fg(Color::Yellow)),
                Span::styled(format!("{}/", component), style),
            ]);
            items.push(ListItem::new(dir_line));
        }

        let list = List::new(items).block(block);
        frame.render_widget(list, area);
    }
}

impl Default for TreeView {
    fn default() -> Self {
        Self::new()
    }
}

fn build_path_components(current_dir: &str, project_root: &str) -> Vec<String> {
    let current = Path::new(current_dir);
    let root = Path::new(project_root);

    if let Ok(relative) = current.strip_prefix(root) {
        relative
            .components()
            .filter_map(|c| c.as_os_str().to_str().map(|s| s.to_string()))
            .collect()
    } else {
        Vec::new()
    }
}

fn shorten_path(path: &str) -> String {
    let p = Path::new(path);
    match p.file_name().and_then(|n| n.to_str()) {
        Some(name) => name.to_string(),
        None => path.to_string(),
    }
}
