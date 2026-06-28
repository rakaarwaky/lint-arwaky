// AES205: Circular Import violation - creates a circular dependency between modules
pub struct CycleEnd;

impl CycleEnd {
    pub fn new() -> Self {
        Self
    }
}