// PURPOSE: Response VOs for the logging feature.
//
// Pure response types that cross layer boundaries. No logic, no I/O.

use crate::taxonomy_logging_vo::PhaseTimerVO;

/// The kind of logging response returned.
pub enum LoggingResponseKind {
    Done,
    PhaseTimer { timer: PhaseTimerVO },
}

/// Response from the logging feature.
pub struct LoggingResponse {
    pub kind: LoggingResponseKind,
}
