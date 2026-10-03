// PURPOSE: TreeView — TUI surface component for the directory tree panel (left panel)
//
// Renders the path from project_root to current_dir as a browsable tree.
// The root is shortened to its basename. Each directory component is indented
// by depth, with the current (leaf) component highlighted in cyan.
//
// Uses simple string-based rendering (no ratatui Tree widget) for compatibility.
use crate::{AppState, PanelFocus};
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{
    Block, Borders, List, ListItem, ListState, Scrollbar, ScrollbarOrientation, ScrollbarState,
};
use shared_tui::utility_tui_theme as theme;
use std::path::Path;

pub struct TreeView;

impl TreeView {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, state: &AppState, frame: &mut Frame, area: Rect) {
        let is_focused = state.navigation.panel_focus == PanelFocus::Tree;
        let border_style = if is_focused {
            Style::default()
                .fg(theme::color(theme::ACCENT))
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(theme::color(theme::SEPARATOR))
        };

        let block = Block::default()
            .title(" Tree ")
            .borders(Borders::ALL)
            .border_style(border_style);

        let mut items = Vec::new();
        let components = build_path_components(
            &state.navigation.current_dir,
            &state.navigation.project_root,
        );

        let root_line = Line::from(vec![
            Span::styled("[*] ", Style::default().fg(theme::color(theme::KEY))),
            Span::styled(
                shorten_path(&state.navigation.project_root),
                Style::default()
                    .fg(theme::color(theme::LABEL))
                    .add_modifier(Modifier::BOLD),
            ),
        ]);
        items.push(ListItem::new(root_line));

        for (i, component) in components.iter().enumerate() {
            let indent = "  ".repeat(i + 1);
            let is_current = i == components.len().saturating_sub(1);
            let style = if is_current {
                Style::default()
                    .fg(theme::color(theme::ACCENT))
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme::color(theme::DIRECTORY))
            };
            let dir_line = Line::from(vec![
                Span::raw(indent.clone()),
                Span::styled("[-] ", Style::default().fg(theme::color(theme::KEY))),
                Span::styled(format!("{}/", component), style),
            ]);
            items.push(ListItem::new(dir_line));
        }

        // Keep the leaf reachable even when a deeply nested path is taller than
        // the panel. `tree_scroll` remains the user's requested offset, while
        // the automatic offset guarantees the current directory is visible.
        let visible_height = area.height.saturating_sub(2) as usize;
        let last = items.len().saturating_sub(1);
        let max_offset = items.len().saturating_sub(visible_height.max(1));
        let leaf_offset = last.saturating_sub(visible_height.saturating_sub(1));
        let offset = state
            .navigation
            .tree_scroll
            .max(leaf_offset)
            .min(max_offset);
        let mut list_state = ListState::default();
        list_state.select(Some(last));
        *list_state.offset_mut() = offset;

        let list = List::new(items).block(block);
        frame.render_stateful_widget(list, area, &mut list_state);

        if max_offset > 0 {
            let inner_area = area.inner(ratatui::layout::Margin {
                vertical: 1,
                horizontal: 0,
            });
            if inner_area.width > 0 && inner_area.height > 0 {
                let mut scrollbar_state = ScrollbarState::new(last + 1).position(last);
                let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
                    .thumb_style(Style::default().fg(theme::color(theme::SCROLLBAR)))
                    .track_style(Style::default().fg(theme::color(theme::SCROLLBAR)));
                frame.render_stateful_widget(scrollbar, inner_area, &mut scrollbar_state);
            }
        }
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
