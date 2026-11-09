// PURPOSE: PathScreen — TUI surface component for the path input dialog (overlay)
//
// Renders a centered popup overlay when show_path_dialog is true.
// User can type a path, confirm, or use current directory.
// Uses centered_rect() helper to calculate popup dimensions.
use crate::AppState;
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};
use shared_tui::utility_tui_theme as theme;
use unicode_width::UnicodeWidthChar;

pub struct PathScreen;

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
                    .fg(theme::color(theme::ACCENT))
                    .add_modifier(theme::EMPHASIS_HEADING),
            )
            .style(Style::default().bg(theme::color(theme::BACKGROUND)));

        let cwd = std::env::current_dir()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|_| ".".to_string());

        let input_source = if state.path_dialog.input.is_empty() {
            cwd.as_str()
        } else {
            state.path_dialog.input.as_str()
        };
        let input_width = popup_area.width.saturating_sub(7) as usize;
        let (before_cursor, after_cursor) = visible_input_window(
            input_source,
            if state.path_dialog.input.is_empty() {
                input_source.len()
            } else {
                state.path_dialog.cursor.min(input_source.len())
            },
            input_width,
        );

        let text = vec![
            Line::from(""),
            Line::from(Span::styled(
                "  Type path or use the cursor keys to edit:",
                Style::default().fg(theme::color(theme::LABEL)),
            )),
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    "  > ",
                    Style::default().fg(theme::color(theme::FOCUS_CONFIRM)),
                ),
                Span::styled(
                    before_cursor,
                    Style::default()
                        .fg(theme::color(theme::PATH_INPUT))
                        .add_modifier(theme::EMPHASIS_SELECTED),
                ),
                Span::styled("_", Style::default().fg(theme::color(theme::LABEL))),
                Span::styled(
                    after_cursor,
                    Style::default()
                        .fg(theme::color(theme::PATH_INPUT))
                        .add_modifier(theme::EMPHASIS_SELECTED),
                ),
            ]),
            Line::from(""),
            Line::from(Span::styled(
                "  [Enter] Confirm   [Tab] Use current dir   [Esc] Cancel",
                Style::default().fg(theme::color(theme::SEPARATOR)),
            )),
        ];

        let paragraph = Paragraph::new(text).block(block).alignment(Alignment::Left);

        frame.render_widget(paragraph, popup_area);
    }
}

impl Default for PathScreen {
    fn default() -> Self {
        Self::new()
    }
}

/// Return the visible text before and after the insertion point, reserving one
/// terminal cell for each ellipsis when text falls outside the horizontal window.
fn visible_input_window(input: &str, cursor: usize, budget: usize) -> (String, String) {
    let cursor = if cursor >= input.len() {
        input.len()
    } else {
        input
            .char_indices()
            .map(|(index, _)| index)
            .take_while(|index| *index <= cursor)
            .last()
            .unwrap_or(0)
    };
    let left = &input[..cursor];
    let right = &input[cursor..];
    let width = |text: &str| {
        text.chars()
            .map(|ch| UnicodeWidthChar::width(ch).unwrap_or(0))
            .sum::<usize>()
    };
    if width(input) <= budget {
        return (left.to_string(), right.to_string());
    }
    if budget == 0 {
        return (String::new(), String::new());
    }

    let left_width = width(left);
    let right_width = width(right);
    let mut left_take = left_width.min(budget / 2);
    let mut right_take = right_width.min(budget.saturating_sub(left_take));
    let spare = budget.saturating_sub(left_take + right_take);
    left_take += spare.min(left_width.saturating_sub(left_take));
    let spare = budget.saturating_sub(left_take + right_take);
    right_take += spare.min(right_width.saturating_sub(right_take));

    loop {
        let marker_width = (left_take < left_width) as usize + (right_take < right_width) as usize;
        let text_budget = budget.saturating_sub(marker_width);
        if left_take + right_take <= text_budget {
            break;
        }
        if left_take > right_take && left_take > 0 {
            left_take -= 1;
        } else if right_take > 0 {
            right_take -= 1;
        } else {
            break;
        }
    }
    let left_hidden = left_take < left_width;
    let right_hidden = right_take < right_width;

    let mut visible_left = String::new();
    let mut used = 0;
    for ch in left.chars().rev() {
        let ch_width = UnicodeWidthChar::width(ch).unwrap_or(0);
        if used + ch_width > left_take {
            break;
        }
        visible_left.insert(0, ch);
        used += ch_width;
    }
    let mut visible_right = String::new();
    used = 0;
    for ch in right.chars() {
        let ch_width = UnicodeWidthChar::width(ch).unwrap_or(0);
        if used + ch_width > right_take {
            break;
        }
        visible_right.push(ch);
        used += ch_width;
    }
    if left_hidden {
        visible_left.insert(0, '…');
    }
    if right_hidden {
        visible_right.push('…');
    }
    (visible_left, visible_right)
}

/// Center a popup of the given percentage size inside `area`.
/// Shared with the confirm modal (#554) so every overlay lands in the same place.
pub fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
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
