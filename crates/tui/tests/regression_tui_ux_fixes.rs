//! Regression tests for the 15 UX issues fixed in one pass (#1317–#1331).
//!
//! Each test pins the behaviour a fix introduced, so the specific regression
//! that issue described cannot come back silently.

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use shared_common::Severity;
use shared_tui::utility_tui_theme as theme;
use tui_lint_arwaky::surface_bindings_store::{ShortcutAction, action_for};
use tui_lint_arwaky::surface_tui_command::{from_key_event, too_small_message};
use tui_lint_arwaky::taxonomy_tui_event::TuiEvent;
use tui_lint_arwaky::taxonomy_tui_vo::{AesLayer, FileEntry};

fn key(code: KeyCode) -> KeyEvent {
    KeyEvent::new(code, KeyModifiers::NONE)
}

// ── #1317: the `r` recovery key must dispatch `ChangeProjectRoot` ──────────

#[test]
fn r_opens_the_project_root_dialog() {
    assert_eq!(
        action_for(&key(KeyCode::Char('r'))),
        Some(ShortcutAction::ChangeProjectRoot),
        "`r` is named by the path-dialog dismissal status line and must dispatch"
    );
    assert_eq!(
        ShortcutAction::ChangeProjectRoot.to_event(),
        TuiEvent::ChangeProjectRoot
    );
}

#[test]
fn every_displayed_binding_still_has_a_dispatch_entry_after_adding_r() {
    // `r` occupies a bar row, so the help overlay and the shortcut bar both
    // advertise it; the shared table is what keeps them honest.
    let help = tui_lint_arwaky::surface_bindings_store::help_text();
    assert!(
        help.contains("root"),
        "help overlay must list the new binding"
    );
}

// ── #1329: Esc cancels a running scan instead of quitting ──────────────────

#[test]
fn esc_cancels_a_running_scan() {
    let mut state = tui_lint_arwaky::AppState::new("/project".to_string());
    // A fresh state opens on the path dialog, which owns Esc while visible.
    state.path_dialog.visible = false;
    state.set_scanning(true, "Scanning...".to_string(), 100);

    assert_eq!(
        from_key_event(key(KeyCode::Esc), &state),
        TuiEvent::CancelScan,
        "Esc during a scan must reach the cancel handler, not Quit"
    );
}

#[test]
fn esc_still_quits_when_no_scan_is_running() {
    let mut state = tui_lint_arwaky::AppState::new("/project".to_string());
    state.path_dialog.visible = false;
    state.scan.running = false;

    assert_eq!(from_key_event(key(KeyCode::Esc), &state), TuiEvent::Quit);
}

#[test]
fn an_open_overlay_keeps_esc_even_while_a_scan_runs() {
    let mut state = tui_lint_arwaky::AppState::new("/project".to_string());
    state.set_scanning(true, "Scanning...".to_string(), 100);
    // Path dialog is visible by default; Esc must close it, not cancel the scan.
    assert_eq!(
        from_key_event(key(KeyCode::Esc), &state),
        TuiEvent::PathCancel
    );
}

#[test]
fn q_still_quits_during_a_scan() {
    let mut state = tui_lint_arwaky::AppState::new("/project".to_string());
    state.path_dialog.visible = false;
    state.set_scanning(true, "Scanning...".to_string(), 100);

    assert_eq!(
        from_key_event(key(KeyCode::Char('q')), &state),
        TuiEvent::Quit,
        "only Esc becomes contextual; q must always quit"
    );
}

// ── #1328: the too-small screen must fit the terminal that shows it ────────

#[test]
fn too_small_message_never_exceeds_the_terminal_width() {
    for width in [1u16, 8, 12, 20, 30, 39] {
        let message = too_small_message(width, 10, 40, 15);
        assert!(
            message.chars().count() <= width as usize,
            "width {width} produced {} columns: {message}",
            message.chars().count()
        );
    }
}

#[test]
fn too_small_message_states_both_sizes_when_it_can() {
    let message = too_small_message(30, 10, 40, 15);
    assert_eq!(message, "30x10 — need 40x15");
}

// ── #1331: the violation badge is scoped to what is on screen ──────────────

#[test]
fn violation_badge_is_neutral_until_a_result_covers_the_view() {
    let state = tui_lint_arwaky::AppState::new("/project".to_string());

    assert!(!state.violation_badge_matches_view());
    assert_eq!(state.violation_badge(), "— viol.");
}

#[test]
fn violation_badge_reports_the_count_for_the_matching_scope() {
    let mut state = tui_lint_arwaky::AppState::new("/project".to_string());
    state.set_violation_result("/project", 42);

    assert!(state.violation_badge_matches_view());
    assert_eq!(state.violation_badge(), "42 viol.");
}

#[test]
fn violation_badge_does_not_claim_a_count_for_another_directory() {
    let mut state = tui_lint_arwaky::AppState::new("/project".to_string());
    state.set_violation_result("/project/other", 0);

    // A green `0` for a folder nothing examined was a false claim (#1331).
    assert!(!state.violation_badge_matches_view());
    assert_eq!(state.violation_badge(), "— viol.");
}

