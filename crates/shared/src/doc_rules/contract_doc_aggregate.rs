// PURPOSE: IDocRunnerAggregate — single entry point over the doc-rules feature
///
/// The single door consumers knock on. The agent behind the aggregate
/// routes `DocRequest::AuditAll` to the invariant-auditor capability and
/// folds its findings into the response. Consumers never see the protocol.
use crate::taxonomy_doc_rules_request::DocRequest;
use crate::taxonomy_doc_rules_response::DocResponse;

/// Single entry point over the doc-rules feature.
pub trait IDocRunnerAggregate: Send + Sync {
    /// Dispatch the request to the capability seam and return the response.
    fn execute(&self, request: DocRequest) -> DocResponse;
}
