// PURPOSE: ILoggingAggregate — single entry point over the logging feature.
//
// This is NOT a standard feature aggregate with execute(request) -> response.
// Logging is cross-cutting infrastructure. The aggregate provides direct
// methods that capabilities implement, similar to how ISubscriberInitProtocol
// provides build_filter() and init().
//
// Root callers use these methods; the dispatcher will later use them via
// the walker reporter and phase timer.

use crate::taxonomy_logging_vo::SkipReason;
use shared_common::taxonomy_logging_vo::LogVerbosity;

/// Single entry point over the logging feature.
pub trait ILoggingAggregate: Send + Sync {
    /// Install the global subscriber: build the filter, then install it.
    fn init(&self, verbosity: LogVerbosity, with_ansi: bool);

    /// Begin timing a phase; returns a PhaseTimer for the caller to hold.
    fn phase_started(&self, phase: &'static str) -> crate::taxonomy_logging_vo::PhaseTimer;

    /// End timing a phase; emit the elapsed_ms event.
    fn phase_finished(&self, timer: &crate::taxonomy_logging_vo::PhaseTimer, count: Option<usize>);

    /// Emit a walker-enter event.
    fn walker_enter(&self, dir: &str);

    /// Emit a walker-skip event with reason.
    fn walker_skip(&self, dir: &str, reason: SkipReason);

    /// Emit a files-discovered event with count and elapsed time.
    fn files_discovered(&self, count: usize, elapsed_ms: u64);
}
