// PURPOSE: single-subsystem agent — the P14 skip case
//
// This agent injects exactly one protocol, which would trip a flat
// ">= 2 subsystems" rule. The feature declares exactly one protocol in its
// shared module, so the agent coordinates every subsystem the feature has
// and AES405 P14 skips it.
use std::sync::Arc;

use calculator_shared::taxonomy_single_request::SingleRequest;
use calculator_shared::taxonomy_single_response::SingleResponse;

use crate::contract_single_protocol::{ISingleCheckerProtocol, ISingleRunnerAggregate};

pub struct SingleGoalOrchestrator {
    checker: Arc<dyn ISingleCheckerProtocol>,
}

impl ISingleRunnerAggregate for SingleGoalOrchestrator {
    fn execute(&self, request: SingleRequest) -> SingleResponse {
        self.checker.audit(request)
    }
}

impl SingleGoalOrchestrator {
    pub fn new(checker: Arc<dyn ISingleCheckerProtocol>) -> Self {
        Self { checker }
    }
}
