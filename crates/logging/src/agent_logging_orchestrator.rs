// PURPOSE: LoggingOrchestrator — agent layer, composes the four
// capability seams. Handles lifecycle (init) and emits timing/walker
// events through the capability seams.

use std::sync::Arc;

use shared_common::taxonomy_logging_vo::LogVerbosity;
use shared_logging::contract_logging_aggregate::ILoggingAggregate;
use shared_logging::contract_logging_protocol::{
    IPhaseTimerProtocol, ISubscriberInstallProtocol, IWalkerReportProtocol,
};
use shared_logging::taxonomy_logging_vo::{PhaseTimer, SkipReason};

// ─── Block 1: Struct Definition ───────────────────────────

pub struct LoggingOrchestrator {
    subscriber: Arc<dyn ISubscriberInstallProtocol>,
    timer: Arc<dyn IPhaseTimerProtocol>,
    walker: Arc<dyn IWalkerReportProtocol>,
}

// ─── Block 2: Aggregate Trait Implementation ──────────────

impl ILoggingAggregate for LoggingOrchestrator {
    fn init(&self, verbosity: LogVerbosity, with_ansi: bool) {
        self.subscriber.install(verbosity, with_ansi);
    }

    fn phase_started(&self, phase: &'static str) -> PhaseTimer {
        self.timer.phase_started(phase)
    }

    fn phase_finished(&self, timer: &PhaseTimer, count: Option<usize>) {
        self.timer.phase_finished(timer, count);
    }

    fn walker_enter(&self, dir: &str) {
        self.walker.walker_enter(dir);
    }

    fn walker_skip(&self, dir: &str, reason: SkipReason) {
        self.walker.walker_skip(dir, reason);
    }

    fn files_discovered(&self, count: usize, elapsed_ms: u64) {
        self.walker.files_discovered(count, elapsed_ms);
    }
}

// ─── Block 3: Constructors, Std Traits, Helpers ───────────

impl LoggingOrchestrator {
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

impl Default for LoggingOrchestrator {
    fn default() -> Self {
        Self::new(
            Arc::new(crate::capabilities_subscriber_init::SubscriberInit::default()),
            Arc::new(crate::capabilities_phase_timer::PhaseTimerCapability::new()),
            Arc::new(crate::capabilities_walker_reporter::WalkerReporter::new()),
        )
    }
}
