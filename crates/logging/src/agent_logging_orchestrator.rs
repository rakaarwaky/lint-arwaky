// PURPOSE: LoggingOrchestrator — agent layer, composes the four
// capability seams. The orchestrator holds an Arc<dyn I*Protocol> for
// each seam and exposes the ILoggingAggregate entry that the root
// container calls.
//
// LogVerbosity is shared-common; the agent layer depends only on
// shared-common (taxonomy) and shared-logging (contracts), never on
// the concrete capability types.

use std::sync::Arc;

use shared_common::taxonomy_logging_vo::LogVerbosity;
use shared_logging::contract_logging_aggregate::ILoggingAggregate;
use shared_logging::taxonomy_logging_vo::SkipReason;
use shared_logging::{IPhaseTimerProtocol, ISubscriberInstallProtocol, IWalkerReportProtocol};

// ─── Block 1: Struct Definition ───────────────────────────

/// Single composite entry point for the logging feature.
///
/// Root-layer callers construct a `LoggingContainer` and call `init`
/// once at process start. The MCP entry point passes `with_ansi = false`;
/// the CLI passes `true`.
pub struct LoggingOrchestrator {
    subscriber: Arc<dyn ISubscriberInstallProtocol>,
    timer: Arc<dyn IPhaseTimerProtocol>,
    walker: Arc<dyn IWalkerReportProtocol>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl ILoggingAggregate for LoggingOrchestrator {
    fn init(&self, verbosity: LogVerbosity, with_ansi: bool) {
        self.subscriber.install(verbosity, with_ansi);
    }

    fn time_phase<R>(&self, phase: &'static str, f: impl FnOnce() -> R) -> R {
        let timer = self.timer.phase_started(phase);
        let result = f();
        self.timer.phase_finished(&timer, None);
        result
    }

    fn report_walk_enter(&self, dir: &str) {
        self.walker.walker_enter(dir);
    }

    fn report_walk_skip(&self, dir: &str, reason: SkipReason) {
        self.walker.walker_skip(dir, reason);
    }

    fn report_files_discovered(&self, count: usize, elapsed_ms: u64) {
        self.walker.files_discovered(count, elapsed_ms);
    }
}

// ─── Block 3: Constructors, Std Traits, Helpers ───────────

impl LoggingOrchestrator {
    /// Build the orchestrator with all four capability seams.
    pub fn new(
        subscriber: Arc<dyn ISubscriberInstallProtocol>,
        timer: Arc<dyn IPhaseTimerProtocol>,
        walker: Arc<dyn IWalkerReportProtocol>,
    ) -> Self {
        Self {
            subscriber,
            timer,
            walker,
        }
    }
}
