// Agent: Orchestrator container
// Referenced by surface files for AES019/AES022 tests

use crate::taxonomy::removal_types::RemovalType;

pub struct AgentOrchestrator {
    pub removal_type: RemovalType,
}

impl AgentOrchestrator {
    pub fn new() -> Self {
        Self {
            removal_type: RemovalType::Background,
        }
    }
}
