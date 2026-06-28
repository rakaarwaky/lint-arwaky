// AES201: Forbidden Import violation - taxonomy file importing from forbidden agent layer
use crate::di_containers::agent_orphan_aggregate_orchestrator::AgentOrphanAggregateOrchestrator;

pub struct ForbiddenImportEntity {
    pub name: String,
}

impl ForbiddenImportEntity {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}