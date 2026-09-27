// PURPOSE: IDocCheckerProtocol — invariant auditor contract for the doc-rules feature
///
/// One trait for the single capability seam: audit Markdown documents against
/// the invariants that define the document chain. Each method reports a list of
/// findings; the agent behind the aggregate invokes them all and returns a
/// combined list.
use crate::doc_rules::taxonomy_doc_request::DocRequest;
use crate::doc_rules::taxonomy_doc_response::DocResponse;

/// Capability contract for doc-rules: invariant audit over Markdown files.
pub trait IDocCheckerProtocol: Send + Sync {
    /// Run every doc invariant over the documents described in *request*,
    /// returning an ordered list of findings.
    ///
    /// The list is sorted by (path, code, message) so the output is stable.
    fn audit(&self, request: DocRequest) -> DocResponse;
}

/// Agent contract for doc-rules: accepts a request and returns findings.
/// The agent orchestrates one or more capability protocols; this trait
/// exposes the execution seam.
pub trait IDocAuditProtocol: Send + Sync {
    /// Execute the audit over the documents described in *request*.
    fn audit(&self, request: DocRequest) -> DocResponse;
}
