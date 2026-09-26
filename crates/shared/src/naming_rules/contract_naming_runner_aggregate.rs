// PURPOSE: INamingRunnerAggregate — single entry point over the naming-rules domain
// The agent behind the aggregate dispatches each NamingRequest to the rich
// INamingCheckerProtocol operation. Consumers never see the protocol.
use crate::naming_rules::taxonomy_naming_request::NamingRequest;
use crate::naming_rules::taxonomy_naming_response::NamingResponse;

/// Single entry point over naming-rules; the agent dispatches internally.
pub trait INamingRunnerAggregate: Send + Sync {
    fn execute(&self, request: NamingRequest) -> NamingResponse;
}
