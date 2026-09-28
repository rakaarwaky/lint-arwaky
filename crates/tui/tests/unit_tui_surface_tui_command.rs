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

#[test]
fn confirm_gate_blocks_all_other_keys() {
    let mut state = browsing_state();
    state.pending_confirm = Some(ConfirmState {
        pending: TuiEvent::ActionUninstallHook,
        label: "Uninstall pre-commit hook".to_string(),
    });

    // 'c' (check) and 'F' (live fix) must not fire behind a pending confirm.
    assert_eq!(from_key_event(key(KeyCode::Char('c')), &state), TuiEvent::None);
    assert_eq!(from_key_event(key(KeyCode::Char('F')), &state), TuiEvent::None);
}

// ─── UX-1-02: path dialog Esc semantics ───

#[test]
fn path_dialog_esc_quits_only_on_first_run() {
    let mut state = AppState::new("/tmp/project".to_string());
    // First run: dialog up, no root confirmed yet → Esc quits the TUI.
    assert!(state.show_path_dialog);
    assert!(state.dialog_is_first_run);
    assert_eq!(from_key_event(key(KeyCode::Esc), &state), TuiEvent::Quit);

    // Reopened dialog (via `r`): Esc cancels, keeping the current root.
    state.dialog_is_first_run = false;
    assert_eq!(from_key_event(key(KeyCode::Esc), &state), TuiEvent::PathCancel);
}

// ─── UX-2-03: help overlay blocks background actions ───

#[test]
fn help_overlay_only_accepts_overlay_keys() {
    let mut state = browsing_state();
    state.show_help = true;

    assert_eq!(from_key_event(key(KeyCode::Char('c')), &state), TuiEvent::None);
    assert_eq!(from_key_event(key(KeyCode::Char('?')), &state), TuiEvent::ToggleHelp);
    assert_eq!(from_key_event(key(KeyCode::Esc), &state), TuiEvent::ToggleHelp);
}

// ─── UX-4-01: shifted letters and terminal-safe bindings ───

#[test]
fn capital_f_maps_to_live_fix_regardless_of_shift_modifier() {
    let state = browsing_state();
    // crossterm delivers Shift+F as Char('F') on standard terminals — with or
    // without the SHIFT modifier bit, the key must trigger the live fix.
    assert_eq!(
        from_key_event(KeyEvent::new(KeyCode::Char('F'), KeyModifiers::SHIFT), &state),
        TuiEvent::ActionFixLive
    );
    assert_eq!(from_key_event(key(KeyCode::Char('F')), &state), TuiEvent::ActionFixLive);
    assert_eq!(from_key_event(key(KeyCode::Char('f')), &state), TuiEvent::ActionFix);
}

#[test]
fn x_runs_security_scan_without_xoff_risk() {
    let state = browsing_state();
    assert_eq!(from_key_event(key(KeyCode::Char('x')), &state), TuiEvent::ActionSecurity);
}
