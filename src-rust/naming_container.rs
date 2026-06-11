// PURPOSE: NamingContainer — wiring for naming-rules feature (root layer, wiring only)
use std::sync::Arc;
use crate::naming_rules::capabilities_naming_checker::ArchNamingChecker;
use crate::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;

pub struct NamingContainer {
    checker: ArchNamingChecker,
}

impl NamingContainer {
    pub fn new() -> Self {
        Self {
            checker: ArchNamingChecker::new(),
        }
    }

    pub fn checker(&self) -> &ArchNamingChecker {
        &self.checker
    }

    pub fn orchestrator(&self) -> Arc<dyn INamingRunnerAggregate> {
        Arc::new(crate::naming_rules::agent_naming_orchestrator::NamingOrchestrator::new())
    }
}
