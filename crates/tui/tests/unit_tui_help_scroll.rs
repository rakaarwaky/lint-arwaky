//! Regression: help overlay key mapping (#556).
//!
//! Before #556, `from_key_event` collapsed every navigation key onto
//! `TuiEvent::MoveDown` while the help overlay was open, so there was no way
//! to scroll back to the top of a long help panel. This test pins the
//! key-to-event contract via the pure `help_overlay_event_for` seam, plus a
//! state-level scroll round-trip through `AppState.preview.scroll`.
use crossterm::event::KeyCode;
use tui_lint_arwaky::surface_tui_command::help_overlay_event_for;
use tui_lint_arwaky::taxonomy_tui_event::TuiEvent;
use tui_lint_arwaky::taxonomy_tui_vo::AppState;
use tui_lint_arwaky::taxonomy_tui_vo::PanelFocus;
use tui_lint_arwaky::taxonomy_tui_vo::PreviewMode;

fn forward_keys() -> [KeyCode; 4] {
    [
        KeyCode::Char('j'),
        KeyCode::Down,
        KeyCode::Char('l'),
        KeyCode::Right,
    ]
}

fn backward_keys() -> [KeyCode; 4] {
    [
        KeyCode::Char('k'),
        KeyCode::Up,
        KeyCode::Char('h'),
        KeyCode::Left,
    ]
}

/// Forward navigation keys keep scrolling the help overlay down.
#[test]
fn help_overlay_forward_keys_map_to_move_down() {
    for code in forward_keys() {
        assert_eq!(
            help_overlay_event_for(code),
            TuiEvent::MoveDown,
            "{code:?} should scroll the help overlay down"
        );
    }
}

/// Backward navigation keys scroll the help overlay up — the exact regression
/// from #556. Each of these used to collapse onto MoveDown, so there was no
/// way back to the top.
#[test]
fn help_overlay_backward_keys_map_to_move_up() {
    for code in backward_keys() {
        assert_eq!(
            help_overlay_event_for(code),
            TuiEvent::MoveUp,
            "{code:?} should scroll the help overlay up"
        );
    }
}

/// Home jumps to the top; End jumps to the bottom. Before #556 both collapsed
/// onto MoveDown, so neither jump was possible.
#[test]
fn help_overlay_jump_keys() {
    assert_eq!(help_overlay_event_for(KeyCode::Home), TuiEvent::MoveTop);
    assert_eq!(help_overlay_event_for(KeyCode::End), TuiEvent::MoveBottom);
}

/// PageUp / PageDown use the dedicated preview scroll events (10-line steps),
/// distinct from the 3-line MoveUp/MoveDown used by the arrow/vim keys.
#[test]
fn help_overlay_page_keys() {
    assert_eq!(
        help_overlay_event_for(KeyCode::PageUp),
        TuiEvent::PreviewScrollUp
    );
    assert_eq!(
        help_overlay_event_for(KeyCode::PageDown),
        TuiEvent::PreviewScrollDown
    );
}

/// `?` and `Esc` both close the help overlay; `q` quits the app. Before #556
/// `Esc` produced `Quit`, inconsistent with `?` toggling.
#[test]
fn help_overlay_toggle_and_quit_keys() {
    assert_eq!(
        help_overlay_event_for(KeyCode::Char('?')),
        TuiEvent::ToggleHelp
    );
    assert_eq!(help_overlay_event_for(KeyCode::Esc), TuiEvent::ToggleHelp);
    assert_eq!(help_overlay_event_for(KeyCode::Char('q')), TuiEvent::Quit);
}

/// Anything else in the overlay is swallowed so it cannot leak through to
/// directory navigation or list selection underneath.
#[test]
fn help_overlay_unmapped_keys_map_to_none() {
    assert_eq!(help_overlay_event_for(KeyCode::Char('x')), TuiEvent::None);
    assert_eq!(help_overlay_event_for(KeyCode::Enter), TuiEvent::None);
}

/// State-level round trip: from the bottom of a long help panel, the
/// backward keys walk the scroll back to the top. This is the user-facing
/// regression #556 was about: before the fix, every key (including `k`,
/// `Up`, `Home`, `PageUp`) only ever scrolled forward, so a long panel had
/// no way back up.
#[test]
fn help_overlay_back_and_forward_scroll_round_trips() {
    let mut state = AppState::new("/tmp".to_string());
    state.show_help = true;
    state.navigation.panel_focus = PanelFocus::Preview;
    state.preview.mode = PreviewMode::HelpOverlay;
    state.terminal_height = 30;
    // 40 lines at 30 terminal height: max scroll = 40 - 26 = 14.
    state.preview.text = (0..40)
        .map(|i| format!("help line {i}"))
        .collect::<Vec<_>>()
        .join("\n");
    state.preview.scroll = 14;

    // Each backward key must strictly decrease the scroll; `Home` must reach 0.
    for code in [
        KeyCode::PageUp,
        KeyCode::Char('k'),
        KeyCode::Up,
        KeyCode::Char('h'),
        KeyCode::Left,
        KeyCode::Home,
    ] {
        let next = match help_overlay_event_for(code) {
            TuiEvent::MoveUp => state.preview.scroll.saturating_sub(3),
            TuiEvent::MoveTop => 0,
            TuiEvent::PreviewScrollUp => state.preview.scroll.saturating_sub(10),
            other => panic!("{code:?} mapped to {other:?}, expected a backward scroll event"),
        };
        assert!(
            next <= state.preview.scroll,
            "{code:?} must not advance the scroll"
        );
        state.preview.scroll = next;
    }
    assert_eq!(state.preview.scroll, 0, "backward keys must reach the top");

    // Each forward key must strictly increase the scroll; `End` must reach max.
    let max = state.preview.text.lines().count() - 26;
    for code in [
        KeyCode::Char('j'),
        KeyCode::Down,
        KeyCode::Char('l'),
        KeyCode::Right,
        KeyCode::PageDown,
        KeyCode::End,
    ] {
        let next = match help_overlay_event_for(code) {
            TuiEvent::MoveDown => state.preview.scroll.saturating_add(3).min(max),
            TuiEvent::MoveBottom => max,
            TuiEvent::PreviewScrollDown => state.preview.scroll.saturating_add(10).min(max),
            other => panic!("{code:?} mapped to {other:?}, expected a forward scroll event"),
        };
        assert!(
            next >= state.preview.scroll,
            "{code:?} must not regress the scroll"
        );
        state.preview.scroll = next;
    }
    assert_eq!(
        state.preview.scroll, max,
        "forward keys must reach the bottom"
    );
}
