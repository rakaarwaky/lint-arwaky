// PURPOSE: single-subsystem feature shared contract (P14 skip case)
//
// One protocol, one aggregate. The agent in `agent_single_orchestrator.rs`
// injects that one protocol and coordinates the whole feature. The two
// taxonomy types give the protocol and the aggregate their request and
// response, so neither has to invent its own shapes.
use crate::taxonomy_single_request::SingleRequest;
use crate::taxonomy_single_response::SingleResponse;

pub trait ISingleCheckerProtocol: Send + Sync {
    fn audit(&self, request: SingleRequest) -> SingleResponse;
}

pub trait ISingleRunnerAggregate: Send + Sync {
    fn execute(&self, request: SingleRequest) -> SingleResponse;
}
