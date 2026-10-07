// PURPOSE: ILoggingAggregate — single entry point over the logging feature.
//
// One `execute` method that routes LoggingRequest to capability seams.

use crate::taxonomy_logging_request::LoggingRequest;
use crate::taxonomy_logging_response::LoggingResponse;

/// Single entry point over the logging feature.
///
/// ONE method only: `execute`. All behavior routes through request variants.
pub trait ILoggingAggregate: Send + Sync {
    fn execute(&self, request: LoggingRequest) -> LoggingResponse;
}
