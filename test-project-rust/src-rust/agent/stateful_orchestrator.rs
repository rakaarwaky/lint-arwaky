// This agent orchestrator file violates AES021 (agent-role-violation)
// because agent orchestrators must be stateless, but this holds internal state.
use crate::taxonomy::removal_types::RemovalType;
use crate::contract::removal_io::IRemovalIO;

pub struct StatefulOrchestrator {
    // Non-stateless state violation!
    pub execution_counter: u32,
    pub last_processed_id: String,
}

impl StatefulOrchestrator {
    pub fn process(&mut self) {
        self.execution_counter += 1;
    }
}
