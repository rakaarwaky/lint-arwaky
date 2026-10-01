//! Regression coverage for the worker boundary used by every long action.
use std::sync::mpsc::sync_channel;
use std::time::{Duration, Instant};
use tui_lint_arwaky::surface_event_action::spawn_background_task;

#[test]
fn slow_action_returns_control_to_the_event_loop_immediately() {
    let (tx, rx) = sync_channel(1);
    let started = Instant::now();
    spawn_background_task(
        || {
            std::thread::sleep(Duration::from_millis(200));
            42_u8
        },
        tx,
    );

    assert!(
        started.elapsed() < Duration::from_millis(50),
        "dispatch blocked for {:?}",
        started.elapsed()
    );
    assert_eq!(rx.recv_timeout(Duration::from_secs(1)).unwrap(), 42);
}
