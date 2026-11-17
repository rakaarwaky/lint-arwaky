// PURPOSE: DocResponse — response payload for the doc-rules aggregate

use crate::taxonomy_doc_rules_request::DocFinding;

/// Result of a doc invariant audit.
pub enum DocResponse {
    /// Every finding across all inspected documents, plus the audit manifest
    /// — which documents were read, and which required documents were absent
    /// (missing or unreadable) — so a consumer can report coverage, not just
    /// violations.
    Findings {
        /// Every finding across all inspected documents.
        findings: Vec<DocFinding>,
        /// Paths (relative to the audit root) of every document the audit
        /// actually read and checked.
        audited: Vec<String>,
        /// Required root documents that were absent — missing from disk or
        /// present but unreadable — and therefore never audited.
        absent: Vec<String>,
    },
}

impl DocResponse {
    /// Total number of findings across all documents.
    pub fn count(&self) -> usize {
        match self {
            Self::Findings { findings, .. } => findings.len(),
        }
    }

    /// Whether every document passed the invariants.
    pub fn is_clean(&self) -> bool {
        matches!(self, Self::Findings { findings, .. } if findings.is_empty())
    }

    /// Consume and return the flat findings list.
    pub fn into_findings(self) -> Vec<DocFinding> {
        match self {
            Self::Findings { findings, .. } => findings,
        }
    }
}
