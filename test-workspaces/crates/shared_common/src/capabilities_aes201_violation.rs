// AES201: capabilities importing from agent (forbidden layer)
use crate::agent::agent_large_orchestrator::LargeOrchestrator;

pub struct ForbiddenImportChecker;

impl ForbiddenImportChecker {
    pub fn check(&self) -> bool {
        let _o = LargeOrchestrator::new();
        true
    }
}
