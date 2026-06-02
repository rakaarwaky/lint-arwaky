// This surface file violates AES019 and AES022 (passive-surface-violation)
// because it contains complex nested decision logic inside a passive boundary.
use crate::agent::orchestrator::AgentOrchestrator;
use crate::taxonomy::removal_types::RemovalType;

pub struct ComplexLogicRouter {
    pub enabled: bool,
}

impl ComplexLogicRouter {
    pub fn process_order(&self, items: Vec<i32>) -> i32 {
        let mut sum = 0;
        if self.enabled {
            for item in items {
                if item > 10 {
                    for i in 0..5 {
                        if i % 2 == 0 {
                            sum += item * i;
                        }
                    }
                }
            }
        }
        sum
    }
}
