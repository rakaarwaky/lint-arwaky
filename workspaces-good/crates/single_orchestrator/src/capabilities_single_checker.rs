use calculator_shared::single_orchestrator::contract_single_protocol::ISingleCheckerProtocol;
use calculator_shared::taxonomy_single_request::SingleRequest;
use calculator_shared::taxonomy_single_response::SingleResponse;

/// The feature's one and only subsystem: it checks a single target.
///
/// The whole feature is one protocol, one capability, one agent, so the agent
/// has nothing to coordinate beyond this — which is the shape AES405 P14
/// exempts.
pub struct SingleChecker;

impl ISingleCheckerProtocol for SingleChecker {
    fn audit(&self, request: SingleRequest) -> SingleResponse {
        SingleResponse {
            passed: !request.target.is_empty(),
            detail: request.target,
        }
    }
}
