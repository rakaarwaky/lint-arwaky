// PURPOSE: DocResponse — response payload for the doc-rules aggregate

use crate::doc_rules::taxonomy_doc_request::DocFinding;

/// Result of a doc invariant audit.
pub enum DocResponse {
    /// Every finding across all inspected documents.
    Findings { findings: Vec<DocFinding> },
}

impl DocResponse {
    /// Total number of findings across all documents.
    pub fn count(&self) -> usize {
        match self {
            Self::Findings { findings } => findings.len(),
        }
    }

    /// Whether every document passed the invariants.
    pub fn is_clean(&self) -> bool {
        matches!(self, Self::Findings { findings } if findings.is_empty())
    }

    /// Consume and return the flat findings list.
    pub fn into_findings(self) -> Vec<DocFinding> {
        match self {
            Self::Findings { findings } => findings,
        }
    }
}
