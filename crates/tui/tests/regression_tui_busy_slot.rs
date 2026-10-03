//! Regression coverage for the single busy gate shared by scans and background
//! actions. Both fan out onto the process-global rayon pool, so only one may be
//! in flight (#577).
use tui_lint_arwaky::taxonomy_tui_vo::{AppState, BusySlot};

fn state() -> AppState {
    AppState::new("/tmp/project".to_string())
}

#[test]
fn scan_is_rejected_while_a_background_action_is_pending() {
    let mut state = state();
    assert!(state.try_claim_busy_slot(BusySlot::Action));

    assert!(!state.try_claim_busy_slot(BusySlot::Scan));
    assert_eq!(state.status_message, "Busy: action is running");
}

#[test]
fn background_action_is_rejected_while_a_scan_is_running() {
    let mut state = state();
    assert!(state.try_claim_busy_slot(BusySlot::Scan));

    assert!(!state.try_claim_busy_slot(BusySlot::Action));
    assert_eq!(state.status_message, "Busy: scan is running");
}

#[test]
fn a_rejected_request_does_not_set_the_other_flag() {
    let mut state = state();
    assert!(state.try_claim_busy_slot(BusySlot::Scan));

    assert!(!state.try_claim_busy_slot(BusySlot::Action));
    assert!(
        !state.actions.pending,
        "declined action must not mark itself pending"
    );
    assert!(
        state.scan.running,
        "declined request must not clear the holder"
    );

    assert!(!state.try_claim_busy_slot(BusySlot::Scan));
    assert!(state.scan.running);
}

#[test]
fn the_slot_is_reusable_after_the_holder_finishes() {
    let mut state = state();
    assert!(state.try_claim_busy_slot(BusySlot::Action));
    assert!(!state.try_claim_busy_slot(BusySlot::Scan));

    state.actions.pending = false;
    assert!(state.try_claim_busy_slot(BusySlot::Scan));
    assert!(state.scan.running);
}

#[test]
fn scan_finishing_frees_the_slot_for_a_background_action() {
    let mut state = state();
    assert!(state.try_claim_busy_slot(BusySlot::Scan));
    assert!(!state.try_claim_busy_slot(BusySlot::Action));

    state.finish_scan(0);
    assert!(!state.scan.running);
    assert!(state.try_claim_busy_slot(BusySlot::Action));
    assert!(state.actions.pending);
}