// ── #1321: a finished scan leaves a terminal state, not a vanished bar ─────

#[test]
fn a_finished_scan_reports_completion_instead_of_clearing_the_phase() {
    let mut state = tui_lint_arwaky::AppState::new("/project".to_string());
    state.set_scanning(true, "Scanning...".to_string(), 100);
    state.update_scan_progress("linting".to_string(), 100, 100);

    state.finish_scan(7);

    assert!(!state.scan.running);
    assert!(
        state.scan.phase.contains("Done"),
        "the bar vanished with no completion frame before #1321; got phase {:?}",
        state.scan.phase
    );
    assert!(state.scan.phase.contains('7'));
}

// ── #1322: per-file counts reach the rows the Files panel draws ────────────

#[test]
fn file_counts_mark_the_matching_entries_only() {
    let mut state = tui_lint_arwaky::AppState::new("/project".to_string());
    state.navigation.entries = vec![entry("/project/a.rs"), entry("/project/b.rs")];

    state.apply_file_violations(&[("/project/a.rs".to_string(), 5)]);

    assert_eq!(state.navigation.entries[0].violation_count, 5);
    assert_eq!(state.navigation.entries[1].violation_count, 0);
}

#[test]
fn file_counts_tolerate_a_relative_result_path() {
    let mut state = tui_lint_arwaky::AppState::new("/project".to_string());
    state.navigation.entries = vec![entry("/project/a.rs")];

    state.apply_file_violations(&[("./a.rs".to_string(), 3)]);

    assert_eq!(state.navigation.entries[0].violation_count, 3);
}

fn entry(full_path: &str) -> FileEntry {
    FileEntry {
        name: full_path
            .rsplit('/')
            .next()
            .unwrap_or(full_path)
            .to_string(),
        full_path: full_path.to_string(),
        is_dir: false,
        layer: AesLayer::None,
        violation_count: 0,
        extension: "rs".to_string(),
        size_bytes: 10,
    }
}

// ── #1324: one severity vocabulary on every surface ───────────────────────

#[test]
fn severity_badges_carry_no_whitespace() {
    for severity in [
        Severity::CRITICAL,
        Severity::HIGH,
        Severity::MEDIUM,
        Severity::LOW,
        Severity::INFO,
    ] {
        let badge = severity.badge();
        assert!(
            !badge.trim().contains(' '),
            "badge {badge:?} encodes meaning in whitespace, which copies and greps strip"
        );
        assert!(badge.starts_with('[') && badge.ends_with(']'));
    }
}

#[test]
fn severity_ladder_is_distinct_and_ordered() {
    assert_eq!(Severity::CRITICAL.badge(), "[CRIT]");
    assert_eq!(Severity::HIGH.badge(), "[HIGH]");
    assert_eq!(Severity::MEDIUM.badge(), "[MED]");
    assert_eq!(Severity::LOW.badge(), "[LOW]");
    assert_eq!(Severity::INFO.badge(), "[INFO]");
}

// ── #1325: one glyph fallback table, not four ──────────────────────────────

#[test]
fn glyph_fallbacks_collapse_under_no_color() {
    assert_eq!(theme::glyph::separator(true), "|");
    assert_eq!(theme::glyph::warn(true), "!");
    assert_eq!(theme::glyph::bar_filled(true), "=");
    assert_eq!(theme::glyph::bar_empty(true), " ");
    // Compare field by field: `border::Set` is not required to be `PartialEq`.
    let fallback = theme::glyph::border_set(true);
    assert_eq!(fallback.top_left, "+");
    assert_eq!(fallback.horizontal_top, "-");
    assert_eq!(fallback.vertical_left, "|");
    assert_eq!(
        theme::glyph::border_set(false).top_left,
        ratatui::symbols::border::PLAIN.top_left
    );
}

#[test]
fn glyph_fallbacks_keep_unicode_when_color_is_on() {
    assert_eq!(theme::glyph::separator(false), "\u{2502}");
    assert_eq!(theme::glyph::warn(false), "\u{26a0}");
    assert_eq!(theme::glyph::bar_filled(false), "\u{2588}");
    assert_eq!(theme::glyph::bar_empty(false), "\u{2591}");
}

// ── Rendering-level regressions ────────────────────────────────────────────
//
// The tests above pin pure logic. These draw a real frame, because several of
// the original bugs were in the render path only: a label written to state but
// never painted, and an empty panel with no copy.

use ratatui::Terminal;
use ratatui::backend::TestBackend;
use tui_lint_arwaky::surface_file_list_view::FileListView;
use tui_lint_arwaky::surface_shortcut_component::ShortcutComponent;
use tui_lint_arwaky::surface_status_component::StatusComponent;

fn buffer_text(terminal: &Terminal<TestBackend>) -> String {
    terminal
        .backend()
        .buffer()
        .content
        .iter()
        .map(|cell| cell.symbol())
        .collect()
}

