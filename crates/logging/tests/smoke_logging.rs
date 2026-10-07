// PURPOSE: smoke tests — fast-boot (< 5 s) sanity checks for the
// logging crate: LoggingContainer::default() constructs and
// LogVerbosity::default() returns LogVerbosity::Default.
use logging_lint_arwaky::LogVerbosity;
use logging_lint_arwaky::root_logging_container::LoggingContainer;

#[test]
fn logging_container_default_constructs() {
    let _container = LoggingContainer::default();
}

#[test]
fn log_verbosity_default_is_default() {
    assert_eq!(LogVerbosity::default(), LogVerbosity::Default);
}
