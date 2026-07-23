// AES503: Orphan capabilities file - not exported in lib.rs, no inbound imports
pub struct OrphanCapabilitiesChecker;

impl OrphanCapabilitiesChecker {
    pub fn check(&self) -> bool {
        true
    }
}
