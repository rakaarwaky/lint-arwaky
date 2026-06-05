// This surface file violates AES018 (surface-hierarchy-violation)
// because it is located in the surfaces layer but is omitted from surfaces/mod.rs barrel file.
use crate::agent::orchestrator::AgentOrchestrator;
use crate::taxonomy::removal_types::RemovalType;

pub struct OrphanSurfaceRouter {
    pub active: bool,
}

impl OrphanSurfaceRouter {
    pub fn handle_request(&self) -> bool {
        self.active
    }
}
