// PURPOSE: Unit tests for the TUI keymap gates in surface_tui_command::from_key_event.
// Covers: confirm-gate routing (UX-1-01), path-dialog Esc semantics (UX-1-02),
// help-overlay input block (UX-2-03), and standard-terminal key delivery (UX-4-01).
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use shared::tui::{AppState, ConfirmState, TuiEvent};
use tui_lint_arwaky::surface_tui_command::from_key_event;

/// A browsing state: past the startup path dialog, nothing modal open.
fn browsing_state() -> AppState {
    let mut state = AppState::new("/tmp/project".to_string());
    state.show_path_dialog = false;
    state.dialog_is_first_run = false;
    state
}

fn key(code: KeyCode) -> KeyEvent {
    KeyEvent::new(code, KeyModifiers::NONE)
}

// ─── UX-1-01: confirm gate intercepts all input while pending ───

#[test]
fn confirm_gate_routes_answer_keys_to_decision() {
    let mut state = browsing_state();
    state.pending_confirm = Some(ConfirmState {
        pending: TuiEvent::ActionInstall,
        label: "Install lint-arwaky binaries into PATH".to_string(),
    });

    assert_eq!(from_key_event(key(KeyCode::Char('y')), &state), TuiEvent::ConfirmAction);
    assert_eq!(from_key_event(key(KeyCode::Enter), &state), TuiEvent::ConfirmAction);
    assert_eq!(from_key_event(key(KeyCode::Char('n')), &state), TuiEvent::CancelConfirm);
    assert_eq!(from_key_event(key(KeyCode::Esc), &state), TuiEvent::CancelConfirm);
}
