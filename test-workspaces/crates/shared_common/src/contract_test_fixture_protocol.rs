// PURPOSE: TestAes201Fixture — test file for AES201 forbidden import detection
use capabilities::cycle_violation_processor::CycleViolationProcessor;

pub struct TestAes201Entity {
    pub value: i32,
}

impl TestAes201Entity {
    pub fn new(value: i32) -> Self {
        Self { value }
    }
}