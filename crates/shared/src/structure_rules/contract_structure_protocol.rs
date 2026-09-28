// PURPOSE: IStructureAuditProtocol — structural auditor contract for AES701–AES703
///
/// One trait for the single capability seam: audit folder layout against the
/// invariants that define AES folder structure. Each method reports a list of
/// findings; the agent behind the aggregate invokes them all and returns a
/// combined list.
use crate::structure_rules::taxonomy_structure_request::StructureRequest;
use crate::structure_rules::taxonomy_structure_response::StructureResponse;

/// Capability contract for structure-rules: folder-layout audit.
pub trait IStructureAuditProtocol: Send + Sync {
    /// Run every structural invariant over the folders under *request*,
    /// returning an ordered list of findings.
    ///
    /// The list is sorted by (path, code, message) so the output is stable.
    fn audit(&self, request: StructureRequest) -> StructureResponse;
}
