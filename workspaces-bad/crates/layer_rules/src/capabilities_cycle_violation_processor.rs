// AES205: circular dependency — capabilities imports from contract
use crate::contract::base_aggregate::BaseAggregate;

pub struct CycleViolationProcessor;

impl CycleViolationProcessor {
    pub fn use_base(&self) -> BaseAggregate {
        BaseAggregate
    }
}
