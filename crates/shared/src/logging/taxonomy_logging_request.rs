// PURPOSE: Request VOs for the logging feature.
//
// Pure request types that cross layer boundaries. No logic, no I/O.

use super::taxonomy_logging_vo::{LogVerbosity, PhaseTimerVO, SkipReason};

/// The kind of logging operation requested.
pub enum LoggingRequestKind {
    InstallSubscriber { verbosity: LogVerbosity, with_ansi: bool },
    PhaseStarted { phase: &'static str },
    PhaseFinished { timer: PhaseTimerVO, count: Option<usize> },
    WalkerEnter { dir: String },
    WalkerSkip { dir: String, reason: SkipReason },
    FilesDiscovered { count: super::taxonomy_logging_vo::Count, elapsed_ms: super::taxonomy_logging_vo::DurationMs },
}

/// Request for the logging feature.
pub struct LoggingRequest {
    pub kind: LoggingRequestKind,
}
