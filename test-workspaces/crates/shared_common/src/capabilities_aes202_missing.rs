// AES202: capabilities missing mandatory taxonomy import - only imports from agent
use crate::agent::agent_large_orchestrator::LargeOrchestrator;

pub struct MandatoryMissingChecker;

impl MandatoryMissingChecker {
    pub fn check(&self) -> bool {
        let _o = LargeOrchestrator::new();
        true
    }
}
