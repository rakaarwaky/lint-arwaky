// PURPOSE: INamingRunnerAggregate — single entry point over the naming-rules domain
// The agent behind the aggregate dispatches each NamingRequest to the rich
// INamingConventionProtocol / ISuffixPolicyProtocol operations. Consumers never see the protocol.
use crate::taxonomy_naming_rules_request::NamingRequest;
use crate::taxonomy_naming_rules_response::NamingResponse;

/// Single entry point over naming-rules; the agent dispatches internally.
pub trait INamingRunnerAggregate: Send + Sync {
    fn execute(&self, request: NamingRequest) -> NamingResponse;
}
