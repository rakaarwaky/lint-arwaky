// AES205: Circular Import violation - creates a circular dependency between modules
pub struct CycleStart;

impl CycleStart {
    pub fn new() -> Self {
        Self
    }
}