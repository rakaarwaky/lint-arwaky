// PURPOSE: DocOrchestrator — agent that orchestrates doc-invariant checks
//
// The single entry point over the doc-rules feature. Consumers call
// `execute(DocRequest)`; the agent routes it to the protocol capability via
// the `IDocCheckerProtocol` trait, keeping the agent free of concrete
// capability imports.
use shared::doc_rules::contract_doc_aggregate::IDocRunnerAggregate;
use shared::doc_rules::contract_doc_protocol::IDocCheckerProtocol;
use shared::doc_rules::taxonomy_doc_request::DocRequest;
use shared::doc_rules::taxonomy_doc_response::DocResponse;
use std::sync::Arc;

/// Stateless orchestrator: the capability it holds has no state, so the
/// unit struct plus a shared reference is the whole implementation.
pub struct DocOrchestrator {
    checker: Arc<dyn IDocCheckerProtocol>,
}

impl DocOrchestrator {
    pub fn new(checker: Arc<dyn IDocCheckerProtocol>) -> Self {
        Self { checker }
    }
}

impl IDocRunnerAggregate for DocOrchestrator {
    fn execute(&self, request: DocRequest) -> DocResponse {
        self.checker.audit(request)
    }
}