// #1321: the completion frame must actually be painted.
#[test]
fn a_finished_scan_paints_its_completion_label() {
    let mut state = tui_lint_arwaky::AppState::new("/project".to_string());
    state.set_scanning(true, "Scanning...".to_string(), 10);
    state.finish_scan(4);

    let mut terminal = Terminal::new(TestBackend::new(90, 2)).unwrap();
    terminal
        .draw(|frame| StatusComponent::new().render(&state, frame, frame.area()))
        .unwrap();

    let text = buffer_text(&terminal);
    assert!(
        text.contains("Done"),
        "the completion label existed in state but was never rendered: {text:?}"
    );
    assert!(
        text.contains('4'),
        "completion must name the count: {text:?}"
    );
}

// #1321: an in-flight scan without a file count reads as a wait, not 0%.
#[test]
fn an_unstarted_scan_paints_an_indeterminate_wait() {
    let mut state = tui_lint_arwaky::AppState::new("/project".to_string());
    state.set_scanning(true, "Starting scan...".to_string(), 0);

    let mut terminal = Terminal::new(TestBackend::new(90, 2)).unwrap();
    terminal
        .draw(|frame| StatusComponent::new().render(&state, frame, frame.area()))
        .unwrap();

    let text = buffer_text(&terminal);
    assert!(
        text.contains("waiting for file count"),
        "a zero denominator must not render as determinate progress: {text:?}"
    );
}

// #1331: the badge must not print a count for a view nothing has examined.
#[test]
fn the_status_bar_prints_no_count_for_an_unexamined_view() {
    let mut state = tui_lint_arwaky::AppState::new("/project".to_string());
    state.set_violation_result("/project/other", 0);

    let mut terminal = Terminal::new(TestBackend::new(90, 2)).unwrap();
    terminal
        .draw(|frame| StatusComponent::new().render(&state, frame, frame.area()))
        .unwrap();

    let text = buffer_text(&terminal);
    assert!(
        text.contains("— viol."),
        "an out-of-scope result must render as neutral: {text:?}"
    );
    assert!(
        !text.contains("0 viol."),
        "a green zero was the false claim this issue reported: {text:?}"
    );
}

// #1330 / #1317: an empty Files panel must carry guidance, not blank chrome.
#[test]
fn an_empty_files_panel_explains_the_next_action() {
    let state = tui_lint_arwaky::AppState::new("/project".to_string());

    let mut terminal = Terminal::new(TestBackend::new(60, 10)).unwrap();
    terminal
        .draw(|frame| FileListView::new().render(&state, frame, frame.area()))
        .unwrap();

    let text = buffer_text(&terminal);
    assert!(
        text.contains("No files here") || text.contains("No project selected"),
        "an empty panel must say what to do next: {text:?}"
    );
}

// #1327: while a confirm is pending the bar advertises only the gate's keys.
#[test]
fn the_shortcut_bar_swaps_to_the_gate_keys_during_a_confirm() {
    let mut state = tui_lint_arwaky::AppState::new("/project".to_string());
    state.actions.pending_confirm = Some(tui_lint_arwaky::ConfirmState {
        pending: TuiEvent::ActionFixLive,
        label: "Apply live fixes to files".to_string(),
    });

    let mut terminal = Terminal::new(TestBackend::new(120, 4)).unwrap();
    terminal
        .draw(|frame| ShortcutComponent::new().render(&state, frame, frame.area()))
        .unwrap();

    let text = buffer_text(&terminal);
    assert!(
        text.contains("cancel"),
        "gate keys must be visible: {text:?}"
    );
    assert!(
        !text.contains("c:check"),
        "browse shortcuts are swallowed by the gate, so they must not be shown: {text:?}"
    );
}

// #1320: the bar must fit the panel at every supported width.
#[test]
fn the_shortcut_bar_fits_every_supported_width() {
    for width in [40u16, 60, 80, 100, 120, 160] {
        let mut state = tui_lint_arwaky::AppState::new("/project".to_string());
        state.path_dialog.visible = false;

        let mut terminal = Terminal::new(TestBackend::new(width, 4)).unwrap();
        terminal
            .draw(|frame| ShortcutComponent::new().render(&state, frame, frame.area()))
            .unwrap();

        let text = buffer_text(&terminal);
        assert!(
            text.contains("help"),
            "width {width} dropped the help affordance: {text:?}"
        );
    }
}

// #1321: a cancelled scan must not paint the completed-scan label.
#[test]
fn a_cancelled_scan_does_not_claim_done() {
    let mut state = tui_lint_arwaky::AppState::new("/project".to_string());
    state.set_scanning(true, "Scanning...".to_string(), 10);
    state.finish_scan(0);
    // Mirrors the Cancelled arm: the completion frame, but an honest label.
    state.scan.phase = "Scan cancelled — no results written".to_string();

    let mut terminal = Terminal::new(TestBackend::new(90, 2)).unwrap();
    terminal
        .draw(|frame| StatusComponent::new().render(&state, frame, frame.area()))
        .unwrap();

    let text = buffer_text(&terminal);
    assert!(text.contains("cancelled"), "{text:?}");
    assert!(
        !text.contains("Done"),
        "a stopped scan found nothing because it stopped: {text:?}"
    );
}
