// Smoke tests — container creation, key types accessible within 5s.
use shared::common::{DisplayContent, FilePath};
use tui_lint_arwaky::utility_file_system;

#[test]
fn smoke_is_valid_directory_completes_quickly() {
    let start = std::time::Instant::now();
    let tmp = FilePath::new("/tmp".to_string()).unwrap();
    let _ = utility_file_system::is_valid_directory(&tmp);
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "Smoke test exceeded 5s: {:?}",
        elapsed
    );
}

#[test]
fn smoke_tui_types_are_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<shared::tui::FileEntry>();
    assert_send_sync::<shared::tui::LintExecutionResult>();
    assert_send_sync::<FilePath>();
    assert_send_sync::<DisplayContent>();
}
