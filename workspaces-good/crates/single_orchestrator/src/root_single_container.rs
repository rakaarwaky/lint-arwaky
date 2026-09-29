use crate::agent_single_orchestrator::SingleGoalOrchestrator;
use crate::capabilities_single_checker::SingleChecker;
use calculator_shared::single_orchestrator::contract_single_protocol::ISingleRunnerAggregate;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct SingleContainer {
    orchestrator: SingleGoalOrchestrator,
}

// ─── Block 2: Wiring & Factory ────────────────────────────

impl SingleContainer {
    pub fn new() -> Self {
        let orchestrator = SingleGoalOrchestrator::new(Arc::new(SingleChecker));
        Self { orchestrator }
    }

    pub fn orchestrator(&self) -> &dyn ISingleRunnerAggregate {
        &self.orchestrator
    }
}

impl Default for SingleContainer {
    fn default() -> Self {
        Self::new()
    }
}
