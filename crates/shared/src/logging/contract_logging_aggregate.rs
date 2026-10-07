// PURPOSE: ILoggingAggregate — single entry point over the logging feature.
//
// The composition root's door. The agent behind the aggregate routes
// `init` to the filter + subscriber seams, `time_phase` to the phase
// timer, and `report_walk` to the walker reporter. Consumers never see
// the individual protocols.

use crate::taxonomy_logging_vo::SkipReason;
use shared_common::LogVerbosity;

/// Single entry point over the logging feature.
pub trait ILoggingAggregate: Send + Sync {
    /// Install the global subscriber: build the filter, then install it.
    fn init(&self, verbosity: LogVerbosity, with_ansi: bool);

    /// Run `f` with the phase timed; emits `phase_start` and `phase_done`.
    fn time_phase<R>(&self, phase: &'static str, f: impl FnOnce() -> R) -> R;

    /// Emit one walker event: enter, skip(reason), or files-discovered.
    fn report_walk_enter(&self, dir: &str);
    fn report_walk_skip(&self, dir: &str, reason: SkipReason);
    fn report_files_discovered(&self, count: usize, elapsed_ms: u64);
}
