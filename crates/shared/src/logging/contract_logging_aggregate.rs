// PURPOSE: ILoggingAggregate — single entry point over the logging feature.
//
// One `execute` method that routes LoggingRequest to capability seams.
// Uses PhaseTimerVO (not raw primitives) in signatures per AES402.

use crate::taxonomy_logging_vo::{PhaseTimerVO, SkipReason};
use shared_common::taxonomy_logging_vo::LogVerbosity;

/// Request for the logging feature.
pub struct LoggingRequest {
    pub kind: LoggingRequestKind,
}

/// The kind of logging operation requested.
pub enum LoggingRequestKind {
    InstallSubscriber {
        verbosity: LogVerbosity,
        with_ansi: bool,
    },
    PhaseStarted {
        phase: &'static str,
    },
    PhaseFinished {
        timer: PhaseTimerVO,
        count: Option<usize>,
    },
    WalkerEnter {
        dir: String,
    },
    WalkerSkip {
        dir: String,
        reason: SkipReason,
    },
    FilesDiscovered {
        count: usize,
        elapsed_ms: u64,
    },
}

/// Response from the logging feature.
pub struct LoggingResponse {
    pub kind: LoggingResponseKind,
}

/// The kind of logging response returned.
pub enum LoggingResponseKind {
    Done,
    PhaseTimer { timer: PhaseTimerVO },
}

/// Single entry point over the logging feature.
///
/// ONE method only: `execute`. All behavior routes through request variants.
pub trait ILoggingAggregate: Send + Sync {
    fn execute(&self, request: LoggingRequest) -> LoggingResponse;
}
