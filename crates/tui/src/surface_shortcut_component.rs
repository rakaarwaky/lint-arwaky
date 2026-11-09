// PURPOSE: ShortcutComponent — TUI surface component for the keyboard shortcut bar (bottom area)
//
// Renders up to 3 rows of keyboard shortcuts with key:label pairs.
// Has four modes:
//   - default_rows: shows navigation/action shortcuts during normal browsing
//   - context_sensitive_rows: shows context-appropriate shortcuts when viewing
//     lint results or action output
//   - confirm gate: shows only confirm/cancel while a destructive action waits
//   - search: shows the live search line
//
// format_shortcuts() renders a row as colored spans (yellow keys, white labels)
// bounded by the panel width, so a row that cannot fit ends with an explicit
// `?:help` affordance instead of being clipped mid-word.
use crate::{AppState, PreviewMode};
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use shared_tui::utility_tui_theme as theme;

/// The escape hatch appended to a truncated row. `?` is itself always kept.
const HELP_HINT: (&str, &str) = ("?", "help");

pub struct ShortcutComponent;

impl ShortcutComponent {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, state: &AppState, frame: &mut Frame, area: Rect) {
        // A pending confirm is a gate: advertising browse shortcuts underneath
        // it showed affordances the key gate then swallowed (#1327).
        if state.actions.pending_confirm.is_some() {
            let text = vec![Line::from(vec![
                Span::styled("Esc:", Style::default().fg(theme::color(theme::KEY))),
                Span::styled("cancel  ", Style::default().fg(theme::color(theme::LABEL))),
                Span::styled("Enter:", Style::default().fg(theme::color(theme::KEY))),
                Span::styled("confirm", Style::default().fg(theme::color(theme::LABEL))),
            ])];
            let paragraph =
                Paragraph::new(text).style(Style::default().bg(theme::color(theme::BACKGROUND)));
            frame.render_widget(paragraph, area);
            return;
        }

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

        let results = matches!(
            state.preview.mode,
            PreviewMode::LintResults | PreviewMode::ActionOutput | PreviewMode::FileContent
        );
        let rows = crate::surface_bindings_store::rows_for_context(results);
        let budget = area.width as usize;

        // `budget` is the full band width, which the too-small guard already
        // holds at or above MIN_TERMINAL_WIDTH, so every row gets a real budget
        // rather than a fallback line (#1320).
        let text: Vec<Line> = rows
            .iter()
            .map(|row| Line::from(format_shortcuts(row, budget)))
            .collect();

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

/// One row of `key:label` cells that fits inside `budget` columns.
///
/// Cells are emitted in table order until the budget is spent; a row that
/// cannot hold everything ends with `… ?:help` rather than being clipped by
/// the paragraph. `?` and `/` are the two cells a new user needs most, so the
/// help cell is reserved from the start and the ellipsis marks the truncation
/// instead of leaving it invisible.
fn format_shortcuts<'a>(shortcuts: &'a [(&'a str, &'a str)], budget: usize) -> Vec<Span<'a>> {
    let hint = cell_width(HELP_HINT.0, HELP_HINT.1);
    // "  …  " is five cells (two spaces, the ellipsis, two spaces), not four —
    // budgeting four overflowed the row by exactly one column.
    let hint_block = 5 + hint;

    // Everything fits: draw the row as-is, with no ellipsis.
    let full: usize = shortcuts
        .iter()
        .enumerate()
        .map(|(i, (k, l))| cell_width(k, l) + if i == 0 { 0 } else { 2 })
        .sum();
    if full <= budget {
        return row_spans(shortcuts);
    }

    // Otherwise reserve the help cell up front. Emitting cells first and
    // appending the hint afterwards overflowed the budget and could drop the
    // very affordance the truncation was supposed to advertise (#1320).
    let mut keep = 0usize;
    let mut used = 0usize;
    for (i, (key, label)) in shortcuts.iter().enumerate() {
        let step = cell_width(key, label) + if i == 0 { 0 } else { 2 };
        if used + step + hint_block > budget {
            break;
        }
        used += step;
        keep = i + 1;
    }

    if keep == 0 {
        // Not even one cell plus the hint: show the hint alone rather than a
        // clipped cell.
        return vec![
            Span::styled(
                format!("{}:", HELP_HINT.0),
                Style::default().fg(theme::color(theme::KEY)),
            ),
            Span::styled(
                HELP_HINT.1.to_string(),
                Style::default().fg(theme::color(theme::LABEL)),
            ),
        ];
    }

    let mut spans = row_spans(&shortcuts[..keep]);
    spans.push(Span::styled(
        "  …  ",
        Style::default().fg(theme::color(theme::SEPARATOR)),
    ));
    spans.push(Span::styled(
        format!("{}:", HELP_HINT.0),
        Style::default().fg(theme::color(theme::KEY)),
    ));
    spans.push(Span::styled(
        HELP_HINT.1.to_string(),
        Style::default().fg(theme::color(theme::LABEL)),
    ));
    spans
}

/// `key:label` cells for one row, separated by two spaces.
fn row_spans<'a>(shortcuts: &'a [(&'a str, &'a str)]) -> Vec<Span<'a>> {
    let mut spans = Vec::new();
    for (i, (key, label)) in shortcuts.iter().enumerate() {
        if i > 0 {
            spans.push(Span::raw("  "));
        }
        spans.push(Span::styled(
            format!("{key}:"),
            Style::default().fg(theme::color(theme::KEY)),
        ));
        spans.push(Span::styled(
            label.to_string(),
            Style::default().fg(theme::color(theme::LABEL)),
        ));
    }
    spans
}

/// Columns one `key:label` cell occupies.
fn cell_width(key: &str, label: &str) -> usize {
    key.chars().count() + 1 + label.chars().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The row that overflowed at every supported width (#1320): 15 cells in
    /// row 0 of the normal-mode bar.
    fn widest_row() -> Vec<(&'static str, &'static str)> {
        vec![
            ("j/↓", "down"),
            ("k/↑", "up"),
            ("h/←", "back"),
            ("l/→/⏎", "open"),
            ("Home", "top"),
            ("End", "bottom"),
            ("c", "check"),
            ("s", "scan"),
            ("f", "fix (dry)"),
            ("F", "fix (live)"),
            ("t", "ci"),
            ("w", "watch (CLI only)"),
            ("o", "orphan"),
            ("d", "doctor"),
            ("i", "init"),
        ]
    }

    fn width(spans: &[Span<'_>]) -> usize {
        spans.iter().map(|s| s.content.chars().count()).sum()
    }

    #[test]
    fn a_row_never_exceeds_its_budget() {
        for budget in [20usize, 40, 60, 80, 100, 120, 160] {
            let spans = format_shortcuts(&widest_row(), budget);
            let w = width(&spans);
            assert!(
                w <= budget,
                "budget {budget} produced {w} columns: {spans:?}"
            );
        }
    }

    #[test]
    fn truncated_rows_keep_an_explicit_help_affordance() {
        for budget in [40usize, 80, 100] {
            let spans = format_shortcuts(&widest_row(), budget);
            let text: String = spans.iter().map(|s| s.content.as_ref()).collect();
            assert!(
                text.contains("?"),
                "budget {budget} dropped the help cell: {text}"
            );
        }
    }

    #[test]
    fn a_row_that_fits_shows_no_ellipsis() {
        let row = vec![("c", "check"), ("s", "scan")];
        let spans = format_shortcuts(&row, 60);
        let text: String = spans.iter().map(|s| s.content.as_ref()).collect();
        assert_eq!(text, "c:check  s:scan");
    }
}
