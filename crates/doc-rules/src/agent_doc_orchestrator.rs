// PURPOSE: DocOrchestrator — agent that orchestrates doc-invariant checks
//
// The single entry point over the doc-rules feature. Consumers call
// `execute(DocRequest)`; the agent routes it to the protocol capability via
// the `IDocCheckerProtocol` trait, keeping the agent free of concrete
// capability imports.
use shared::doc_rules::contract_doc_aggregate::IDocRunnerAggregate;
use shared::doc_rules::contract_doc_protocol::IDocCheckerProtocol;
use shared::doc_rules::taxonomy_doc_rules_request::DocRequest;
use shared::doc_rules::taxonomy_doc_rules_response::DocResponse;
use std::sync::Arc;

/// Stateless orchestrator: the capability it holds has no state, so the
/// unit struct plus a shared reference is the whole implementation.
pub struct DocOrchestrator {
    checker: Arc<dyn IDocCheckerProtocol>,
}

/// Aggregate trait implementation: the agent delegates to the injected
/// checker and returns the response. Blocks come in the prescribed order
/// so the file is a clean AES405 composition (aggregate impl before the
/// constructor helper).
impl IDocRunnerAggregate for DocOrchestrator {
    fn execute(&self, request: DocRequest) -> DocResponse {
        self.checker.audit(request)
    }
}

/// Block 3: constructor — the only entry point into the orchestrator.
impl DocOrchestrator {
    pub fn new(checker: Arc<dyn IDocCheckerProtocol>) -> Self {
        Self { checker }
    }
}
