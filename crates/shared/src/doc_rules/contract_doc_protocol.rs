// PURPOSE: IDocCheckerProtocol — invariant auditor contract for the doc-rules feature
///
/// One trait for the single capability seam: audit Markdown documents against
/// the invariants that define the document chain. Each method reports a list of
/// findings; the agent behind the aggregate invokes them all and returns a
/// combined list.
use crate::taxonomy_doc_rules_request::DocRequest;
use crate::taxonomy_doc_rules_response::DocResponse;

/// Capability contract for doc-rules: invariant audit over Markdown files.
pub trait IDocCheckerProtocol: Send + Sync {
    /// Run every doc invariant over the documents described in *request*,
    /// returning an ordered list of findings.
    fn audit(&self, request: DocRequest) -> DocResponse;
}
