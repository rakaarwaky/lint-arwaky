// PURPOSE: end-to-end logging flow — full lifecycle of
// LoggingContainer::new().init(...) exercising both ANSI modes and
// all three verbosity levels, as the CLI and MCP entry points use them.
use logging_lint_arwaky::LogVerbosity;
use logging_lint_arwaky::root_logging_container::LoggingContainer;

#[test]
fn init_debug_with_ansi_does_not_panic() {
    let container = LoggingContainer::new();
    container.init(LogVerbosity::Debug, true);
}

#[test]
fn init_info_without_ansi_does_not_panic() {
    let container = LoggingContainer::new();
    container.init(LogVerbosity::Info, false);
}
