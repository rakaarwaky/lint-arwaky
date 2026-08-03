// AES205: circular dependency — contract imports from capabilities
use crate::capabilities::cycle_violation_processor::CycleViolationProcessor;

pub struct ContractCycleViolationAggregate;

impl ContractCycleViolationAggregate {
    pub fn use_processor(&self) -> CycleViolationProcessor {
        CycleViolationProcessor
    }
}
