// AES505: Agent Orphan violation - this agent implements an aggregate but is never called by any surface
use crate::taxonomy::aggregates::aggregate_processor::AggregateProcessor;

pub struct OrphanAgent;

impl OrphanAgent {
    pub fn new() -> Self {
        Self
    }
}

// This implements the AggregateProcessor trait but is never actually called by any surface
// making it an orphan according to AES505 rules
impl AggregateProcessor for OrphanAgent {
    fn process(&self, data: &str) -> String {
        data.to_uppercase()
    }
}