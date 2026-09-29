// PURPOSE: IDocCheckerProtocol — invariant auditor contract for the doc-rules feature
//
// Single capability-seam trait for the doc-rules feature. The agent behind the
// aggregate invokes this protocol to audit Markdown documents against the invariants
// that define the document chain. Each method reports a list of findings; the
// list is sorted by (path, code, message) so the output is stable.
use crate::doc_rules::taxonomy_doc_request::DocRequest;
use crate::doc_rules::taxonomy_doc_response::DocResponse;

/// Capability contract for doc-rules: invariant audit over Markdown files.
pub trait IDocCheckerProtocol: Send + Sync {
    /// Run every doc invariant over the documents described in *request*,
    /// returning an ordered list of findings.
    fn audit(&self, request: DocRequest) -> DocResponse;
}
