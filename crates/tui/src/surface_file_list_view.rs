// PURPOSE: FileListView — TUI surface component for the file listing panel (middle panel)
//
// Renders the list of files and directories in the current directory, with:
//   - AES layer badge coloring (taxonomy=cyan, contract=blue, capabilities=magenta, etc.)
//   - Directory names in blue bold
//   - Selected item highlighted with dark gray background
//   - Focus indicator on the panel border (cyan when focused, gray when not)
use crate::AesLayer;
use crate::{AppState, PanelFocus};
use ratatui::Frame;
use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph};
use shared_tui::utility_tui_theme as theme;

pub struct FileListView;

impl FileListView {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, state: &AppState, frame: &mut Frame, area: Rect) {
        let is_focused = state.navigation.panel_focus == PanelFocus::FileList;
        let border_style = if is_focused {
            Style::default()
                .fg(theme::color(theme::ACCENT))
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(theme::color(theme::SEPARATOR))
        };

        let block = Block::default()
            .title(" Files ")
            .borders(Borders::ALL)
            .border_style(border_style);

        // In search mode, an empty filtered set is meaningful: it represents a
        // completed search with no matches rather than an unfiltered list.
        let display_indices: Vec<usize> = if state.search.mode {
            state.search.filtered_indices.clone()
        } else {
            (0..state.navigation.entries.len()).collect()
        };

        // In search mode with zero matches, show empty-state guidance instead of the full list (#367).
        let empty_hint =
            if state.search.mode && !state.search.query.is_empty() && display_indices.is_empty() {
                Some(format!(
                    "No matches for '{}' — press Esc to clear filter",
                    state.search.query
                ))
            } else {
                None
            };

        // In search mode, filter_pos is the highlight position in the displayed list.
        // In normal mode, selected_index is the highlight position.
        let display_selected = if state.search.mode {
            (!state.search.filtered_indices.is_empty()).then_some(state.search.filter_pos)
        } else {
            Some(state.navigation.selected_index)
        };

        let items: Vec<ListItem> = display_indices
            .iter()
            .enumerate()
            .map(|(i, &orig_idx)| {
                let entry = &state.navigation.entries[orig_idx];
                let badge_color = layer_color(&entry.layer);
                let badge = entry.layer.badge_label();
                let name = entry.display_name();

                let name_style = if entry.is_dir {
                    Style::default()
                        .fg(theme::color(theme::DIRECTORY))
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(theme::color(theme::LABEL))
                };

                let line = Line::from(vec![
                    Span::styled(format!("{} ", badge), Style::default().fg(badge_color)),
                    Span::styled(name, name_style),
                ]);

                let item_style = if Some(i) == display_selected {
                    Style::default()
                        .bg(theme::color(theme::HIGHLIGHT))
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };

                ListItem::new(line).style(item_style)
            })
            .collect();

        let mut list_state = ListState::default();
        list_state.select(display_selected);

        // Show search query in panel title when search mode is active
        let block = if state.search.mode {
            Block::default()
                .title(format!(" Search: {} ", state.search.query))
                .borders(Borders::ALL)
                .border_style(border_style)
        } else {
            block
        };

        if let Some(hint) = empty_hint {
            let hint_line = Line::from(Span::styled(
                hint,
                Style::default().fg(theme::color(theme::SEPARATOR)),
            ));
            let paragraph = Paragraph::new(hint_line)
                .block(block)
                .alignment(Alignment::Center);
            frame.render_widget(paragraph, area);
        } else {
            let list = List::new(items).block(block).highlight_style(
                Style::default()
                    .bg(theme::color(theme::HIGHLIGHT))
                    .add_modifier(Modifier::BOLD),
            );
            frame.render_stateful_widget(list, area, &mut list_state);
        }
    }
}

impl Default for FileListView {
    fn default() -> Self {
        Self::new()
    }
}

/// Single source of truth for the `AesLayer` → color mapping in the TUI.
///
/// Every layer badge reads this function, and every token comes from
/// `shared_tui::utility_tui_theme`. Do not add a second per-layer color table
/// elsewhere: a duplicate raw-palette table shipped as `AesLayer::color_index`
/// drifted from these tokens for the lifetime of issue #558 because `pub` items
/// in a `pub mod` are never flagged by the `dead_code` lint.
///
/// Extend `utility_tui_theme` if a layer needs a new color.
fn layer_color(layer: &AesLayer) -> Color {
    match layer {
        AesLayer::Taxonomy => theme::color(theme::ACCENT),
        AesLayer::Contract => theme::color(theme::DIRECTORY),
        AesLayer::Utility => theme::color(theme::KEY),
        AesLayer::Capabilities => theme::color(theme::CAPABILITIES_BADGE),
        AesLayer::Agent => theme::color(theme::CLEAN),
        AesLayer::Surfaces => theme::color(theme::VIOLATIONS),
        AesLayer::Root => theme::color(theme::LABEL),
        AesLayer::None => theme::color(theme::SEPARATOR),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Guards the single-mapping invariant behind #558: every `AesLayer`
    /// variant resolves to one `utility_tui_theme` token, and each variant is
    /// listed here. A new variant fails to compile above (non-exhaustive match)
    /// and fails this assertion until its token is pinned.
    #[test]
    fn every_layer_variant_pins_one_theme_token() {
        const EXPECTED: [(AesLayer, Color); 8] = [
            (AesLayer::Taxonomy, theme::ACCENT),
            (AesLayer::Contract, theme::DIRECTORY),
            (AesLayer::Utility, theme::KEY),
            (AesLayer::Capabilities, theme::CAPABILITIES_BADGE),
            (AesLayer::Agent, theme::CLEAN),
            (AesLayer::Surfaces, theme::VIOLATIONS),
            (AesLayer::Root, theme::LABEL),
            (AesLayer::None, theme::SEPARATOR),
        ];

        for (layer, token) in EXPECTED {
            assert_eq!(
                layer_color(&layer),
                theme::color(token),
                "{layer:?} must map to exactly one shared_tui::utility_tui_theme token"
            );
        }
    }
}
