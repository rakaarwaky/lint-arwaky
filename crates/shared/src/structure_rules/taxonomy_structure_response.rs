// PURPOSE: StructureResponse — response payload for the structure-rules aggregate

use crate::structure_rules::taxonomy_structure_request::StructureFinding;

/// Result of a structure invariant audit.
pub enum StructureResponse {
    /// Every finding across all inspected folders.
    Findings { findings: Vec<StructureFinding> },
}

impl StructureResponse {
    /// Total number of findings across all folders.
    pub fn count(&self) -> usize {
        match self {
            Self::Findings { findings } => findings.len(),
        }
    }

    /// Whether every folder passed the invariants.
    pub fn is_clean(&self) -> bool {
        matches!(self, Self::Findings { findings } if findings.is_empty())
    }

    /// Consume and return the flat findings list.
    pub fn into_findings(self) -> Vec<StructureFinding> {
        match self {
            Self::Findings { findings } => findings,
        }
    }
}
