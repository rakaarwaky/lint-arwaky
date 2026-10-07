// PURPOSE: LoggingContainer — root-layer composition root for the logging
// feature. Wires all four capability seams into the orchestrator.
// Root entry points construct this and call `init`; they never touch
// the capability directly, preserving the layer boundary.

use std::sync::Arc;

use crate::agent_logging_orchestrator::LoggingOrchestrator;
use crate::capabilities_phase_timer::PhaseTimerCapability;
use crate::capabilities_subscriber_init::SubscriberInit;
use crate::capabilities_walker_reporter::WalkerReporter;
use shared_common::taxonomy_logging_vo::LogVerbosity;
use shared_logging::ILoggingAggregate;

// ─── Block 1: Struct Definition ───────────────────────────

/// Composition root. One instance per process; held by the entry point.
pub struct LoggingContainer {
    orchestrator: LoggingOrchestrator,
}

// ─── Block 2: Wiring ──────────────────────────────────────

impl LoggingContainer {
    /// Build the container. Stateless — no filesystem, no config.
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

    /// Return a reference to the orchestrator.
    pub fn orchestrator(&self) -> &LoggingOrchestrator {
        &self.orchestrator
    }

    /// Convenience: initialize the global subscriber from the container.
    pub fn init(&self, verbosity: LogVerbosity, with_ansi: bool) {
        self.orchestrator.init(verbosity, with_ansi);
    }
}

impl Default for LoggingContainer {
    fn default() -> Self {
        Self::new()
    }
}
