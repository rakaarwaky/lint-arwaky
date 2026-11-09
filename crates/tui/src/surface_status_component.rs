// PURPOSE: StatusComponent — TUI surface component for the status bar (bottom line)
//
// Displays current status message, selected file name, and violation count.
// Violation count is colored red when > 0, green when 0.
// Uses ASCII box-drawing fallback when NO_COLOR is set (#365).
use crate::AppState;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use shared_tui::utility_tui_theme as theme;

// Glyph fallbacks are owned by `utility_tui_theme::glyph`, not re-implemented
// here: the confirm modal and the preview panel had grown their own copies, so
// the NO_COLOR contract had four independent implementations (#1325).
fn vsep() -> &'static str {
    theme::glyph::separator(theme::no_color())
}

fn warn() -> &'static str {
    theme::glyph::warn(theme::no_color())
}

fn bar_filled() -> &'static str {
    theme::glyph::bar_filled(theme::no_color())
}

fn bar_empty() -> &'static str {
    theme::glyph::bar_empty(theme::no_color())
}

pub struct StatusComponent;

impl StatusComponent {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, state: &AppState, frame: &mut Frame, area: Rect) {
        let line = if state.scan.running {
            // Scan progress indicator: progress bar + phase + counts
            let bar_width = 20;
            let (filled, _) = if state.scan.files_total > 0 {
                let ratio = state.scan.files_done as f64 / state.scan.files_total as f64;
                let filled = (ratio * bar_width as f64) as usize;
                (filled.min(bar_width), bar_width)
            } else {
                (0, bar_width)
            };
            let empty = bar_width - filled;
            let bar: String = bar_filled().repeat(filled) + &bar_empty().repeat(empty);

            let phase_display = if state.scan.phase.is_empty() {
                "Scanning...".to_string()
            } else {
                state.scan.phase.clone()
            };

            // A zero denominator means the dispatcher has not published its
            // file count yet. That is an indeterminate wait, not 0% progress:
            // an empty determinate bar is indistinguishable from a stuck one.
            let progress_detail = if state.scan.files_total > 0 {
                format!(
                    " {} {}/{} files",
                    bar, state.scan.files_done, state.scan.files_total,
                )
            } else {
                format!(" {} waiting for file count…", bar)
            };

            let violation_style = if state.scan.violations > 0 {
                Style::default().fg(theme::color(theme::VIOLATIONS))
            } else {
                Style::default().fg(theme::color(theme::PENDING))
            };

            Line::from(vec![
                Span::styled(
                    format!(" {} ", phase_display),
                    Style::default()
                        .fg(theme::color(theme::ACCENT))
                        .add_modifier(theme::EMPHASIS_HEADING),
                ),
                Span::styled(
                    progress_detail,
                    Style::default().fg(theme::color(theme::LABEL)),
                ),
                Span::styled(
                    format!(" {} ", vsep()),
                    Style::default().fg(theme::color(theme::SEPARATOR)),
                ),
                Span::styled(
                    format!("{} {} violations", warn(), state.scan.violations),
                    violation_style,
                ),
                Span::styled(
                    "  Esc cancel",
                    Style::default().fg(theme::color(theme::SEPARATOR)),
                ),
            ])
        } else if state.actions.pending {
            // Busy indicator for a pending background action (#565):
            // mirrors the scan busy path with a distinct "Running …" line.
            let selected_name = match state.selected_entry() {
                Some(entry) => entry.display_name(),
                None => "(none)".to_string(),
            };

            // Neutral when no result covers the current view: a green `0` for
            // a folder nothing has examined is a false claim (#1331).
            let violation_style = if !state.violation_badge_matches_view() {
                Style::default().fg(theme::color(theme::SEPARATOR))
            } else if state.violation_count > 0 {
                Style::default().fg(theme::color(theme::VIOLATIONS))
            } else {
                Style::default().fg(theme::color(theme::CLEAN))
            };

            Line::from(vec![
                Span::styled(
                    " Running ",
                    Style::default()
                        .fg(theme::color(theme::ACCENT))
                        .add_modifier(theme::EMPHASIS_SELECTED),
                ),
                Span::styled(
                    &state.actions.pending_label,
                    Style::default().fg(theme::color(theme::PENDING)),
                ),
                Span::styled(
                    format!(" {} ", vsep()),
                    Style::default().fg(theme::color(theme::SEPARATOR)),
                ),
                Span::styled(
                    "Selected: ",
                    Style::default().fg(theme::color(theme::SEPARATOR)),
                ),
                Span::styled(
                    selected_name,
                    Style::default().fg(theme::color(theme::ACCENT)),
                ),
                Span::styled(
                    format!(" {} ", vsep()),
                    Style::default().fg(theme::color(theme::SEPARATOR)),
                ),
                Span::styled(
                    format!("{} {}", warn(), state.violation_badge()),
                    violation_style,
                ),
            ])
        } else {
            let selected_name = match state.selected_entry() {
                Some(entry) => entry.display_name(),
                None => "(none)".to_string(),
            };

            // Neutral when no result covers the current view: a green `0` for
            // a folder nothing has examined is a false claim (#1331).
            let violation_style = if !state.violation_badge_matches_view() {
                Style::default().fg(theme::color(theme::SEPARATOR))
            } else if state.violation_count > 0 {
                Style::default().fg(theme::color(theme::VIOLATIONS))
            } else {
                Style::default().fg(theme::color(theme::CLEAN))
            };

            Line::from(vec![
                Span::styled(
                    " Status: ",
                    Style::default().fg(theme::color(theme::SEPARATOR)),
                ),
                Span::styled(
                    &state.status_message,
                    Style::default().fg(theme::color(theme::LABEL)),
                ),
                Span::styled(
                    format!(" {} ", vsep()),
                    Style::default().fg(theme::color(theme::SEPARATOR)),
                ),
                Span::styled(
                    "Selected: ",
                    Style::default().fg(theme::color(theme::SEPARATOR)),
                ),
                Span::styled(
                    selected_name,
                    Style::default().fg(theme::color(theme::ACCENT)),
                ),
                Span::styled(
                    format!(" {} ", vsep()),
                    Style::default().fg(theme::color(theme::SEPARATOR)),
                ),
                Span::styled(
                    format!("{} {}", warn(), state.violation_badge()),
                    violation_style,
                ),
            ])
        };

        let paragraph =
            Paragraph::new(line).style(Style::default().bg(theme::color(theme::BACKGROUND)));
        frame.render_widget(paragraph, area);
    }
}

impl Default for StatusComponent {
    fn default() -> Self {
        Self::new()
    }
}
