// PURPOSE: LoggingOrchestrator — agent layer, composes the four
// capability seams. One execute() method routes requests to capabilities.

use std::sync::Arc;

use shared_logging::contract_logging_aggregate::ILoggingAggregate;
use shared_logging::contract_logging_protocol::{
    IPhaseTimerProtocol, ISubscriberInstallProtocol, IWalkerReportProtocol,
};
use shared_logging::taxonomy_logging_request::{LoggingRequest, LoggingRequestKind};
use shared_logging::taxonomy_logging_response::{LoggingResponse, LoggingResponseKind};

// ─── Block 1: Struct Definition ───────────────────────────

pub struct LoggingOrchestrator {
    subscriber: Arc<dyn ISubscriberInstallProtocol>,
    timer: Arc<dyn IPhaseTimerProtocol>,
    walker: Arc<dyn IWalkerReportProtocol>,
}

// ─── Block 2: Aggregate Trait Implementation ──────────────

impl ILoggingAggregate for LoggingOrchestrator {
    fn execute(&self, request: LoggingRequest) -> LoggingResponse {
        match request.kind {
            LoggingRequestKind::InstallSubscriber { verbosity, with_ansi } => {
                let _ = verbosity;
                self.subscriber.install(verbosity, with_ansi);
                LoggingResponse {
                    kind: LoggingResponseKind::Done,
                }
            }
            LoggingRequestKind::PhaseStarted { phase } => {
                let timer = self.timer.phase_started(phase);
                LoggingResponse {
                    kind: LoggingResponseKind::PhaseTimer { timer },
                }
            }
            LoggingRequestKind::PhaseFinished { timer, count } => {
                self.timer.phase_finished(&timer, count);
                LoggingResponse {
                    kind: LoggingResponseKind::Done,
                }
            }
            LoggingRequestKind::WalkerEnter { dir } => {
                self.walker.walker_enter(&dir);
                LoggingResponse {
                    kind: LoggingResponseKind::Done,
                }
            }
            LoggingRequestKind::WalkerSkip { dir, reason } => {
                self.walker.walker_skip(&dir, reason);
                LoggingResponse {
                    kind: LoggingResponseKind::Done,
                }
            }
            LoggingRequestKind::FilesDiscovered { count, elapsed_ms } => {
                self.walker.files_discovered(count, elapsed_ms);
                LoggingResponse {
                    kind: LoggingResponseKind::Done,
                }
            }
        }
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
