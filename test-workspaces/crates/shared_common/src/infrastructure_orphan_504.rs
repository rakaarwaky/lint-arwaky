// AES504: Infrastructure Orphan violation - this infrastructure adapter is not wired in any container AND unreachable
pub struct OrphanInfraAdapter;

impl OrphanInfraAdapter {
    pub fn new() -> Self {
        Self
    }

    pub fn adapt(&self, _data: &str) -> String {
        "adapted".to_string()
    }
}