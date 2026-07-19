// BAD: Data class defined in capabilities layer (AES201)
pub struct OrphanResult {
    is_orphan: bool,
    reason: String,
}

pub struct CapabilitiesOrphanAnalyzer;

impl CapabilitiesOrphanAnalyzer {
    fn analyze(&self) -> OrphanResult {
        OrphanResult { is_orphan: true, reason: String::new() }
    }
}
