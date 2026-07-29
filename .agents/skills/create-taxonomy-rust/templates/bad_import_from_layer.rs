// BAD: Taxonomy importing layer code (AES201)
use crate::capabilities_orphan_analyzer::OrphanAnalyzer; // BAD

pub struct OrphanResult {
    is_orphan: bool,
    reason: String,
}
