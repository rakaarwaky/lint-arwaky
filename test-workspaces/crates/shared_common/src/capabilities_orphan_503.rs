// AES503: Capabilities Orphan violation - this capability is not wired in any container AND unreachable
pub struct OrphanCapabilitiesProcessor;

impl OrphanCapabilitiesProcessor {
    pub fn new() -> Self {
        Self
    }

    pub fn process(&self, _input: &str) -> String {
        "processed".to_string()
    }
}