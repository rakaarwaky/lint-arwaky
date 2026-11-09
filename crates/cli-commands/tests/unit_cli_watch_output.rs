use cli_commands::surface_watch_command::format_watch_output;
use shared_file_watch::WatchOutputEvent;

#[test]
fn watch_output_formats_start_baseline_changes_and_stop_at_default_verbosity() {
    let cases = [
        (
            WatchOutputEvent::Started {
                path: "workspace".to_string(),
            },
            false,
            "Watching workspace — press Ctrl+C to stop",
        ),
        (
            WatchOutputEvent::InitialScanComplete {
                violations: 3,
                score: 92.5,
            },
            false,
            "Initial scan complete: 3 violations, score 92.5",
        ),
        (
            WatchOutputEvent::FileLinted {
                path: "src/lib.rs".to_string(),
                succeeded: true,
            },
            false,
            "Linted src/lib.rs",
        ),
        (
            WatchOutputEvent::FileLinted {
                path: "src/lib.rs".to_string(),
                succeeded: false,
            },
            true,
            "Lint failed for src/lib.rs",
        ),
        (WatchOutputEvent::Stopped, false, "Watcher stopped"),
    ];

    for (event, expected_error, expected_message) in cases {
        assert_eq!(
            format_watch_output(event),
            (expected_error, expected_message.to_string())
        );
    }
}
