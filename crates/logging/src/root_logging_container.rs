// PURPOSE: LoggingContainer — root-layer composition root for the logging
// feature. Wires all four capability seams into the orchestrator.

use std::sync::Arc;

use crate::agent_logging_orchestrator::LoggingOrchestrator;
use crate::capabilities_phase_timer::PhaseTimerCapability;
use crate::capabilities_subscriber_init::SubscriberInit;
use crate::capabilities_walker_reporter::WalkerReporter;
use shared_logging::LogVerbosity;
use shared_logging::contract_logging_aggregate::ILoggingAggregate;
use shared_logging::taxonomy_logging_request::{LoggingRequest, LoggingRequestKind};

// ─── Block 1: Struct Definition ───────────────────────────

pub struct LoggingContainer {
    orchestrator: LoggingOrchestrator,
}

// ─── Block 2: Wiring ──────────────────────────────────────

impl LoggingContainer {
    pub fn new() -> Self {
        let subscriber: Arc<dyn shared_logging::ISubscriberInstallProtocol> =
            Arc::new(SubscriberInit::default());
        let timer: Arc<dyn shared_logging::IPhaseTimerProtocol> =
            Arc::new(PhaseTimerCapability::new());
        let walker: Arc<dyn shared_logging::IWalkerReportProtocol> =
            Arc::new(WalkerReporter::new());
        Self {
            orchestrator: LoggingOrchestrator::new(subscriber, timer, walker),
        }
    }

    pub fn orchestrator(&self) -> &LoggingOrchestrator {
        &self.orchestrator
    }

    pub fn init(&self, verbosity: LogVerbosity, with_ansi: bool) {
        self.orchestrator.execute(LoggingRequest {
            kind: LoggingRequestKind::InstallSubscriber {
                verbosity,
                with_ansi,
            },
        });
    }
}

impl Default for LoggingContainer {
    fn default() -> Self {
        Self::new()
    }
}
